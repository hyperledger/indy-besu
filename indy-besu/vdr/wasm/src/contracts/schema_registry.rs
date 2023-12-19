use indy2_vdr::{schema_registry, Address, Schema, SchemaId};
use wasm_bindgen::prelude::*;

use crate::transaction::TransactionWrapper;
use crate::client::LedgerClientWrapper;
use crate::error::{JsResult, Result};

#[wasm_bindgen(js_name = SchemaRegistry)]
pub struct SchemaRegistry;

#[wasm_bindgen(js_class = SchemaRegistry)]
impl SchemaRegistry {
    #[wasm_bindgen(js_name = buildCreateSchemaTransaction)]
    pub async fn build_create_schema_transaction(client: &LedgerClientWrapper,
                                                 from: &str,
                                                 schema: JsValue) -> Result<TransactionWrapper> {
        let schema: Schema = serde_wasm_bindgen::from_value(schema)?;
        let address = Address::new(from);
        let transaction = schema_registry::build_create_schema_transaction(&client.0, &address, &schema).await.as_js()?;
        Ok(TransactionWrapper(transaction))
    }

    #[wasm_bindgen(js_name = buildResolveSchemaTransaction)]
    pub async fn build_resolve_schema_transaction(client: &LedgerClientWrapper,
                                                  id: &str) -> Result<TransactionWrapper> {
        let id = SchemaId::new(id);
        let transaction = schema_registry::build_resolve_schema_transaction(&client.0, &id).await.as_js()?;
        Ok(TransactionWrapper(transaction))
    }

    #[wasm_bindgen(js_name = parseResolveSchemaResult)]
    pub fn parse_resolve_schema_result(client: &LedgerClientWrapper,
                                       bytes: Vec<u8>) -> Result<JsValue> {
        let schema = schema_registry::parse_resolve_schema_result(&client.0, bytes).as_js()?;
        let result: JsValue = serde_wasm_bindgen::to_value(&schema)?;
        Ok(result)
    }
}