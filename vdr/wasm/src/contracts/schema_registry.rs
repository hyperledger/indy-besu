use indy_besu_vdr::{schema_registry, Address, Schema, SchemaId, SignatureData};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::{
    client::LedgerClientWrapper,
    error::{JsResult, Result},
    transaction::{TransactionEndorsingDataWrapper, TransactionWrapper},
};

#[wasm_bindgen(js_name = SchemaRegistry)]
pub struct SchemaRegistry;

#[wasm_bindgen(js_class = SchemaRegistry)]
impl SchemaRegistry {
    #[wasm_bindgen(js_name = buildCreateSchemaTransaction)]
    pub async fn build_create_schema_transaction(
        client: &LedgerClientWrapper,
        from: &str,
        schema: JsValue,
    ) -> Result<TransactionWrapper> {
        let schema: Schema = serde_wasm_bindgen::from_value(schema)?;
        let address = Address::from(from);
        let transaction =
            schema_registry::build_create_schema_transaction(&client.0, &address, &schema)
                .await
                .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = buildCreateSchemaEndorsingData)]
    pub async fn build_create_schema_endorsing_data(
        client: &LedgerClientWrapper,
        schema: JsValue,
    ) -> Result<TransactionEndorsingDataWrapper> {
        let schema: Schema = serde_wasm_bindgen::from_value(schema)?;
        let data = schema_registry::build_create_schema_endorsing_data(&client.0, &schema)
            .await
            .as_js()?;
        Ok(TransactionEndorsingDataWrapper(Rc::new(data)))
    }

    #[wasm_bindgen(js_name = buildCreateSchemaSignedTransaction)]
    pub async fn build_create_schema_signed_transaction(
        client: &LedgerClientWrapper,
        from: &str,
        schema: JsValue,
        signature_data: JsValue,
    ) -> Result<TransactionWrapper> {
        let schema: Schema = serde_wasm_bindgen::from_value(schema)?;
        let address = Address::from(from);
        let signature_data: SignatureData = serde_wasm_bindgen::from_value(signature_data)?;
        let transaction = schema_registry::build_create_schema_signed_transaction(
            &client.0,
            &address,
            &schema,
            &signature_data,
        )
        .await
        .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = buildResolveSchemaTransaction)]
    pub async fn build_resolve_schema_transaction(
        client: &LedgerClientWrapper,
        id: &str,
    ) -> Result<TransactionWrapper> {
        let id = SchemaId::from(id);
        let transaction = schema_registry::build_resolve_schema_transaction(&client.0, &id)
            .await
            .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = parseResolveSchemaResult)]
    pub fn parse_resolve_schema_result(
        client: &LedgerClientWrapper,
        bytes: Vec<u8>,
    ) -> Result<JsValue> {
        let schema = schema_registry::parse_resolve_schema_result(&client.0, &bytes).as_js()?;
        let result: JsValue = serde_wasm_bindgen::to_value(&schema)?;
        Ok(result)
    }

    #[wasm_bindgen(js_name = resolveSchema)]
    pub async fn resolve_schema(client: &LedgerClientWrapper, id: &str) -> Result<JsValue> {
        let id = SchemaId::from(id);
        let schema = schema_registry::resolve_schema(&client.0, &id)
            .await
            .as_js()?;
        let result: JsValue = serde_wasm_bindgen::to_value(&schema)?;
        Ok(result)
    }
}
