use wasm_bindgen::prelude::*;

use indy2_vdr::{ContractConfig, LedgerClient};

use crate::error::{JsResult, Result};
use crate::transaction::TransactionWrapper;

#[wasm_bindgen(js_name = LedgerClient)]
pub struct LedgerClientWrapper(pub(crate) LedgerClient);

#[wasm_bindgen(js_class = LedgerClient)]
impl LedgerClientWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(
        chain_id: u32,
        node_address: String,
        contract_configs: JsValue,
    ) -> Result<LedgerClientWrapper> {
        let contract_configs: Vec<ContractConfig> = serde_wasm_bindgen::from_value(contract_configs)?;
        let client = LedgerClient::new(
            chain_id as u64,
            &node_address,
            &contract_configs,
        )
            .as_js()?;
        Ok(LedgerClientWrapper(client))
    }

    pub async fn ping(&self) -> Result<JsValue> {
        let status = self.0.ping().await.as_js()?;
        let result: JsValue = serde_wasm_bindgen::to_value(&status)?;
        Ok(result)
    }

    #[wasm_bindgen(js_name = submitTransaction)]
    pub async fn submit_transaction(&self, transaction: &TransactionWrapper) -> Result<JsValue> {
        let response = self.0.submit_transaction(&transaction.0).await.as_js()?;
        let result: JsValue = serde_wasm_bindgen::to_value(&response)?;
        Ok(result)
    }

    #[wasm_bindgen(js_name = getReceipt)]
    pub async fn get_receipt(&self, hash: Vec<u8>) -> Result<String> {
        let receipt = self.0.get_receipt(&hash).await.as_js()?;
        Ok(receipt)
    }
}