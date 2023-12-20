use std::{collections::HashMap, sync::Arc};

use log::{info, trace, warn};
use web3::types::H256;

use crate::{
    client::{
        implementation::web3::{client::Web3Client, contract::Web3Contract},
        Client, Contract,
    },
    error::{VdrError, VdrResult},
    types::{ContractConfig, ContractSpec, PingStatus, Transaction, TransactionType},
    Address, QuorumConfig,
};

use super::quorum::*;

pub struct LedgerClient {
    chain_id: u64,
    clients: Vec<Arc<Box<dyn Client>>>,
    contracts: HashMap<String, Box<dyn Contract>>,
}

impl LedgerClient {
    /// Create client interacting with ledger
    ///
    /// # Params
    ///  - chain_id chain id of network (chain ID is part of the transaction signing process to protect against transaction replay attack)
    ///  - param node_address: string - RPC node endpoint
    ///  - param contract_specs: Vec<ContractSpec> - specifications for contracts  deployed on the network
    ///
    /// # Returns
    ///  client to use for building and sending transactions
    pub fn new(
        chain_id: u64,
        mut node_addresses: Vec<&str>,
        contract_configs: &[ContractConfig],
    ) -> VdrResult<LedgerClient> {
        let rpc_node_address = node_addresses.remove(0);

        trace!(
            "Started creating new LedgerClient. Chain id: {}, node address: {}",
            chain_id,
            rpc_node_address
        );

        let mut clients = node_addresses
            .into_iter()
            .map(|node_address| {
                let client: Box<dyn Client> = Box::new(Web3Client::new(node_address)?);
                Ok(Arc::new(client))
            })
            .collect::<Result<Vec<_>, VdrError>>()
            .map_err(|_| VdrError::ClientNodeUnreachable)?;

        let client = Box::new(Web3Client::new(rpc_node_address)?);
        let contracts = Self::init_contracts(&client, contract_configs)?;

        clients.insert(0, Arc::new(client));

        let ledger_client = LedgerClient {
            chain_id,
            clients,
            contracts,
        };

        info!(
            "Created new LedgerClient. Chain id: {}, node address: {}",
            chain_id, rpc_node_address
        );

        Ok(ledger_client)
    }

    /// Ping Ledger.
    ///
    /// # Returns
    ///  ping status
    pub async fn ping(&self) -> VdrResult<PingStatus> {
        self.clients[0].ping().await
    }

    /// Submit prepared transaction to the ledger
    ///     Depending on the transaction type Write/Read ethereum methods will be used
    ///
    /// #Params
    ///  transaction - transaction to submit
    ///
    /// #Returns
    ///  transaction execution result:
    ///    depending on the type it will be either result bytes or block hash
    pub async fn submit_transaction(
        &self,
        transaction: &Transaction,
        quorum_config: &QuorumConfig,
    ) -> VdrResult<Vec<u8>> {
        match transaction.type_ {
            TransactionType::Read => {
                let call_result = self.clients[0].call_transaction(transaction).await?;

                read_quorum::quorum_check(&self.clients, transaction, &call_result, quorum_config)
                    .await?;

                Ok(call_result)
            }
            TransactionType::Write => {
                let submit_result = self.clients[0].submit_transaction(transaction).await?;

                write_quorum::quorum_check(
                    &self.clients,
                    H256::from_slice(&submit_result),
                    quorum_config,
                )
                .await?;

                Ok(submit_result)
            }
        }
    }

    /// Get receipt for the given block hash
    ///
    /// # Params
    ///  transaction - transaction to submit
    ///
    /// # Returns
    ///  receipt for the given block
    pub async fn get_receipt(&self, hash: &[u8]) -> VdrResult<String> {
        self.clients[0].get_receipt(hash).await
    }

