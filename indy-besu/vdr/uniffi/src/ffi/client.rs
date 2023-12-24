use crate::{
    ffi::{
        error::VdrResult,
        transaction::Transaction,
        types::{ContractConfig, PingStatus},
    },
    VdrError,
};
use indy2_vdr::{ContractConfig as ContractConfig_, LedgerClient as LedgerClient_};

#[derive(uniffi::Object)]
pub struct LedgerClient {
    pub client: LedgerClient_,
}

#[uniffi::export(async_runtime = "tokio")]
impl LedgerClient {
    #[uniffi::constructor]
    pub fn new(
        chain_id: u64,
        node_address: String,
        contract_configs: Vec<ContractConfig>,
    ) -> VdrResult<LedgerClient> {
        let contract_configs: Vec<ContractConfig_> = contract_configs
            .into_iter()
            .map(ContractConfig::into)
            .collect();
        let client = LedgerClient_::new(chain_id, &node_address, &contract_configs)?;
        Ok(LedgerClient { client })
    }

    pub async fn ping(&self) -> VdrResult<PingStatus> {
        let ping = self.client.ping().await?;
        Ok(ping.into())
    }

    pub async fn submit_transaction(&self, transaction: &Transaction) -> VdrResult<Vec<u8>> {
        self.client
            .submit_transaction(&transaction.transaction)
            .await
            .map_err(VdrError::from)
    }

    pub async fn get_receipt(&self, hash: Vec<u8>) -> VdrResult<String> {
        self.client.get_receipt(&hash).await.map_err(VdrError::from)
    }
}
