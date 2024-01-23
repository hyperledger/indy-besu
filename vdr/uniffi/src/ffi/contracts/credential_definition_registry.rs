use crate::ffi::{
    client::LedgerClient,
    error::{VdrError, VdrResult},
    transaction::Transaction,
};
use indy2_vdr::{credential_definition_registry, Address, CredentialDefinitionId};
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
pub async fn build_resolve_credential_definition_transaction(
    client: &LedgerClient,
    id: &str,
) -> VdrResult<Transaction> {
    let transaction =
        credential_definition_registry::build_resolve_credential_definition_transaction(
            &client.client,
            &CredentialDefinitionId::from(id),
        )
        .await?;
    Ok(Transaction { transaction })
}

#[uniffi::export]
pub fn parse_resolve_credential_definition_result(
    client: &LedgerClient,
    bytes: Vec<u8>,
) -> VdrResult<String> {
    let cred_def = credential_definition_registry::parse_resolve_credential_definition_result(
        &client.client,
        &bytes,
    )?;
    Ok(json!(cred_def).to_string())
}
