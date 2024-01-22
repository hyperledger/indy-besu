use indy2_vdr::{schema_registry, Address, Block, EventLog, Schema, SchemaId, SignatureData};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::{
    client::LedgerClientWrapper,
    error::{JsResult, Result},
    event_query::EventQueryWrapper,
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
        id: &str,
        schema: JsValue,
    ) -> Result<TransactionWrapper> {
        let schema: Schema = serde_wasm_bindgen::from_value(schema)?;
        let address = Address::from(from);
        let id = SchemaId::from(id);
        let transaction =
            schema_registry::build_create_schema_transaction(&client.0, &address, &id, &schema)
                .await
                .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = buildCreateSchemaEndorsingData)]
    pub async fn build_create_schema_endorsing_data(
        client: &LedgerClientWrapper,
        id: &str,
        schema: JsValue,
    ) -> Result<TransactionEndorsingDataWrapper> {
        let schema: Schema = serde_wasm_bindgen::from_value(schema)?;
        let id = SchemaId::from(id);
        let data = schema_registry::build_create_schema_endorsing_data(&client.0, &id, &schema)
            .await
            .as_js()?;
        Ok(TransactionEndorsingDataWrapper(Rc::new(data)))
    }

    #[wasm_bindgen(js_name = buildCreateSchemaSignedTransaction)]
    pub async fn build_create_schema_signed_transaction(
        client: &LedgerClientWrapper,
        from: &str,
        id: &str,
        schema: JsValue,
        signature_data: JsValue,
    ) -> Result<TransactionWrapper> {
        let schema: Schema = serde_wasm_bindgen::from_value(schema)?;
        let address = Address::from(from);
        let id = SchemaId::from(id);
        let signature_data: SignatureData = serde_wasm_bindgen::from_value(signature_data)?;
        let transaction = schema_registry::build_create_schema_signed_transaction(
            &client.0,
            &address,
            &id,
            &schema,
            &signature_data,
        )
        .await
        .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = buildGetSchemaCreatedTransaction)]
    pub async fn build_get_schema_created_transaction(
        client: &LedgerClientWrapper,
        id: &str,
    ) -> Result<TransactionWrapper> {
        let id = SchemaId::from(id);
        let transaction = schema_registry::build_get_schema_created_transaction(&client.0, &id)
            .await
            .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = buildGetSchemaQuery)]
    pub async fn build_get_schema_query(
        client: &LedgerClientWrapper,
        id: &str,
        from_block: Option<u64>,
        to_block: Option<u64>,
    ) -> Result<EventQueryWrapper> {
        let id = SchemaId::from(id);
        let from_block = from_block.map(Block::from);
        let to_block = to_block.map(Block::from);
        let query = schema_registry::build_get_schema_query(
            &client.0,
            &id,
            from_block.as_ref(),
            to_block.as_ref(),
        )
        .await
        .as_js()?;
        Ok(EventQueryWrapper(Rc::new(query)))
    }

    #[wasm_bindgen(js_name = parseSchemaCreatedResult)]
    pub fn parse_schema_created_result(
        client: &LedgerClientWrapper,
        bytes: Vec<u8>,
    ) -> Result<u64> {
        let block = schema_registry::parse_schema_created_result(&client.0, &bytes).as_js()?;
        Ok(block.value())
    }

    #[wasm_bindgen(js_name = parseSchemaCreatedEvent)]
    pub fn parse_schema_created_event(
        client: &LedgerClientWrapper,
        log: JsValue,
    ) -> Result<JsValue> {
        let log: EventLog = serde_wasm_bindgen::from_value(log)?;
        let event = schema_registry::parse_schema_created_event(&client.0, &log).as_js()?;
        let result: JsValue = serde_wasm_bindgen::to_value(&event)?;
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
