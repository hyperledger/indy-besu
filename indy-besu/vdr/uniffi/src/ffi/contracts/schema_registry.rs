use crate::ffi::{
    client::LedgerClient,
    error::{VdrError, VdrResult},
    transaction::Transaction,
};
use indy2_vdr::{schema_registry, Address, SchemaId};
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
pub async fn build_resolve_schema_transaction(
    client: &LedgerClient,
    id: &str,
) -> VdrResult<Transaction> {
    let transaction =
        schema_registry::build_resolve_schema_transaction(&client.client, &SchemaId::from(id))
            .await?;
    Ok(Transaction { transaction })
}

#[uniffi::export]
pub fn parse_resolve_schema_result(client: &LedgerClient, bytes: Vec<u8>) -> VdrResult<String> {
    let schema = schema_registry::parse_resolve_schema_result(&client.client, &bytes)?;
    Ok(json!(schema).to_string())
}
