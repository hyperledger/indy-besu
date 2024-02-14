use indy_besu_vdr::{
    credential_definition_registry, Address, CredentialDefinition, CredentialDefinitionId,
    SignatureData,
};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::{
    client::LedgerClientWrapper,
    error::{JsResult, Result},
    transaction::{TransactionEndorsingDataWrapper, TransactionWrapper},
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
        let client = client.0.clone();
        let cred_def: CredentialDefinition = serde_wasm_bindgen::from_value(cred_def)?;
        let address = Address::from(from);
        let transaction =
            credential_definition_registry::build_create_credential_definition_transaction(
                &client, &address, &cred_def,
            )
            .await
            .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = buildCreateCredentialDefinitionEndorsingData)]
    pub async fn build_create_credential_definition_endorsing_data(
        client: &LedgerClientWrapper,
        cred_def: JsValue,
    ) -> Result<TransactionEndorsingDataWrapper> {
        let cred_def: CredentialDefinition = serde_wasm_bindgen::from_value(cred_def)?;
        let data =
            credential_definition_registry::build_create_credential_definition_endorsing_data(
                &client.0, &cred_def,
            )
            .await
            .as_js()?;
        Ok(TransactionEndorsingDataWrapper(Rc::new(data)))
    }

    #[wasm_bindgen(js_name = buildCreateCredentialDefinitionSignedTransaction)]
    pub async fn build_create_credential_definition_signed_transaction(
        client: &LedgerClientWrapper,
        from: &str,
        cred_def: JsValue,
        signature_data: JsValue,
    ) -> Result<TransactionWrapper> {
        let client = client.0.clone();
        let cred_def: CredentialDefinition = serde_wasm_bindgen::from_value(cred_def)?;
        let address = Address::from(from);
        let signature_data: SignatureData = serde_wasm_bindgen::from_value(signature_data)?;
        let transaction =
            credential_definition_registry::build_create_credential_definition_signed_transaction(
                &client,
                &address,
                &cred_def,
                &signature_data,
            )
            .await
            .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
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
        Ok(TransactionWrapper(Rc::new(transaction)))
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

    #[wasm_bindgen(js_name = resolveCredentialDefinition)]
    pub async fn resolve_credential_definition(
        client: &LedgerClientWrapper,
        id: &str,
    ) -> Result<JsValue> {
        let id = CredentialDefinitionId::from(id);
        let cred_def =
            credential_definition_registry::resolve_credential_definition(&client.0, &id)
                .await
                .as_js()?;
        let result: JsValue = serde_wasm_bindgen::to_value(&cred_def)?;
        Ok(result)
    }
}
