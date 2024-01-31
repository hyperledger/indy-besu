use chrono::Utc;
use log_derive::{logfn, logfn_inputs};

use crate::{
    contracts::{
        did::{types::did_doc_attribute::PublicKeyPurpose, DidResolutionError},
        DidDocumentWithMeta, DidMetadata, DidResolutionMetadata,
    },
    did_ethr_registry::{
        build_get_did_changed_transaction, build_get_did_events_query, parse_did_changed_result,
        parse_did_event_response, ETHR_DID_METHOD,
    },
    Block, DelegateType, DidAttributeChanged, DidDelegateChanged, DidDocAttribute, DidDocument,
    DidDocumentBuilder, DidEvents, DidOwnerChanged, DidResolutionOptions, LedgerClient, VdrResult,
    VerificationKeyType, DID,
};

#[logfn(Info)]
#[logfn_inputs(Debug)]
pub(crate) async fn resolve_did(
    client: &LedgerClient,
    did: &DID,
    options: Option<&DidResolutionOptions>,
) -> VdrResult<DidDocumentWithMeta> {
    // TODO: Validate DID
    // DID without network identifier
    let did = match did.short() {
        Ok(did) => did,
        Err(_) => {
            return Ok(DidDocumentWithMeta {
                did_document: None,
                did_document_metadata: DidMetadata::default(),
                did_resolution_metadata: DidResolutionMetadata {
                    content_type: None,
                    error: Some(DidResolutionError::InvalidDid),
                    message: Some(format!("Not a valid did:ethr: {}", did.as_ref())),
                },
            });
        }
    };

    let content_type = options
        .map(|options| options.accept.clone())
        .unwrap_or_default();

    match _resolve_did(client, &did, options).await {
        Ok((did_document, did_metadata)) => Ok(DidDocumentWithMeta {
            did_document: Some(did_document),
            did_document_metadata: did_metadata,
            did_resolution_metadata: DidResolutionMetadata {
                content_type: Some(content_type),
                error: None,
                message: None,
            },
        }),
        Err(err) => Ok(DidDocumentWithMeta {
            did_document: None,
            did_document_metadata: DidMetadata::default(),
            did_resolution_metadata: DidResolutionMetadata {
                content_type: None,
                error: Some(DidResolutionError::NotFound),
                message: Some(err.to_string()),
            },
        }),
    }
}

#[logfn(Info)]
#[logfn_inputs(Debug)]
pub(crate) async fn _resolve_did(
    client: &LedgerClient,
    did: &DID,
    options: Option<&DidResolutionOptions>,
) -> VdrResult<(DidDocument, DidMetadata)> {
    // time in seconds for attributes validity check
    let now = match options.and_then(|options| options.block_tag.as_ref()) {
        Some(block) => {
            // request block time if the resolution happens for specific block
            client.get_block(Some(block)).await?.timestamp
        }
        None => {
            // else current time
            Utc::now().timestamp() as u64
        }
    };

    // Build base DID document for ethr DID
    let mut did_doc_builder = DidDocumentBuilder::base_for_did(did, client.chain_id())?;

    // TODO: support the case when DID identifier is public key

    // Query block number when DID was changed last time
    let did_changed_block = get_did_changed_block(client, did).await?;

    // if DID has not been ever changed, we do not need to query events and just return base did document
    if did_changed_block.is_none() {
        let did_document = did_doc_builder.build();
        return Ok((did_document, DidMetadata::default()));
    }

    let block_height: i64 = match options.and_then(|options| options.block_tag.as_ref()) {
        Some(block_tag) => block_tag.value() as i64,
        None => -1, // latest
    };
    let mut version_id = 0;
    let mut next_version_id = u64::MAX;

    // request events for a specific block until previous exists
    let did_history = receive_did_history(client, did, did_changed_block).await?;

    // assemble Did Document from the history events
    //  iterate in the reverse order -> oldest to newest
    for (event_block, event) in did_history.iter().rev() {
        if block_height != -1 && event_block.value() as i64 > block_height {
            if next_version_id > event_block.value() {
                next_version_id = event_block.value()
            }
        } else {
            version_id = event_block.value();
        }

        handle_did_event(&mut did_doc_builder, event, client, now)?;

        if did_doc_builder.deactivated {
            break;
        }
    }

    let did_document_metadata = build_did_metadata(
        client,
        did_doc_builder.deactivated,
        version_id,
        next_version_id,
    )
    .await?;
    let did_document = did_doc_builder.build();
    Ok((did_document, did_document_metadata))
}

#[logfn(Trace)]
#[logfn_inputs(Trace)]
async fn get_did_changed_block(client: &LedgerClient, did: &DID) -> VdrResult<Block> {
    let transaction = build_get_did_changed_transaction(client, did).await?;
    let response = client.submit_transaction(&transaction).await?;
    parse_did_changed_result(client, &response)
}

#[logfn(Trace)]
#[logfn_inputs(Trace)]
async fn receive_did_history(
    client: &LedgerClient,
    did: &DID,
    first_block: Block,
) -> VdrResult<Vec<(Block, DidEvents)>> {
    let mut history: Vec<(Block, DidEvents)> = Vec::new();
    let mut previous_block: Option<Block> = Some(first_block);
    while previous_block.is_some() {
        let transaction = build_get_did_events_query(
            client,
            did,
            previous_block.as_ref(),
            previous_block.as_ref(),
        )
        .await?;
        let logs = client.query_events(&transaction).await?;

        // if no logs, break the loop as nothing to add to the change history
        if logs.is_empty() {
            break;
        }

        // parse events
        for log in logs {
            let event = parse_did_event_response(client, &log)?;
            previous_block = Some(event.previous_change());
            history.push((log.block, event));
        }
    }
    Ok(history)
}

