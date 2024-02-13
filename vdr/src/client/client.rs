use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};
use serde_json::Value;

use log::warn;
use log_derive::{logfn, logfn_inputs};

use crate::{
    client::{
        implementation::web3::{client::Web3Client, contract::Web3Contract},
        Client, Contract, QuorumHandler,
    },
    error::{VdrError, VdrResult},
    types::{
        Block, ContractConfig, ContractSpec, EventLog, EventQuery, PingStatus, Transaction,
        TransactionType,
    },
    Address, BlockDetails, QuorumConfig,
};

pub struct LedgerClient {
    chain_id: u64,
    client: Box<dyn Client>,
    contracts: HashMap<String, Box<dyn Contract>>,
    quorum_handler: Option<QuorumHandler>,
}

impl LedgerClient {
    /// Create client interacting with ledger
    ///
    /// # Params
    ///  - `chain_id` - chain id of network (chain ID is part of the transaction signing process to protect against transaction replay attack)
    ///  - `rpc_node` - string - RPC node endpoint
    ///  - `contract_configs` - [ContractSpec] specifications for contracts  deployed on the network
    ///  - `quorum_config` - Option<[QuorumConfig]> quorum configuration. Can be None if quorum check is not needed
    ///
    /// # Returns
    ///  client to use for building and sending transactions
    #[logfn(Info)]
    #[logfn_inputs(Debug)]
    pub fn new(
        chain_id: u64,
        rpc_node: &str,
        contract_configs: &[ContractConfig],
        quorum_config: Option<&QuorumConfig>,
    ) -> VdrResult<LedgerClient> {
        let client = Box::new(Web3Client::new(rpc_node)?);

        let contracts = Self::init_contracts(&client, contract_configs)?;

        let quorum_handler = match quorum_config {
            Some(quorum_config) => Some(QuorumHandler::new(quorum_config.clone())?),
            None => None,
        };

        let ledger_client = LedgerClient {
            chain_id,
            client,
            contracts,
            quorum_handler,
        };
        Ok(ledger_client)
    }

    /// Ping Ledger.
    ///
    /// # Returns
    ///  ping status
    #[logfn(Info)]
    #[logfn_inputs(Debug)]
    pub async fn ping(&self) -> VdrResult<PingStatus> {
        match self.client.get_block(None).await {
            Ok(block) => Ok(PingStatus::ok(block.number, block.timestamp)),
            Err(err) => Ok(PingStatus::err(err.to_string().as_str())),
        }
    }

    /// Submit prepared transaction to the ledger
    ///     Depending on the transaction type Write/Read ethereum methods will be used
    ///
    /// #Params
    ///  `transaction` - transaction to submit
    ///
    /// #Returns
    ///  transaction execution result:
    ///    depending on the type it will be either result bytes or block hash
    #[logfn(Info)]
    #[logfn_inputs(Debug)]
    pub async fn submit_transaction(&self, transaction: &Transaction) -> VdrResult<Vec<u8>> {
        let result = match transaction.type_ {
            TransactionType::Read => {
                self.client
                    .call_transaction(transaction.to.as_ref(), &transaction.data)
                    .await
            }
            TransactionType::Write => self.client.submit_transaction(&transaction.encode()?).await,
        }?;

        if let Some(quorum_handler) = &self.quorum_handler {
            quorum_handler.check(transaction, &result).await?;
        };

        Ok(result)
    }

    /// Submit prepared events query to the ledger
    ///
    /// #Params
    ///  `query` - events query to submit
    ///
    /// #Returns
    ///  log events received from the ledger
    #[logfn(Info)]
    #[logfn_inputs(Debug)]
    pub async fn query_events(&self, query: &EventQuery) -> VdrResult<Vec<EventLog>> {
        let events = self.client.query_events(query).await?;
        // TODO: Check quorum for events
        Ok(events)
    }

    /// Get receipt for the given block hash
    ///
    /// # Params
    ///  `transaction` - transaction to submit
    ///
    /// # Returns
    ///  receipt for the given block
    #[logfn(Info)]
    #[logfn_inputs(Debug)]
    pub async fn get_receipt(&self, hash: &[u8]) -> VdrResult<String> {
        self.client.get_receipt(hash).await
    }

