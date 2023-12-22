use indy2_vdr::{
    credential_definition_registry, Address, CredentialDefinition, CredentialDefinitionId,
};
use wasm_bindgen::prelude::*;

use crate::{
    client::LedgerClientWrapper,
    error::{JsResult, Result},
    transaction::TransactionWrapper,
};

#[wasm_bindgen(js_name = CredentialDefinitionRegistry)]
pub struct CredentialDefinitionRegistry;

#[wasm_bindgen(js_class = CredentialDefinitionRegistry)]
impl CredentialDefinitionRegistry {
    #[wasm_bindgen(js_name = buildCreateCredentialDefinitionTransaction)]
    pub async fn build_create_credential_definition_transaction(
        client: &LedgerClientWrapper,
        from: &str,
        cred_def: JsValue,
    ) -> Result<TransactionWrapper> {
        let cred_def: CredentialDefinition = serde_wasm_bindgen::from_value(cred_def)?;
        let address = Address::from(from);
        let transaction =
            credential_definition_registry::build_create_credential_definition_transaction(
                &client.0, &address, &cred_def,
            )
            .await
            .as_js()?;
        Ok(TransactionWrapper(transaction))
    }

    #[wasm_bindgen(js_name = buildResolveCredentialDefinitionTransaction)]
    pub async fn build_resolve_credential_definition_transaction(
        client: &LedgerClientWrapper,
        id: &str,
    ) -> Result<TransactionWrapper> {
        let id = CredentialDefinitionId::from(id);
        let transaction =
            credential_definition_registry::build_resolve_credential_definition_transaction(
                &client.0, &id,
            )
            .await
            .as_js()?;
        Ok(TransactionWrapper(transaction))
    }

    #[wasm_bindgen(js_name = parseResolveCredentialDefinitionResult)]
    pub fn parse_resolve_credential_definition_result(
        client: &LedgerClientWrapper,
        bytes: Vec<u8>,
    ) -> Result<JsValue> {
        let cred_def = credential_definition_registry::parse_resolve_credential_definition_result(
            &client.0, &bytes,
        )
        .as_js()?;
        let result: JsValue = serde_wasm_bindgen::to_value(&cred_def)?;
        Ok(result)
    }
}
