use std::collections::HashMap;

use log::{info, trace, warn};

use crate::{
    client::{
        implementation::web3::{client::Web3Client, contract::Web3Contract},
        Client, Contract, QuorumHandler,
    },
    error::{VdrError, VdrResult},
    types::{ContractConfig, ContractSpec, PingStatus, Transaction, TransactionType},
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
    ///  - chain_id chain id of network (chain ID is part of the transaction signing process to protect against transaction replay attack)
    ///  - param rpc_node: string - RPC node endpoint
    ///  - param contract_configs: [ContractSpec] - specifications for contracts  deployed on the network
    /// -  param: quorum_config: Option<QuorumConfig> - quorum configuration. Can be None if quorum is not needed
    ///
    /// # Returns
    ///  client to use for building and sending transactions
    pub fn new(
        chain_id: u64,
        rpc_node: &str,
        contract_configs: &[ContractConfig],
        quorum_config: Option<&QuorumConfig>,
    ) -> VdrResult<LedgerClient> {
        trace!(
            "Started creating new LedgerClient. Chain id: {}, node address: {}",
            chain_id,
            rpc_node
        );

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

        info!(
            "Created new LedgerClient. Chain id: {}, node address: {}",
            chain_id, rpc_node
        );

        Ok(ledger_client)
    }

    /// Ping Ledger.
    ///
    /// # Returns
    ///  ping status
    pub async fn ping(&self) -> VdrResult<PingStatus> {
        self.client.ping().await
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
    pub async fn submit_transaction(&self, transaction: &Transaction) -> VdrResult<Vec<u8>> {
        let result = match transaction.type_ {
            TransactionType::Read => self.client.call_transaction(transaction).await,
            TransactionType::Write => self.client.submit_transaction(transaction).await,
        }?;

        if let Some(quorum_handler) = &self.quorum_handler {
            quorum_handler.check(transaction, &result).await?;
        };

        Ok(result)
    }

    /// Get receipt for the given block hash
    ///
    /// # Params
    ///  transaction - transaction to submit
    ///
    /// # Returns
    ///  receipt for the given block
    pub async fn get_receipt(&self, hash: &[u8]) -> VdrResult<String> {
        self.client.get_receipt(hash).await
    }

    pub(crate) async fn get_transaction_count(&self, address: &Address) -> VdrResult<Vec<u64>> {
        let nonce = self.client.get_transaction_count(address).await?;
        Ok(nonce.to_vec())
    }

    pub(crate) fn contract(&self, name: &str) -> VdrResult<&dyn Contract> {
        self.contracts
            .get(name)
            .map(|contract| contract.as_ref())
            .ok_or_else(|| {
                let vdr_error = VdrError::ContractInvalidName {
                    msg: name.to_string(),
                };

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
                    return Err(VdrError::ContractInvalidSpec {
                        msg: "Either `spec_path` or `spec` must be provided".to_string(),
                    });
                }
                (None, None) => {
                    return Err(VdrError::ContractInvalidSpec {
                        msg: "Either `spec_path` or `spec` must be provided".to_string(),
                    });
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
    use async_trait::async_trait;
    use ethereum_types::H256;
    use once_cell::sync::Lazy;
    use std::{env, fs, ops::Deref};

    pub const CHAIN_ID: u64 = 1337;
    pub const CONTRACTS_SPEC_BASE_PATH: &str = "../smart_contracts/artifacts/contracts/";
    pub const DID_REGISTRY_SPEC_PATH: &str = "did/IndyDidRegistry.sol/IndyDidRegistry.json";
    pub const SCHEMA_REGISTRY_SPEC_PATH: &str = "cl/SchemaRegistry.sol/SchemaRegistry.json";
    pub const CRED_DEF_REGISTRY_SPEC_PATH: &str =
        "cl/CredentialDefinitionRegistry.sol/CredentialDefinitionRegistry.json";
    pub const VALIDATOR_CONTROL_PATH: &str = "network/ValidatorControl.sol/ValidatorControl.json";
    pub const ROLE_CONTROL_PATH: &str = "auth/RoleControl.sol/RoleControl.json";
    pub static DEFAULT_NONCE: Lazy<Vec<u64>> = Lazy::new(|| vec![0, 0, 0, 0]);

    pub static DID_REGISTRY_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from("0x0000000000000000000000000000000000003333"));

    pub static SCHEMA_REGISTRY_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from("0x0000000000000000000000000000000000005555"));

    pub static CRED_DEF_REGISTRY_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from("0x0000000000000000000000000000000000004444"));

    pub static VALIDATOR_CONTROL_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from("0x0000000000000000000000000000000000007777"));

    pub static ROLE_CONTROL_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from("0x0000000000000000000000000000000000006666"));

    pub static TRUSTEE_ACC: Lazy<Address> =
        Lazy::new(|| Address::from("0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5"));

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
                address: DID_REGISTRY_ADDRESS.deref().to_string(),
                spec_path: Some(build_contract_path(DID_REGISTRY_SPEC_PATH)),
                spec: None,
            },
            ContractConfig {
                address: SCHEMA_REGISTRY_ADDRESS.deref().to_string(),
                spec_path: Some(build_contract_path(SCHEMA_REGISTRY_SPEC_PATH)),
                spec: None,
            },
            ContractConfig {
                address: CRED_DEF_REGISTRY_ADDRESS.deref().to_string(),
                spec_path: Some(build_contract_path(CRED_DEF_REGISTRY_SPEC_PATH)),
                spec: None,
            },
            ContractConfig {
                address: VALIDATOR_CONTROL_ADDRESS.deref().to_string(),
                spec_path: Some(build_contract_path(VALIDATOR_CONTROL_PATH)),
                spec: None,
            },
            ContractConfig {
                address: ROLE_CONTROL_ADDRESS.deref().to_string(),
                spec_path: Some(build_contract_path(ROLE_CONTROL_PATH)),
                spec: None,
            },
        ]
    }

    pub fn client() -> LedgerClient {
        LedgerClient::new(
            CHAIN_ID,
            RPC_NODE_ADDRESS,
            &contracts(),
            Some(&QuorumConfig::default()),
        )
        .unwrap()
    }

    pub struct MockClient {}

    #[async_trait]
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