    /// Get a number of transactions sent by the given account address
    ///
    /// # Params
    ///  `address` - target account address
    ///
    /// # Returns
    ///  number of sent transaction
    #[logfn(Info)]
    #[logfn_inputs(Debug)]
    pub(crate) async fn get_transaction_count(&self, address: &Address) -> VdrResult<u64> {
        self.client.get_transaction_count(address).await
    }

    pub(crate) fn contract(&self, name: &str) -> VdrResult<&dyn Contract> {
        self.contracts
            .get(name)
            .map(|contract| contract.as_ref())
            .ok_or_else(|| {
                let vdr_error = VdrError::ContractInvalidName(name.to_string());

                warn!("Error during getting contract: {:?}", vdr_error);

                vdr_error
            })
    }

    pub(crate) fn chain_id(&self) -> u64 {
        self.chain_id
    }

    #[logfn(Info)]
    #[logfn_inputs(Debug)]
    fn init_contracts(
        client: &Web3Client,
        contract_configs: &[ContractConfig],
    ) -> VdrResult<HashMap<String, Box<dyn Contract>>> {
        let mut contracts: HashMap<String, Box<dyn Contract>> = HashMap::new();
        for contract_config in contract_configs {
            let spec = match (
                contract_config.spec_path.as_ref(),
                contract_config.spec.as_ref(),
            ) {
                (Some(spec_path), None) => ContractSpec::from_file(spec_path)?,
                (None, Some(spec)) => spec.clone(),
                (Some(_), Some(_)) => {
                    return Err(VdrError::ContractInvalidSpec(
                        "Either `spec_path` or `spec` must be provided".to_string(),
                    ));
                }
                (None, None) => {
                    return Err(VdrError::ContractInvalidSpec(
                        "Either `spec_path` or `spec` must be provided".to_string(),
                    ));
                }
            };

            let contract = Web3Contract::new(client, &contract_config.address, &spec)?;
            contracts.insert(spec.name.clone(), Box::new(contract));
        }

        Ok(contracts)
    }

    #[logfn(Info)]
    #[logfn_inputs(Debug)]
    pub(crate) async fn get_block(&self, block: Option<&Block>) -> VdrResult<BlockDetails> {
        self.client
            .get_block(block.map(|block| block.value()))
            .await
    }
}

