use indy_besu_vdr::{
    credential_definition_registry, Address, Block, CredentialDefinition, CredentialDefinitionId,
    EventLog, SignatureData,
};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::{
    client::LedgerClientWrapper,
    error::{JsResult, Result},
    event_query::EventQueryWrapper,
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
        id: &str,
        cred_def: JsValue,
    ) -> Result<TransactionWrapper> {
        let client = client.0.clone();
        let cred_def: CredentialDefinition = serde_wasm_bindgen::from_value(cred_def)?;
        let address = Address::from(from);
        let id = CredentialDefinitionId::from(id);
        let transaction =
            credential_definition_registry::build_create_credential_definition_transaction(
                &client, &address, &id, &cred_def,
            )
            .await
            .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = buildCreateSchemaEndorsingData)]
    pub async fn build_create_schema_endorsing_data(
        client: &LedgerClientWrapper,
        id: &str,
        cred_def: JsValue,
    ) -> Result<TransactionEndorsingDataWrapper> {
        let cred_def: CredentialDefinition = serde_wasm_bindgen::from_value(cred_def)?;
        let id = CredentialDefinitionId::from(id);
        let data =
            credential_definition_registry::build_create_credential_definition_endorsing_data(
                &client.0, &id, &cred_def,
            )
            .await
            .as_js()?;
        Ok(TransactionEndorsingDataWrapper(Rc::new(data)))
    }

    #[wasm_bindgen(js_name = buildCreateCredentialDefinitionSignedTransaction)]
    pub async fn build_create_credential_definition_signed_transaction(
        client: &LedgerClientWrapper,
        from: &str,
        id: &str,
        cred_def: JsValue,
        signature_data: JsValue,
    ) -> Result<TransactionWrapper> {
        let client = client.0.clone();
        let cred_def: CredentialDefinition = serde_wasm_bindgen::from_value(cred_def)?;
        let address = Address::from(from);
        let id = CredentialDefinitionId::from(id);
        let signature_data: SignatureData = serde_wasm_bindgen::from_value(signature_data)?;
        let transaction =
            credential_definition_registry::build_create_credential_definition_signed_transaction(
                &client,
                &address,
                &id,
                &cred_def,
                &signature_data,
            )
            .await
            .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = buildGetCredentialDefinitionCreatedTransaction)]
    pub async fn build_get_credential_definition_created_transaction(
        client: &LedgerClientWrapper,
        id: &str,
    ) -> Result<TransactionWrapper> {
        let id = CredentialDefinitionId::from(id);
        let transaction =
            credential_definition_registry::build_get_credential_definition_created_transaction(
                &client.0, &id,
            )
            .await
            .as_js()?;
        Ok(TransactionWrapper(Rc::new(transaction)))
    }

    #[wasm_bindgen(js_name = buildGetCredentialDefinitionQuery)]
    pub async fn build_get_credential_definition_query(
        client: &LedgerClientWrapper,
        id: &str,
        from_block: Option<u64>,
        to_block: Option<u64>,
    ) -> Result<EventQueryWrapper> {
        let id = CredentialDefinitionId::from(id);
        let from_block = from_block.map(Block::from);
        let to_block = to_block.map(Block::from);
        let query = credential_definition_registry::build_get_credential_definition_query(
            &client.0,
            &id,
            from_block.as_ref(),
            to_block.as_ref(),
        )
        .await
        .as_js()?;
        Ok(EventQueryWrapper(Rc::new(query)))
    }

    #[wasm_bindgen(js_name = parseCredentialDefinitionCreatedResult)]
    pub fn parse_credential_definition_created_result(
        client: &LedgerClientWrapper,
        bytes: Vec<u8>,
    ) -> Result<u64> {
        let block = credential_definition_registry::parse_credential_definition_created_result(
            &client.0, &bytes,
        )
        .as_js()?;
        Ok(block.value())
    }

    #[wasm_bindgen(js_name = parseCredentialDefinitionCreatedEvent)]
    pub fn parse_credential_definition_created_event(
        client: &LedgerClientWrapper,
        log: JsValue,
    ) -> Result<JsValue> {
        let log: EventLog = serde_wasm_bindgen::from_value(log)?;
        let event = credential_definition_registry::parse_credential_definition_created_event(
            &client.0, &log,
        )
        .as_js()?;
        let result: JsValue = serde_wasm_bindgen::to_value(&event)?;
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
