use crate::ffi::{client::LedgerClient, error::VdrResult, transaction::Transaction};
use indy2_vdr::{validator_control, Address};
use serde_json::json;

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_add_validator_transaction(
    client: &LedgerClient,
    from: &str,
    validator_address: &str,
) -> VdrResult<Transaction> {
    let transaction = validator_control::build_add_validator_transaction(
        &client.client,
        &Address::from(from),
        &Address::from(validator_address),
    )
    .await?;
    Ok(Transaction { transaction })
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_remove_validator_transaction(
    client: &LedgerClient,
    from: &str,
    validator_address: &str,
) -> VdrResult<Transaction> {
    let transaction = validator_control::build_remove_validator_transaction(
        &client.client,
        &Address::from(from),
        &Address::from(validator_address),
    )
    .await?;
    Ok(Transaction { transaction })
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_get_validators_transaction(client: &LedgerClient) -> VdrResult<Transaction> {
    let transaction = validator_control::build_get_validators_transaction(&client.client).await?;
    Ok(Transaction { transaction })
}

#[uniffi::export]
pub fn parse_get_validators_result(client: &LedgerClient, bytes: Vec<u8>) -> VdrResult<String> {
    let validators = validator_control::parse_get_validators_result(&client.client, &bytes)?;
    Ok(json!(validators).to_string())
}