impl Debug for LedgerClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"LedgerClient {{ chain_id: {} }}"#, self.chain_id)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        client::MockClient, types::transaction::test::read_transaction, utils::init_env_logger,
    };
    use once_cell::sync::Lazy;
    use std::{env, fs};

    pub const CHAIN_ID: u64 = 1337;
    pub const RPC_NODE_ADDRESS: &str = "http://127.0.0.1:8545";
    pub const CLIENT_NODE_ADDRESSES: [&str; 4] = [
        "http://127.0.0.1:21001",
        "http://127.0.0.1:21002",
        "http://127.0.0.1:21003",
        "http://127.0.0.1:21004",
    ];
    pub const DEFAULT_NONCE: u64 = 0;
    pub const INVALID_ADDRESS: &str = "123";


    pub static TRUSTEE_ACCOUNT: Lazy<Address> =
        Lazy::new(|| Address::from("0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5"));

    pub static TEST_ACCOUNT: Lazy<Address> =
        Lazy::new(|| Address::from("0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5"));

    fn build_contract_path(contract_path: &str) -> String {
        let mut cur_dir = env::current_dir().unwrap();
        cur_dir.push(".."); // project root directory
        cur_dir.push(contract_path);
        fs::canonicalize(&cur_dir)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    fn get_contract_conf(contract_name: &str) -> Option<Value> {
        let mut conf_json_path = env::current_dir().unwrap();
        conf_json_path.push("../config.json");
        let json_content = fs::read_to_string(conf_json_path).expect("config file reading failed");
        let conf: Value = serde_json::from_str(&json_content).expect("config file parsing failed");

        let contracts = conf
            .get("contracts").expect("'contracts' must be in the config file")
            .as_object().expect("'contracts' is expected to be an object");

        match contracts.get(contract_name) {
            Some(value) => {
                return Some(value.clone());
            },
            None => {
                return None;
            }
        }
    }

    pub fn get_contract_address(contract_name: &str) -> Option<Address> {
        match get_contract_conf(contract_name) {
            Some(value) => {
                let s = value
                    .get("address").expect("'address' not found in the contract config")
                    .as_str().expect("'address' must be a string");
                return Some(Address::from(s));
            },
            None => {
                return None;
            }
        }
    }

    pub fn get_contract_spec_path(contract_name: &str) -> Option<String> {
        match get_contract_conf(contract_name) {
            Some(value) => {
                let s = value
                    .get("spec_path").expect("'spec_path' not found in the contract config")
                    .as_str().expect("'spec_path' must be a string");
                return Some(String::from(s));
            },
            None => {
                return None;
            }
        }
    }

    fn contracts() -> Vec<ContractConfig> {
        let mut conf_json_path = env::current_dir().unwrap();
        conf_json_path.push("../config.json");

        let json_content = fs::read_to_string(conf_json_path).expect("config file reading failed");
        let conf: Value = serde_json::from_str(&json_content).expect("config file parsing failed");

        let contracts_conf = conf
            .get("contracts").expect("'contracts' must be in the config file")
            .as_object().expect("'contracts' is expected to be an object");

        let mut result = Vec::new();

        for contract_conf in contracts_conf.values() {
            let addr = contract_conf
                .get("address").expect("'address' not found in the contract config")
                .as_str().expect("'address' must be a string");

            let spec = contract_conf
                .get("spec_path").expect("'spec_path' not found in the contract config")
                .as_str().expect("'spec_path' must be a string");

            result.push(ContractConfig {
                address: String::from(addr),
                spec_path: Some(build_contract_path(spec)),
                spec: None,
            });
        }

        return result;
    }

    pub fn client() -> LedgerClient {
        LedgerClient::new(CHAIN_ID, RPC_NODE_ADDRESS, &contracts(), None).unwrap()
    }

    pub fn mock_client() -> LedgerClient {
        init_env_logger();
        let mut ledger_client = LedgerClient::new(
            CHAIN_ID,
            RPC_NODE_ADDRESS,
            &contracts(),
            Some(&QuorumConfig::default()),
        )
        .unwrap();

        let mut client = MockClient::new();
        client.expect_get_transaction_count().returning(|_| Ok(0));

        ledger_client.client = Box::new(client);
        ledger_client
    }

    pub fn mock_custom_client(client: Box<dyn Client>) -> LedgerClient {
        let mut ledger_client = LedgerClient::new(
            CHAIN_ID,
            RPC_NODE_ADDRESS,
            &contracts(),
            Some(&QuorumConfig::default()),
        )
        .unwrap();

        ledger_client.client = client;
        ledger_client
    }

    mod create {
        use crate::validator_control::test::VALIDATOR_CONTROL_NAME;
        use mockall::predicate::eq;
        use rstest::rstest;
        use serde_json::Value;

        use super::*;

        #[test]
        fn create_client_test() {
            client();
        }

        #[test]
        fn create_client_invalid_node_address() {
            let client_err = LedgerClient::new(CHAIN_ID, "..", &contracts(), None)
                .err()
                .unwrap();

            assert!(matches!(
                client_err,  | VdrError::ClientNodeUnreachable { .. }
            ));
        }

        #[rstest]
        #[case::invalid_contract_data(vec![ContractConfig {
            address: get_contract_address("validator_control").unwrap().to_string(),
            spec_path: None,
            spec: Some(ContractSpec {
                name: VALIDATOR_CONTROL_NAME.to_string(),
                abi: Value::String("".to_string()),
            }),
        }], VdrError::ContractInvalidInputData)]
        #[case::both_contract_path_and_spec_provided(vec![ContractConfig {
            address: get_contract_address("validator_control").unwrap().to_string(),
            spec_path: Some(build_contract_path(get_contract_spec_path("validator_control").unwrap().as_str())),
            spec: Some(ContractSpec {
                name: VALIDATOR_CONTROL_NAME.to_string(),
                abi: Value::Array(vec ! []),
            }),
        }], VdrError::ContractInvalidSpec("Either `spec_path` or `spec` must be provided".to_string()))]
        #[case::non_existent_spec_path(vec![ContractConfig {
            address: get_contract_address("validator_control").unwrap().to_string(),
            spec_path: Some(build_contract_path("")),
            spec: None,
        }], VdrError::ContractInvalidSpec("Unable to read contract spec file. Err: \"Is a directory (os error 21)\"".to_string()))]
        #[case::empty_contract_spec(vec![ContractConfig {
            address: get_contract_address("validator_control").unwrap().to_string(),
            spec_path: None,
            spec: None,
        }], VdrError::ContractInvalidSpec("Either `spec_path` or `spec` must be provided".to_string()))]
        fn test_create_client_errors(
            #[case] contract_config: Vec<ContractConfig>,
            #[case] expected_error: VdrError,
        ) {
            let client_err = LedgerClient::new(CHAIN_ID, RPC_NODE_ADDRESS, &contract_config, None)
                .err()
                .unwrap();

            assert_eq!(client_err, expected_error);
        }

        #[rstest]
        #[case::empty_recipient_address("", VdrError::ClientInvalidTransaction("Invalid transaction target address \"0x\"".to_string()))]
        #[case::invalid_recipient_address(INVALID_ADDRESS, VdrError::ClientInvalidTransaction("Invalid transaction target address \"0x123\"".to_string()))]
        async fn call_transaction_various_recipient_addresses(
            #[case] recipient_address: &str,
            #[case] expected_error: VdrError,
        ) {
            let transaction = Transaction {
                to: Address::from(recipient_address),
                ..read_transaction()
            };
            let client = client();

            let error = client.submit_transaction(&transaction).await.unwrap_err();

            assert_eq!(error, expected_error);
        }

        #[async_std::test]
        async fn get_receipt_invalid_transaction_hash() {
            let client = client();
            let txn_hash = vec![1; 4];

            let receipt_err = client.get_receipt(&txn_hash).await.unwrap_err();

            assert!(matches!(
                receipt_err,  | VdrError::CommonInvalidData { .. }
            ));
        }

        #[async_std::test]
        async fn get_receipt_transaction_does_not_exist() {
            let mut client_mock = MockClient::new();
            let txn_hash = vec![1; 32];
            client_mock
                .expect_get_receipt()
                .with(eq(txn_hash.clone()))
                .returning(|_| {
                    Err(VdrError::ClientInvalidResponse(
                        "Missing transaction receipt".to_string(),
                    ))
                });

            let client = mock_custom_client(Box::new(client_mock));

            let receipt_err = client.get_receipt(&txn_hash).await.unwrap_err();

            assert!(matches!(
                receipt_err,  | VdrError::ClientInvalidResponse { .. }
            ));
        }

        #[async_std::test]
        async fn get_receipt_positive() {
            let mut client_mock = MockClient::new();
            let txn_hash = vec![1; 32];
            client_mock
                .expect_get_receipt()
                .with(eq(txn_hash.clone()))
                .returning(|_| Ok("".to_string()));

            let client = mock_custom_client(Box::new(client_mock));

            client.get_receipt(&txn_hash).await.unwrap();
        }

        #[async_std::test]
        async fn get_transaction_count_invalid_address() {
            let client = client();

            let get_nonce_err = client
                .get_transaction_count(&Address::from(INVALID_ADDRESS))
                .await
                .unwrap_err();

            assert!(matches!(
                get_nonce_err,  | VdrError::ClientInvalidTransaction { .. }
            ));
        }

        #[async_std::test]
        async fn get_contract_does_not_exist() {
            let client = client();

            let contract_err = client.contract(INVALID_ADDRESS).err().unwrap();

            assert!(matches!(
                contract_err,  | VdrError::ContractInvalidName { .. }
            ));
        }
    }

    #[cfg(feature = "ledger_test")]
    mod ping {
        use super::*;
        use crate::types::Status;

        #[async_std::test]
        async fn client_ping_test() {
            let client = client();
            match client.ping().await.unwrap().status {
                Status::Ok { .. } => {}
                Status::Err { .. } => assert!(false, "Ping status expected to be `Ok`."),
            }
        }

        #[async_std::test]
        async fn client_ping_wrong_node_test() {
            let wrong_node_address = "http://127.0.0.1:1111";
            let client = LedgerClient::new(
                CHAIN_ID,
                wrong_node_address,
                &contracts(),
                Some(&QuorumConfig::default()),
            )
            .unwrap();
            match client.ping().await.unwrap().status {
                Status::Err { .. } => {}
                Status::Ok { .. } => assert!(false, "Ping status expected to be `Err`."),
            }
        }
    }
}
