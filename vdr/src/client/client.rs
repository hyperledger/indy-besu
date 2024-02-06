use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};

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
        client::MockClient, signer::basic_signer::test::basic_signer, types::EventLog, Role,
    };
    use async_trait::async_trait;
    use once_cell::sync::Lazy;
    use std::{env, fs, sync::RwLock};

    pub const CONTRACT_NAME_EXAMPLE: &str = "ValidatorControl";
    pub const CONTRACT_METHOD_EXAMPLE: &str = "addValidator";
    pub const VALIDATOR_LIST_BYTES: Lazy<Vec<u8>> = Lazy::new(|| {
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 145, 124, 173, 186, 206, 93,
            252, 225, 50, 185, 145, 115, 44, 108, 218, 155, 204, 91, 138, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 39, 169, 124, 154, 175, 4, 241, 143, 48, 20, 195, 46, 3, 109, 208, 172,
            118, 218, 95, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 206, 65, 47, 152, 131, 119, 227,
            31, 77, 15, 241, 45, 116, 223, 115, 181, 28, 66, 208, 202, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 152, 193, 51, 68, 150, 97, 74, 237, 73, 210, 232, 21, 38, 208, 137, 247, 38,
            79, 237, 156,
        ]
    });

    pub const CHAIN_ID: u64 = 1337;
    pub const CONTRACTS_SPEC_BASE_PATH: &str = "../smart_contracts/artifacts/contracts/";
    pub const SCHEMA_REGISTRY_SPEC_PATH: &str = "cl/SchemaRegistry.sol/SchemaRegistry.json";
    pub const CRED_DEF_REGISTRY_SPEC_PATH: &str =
        "cl/CredentialDefinitionRegistry.sol/CredentialDefinitionRegistry.json";
    pub const VALIDATOR_CONTROL_PATH: &str = "network/ValidatorControl.sol/ValidatorControl.json";
    pub const ROLE_CONTROL_PATH: &str = "auth/RoleControl.sol/RoleControl.json";
    pub const ETHR_DID_REGISTRY_PATH: &str =
        "did/EthereumExtDidRegistry.sol/EthereumExtDidRegistry.json";
    pub const RPC_NODE_ADDRESS: &str = "http://127.0.0.1:8545";
    pub const CLIENT_NODE_ADDRESSES: [&str; 4] = [
        "http://127.0.0.1:21001",
        "http://127.0.0.1:21002",
        "http://127.0.0.1:21003",
        "http://127.0.0.1:21004",
    ];
    pub const DEFAULT_NONCE: u64 = 0;
    pub static ACCOUNT_ROLES: [Role; 4] =
        [Role::Empty, Role::Trustee, Role::Steward, Role::Endorser];

    pub static SCHEMA_REGISTRY_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from("0x0000000000000000000000000000000000005555"));

    pub static CRED_DEF_REGISTRY_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from("0x0000000000000000000000000000000000004444"));

    pub static VALIDATOR_CONTROL_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from("0x0000000000000000000000000000000000007777"));

    pub static ROLE_CONTROL_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from("0x0000000000000000000000000000000000006666"));

    pub static ETHR_DID_REGISTRY_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from("0x0000000000000000000000000000000000018888"));

    pub static TRUSTEE_ACC: Lazy<Address> =
        Lazy::new(|| Address::from("0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5"));

    pub static IDENTITY_ACC: Lazy<Address> =
        Lazy::new(|| Address::from("0xb9059400dcd05158ffd8ca092937989dd27b3bdc"));

    fn build_contract_path(contract_path: &str) -> String {
        let mut cur_dir = env::current_dir().unwrap();
        cur_dir.push(CONTRACTS_SPEC_BASE_PATH);
        cur_dir.push(contract_path);
        fs::canonicalize(&cur_dir)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    fn contracts() -> Vec<ContractConfig> {
        vec![
            ContractConfig {
                address: SCHEMA_REGISTRY_ADDRESS.to_string(),
                spec_path: Some(build_contract_path(SCHEMA_REGISTRY_SPEC_PATH)),
                spec: None,
            },
            ContractConfig {
                address: CRED_DEF_REGISTRY_ADDRESS.to_string(),
                spec_path: Some(build_contract_path(CRED_DEF_REGISTRY_SPEC_PATH)),
                spec: None,
            },
            ContractConfig {
                address: VALIDATOR_CONTROL_ADDRESS.to_string(),
                spec_path: Some(build_contract_path(VALIDATOR_CONTROL_PATH)),
                spec: None,
            },
            ContractConfig {
                address: ROLE_CONTROL_ADDRESS.to_string(),
                spec_path: Some(build_contract_path(ROLE_CONTROL_PATH)),
                spec: None,
            },
            ContractConfig {
                address: ETHR_DID_REGISTRY_ADDRESS.to_string(),
                spec_path: Some(build_contract_path(ETHR_DID_REGISTRY_PATH)),
                spec: None,
            },
        ]
    }

    pub fn client() -> LedgerClient {
        LedgerClient::new(CHAIN_ID, RPC_NODE_ADDRESS, &contracts(), None).unwrap()
    }

    pub fn mock_client() -> LedgerClient {
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

    pub fn write_transaction() -> Transaction {
        let transaction = Transaction {
            type_: TransactionType::Write,
            from: Some(TRUSTEE_ACC.clone()),
            to: VALIDATOR_CONTROL_ADDRESS.clone(),
            nonce: Some(DEFAULT_NONCE.clone()),
            chain_id: CHAIN_ID,
            data: vec![],
            signature: RwLock::new(None),
            hash: None,
        };
        let signer = basic_signer();
        let sign_bytes = transaction.get_signing_bytes().unwrap();
        let signature = signer.sign(&sign_bytes, TRUSTEE_ACC.as_ref()).unwrap();
        transaction.set_signature(signature);

        transaction
    }

    pub fn read_transaction() -> Transaction {
        Transaction {
            type_: TransactionType::Read,
            from: None,
            to: VALIDATOR_CONTROL_ADDRESS.clone(),
            nonce: None,
            chain_id: CHAIN_ID,
            data: vec![],
            signature: RwLock::new(None),
            hash: None,
        }
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
        use mockall::predicate::eq;
        use serde_json::Value;

        use super::*;

        #[test]
        fn create_client_test() {
            client();
        }

        #[test]
        fn create_client_invalid_contract_data() {
            let contract_config = vec![ContractConfig {
                address: VALIDATOR_CONTROL_ADDRESS.to_string(),
                spec_path: None,
                spec: Some(ContractSpec {
                    name: CONTRACT_NAME_EXAMPLE.to_string(),
                    abi: Value::String("".to_string()),
                }),
            }];

            let client_err = LedgerClient::new(CHAIN_ID, RPC_NODE_ADDRESS, &contract_config, None)
                .err()
                .unwrap();

            assert!(matches!(
                client_err,  | VdrError::ContractInvalidInputData { .. }
            ));
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

        #[test]
        fn create_client_contract_path_and_spec_provided() {
            let contract_config = vec![ContractConfig {
                address: VALIDATOR_CONTROL_ADDRESS.to_string(),
                spec_path: Some(build_contract_path(VALIDATOR_CONTROL_PATH)),
                spec: Some(ContractSpec {
                    name: CONTRACT_NAME_EXAMPLE.to_string(),
                    abi: Value::Array(vec![]),
                }),
            }];

            let client_err = LedgerClient::new(CHAIN_ID, RPC_NODE_ADDRESS, &contract_config, None)
                .err()
                .unwrap();

            assert!(matches!(
                client_err,  | VdrError::ContractInvalidSpec { .. }
            ));
        }

        #[test]
        fn create_client_empty_contract_spec() {
            let contract_config = vec![ContractConfig {
                address: VALIDATOR_CONTROL_ADDRESS.to_string(),
                spec_path: None,
                spec: None,
            }];

            let client_err = LedgerClient::new(CHAIN_ID, RPC_NODE_ADDRESS, &contract_config, None)
                .err()
                .unwrap();

            assert!(matches!(
                client_err,  | VdrError::ContractInvalidSpec { .. }
            ));
        }

        #[async_std::test]
        async fn create_client_invalid_contract_address() {
            let contract_config = vec![ContractConfig {
                address: "123".to_string(),
                spec_path: Some(build_contract_path(VALIDATOR_CONTROL_PATH)),
                spec: None,
            }];

            let client_err = LedgerClient::new(CHAIN_ID, RPC_NODE_ADDRESS, &contract_config, None)
                .err()
                .unwrap();

            assert!(matches!(
                client_err,  | VdrError::CommonInvalidData { .. }
            ));
        }

        #[async_std::test]
        async fn call_transaction_empty_recipient_address() {
            let transaction = Transaction {
                to: Address::from(""),
                ..read_transaction()
            };
            let client = client();

            let submit_err = client.submit_transaction(&transaction).await.unwrap_err();

            assert!(matches!(
                submit_err,  | VdrError::ClientInvalidTransaction { .. }
            ));
        }

        #[async_std::test]
        async fn call_transaction_invalid_recipient_address() {
            let transaction = Transaction {
                to: Address::from("123"),
                ..read_transaction()
            };
            let client = client();

            let call_err = client.submit_transaction(&transaction).await.unwrap_err();

            assert!(matches!(
                call_err,  | VdrError::ClientInvalidTransaction { .. }
            ));
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
        async fn get_receipt_transcation_does_not_exist() {
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
                .get_transaction_count(&Address::from("123"))
                .await
                .unwrap_err();

            assert!(matches!(
                get_nonce_err,  | VdrError::ClientInvalidTransaction { .. }
            ));
        }

        #[async_std::test]
        async fn get_contract_does_not_exist() {
            let client = client();

            let contract_err = client.contract("123").err().unwrap();

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
