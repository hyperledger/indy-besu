use crate::ffi::{
    client::LedgerClient,
    error::{VdrError, VdrResult},
    event_query::{EventLog, EventQuery},
    transaction::{Transaction, TransactionEndorsingData},
    types::SignatureData,
};
use indy2_vdr::{
    credential_definition_registry, Address, Block, CredentialDefinitionId,
};
use serde_json::json;

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_credential_definition_transaction(
    client: &LedgerClient,
    from: &str,
    id: &str,
    credential_definition: &str,
) -> VdrResult<Transaction> {
    let credential_definition =
        serde_json::from_str(credential_definition).map_err(|err| VdrError::CommonInvalidData {
            msg: format!("Unable to parse credential definition. Err: {:?}", err),
        })?;
    let transaction =
        credential_definition_registry::build_create_credential_definition_transaction(
            &client.client,
            &Address::from(from),
            &CredentialDefinitionId::from(id),
            &credential_definition,
        )
        .await?;
    Ok(Transaction { transaction })
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_credential_definition_endorsing_data(
    client: &LedgerClient,
    id: &str,
    credential_definition: &str,
) -> VdrResult<TransactionEndorsingData> {
    let credential_definition =
        serde_json::from_str(credential_definition).map_err(|err| VdrError::CommonInvalidData {
            msg: format!("Unable to parse credential definition. Err: {:?}", err),
        })?;
    credential_definition_registry::build_create_credential_definition_endorsing_data(
        &client.client,
        &CredentialDefinitionId::from(id),
        &credential_definition,
    )
    .await
    .map(TransactionEndorsingData::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_credential_definition_signed_transaction(
    client: &LedgerClient,
    from: &str,
    id: &str,
    credential_definition: &str,
    signature: SignatureData,
) -> VdrResult<Transaction> {
    let credential_definition =
        serde_json::from_str(credential_definition).map_err(|err| VdrError::CommonInvalidData {
            msg: format!("Unable to parse credential definition. Err: {:?}", err),
        })?;
    credential_definition_registry::build_create_credential_definition_signed_transaction(
        &client.client,
        &Address::from(from),
        &CredentialDefinitionId::from(id),
        &credential_definition,
        &signature.into(),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_get_credential_definition_created_transaction(
    client: &LedgerClient,
    id: &str,
) -> VdrResult<Transaction> {
    credential_definition_registry::build_get_credential_definition_created_transaction(
        &client.client,
        &CredentialDefinitionId::from(id),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_get_credential_definition_query(
    client: &LedgerClient,
    id: &str,
    from_block: Option<u64>,
    to_block: Option<u64>,
) -> VdrResult<EventQuery> {
    credential_definition_registry::build_get_credential_definition_query(
        &client.client,
        &CredentialDefinitionId::from(id),
        from_block.map(Block::from).as_ref(),
        to_block.map(Block::from).as_ref(),
    )
    .await
    .map(EventQuery::from)
    .map_err(VdrError::from)
}

#[uniffi::export]
pub fn parse_credential_definition_created_result(
    client: &LedgerClient,
    bytes: Vec<u8>,
) -> VdrResult<u64> {
    let create = credential_definition_registry::parse_credential_definition_created_result(
        &client.client,
        &bytes,
    )?;
    Ok(create.value())
}

#[uniffi::export]
pub fn parse_credential_definition_created_event(
    client: &LedgerClient,
    log: EventLog,
) -> VdrResult<String> {
    let event = credential_definition_registry::parse_credential_definition_created_event(
        &client.client,
        &log.into(),
    )?;
    Ok(json!(event).to_string())
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn resolve_credential_definition(client: &LedgerClient, id: &str) -> VdrResult<String> {
    let cred_def = credential_definition_registry::resolve_credential_definition(
        &client.client,
        &CredentialDefinitionId::from(id),
    )
    .await?;
    Ok(json!(cred_def).to_string())
}