#[logfn(Trace)]
#[logfn_inputs(Trace)]
fn handle_did_owner_changed(
    did_doc_builder: &mut DidDocumentBuilder,
    event: &DidOwnerChanged,
) -> VdrResult<()> {
    if event.owner.is_null() {
        // DID is considered to be deactivated
        did_doc_builder.deactivated();
        return Ok(());
    }

    let controller = DID::build(ETHR_DID_METHOD, None, event.owner.as_ref());
    did_doc_builder.set_controller(controller.as_ref());
    Ok(())
}

#[logfn(Trace)]
#[logfn_inputs(Trace)]
fn handle_did_delegate_changed(
    did_doc_builder: &mut DidDocumentBuilder,
    event: &DidDelegateChanged,
    now: u64,
    client: &LedgerClient,
) -> VdrResult<()> {
    let event_index = event.key();
    let delegate_type = DelegateType::try_from(event.delegate_type.as_slice())?;
    let controller = did_doc_builder.id.clone();

    did_doc_builder.increment_key_index();

    if event.valid_to > now {
        did_doc_builder.add_verification_method(
            &event_index,
            None,
            &VerificationKeyType::EcdsaSecp256k1RecoveryMethod2020,
            &controller,
            Some(event.delegate.as_blockchain_id(client.chain_id()).as_str()),
            None,
            None,
            None,
            None,
        );

        match delegate_type {
            DelegateType::VeriKey => {
                did_doc_builder.add_assertion_method_reference(&event_index)?;
            }
            DelegateType::SigAuth => {
                did_doc_builder.add_authentication_reference(&event_index)?;
            }
        }
    } else {
        // delegate expired
        did_doc_builder.remove_verification_method(&event_index);
        match delegate_type {
            DelegateType::VeriKey => {
                did_doc_builder.remove_assertion_method_reference(&event_index);
            }
            DelegateType::SigAuth => {
                did_doc_builder.remove_authentication_reference(&event_index);
            }
        }
    };
    Ok(())
}

#[logfn(Trace)]
#[logfn_inputs(Trace)]
fn handle_did_attribute_changed(
    did_doc_builder: &mut DidDocumentBuilder,
    event: &DidAttributeChanged,
    now: u64,
) -> VdrResult<()> {
    let event_index = event.key();

    match DidDocAttribute::try_from(event)? {
        DidDocAttribute::PublicKey(key) => {
            did_doc_builder.increment_key_index();
            let controller = did_doc_builder.id.clone();

            if event.valid_to > now {
                did_doc_builder.add_verification_method(
                    &event_index,
                    None,
                    &key.type_.into(),
                    &controller,
                    None,
                    None,
                    key.public_key_hex.as_deref(),
                    key.public_key_base58.as_deref(),
                    key.public_key_base64.as_deref(),
                );

                match key.purpose {
                    PublicKeyPurpose::VeriKey => {
                        did_doc_builder.add_assertion_method_reference(&event_index)?;
                    }
                    PublicKeyPurpose::SigAuth => {
                        did_doc_builder.add_authentication_reference(&event_index)?;
                    }
                    PublicKeyPurpose::Enc => {
                        did_doc_builder.add_key_agreement_reference(&event_index)?;
                    }
                }
            } else {
                // key expired
                did_doc_builder.remove_verification_method(&event_index);
                match key.purpose {
                    PublicKeyPurpose::VeriKey => {
                        did_doc_builder.remove_assertion_method_reference(&event_index);
                    }
                    PublicKeyPurpose::SigAuth => {
                        did_doc_builder.remove_authentication_reference(&event_index);
                    }
                    PublicKeyPurpose::Enc => {
                        did_doc_builder.remove_key_agreement_reference(&event_index);
                    }
                }
            }
        }
        DidDocAttribute::Service(service) => {
            did_doc_builder.increment_service_index();

            if event.valid_to > now {
                did_doc_builder.add_service(
                    &event_index,
                    None,
                    &service.type_,
                    &service.service_endpoint,
                );
            } else {
                did_doc_builder.remove_service(&event_index);
            }
        }
    };
    Ok(())
}

#[logfn(Trace)]
#[logfn_inputs(Trace)]
fn handle_did_event(
    did_doc_builder: &mut DidDocumentBuilder,
    event: &DidEvents,
    client: &LedgerClient,
    now: u64,
) -> VdrResult<()> {
    match event {
        DidEvents::OwnerChanged(event) => handle_did_owner_changed(did_doc_builder, event),
        DidEvents::DelegateChanged(event) => {
            handle_did_delegate_changed(did_doc_builder, event, now, client)
        }
        DidEvents::AttributeChangedEvent(event) => {
            handle_did_attribute_changed(did_doc_builder, event, now)
        }
    }
}

#[logfn(Trace)]
#[logfn_inputs(Trace)]
async fn build_did_metadata(
    client: &LedgerClient,
    deactivated: bool,
    version_id: u64,
    next_version_id: u64,
) -> VdrResult<DidMetadata> {
    let mut did_document_metadata = DidMetadata {
        deactivated: Some(deactivated),
        ..DidMetadata::default()
    };

    if version_id != 0 {
        let block = client.get_block(Some(&Block::from(version_id))).await?;
        did_document_metadata.updated = Some(block.timestamp);
        did_document_metadata.version_id = Some(block.number);
    }

    if next_version_id != u64::MAX {
        let block = client
            .get_block(Some(&Block::from(next_version_id)))
            .await?;
        did_document_metadata.next_update = Some(block.timestamp);
        did_document_metadata.next_version_id = Some(block.number);
    }

    Ok(did_document_metadata)
}
