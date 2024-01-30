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
        ContractConfig, ContractSpec, EventLog, EventQuery, PingStatus, Transaction,
        TransactionType,
    },
    Address, QuorumConfig,
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
        self.client.ping().await
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
}

impl Debug for LedgerClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"LedgerClient {{ chain_id: {} }}"#, self.chain_id)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::types::EventLog;
    use async_trait::async_trait;
    use once_cell::sync::Lazy;
    use std::{env, fs};

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
    pub const TEST_NETWORK: &str = "test";

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

    pub struct MockClient {}

    impl Debug for MockClient {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, r#"MockClient {{ }}"#)
        }
    }

    #[async_trait]
    impl Client for MockClient {
        async fn get_transaction_count(&self, _address: &Address) -> VdrResult<u64> {
            Ok(0)
        }

        async fn submit_transaction(&self, _transaction: &[u8]) -> VdrResult<Vec<u8>> {
            todo!()
        }

        async fn call_transaction(&self, _to: &str, _transaction: &[u8]) -> VdrResult<Vec<u8>> {
            todo!()
        }

        async fn query_events(&self, _query: &EventQuery) -> VdrResult<Vec<EventLog>> {
            todo!()
        }

        async fn get_receipt(&self, _hash: &[u8]) -> VdrResult<String> {
            todo!()
        }

        async fn ping(&self) -> VdrResult<PingStatus> {
            todo!()
        }

        async fn get_transaction(&self, _hash: &[u8]) -> VdrResult<Option<Transaction>> {
            todo!()
        }
    }

    pub fn mock_client() -> LedgerClient {
        let mut client = LedgerClient::new(
            CHAIN_ID,
            RPC_NODE_ADDRESS,
            &contracts(),
            Some(&QuorumConfig::default()),
        )
        .unwrap();
        client.client = Box::new(MockClient {});
        client
    }

    mod create {
        use super::*;

        #[test]
        fn create_client_test() {
            client();
        }
    }

    #[cfg(feature = "ledger_test")]
    mod ping {
        use super::*;
        use crate::types::Status;

        #[async_std::test]
        async fn client_ping_test() {
            let client = client();
            assert_eq!(PingStatus::ok(), client.ping().await.unwrap())
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
                Status::Ok => assert!(false, "Ping status expected to be `Err`."),
            }
        }
    }
}
