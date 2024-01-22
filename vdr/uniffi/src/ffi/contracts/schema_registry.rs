use crate::ffi::{
    client::LedgerClient,
    error::{VdrError, VdrResult},
    event_query::{EventLog, EventQuery},
    transaction::{Transaction, TransactionEndorsingData},
    types::SignatureData,
};
use indy2_vdr::{schema_registry, Address, Block, SchemaId};
use serde_json::json;

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_schema_transaction(
    client: &LedgerClient,
    from: &str,
    id: &str,
    schema: &str,
) -> VdrResult<Transaction> {
    let schema = serde_json::from_str(schema).map_err(|err| VdrError::CommonInvalidData {
        msg: format!("Unable to parse credential definition. Err: {:?}", err),
    })?;
    let transaction = schema_registry::build_create_schema_transaction(
        &client.client,
        &Address::from(from),
        &SchemaId::from(id),
        &schema,
    )
    .await?;
    Ok(Transaction { transaction })
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_schema_endorsing_data(
    client: &LedgerClient,
    id: &str,
    schema: &str,
) -> VdrResult<TransactionEndorsingData> {
    let schema = serde_json::from_str(schema).map_err(|err| VdrError::CommonInvalidData {
        msg: format!("Unable to parse credential definition. Err: {:?}", err),
    })?;
    schema_registry::build_create_schema_endorsing_data(
        &client.client,
        &SchemaId::from(id),
        &schema,
    )
    .await
    .map(TransactionEndorsingData::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_schema_signed_transaction(
    client: &LedgerClient,
    from: &str,
    id: &str,
    schema: &str,
    signature: SignatureData,
) -> VdrResult<Transaction> {
    let schema = serde_json::from_str(schema).map_err(|err| VdrError::CommonInvalidData {
        msg: format!("Unable to parse credential definition. Err: {:?}", err),
    })?;
    schema_registry::build_create_schema_signed_transaction(
        &client.client,
        &Address::from(from),
        &SchemaId::from(id),
        &schema,
        &signature.into(),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_get_schema_created_transaction(
    client: &LedgerClient,
    id: &str,
) -> VdrResult<Transaction> {
    schema_registry::build_get_schema_created_transaction(&client.client, &SchemaId::from(id))
        .await
        .map(Transaction::from)
        .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_get_schema_query(
    client: &LedgerClient,
    id: &str,
    from_block: Option<u64>,
    to_block: Option<u64>,
) -> VdrResult<EventQuery> {
    schema_registry::build_get_schema_query(
        &client.client,
        &SchemaId::from(id),
        from_block.map(Block::from).as_ref(),
        to_block.map(Block::from).as_ref(),
    )
    .await
    .map(EventQuery::from)
    .map_err(VdrError::from)
}

#[uniffi::export]
pub fn parse_schema_created_result(client: &LedgerClient, bytes: Vec<u8>) -> VdrResult<u64> {
    let create = schema_registry::parse_schema_created_result(&client.client, &bytes)?;
    Ok(create.value())
}

#[uniffi::export]
pub fn parse_schema_created_event(client: &LedgerClient, log: EventLog) -> VdrResult<String> {
    let event = schema_registry::parse_schema_created_event(&client.client, &log.into())?;
    Ok(json!(event).to_string())
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn resolve_schema(client: &LedgerClient, id: &str) -> VdrResult<String> {
    let schema = schema_registry::resolve_schema(&client.client, &SchemaId::from(id)).await?;
    Ok(json!(schema).to_string())
}
