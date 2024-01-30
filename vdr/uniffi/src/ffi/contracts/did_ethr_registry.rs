use crate::ffi::{
    client::LedgerClient,
    error::{VdrError, VdrResult},
    event_query::{EventLog, EventQuery},
    transaction::{Transaction, TransactionEndorsingData},
    types::SignatureData,
};
use indy_besu_vdr::{
    did_ethr_registry, Address, Block, DelegateType, DidAttributeChanged as DidAttributeChanged_,
    DidDelegateChanged as DidDelegateChanged_, DidDocAttribute, DidEvents as DidEvents_,
    DidOwnerChanged as DidOwnerChanged_, DidResolutionOptions as DidResolutionOptions_, Validity,
    DID,
};
use serde_json::json;

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_change_owner_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    new_owner: &str,
) -> VdrResult<Transaction> {
    did_ethr_registry::build_did_change_owner_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &Address::from(new_owner),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_change_owner_endorsing_data(
    client: &LedgerClient,
    did: &str,
    new_owner: &str,
) -> VdrResult<TransactionEndorsingData> {
    did_ethr_registry::build_did_change_owner_endorsing_data(
        &client.client,
        &DID::from(did),
        &Address::from(new_owner),
    )
    .await
    .map(TransactionEndorsingData::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_change_owner_signed_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    new_owner: &str,
    signature: SignatureData,
) -> VdrResult<Transaction> {
    did_ethr_registry::build_did_change_owner_signed_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &Address::from(new_owner),
        &signature.into(),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_add_delegate_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    delegate_type: &str,
    delegate: &str,
    validity: u64,
) -> VdrResult<Transaction> {
    did_ethr_registry::build_did_add_delegate_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &DelegateType::try_from(delegate_type)?,
        &Address::from(delegate),
        &Validity::from(validity),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_add_delegate_endorsing_data(
    client: &LedgerClient,
    did: &str,
    delegate_type: &str,
    delegate: &str,
    validity: u64,
) -> VdrResult<TransactionEndorsingData> {
    did_ethr_registry::build_did_add_delegate_endorsing_data(
        &client.client,
        &DID::from(did),
        &DelegateType::try_from(delegate_type)?,
        &Address::from(delegate),
        &Validity::from(validity),
    )
    .await
    .map(TransactionEndorsingData::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_add_delegate_signed_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    delegate_type: &str,
    delegate: &str,
    validity: u64,
    signature: SignatureData,
) -> VdrResult<Transaction> {
    did_ethr_registry::build_did_add_delegate_signed_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &DelegateType::try_from(delegate_type)?,
        &Address::from(delegate),
        &Validity::from(validity),
        &signature.into(),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_revoke_delegate_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    delegate_type: &str,
    delegate: &str,
) -> VdrResult<Transaction> {
    did_ethr_registry::build_did_revoke_delegate_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &DelegateType::try_from(delegate_type)?,
        &Address::from(delegate),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_revoke_delegate_endorsing_data(
    client: &LedgerClient,
    did: &str,
    delegate_type: &str,
    delegate: &str,
) -> VdrResult<TransactionEndorsingData> {
    did_ethr_registry::build_did_revoke_delegate_endorsing_data(
        &client.client,
        &DID::from(did),
        &DelegateType::try_from(delegate_type)?,
        &Address::from(delegate),
    )
    .await
    .map(TransactionEndorsingData::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_revoke_delegate_signed_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    delegate_type: &str,
    delegate: &str,
    signature: SignatureData,
) -> VdrResult<Transaction> {
    did_ethr_registry::build_did_revoke_delegate_signed_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &DelegateType::try_from(delegate_type)?,
        &Address::from(delegate),
        &signature.into(),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_set_attribute_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    attribute: &str,
    validity: u64,
) -> VdrResult<Transaction> {
    let attribute: DidDocAttribute =
        serde_json::from_str(attribute).map_err(|err| VdrError::CommonInvalidData {
            msg: format!("Unable to parse DID Attribute. Err: {:?}", err),
        })?;

    did_ethr_registry::build_did_set_attribute_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &attribute,
        &Validity::from(validity),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_set_attribute_endorsing_data(
    client: &LedgerClient,
    did: &str,
    attribute: &str,
    validity: u64,
) -> VdrResult<TransactionEndorsingData> {
    let attribute: DidDocAttribute =
        serde_json::from_str(attribute).map_err(|err| VdrError::CommonInvalidData {
            msg: format!("Unable to parse DID Attribute. Err: {:?}", err),
        })?;

    did_ethr_registry::build_did_set_attribute_endorsing_data(
        &client.client,
        &DID::from(did),
        &attribute,
        &Validity::from(validity),
    )
    .await
    .map(TransactionEndorsingData::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_set_attribute_signed_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    attribute: &str,
    validity: u64,
    signature: SignatureData,
) -> VdrResult<Transaction> {
    let attribute: DidDocAttribute =
        serde_json::from_str(attribute).map_err(|err| VdrError::CommonInvalidData {
            msg: format!("Unable to parse DID Attribute. Err: {:?}", err),
        })?;

    did_ethr_registry::build_did_set_attribute_signed_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &attribute,
        &Validity::from(validity),
        &signature.into(),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_revoke_attribute_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    attribute: &str,
) -> VdrResult<Transaction> {
    let attribute: DidDocAttribute =
        serde_json::from_str(attribute).map_err(|err| VdrError::CommonInvalidData {
            msg: format!("Unable to parse DID Attribute. Err: {:?}", err),
        })?;

    did_ethr_registry::build_did_revoke_attribute_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &attribute,
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_revoke_attribute_endorsing_data(
    client: &LedgerClient,
    did: &str,
    attribute: &str,
) -> VdrResult<TransactionEndorsingData> {
    let attribute: DidDocAttribute =
        serde_json::from_str(attribute).map_err(|err| VdrError::CommonInvalidData {
            msg: format!("Unable to parse DID Attribute. Err: {:?}", err),
        })?;

    did_ethr_registry::build_did_revoke_attribute_endorsing_data(
        &client.client,
        &DID::from(did),
        &attribute,
    )
    .await
    .map(TransactionEndorsingData::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_did_revoke_attribute_signed_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    attribute: &str,
    signature: SignatureData,
) -> VdrResult<Transaction> {
    let attribute: DidDocAttribute =
        serde_json::from_str(attribute).map_err(|err| VdrError::CommonInvalidData {
            msg: format!("Unable to parse DID Attribute. Err: {:?}", err),
        })?;

    did_ethr_registry::build_did_revoke_attribute_signed_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &attribute,
        &signature.into(),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_get_did_owner_transaction(
    client: &LedgerClient,
    did: &str,
) -> VdrResult<Transaction> {
    did_ethr_registry::build_get_did_owner_transaction(&client.client, &DID::from(did))
        .await
        .map(Transaction::from)
        .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_get_did_changed_transaction(
    client: &LedgerClient,
    did: &str,
) -> VdrResult<Transaction> {
    did_ethr_registry::build_get_did_changed_transaction(&client.client, &DID::from(did))
        .await
        .map(Transaction::from)
        .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_get_identity_nonce_transaction(
    client: &LedgerClient,
    identity: &str,
) -> VdrResult<Transaction> {
    did_ethr_registry::build_get_identity_nonce_transaction(
        &client.client,
        &Address::from(identity),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_get_did_events_query(
    client: &LedgerClient,
    did: &str,
    from_block: Option<u64>,
    to_block: Option<u64>,
) -> VdrResult<EventQuery> {
    did_ethr_registry::build_get_did_events_query(
        &client.client,
        &DID::from(did),
        from_block.map(Block::from).as_ref(),
        to_block.map(Block::from).as_ref(),
    )
    .await
    .map(EventQuery::from)
    .map_err(VdrError::from)
}

#[uniffi::export]
pub fn parse_did_changed_result(client: &LedgerClient, bytes: Vec<u8>) -> VdrResult<u64> {
    let block = did_ethr_registry::parse_did_changed_result(&client.client, &bytes)?;
    Ok(block.value())
}

#[uniffi::export]
pub fn parse_did_owner_result(client: &LedgerClient, bytes: Vec<u8>) -> VdrResult<String> {
    let address = did_ethr_registry::parse_did_owner_result(&client.client, &bytes)?;
    Ok(address.to_string())
}

#[uniffi::export]
pub fn parse_did_attribute_changed_event_response(
    client: &LedgerClient,
    log: EventLog,
) -> VdrResult<DidAttributeChanged> {
    did_ethr_registry::parse_did_attribute_changed_event_response(&client.client, &log.into())
        .map(DidAttributeChanged::from)
        .map_err(VdrError::from)
}

#[uniffi::export]
pub fn parse_did_delegate_changed_event_response(
    client: &LedgerClient,
    log: EventLog,
) -> VdrResult<DidDelegateChanged> {
    did_ethr_registry::parse_did_delegate_changed_event_response(&client.client, &log.into())
        .map(DidDelegateChanged::from)
        .map_err(VdrError::from)
}

#[uniffi::export]
pub fn parse_did_owner_changed_event_response(
    client: &LedgerClient,
    log: EventLog,
) -> VdrResult<DidOwnerChanged> {
    did_ethr_registry::parse_did_owner_changed_event_response(&client.client, &log.into())
        .map(DidOwnerChanged::from)
        .map_err(VdrError::from)
}

#[uniffi::export]
pub fn parse_did_event_response(client: &LedgerClient, log: EventLog) -> VdrResult<DidEvents> {
    did_ethr_registry::parse_did_event_response(&client.client, &log.into())
        .map(DidEvents::from)
        .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn resolve_did(
    client: &LedgerClient,
    did: &str,
    options: Option<DidResolutionOptions>,
) -> VdrResult<String> {
    let did_with_meta = did_ethr_registry::resolve_did(
        &client.client,
        &DID::from(did),
        options.map(DidResolutionOptions_::from).as_ref(),
    )
    .await?;
    Ok(json!(did_with_meta).to_string())
}

#[derive(uniffi::Record)]
pub struct DidAttributeChanged {
    pub identity: String,
    pub name: String,
    pub value: Vec<u8>,
    pub valid_to: u64,
    pub previous_change: u64,
}

impl From<DidAttributeChanged_> for DidAttributeChanged {
    fn from(event: DidAttributeChanged_) -> Self {
        DidAttributeChanged {
            identity: event.identity.to_string(),
            name: event.name,
            value: event.value,
            valid_to: event.valid_to,
            previous_change: event.previous_change.value(),
        }
    }
}

#[derive(uniffi::Record)]
pub struct DidDelegateChanged {
    pub identity: String,
    pub delegate: String,
    pub delegate_type: Vec<u8>,
    pub valid_to: u64,
    pub previous_change: u64,
}

impl From<DidDelegateChanged_> for DidDelegateChanged {
    fn from(event: DidDelegateChanged_) -> Self {
        DidDelegateChanged {
            identity: event.identity.to_string(),
            delegate: event.delegate.to_string(),
            delegate_type: event.delegate_type,
            valid_to: event.valid_to,
            previous_change: event.previous_change.value(),
        }
    }
}

#[derive(uniffi::Record)]
pub struct DidOwnerChanged {
    pub identity: String,
    pub owner: String,
    pub previous_change: u64,
}

impl From<DidOwnerChanged_> for DidOwnerChanged {
    fn from(event: DidOwnerChanged_) -> Self {
        DidOwnerChanged {
            identity: event.identity.to_string(),
            owner: event.owner.to_string(),
            previous_change: event.previous_change.value(),
        }
    }
}

#[derive(uniffi::Enum)]
pub enum DidEvents {
    AttributeChangedEvent { event: DidAttributeChanged },
    DelegateChanged { event: DidDelegateChanged },
    OwnerChanged { event: DidOwnerChanged },
}

impl From<DidEvents_> for DidEvents {
    fn from(event: DidEvents_) -> Self {
        match event {
            DidEvents_::AttributeChangedEvent(event) => DidEvents::AttributeChangedEvent {
                event: event.into(),
            },
            DidEvents_::DelegateChanged(event) => DidEvents::DelegateChanged {
                event: event.into(),
            },
            DidEvents_::OwnerChanged(event) => DidEvents::OwnerChanged {
                event: event.into(),
            },
        }
    }
}

#[derive(uniffi::Record)]
pub struct DidResolutionOptions {
    pub accept: String,
}

impl From<DidResolutionOptions> for DidResolutionOptions_ {
    fn from(options: DidResolutionOptions) -> Self {
        DidResolutionOptions_ {
            accept: options.accept,
        }
    }
}