    pub(crate) async fn get_transaction_count(&self, address: &Address) -> VdrResult<[u64; 4]> {
        self.clients[0].get_transaction_count(address).await
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

    fn init_contracts(
        client: &Web3Client,
        contract_configs: &[ContractConfig],
    ) -> VdrResult<HashMap<String, Box<dyn Contract>>> {
        let mut contracts: HashMap<String, Box<dyn Contract>> = HashMap::new();
        for contract_config in contract_configs {
            let spec = match (contract_config.spec_path.as_ref(), contract_config.spec.as_ref()) {
                (Some(spec_path) , None) => ContractSpec::from_file(spec_path)?,
                (None , Some(spec)) => spec.clone(),
                (Some(_), Some(_)) => {
                    return Err(VdrError::ContractInvalidSpec("Either `spec_path` or `spec` must be provided".to_string()))
                }
                (None, None) => {
                    return Err(VdrError::ContractInvalidSpec("Either `spec_path` or `spec` must be provided".to_string()))
                }
            };

            let contract = Web3Contract::new(client, &contract_config.address, &spec)?;
            contracts.insert(spec.name.clone(), Box::new(contract));
        }

        Ok(contracts)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use once_cell::sync::Lazy;
    use std::{env, fs};
    use web3::types::Transaction as Web3Transaction;

    pub const CHAIN_ID: u64 = 1337;
    pub const CONTRACTS_SPEC_BASE_PATH: &str = "../smart_contracts/artifacts/contracts/";
    pub const DID_REGISTRY_ADDRESS: &str = "0x0000000000000000000000000000000000003333";
    pub const DID_REGISTRY_SPEC_PATH: &str = "did/IndyDidRegistry.sol/IndyDidRegistry.json";
    pub const SCHEMA_REGISTRY_ADDRESS: &str = "0x0000000000000000000000000000000000005555";
    pub const SCHEMA_REGISTRY_SPEC_PATH: &str = "cl/SchemaRegistry.sol/SchemaRegistry.json";
    pub const CRED_DEF_REGISTRY_ADDRESS: &str = "0x0000000000000000000000000000000000004444";
    pub const CRED_DEF_REGISTRY_SPEC_PATH: &str =
        "cl/CredentialDefinitionRegistry.sol/CredentialDefinitionRegistry.json";
    pub const VALIDATOR_CONTROL_ADDRESS: &str = "0x0000000000000000000000000000000000007777";
    pub const VALIDATOR_CONTROL_PATH: &str = "network/ValidatorControl.sol/ValidatorControl.json";
    pub const ROLE_CONTROL_ADDRESS: &str = "0x0000000000000000000000000000000000006666";
    pub const ROLE_CONTROL_PATH: &str = "auth/RoleControl.sol/RoleControl.json";
    pub const CLIENT_NODE_ADDRESSES: [&str; 5] = [
        "http://127.0.0.1:8545",
        "http://127.0.0.1:21001",
        "http://127.0.0.1:21002",
        "http://127.0.0.1:21003",
        "http://127.0.0.1:21004",
    ];

    pub static TRUSTEE_ACC: Lazy<Address> =
        Lazy::new(|| Address::new("0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5"));

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
                address: DID_REGISTRY_ADDRESS.to_string(),
                spec_path: Some(build_contract_path(DID_REGISTRY_SPEC_PATH)),
                spec: None,
            },
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
        ]
    }

    pub fn client() -> LedgerClient {
        LedgerClient::new(CHAIN_ID, CLIENT_NODE_ADDRESSES.to_vec(), &contracts()).unwrap()
    }

    pub const DEFAULT_NONCE: [u64; 4] = [0, 0, 0, 0];

    pub struct MockClient {}

    #[async_trait::async_trait]
    impl Client for MockClient {
        async fn get_transaction_count(&self, _address: &Address) -> VdrResult<[u64; 4]> {
            Ok([0, 0, 0, 0])
        }

        async fn submit_transaction(&self, _transaction: &Transaction) -> VdrResult<Vec<u8>> {
            todo!()
        }

        async fn call_transaction(&self, _transaction: &Transaction) -> VdrResult<Vec<u8>> {
            todo!()
        }

        async fn get_receipt(&self, _hash: &[u8]) -> VdrResult<String> {
            todo!()
        }

        async fn ping(&self) -> VdrResult<PingStatus> {
            todo!()
        }

        async fn get_transaction(
            &self,
            _transaction_hash: H256,
        ) -> VdrResult<Option<Web3Transaction>> {
            todo!()
        }
    }

    pub fn mock_client() -> LedgerClient {
        let mut client =
            LedgerClient::new(CHAIN_ID, CLIENT_NODE_ADDRESSES.to_vec(), &contracts()).unwrap();
        client.clients = vec![Arc::new(Box::new(MockClient {}))];
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
            let client =
                LedgerClient::new(CHAIN_ID, vec![wrong_node_address], &contracts()).unwrap();
            match client.ping().await.unwrap().status {
                Status::Err(_) => {}
                Status::Ok => assert!(false, "Ping status expected to be `Err`."),
            }
        }
    }
}
