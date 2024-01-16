use crate::ffi::{
    client::LedgerClient,
    error::{VdrError, VdrResult},
    transaction::Transaction,
};
use indy2_vdr::{did_registry, Address, DID};
use serde_json::json;

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_did_transaction(
    client: &LedgerClient,
    from: &str,
    identity: &str,
    did: &str,
    did_doc: &str,
) -> VdrResult<Transaction> {
    let did_doc = serde_json::from_str(did_doc).map_err(|err| VdrError::CommonInvalidData {
        msg: format!("Unable to parse DID DDocument. Err: {:?}", err),
    })?;
    let transaction =
        did_registry::build_create_did_transaction(
            &client.client,
            &Address::from(from),
            &Address::from(identity),
            &DID::from(did),
            &did_doc,
        )
            .await?;
    Ok(Transaction { transaction })
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_update_did_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    did_doc: &str,
) -> VdrResult<Transaction> {
    let did_doc = serde_json::from_str(did_doc).map_err(|err| VdrError::CommonInvalidData {
        msg: format!("Unable to parse DID DDocument. Err: {:?}", err),
    })?;
    let transaction =
        did_registry::build_update_did_transaction(
            &client.client,
            &Address::from(from),
            &DID::from(did),
            &did_doc
        )
            .await?;
    Ok(Transaction { transaction })
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_deactivate_did_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
) -> VdrResult<Transaction> {
    let transaction = did_registry::build_deactivate_did_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
    )
        .await?;
    Ok(Transaction { transaction })
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_resolve_did_transaction(
    client: &LedgerClient,
    did: &str,
) -> VdrResult<Transaction> {
    let transaction =
        did_registry::build_resolve_did_transaction(&client.client, &DID::from(did)).await?;
    Ok(Transaction { transaction })
}

#[uniffi::export]
pub fn parse_resolve_did_result(client: &LedgerClient, bytes: Vec<u8>) -> VdrResult<String> {
    let did_doc = did_registry::parse_resolve_did_result(&client.client, &bytes)?;
    Ok(json!(did_doc).to_string())
}
