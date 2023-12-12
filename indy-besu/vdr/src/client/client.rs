use std::collections::HashMap;

use log::{info, trace, warn};

use crate::{
    client::{
        implementation::web3::{client::Web3Client, contract::Web3Contract},
        Client, Contract,
    },
    error::{VdrError, VdrResult},
    types::{ContractConfig, ContractSpec, PingStatus, Transaction, TransactionType},
    Address,
};

pub struct LedgerClient {
    chain_id: u64,
    client: Box<dyn Client>,
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
        node_address: &str,
        contract_configs: &[ContractConfig],
    ) -> VdrResult<LedgerClient> {
        trace!(
            "Started creating new LedgerClient. Chain id: {}, node address: {}",
            chain_id,
            node_address
        );

        let client = Web3Client::new(node_address)?;
        let contracts = Self::init_contracts(&client, contract_configs)?;

        let ledger_client = LedgerClient {
            chain_id,
            client: Box::new(client),
            contracts,
        };

        info!(
            "Created new LedgerClient. Chain id: {}, node address: {}",
            chain_id, node_address
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
        match transaction.type_ {
            TransactionType::Read => self.client.call_transaction(transaction).await,
            TransactionType::Write => self.client.submit_transaction(transaction).await,
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
        self.client.get_receipt(hash).await
    }

    pub(crate) async fn get_transaction_count(&self, address: &Address) -> VdrResult<[u64; 4]> {
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

    fn init_contracts(
        client: &Web3Client,
        contract_configs: &[ContractConfig],
    ) -> VdrResult<HashMap<String, Box<dyn Contract>>> {
        let mut contracts: HashMap<String, Box<dyn Contract>> = HashMap::new();
        for contract_config in contract_configs {
            let contract_spec = ContractSpec::from_file(&contract_config.spec_path)?;
            let contract = Web3Contract::new(client, &contract_config.address, &contract_spec)?;
            contracts.insert(contract_spec.name.clone(), Box::new(contract));
        }

        Ok(contracts)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use once_cell::sync::Lazy;
    use std::{env, fs};

    pub const CHAIN_ID: u64 = 1337;
    pub const NODE_ADDRESS: &str = "http://127.0.0.1:8545";
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
                spec_path: build_contract_path(DID_REGISTRY_SPEC_PATH),
            },
            ContractConfig {
                address: SCHEMA_REGISTRY_ADDRESS.to_string(),
                spec_path: build_contract_path(SCHEMA_REGISTRY_SPEC_PATH),
            },
            ContractConfig {
                address: CRED_DEF_REGISTRY_ADDRESS.to_string(),
                spec_path: build_contract_path(CRED_DEF_REGISTRY_SPEC_PATH),
            },
            ContractConfig {
                address: ROLE_CONTROL_ADDRESS.to_string(),
                spec_path: build_contract_path(ROLE_CONTROL_PATH),
            },
        ]
    }

    pub fn client() -> LedgerClient {
        LedgerClient::new(CHAIN_ID, NODE_ADDRESS, &contracts()).unwrap()
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
    }

    pub fn mock_client() -> LedgerClient {
        let mut client = LedgerClient::new(CHAIN_ID, NODE_ADDRESS, &contracts()).unwrap();
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
            let client = LedgerClient::new(CHAIN_ID, wrong_node_address, &contracts()).unwrap();
            match client.ping().await.unwrap().status {
                Status::Err(_) => {}
                Status::Ok => assert!(false, "Ping status expected to be `Err`."),
            }
        }
    }
}
