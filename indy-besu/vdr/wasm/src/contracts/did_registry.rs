use indy2_vdr::{did_registry, Address, DidDocument, DID};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::{
    client::LedgerClientWrapper,
    error::{JsResult, Result},
    transaction::TransactionWrapper,
};

#[wasm_bindgen(js_name = IndyDidRegistry)]
pub struct IndyDidRegistry;

#[wasm_bindgen(js_class = IndyDidRegistry)]
impl IndyDidRegistry {
    #[wasm_bindgen(js_name = buildCreateDidTransaction)]
    pub async fn build_create_did_transaction(
        client: &LedgerClientWrapper,
        from: &str,
        did_doc: JsValue,
    ) -> Result<TransactionWrapper> {
        let did_doc: DidDocument = serde_wasm_bindgen::from_value(did_doc)?;
        let address = Address::from(from);
        let transaction = did_registry::build_create_did_transaction(&client.0, &address, &did_doc)
            .await
            .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = buildUpdateDidTransaction)]
    pub async fn build_update_did_transaction(
        client: &LedgerClientWrapper,
        from: &str,
        did_doc: JsValue,
    ) -> Result<TransactionWrapper> {
        let did_doc: DidDocument = serde_wasm_bindgen::from_value(did_doc)?;
        let address = Address::from(from);
        let transaction = did_registry::build_update_did_transaction(&client.0, &address, &did_doc)
            .await
            .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = buildDeactivateDidTransaction)]
    pub async fn build_deactivate_did_transaction(
        client: &LedgerClientWrapper,
        from: &str,
        did: &str,
    ) -> Result<TransactionWrapper> {
        let address = Address::from(from);
        let did = DID::from(did);
        let transaction = did_registry::build_deactivate_did_transaction(&client.0, &address, &did)
            .await
            .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = buildResolveDidTransaction)]
    pub async fn build_resolve_did_transaction(
        client: &LedgerClientWrapper,
        did: &str,
    ) -> Result<TransactionWrapper> {
        let did = DID::from(did);
        let transaction = did_registry::build_resolve_did_transaction(&client.0, &did)
            .await
            .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = parseResolveDidResult)]
    pub fn parse_resolve_did_result(
        client: &LedgerClientWrapper,
        bytes: Vec<u8>,
    ) -> Result<JsValue> {
        let did_doc = did_registry::parse_resolve_did_result(&client.0, &bytes).as_js()?;
        let result: JsValue = serde_wasm_bindgen::to_value(&did_doc)?;
        Ok(result)
    }
}
