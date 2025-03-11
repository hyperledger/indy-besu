// Copyright (c) 2024 DSR Corporation, Denver, Colorado.
// https://www.dsr-corporation.com
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use chrono::Utc;
use log_derive::{logfn, logfn_inputs};

use crate::{
    client::LedgerClient,
    contracts::anoncreds::types::{
        revocation_registry_definition::RevocationRegistryDefinition,
        revocation_registry_definition_id::RevocationRegistryDefinitionId,
    },
    error::VdrResult,
    types::{
        Address, Transaction, TransactionBuilder, TransactionEndorsingDataBuilder,
        TransactionParser, TransactionType,
    },
    Block, EventLog, EventParser, EventQuery, EventQueryBuilder, RevocationRegistryEntryData,
    TransactionEndorsingData, VdrError,
};

use super::types::{
    revocation_registry_definition::RevocationRegistryDefinitionRecord,
    revocation_registry_definition_id::ParsedRevocationRegistryDefinitionId,
    revocation_registry_delta::{RevocationRegistryDelta, RevocationState, RevocationStatusList},
    revocation_registry_entry::RevocationRegistryEntry,
    revocation_registry_events::{RevRegEntryCreated, RevocationRegistryEvents},
};

const CONTRACT_NAME: &str = "RevocationRegistry";
const METHOD_CREATE_REVOCATION_REGISTRY_DEFINITION: &str = "createRevocationRegistryDefinition";
const METHOD_CREATE_REVOCATION_REGISTRY_ENTRY: &str = "createRevocationRegistryEntry";
const METHOD_RESOLVE_REVOCATION_REGISTRY_DEFINITION: &str = "resolveRevocationRegistryDefinition";
const METHOD_CREATE_REVOCATION_REGISTRY_DEFINITION_SIGNED: &str =
    "createRevocationRegistryDefinitionSigned";

const EVENT_REV_REG_ENTRY_CREATED: &str = "RevocationRegistryEntryCreated";

/// Build a transaction to create a new Revocation Registry Definition record (RevocationRegistry.createRevocationRegistryDefinition contract method)
///
/// # Params
/// - `client`: [LedgerClient] - client connected to the network where contract will be executed
/// - `from`: [Address] - transaction sender account address
/// - `revocation_registry_definition`: [RevocationRegistryDefinition] - object matching to the specification - `<https://hyperledger.github.io/anoncreds-spec/#term:revocation-registry-definition>`
///
/// # Returns
///   transaction: [Transaction] - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_revocation_registry_definition_transaction(
    client: &LedgerClient,
    from: &Address,
    revocation_registry_definition: &RevocationRegistryDefinition,
) -> VdrResult<Transaction> {
    revocation_registry_definition.validate()?;
    let identity = Address::try_from(&revocation_registry_definition.issuer_id)?;

    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_REVOCATION_REGISTRY_DEFINITION)
        .add_param(&identity)?
        .add_param(&revocation_registry_definition.id())?
        .add_param(
            &revocation_registry_definition
                .cred_def_id
                .without_network()?,
        )?
        .add_param(&revocation_registry_definition.issuer_id.without_network()?)?
        .add_param(revocation_registry_definition)?
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await
}

/// Build a transaction to create a new Revocation Registry Entry record (RevocationRegistry.createRevocationRegistryEntry contract method)
///
/// # Params
/// - `client`: [LedgerClient] - client connected to the network where contract will be executed
/// - `from`: [Address] - transaction sender account address
/// - `revocation_registry_entry`: [RevocationRegistryEntry] - object matching to the specification - `<https://hyperledger.github.io/anoncreds-spec/#term:revocation-registry-entries>`
///
/// # Returns
///   transaction: [Transaction] - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_revocation_registry_entry_transaction(
    client: &LedgerClient,
    from: &Address,
    revocation_registry_entry: &RevocationRegistryEntry,
) -> VdrResult<Transaction> {
    revocation_registry_entry.validate()?;
    let identity = Address::try_from(&revocation_registry_entry.issuer_id)?;

    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_REVOCATION_REGISTRY_ENTRY)
        .add_param(&identity)?
        .add_param(&revocation_registry_entry.rev_reg_def_id)?
        .add_param(&revocation_registry_entry.issuer_id.without_network()?)?
        .add_param(&revocation_registry_entry.rev_reg_entry_data)?
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await
}

/// Prepared data for endorsing creation of a new Revocation Registry Definition record
///     (RevocationRegistry.createRevocationRegistryDefinitionSigned contract method)
///
/// #Params
///  - `client`: [LedgerClient] - client connected to the network where contract will be executed
/// - `revocation_registry_definition`: [RevocationRegistryDefinition] - object matching to the specification - `<https://hyperledger.github.io/anoncreds-spec/#term:revocation-registry-definition>`
///
/// #Returns
///   data: [TransactionEndorsingData] - transaction endorsement data to sign
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_revocation_registry_definition_endorsing_data(
    client: &LedgerClient,
    revocation_registry_definition: &RevocationRegistryDefinition,
) -> VdrResult<TransactionEndorsingData> {
    revocation_registry_definition.validate()?;
    TransactionEndorsingDataBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_identity(&Address::try_from(
            &revocation_registry_definition.issuer_id,
        )?)
        .set_method(METHOD_CREATE_REVOCATION_REGISTRY_DEFINITION)
        .set_endorsing_method(METHOD_CREATE_REVOCATION_REGISTRY_DEFINITION_SIGNED)
        .add_param(&revocation_registry_definition.id().without_network()?)?
        .add_param(
            &revocation_registry_definition
                .cred_def_id
                .without_network()?,
        )?
        .add_param(&revocation_registry_definition.issuer_id.without_network()?)?
        .add_param(revocation_registry_definition)?
        .build(client)
        .await
}

/// Build a transaction to resolve an existing Revocation Registry Definition record by the given id
///  (RevocationRegistry.resolveRevocationRegistryDefinition contract method)
///
/// # Params
/// - `client`: [LedgerClient] - client connected to the network where contract will be executed
/// - `id`: [RevocationRegistryId] - id of Revocation Registry to resolve
///
/// # Returns
///   transaction: [Transaction] - prepared read transaction object to submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_resolve_revocation_registry_definition_transaction(
    client: &LedgerClient,
    id: &RevocationRegistryDefinitionId,
) -> VdrResult<Transaction> {
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_RESOLVE_REVOCATION_REGISTRY_DEFINITION)
        .add_param(id)?
        .set_type(TransactionType::Read)
        .build(client)
        .await
}

/// Parse the result of execution RevocationRegistry.resolveRevocationRegistryDefinition contract
/// method to receive a Revocation Registry Definition associated with the id
///
/// # Params
/// - `client`: [LedgerClient] - client connected to the network where contract will be executed
/// - `bytes`: [Vec] - result bytes returned from the ledger
///
/// # Returns
///   record: [RevocationRegistryDefinitionRecord] - parsed Revocation Registry Definition Record
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_resolve_revocation_registry_definition_result(
    client: &LedgerClient,
    bytes: &[u8],
) -> VdrResult<RevocationRegistryDefinitionRecord> {
    TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_RESOLVE_REVOCATION_REGISTRY_DEFINITION)
        .parse::<RevocationRegistryDefinitionRecord>(client, bytes)
}

/// Single step function to resolve a Revocation Registry Definition for the given ID
///
/// # Params
/// - `client`: [LedgerClient] - client connected to the network where contract will be executed
/// - `id`: [RevocationRegistryId] - id of Revocation Registry Definition to resolve
///
/// # Returns
///   credential_definition: [CredentialDefinition] - Resolved Credential Definition object
pub async fn resolve_revocation_registry_definition(
    client: &LedgerClient,
    id: &RevocationRegistryDefinitionId,
) -> VdrResult<RevocationRegistryDefinition> {
    let parsed_id = ParsedRevocationRegistryDefinitionId::try_from(id)?;
    match (parsed_id.network.as_ref(), client.network()) {
        (Some(schema_network), Some(client_network)) if schema_network != client_network => {
            return Err(VdrError::InvalidRevocationRegistryDefinition(format!("Network of request revocation registry definition id {} does not match to the client network {}", schema_network, client_network)));
        }
        _ => {}
    };

    let transaction = build_resolve_revocation_registry_definition_transaction(client, id).await?;
    let response = client.submit_transaction(&transaction).await?;
    if response.is_empty() {
        return Err(VdrError::ClientInvalidResponse(format!(
            "Revocation Registry Definition not found for id: {:?}",
            id
        )));
    }
    let rev_reg_def_record =
        parse_resolve_revocation_registry_definition_result(client, &response)?;

    let rev_reg_def_id = rev_reg_def_record.revocation_registry_definition.id();
    if &rev_reg_def_id != id {
        return Err(VdrError::InvalidRevocationRegistryDefinition(format!(
            "Revocation Registry Definition ID {} does not match to requested {}",
            rev_reg_def_id.to_string(),
            id.to_string()
        )));
    }

    Ok(rev_reg_def_record.revocation_registry_definition)
}

/// Single step function to resolve a Revocation Registry Status List for the given ID at a given
/// timestamp
///
/// # Params
/// - `client`: [LedgerClient] - client connected to the network where contract will be executed
/// - `id`: [RevocationRegistryId] - id of Revocation Registry Definition to resolve
/// - `to_timestamp`: [u64] - timestamp of the Revocation Registry Status List resolution
///
/// # Returns
///   revocation_status_list: [RevocationStatusList] - Resolved Revocation Status List object
pub async fn resolve_revocation_registry_status_list(
    client: &LedgerClient,
    id: &RevocationRegistryDefinitionId,
    to_timestamp: u64,
) -> VdrResult<RevocationStatusList> {
    let rev_reg_def = resolve_revocation_registry_definition(&client, &id).await?;
    let delta = fetch_revocation_delta(&client, &id, to_timestamp)
        .await?
        .ok_or(VdrError::InvalidRevocationRegistryStatusList(format!(
            "No revocation status list found for Revocation Registry Definition ID {}",
            id.as_ref()
        )))?;

    delta.validate(rev_reg_def.value.max_cred_num - 1)?;

    let mut revocation_list: Vec<u32> = vec![0; rev_reg_def.value.max_cred_num.try_into().unwrap()];

    // Set all `issuer` indexes to 0 (not revoked)
    for issue in delta.issued {
        revocation_list[issue as usize] = RevocationState::Active as u32
    }

    // Set all `revoked` indexes to 1 (revoked)
    for revocation in delta.revoked {
        revocation_list[revocation as usize] = RevocationState::Revoked as u32
    }

    Ok(RevocationStatusList {
        issuer_id: rev_reg_def.issuer_id.clone(),
        rev_reg_def_id: id.clone(),
        revocation_list,
        current_accumulator: delta.accum,
        timestamp: to_timestamp,
    })
}

/// Function to create a new Revocation Registry Entry object for a Revocation Registry Status List for the given Revocation Registry Definition ID
/// # Params
/// - `client`: [LedgerClient] - client connected to the network where contract will be executed
/// - `id`: [RevocationRegistryId] - id of Revocation Registry Definition to resolve
/// - `revocation_registry_status_list`: [Vec<u8>] - Revocation Registry Status list according to '<https://hyperledger.github.io/anoncreds-spec/#term:revocation-status-list>'
/// # Returns
///   transaction: [Transaction] - prepared read transaction object to submit
pub async fn build_latest_revocation_registry_entry_from_status_list(
    client: &LedgerClient,
    id: &RevocationRegistryDefinitionId,
    revocation_registry_status_list: &Vec<RevocationState>,
    accumulator: String,
) -> VdrResult<RevocationRegistryEntry> {
    let rev_reg_def = resolve_revocation_registry_definition(&client, &id).await?;
    if revocation_registry_status_list.len() as u32 > rev_reg_def.value.max_cred_num {
        return Err(VdrError::InvalidRevocationRegistryStatusList(format!(
            "Revocation Status List has more elements ({}) than Revocation Registry MaxCredNum ({})",
            revocation_registry_status_list.len(),
            rev_reg_def.value.max_cred_num
        )));
    }

    let timestamp: u64 = Utc::now().timestamp().try_into().unwrap();

    let previous_delta = fetch_revocation_delta(&client, &id, timestamp).await?;

    let rev_reg_entry_data = build_latest_revocation_registry_entry_data(
        revocation_registry_status_list,
        &previous_delta,
        accumulator,
        timestamp,
    )?;

    Ok(RevocationRegistryEntry {
        issuer_id: rev_reg_def.issuer_id,
        rev_reg_def_id: id.clone(),
        rev_reg_entry_data,
    })
}

/// Function to fetch the revocation delta up to a given timestamp for the given Revocation Registry ID
///
/// # Params
/// - `client`: [LedgerClient] - client connected to the network where contract will be executed
/// - `id`: [RevocationRegistryId] - id of Revocation Registry Definition to resolve
/// - `to_timestamp`: [u64] - timestamp of the status list to be retrieved
///
/// # Returns
///   rev_reg_delta: [Option<RevocationRegistryDelta>] - Resolved RevocationRegistryDelta object
pub async fn fetch_revocation_delta(
    client: &LedgerClient,
    id: &RevocationRegistryDefinitionId,
    to_timestamp: u64,
) -> VdrResult<Option<RevocationRegistryDelta>> {
    let rev_reg_entries = fetch_revocation_registry_entries(&client, &id, to_timestamp).await?;

    if rev_reg_entries.is_empty() {
        return Ok(None);
    }

    //TODO: sets? vectors?
    let mut issued: HashSet<u32> = HashSet::new();
    let mut revoked: HashSet<u32> = HashSet::new();

    let accum = rev_reg_entries
        .last()
        .unwrap()
        .rev_reg_entry
        .current_accumulator
        .clone();

    for rev_reg_entry in rev_reg_entries.into_iter() {
        for issue in rev_reg_entry.rev_reg_entry.issued {
            issued.insert(issue);

            revoked.remove(&issue);
        }

        for revocation in rev_reg_entry.rev_reg_entry.revoked {
            issued.remove(&revocation);

            revoked.insert(revocation);
        }
    }

    let mut issued: Vec<u32> = issued.into_iter().collect();
    issued.sort();
    let mut revoked: Vec<u32> = revoked.into_iter().collect();
    revoked.sort();

    Ok(Some(RevocationRegistryDelta {
        revoked,
        issued,
        accum,
    }))
}

/// Function to build a new revocation delta to save on the blockchain from a previous revocation delta
///
/// # Params
/// - `revocation_registry_status_list`: [Vec<RevocationState>] - new desired Revocation Status
/// List
/// - `previous_delta`: [Option<RevocationRegistryDelta>] - previous delta saved on the blockchain
/// - `accumulator`: [String] - new accumulator
/// - `timestamp`: [String] - timestamp
///
/// # Returns
///   rev_reg_delta: [RevocationRegistryEntryData] - RevocationRegistryDelta object
fn build_latest_revocation_registry_entry_data(
    revocation_registry_status_list: &Vec<RevocationState>,
    previous_delta: &Option<RevocationRegistryDelta>,
    accumulator: String,
    timestamp: u64,
) -> VdrResult<RevocationRegistryEntryData> {
    let mut issued: Vec<u32> = Vec::new();
    let mut revoked: Vec<u32> = Vec::new();

    match previous_delta {
        Some(previous_delta) => {
            previous_delta.validate(revocation_registry_status_list.len() as u32 - 1)?;
            // Check whether the revocationStatusList entry is not included in the previous delta issued indices
            for (index, entry) in (0u32..).zip(revocation_registry_status_list.iter()) {
                if RevocationState::Active == *entry && !previous_delta.issued.contains(&index) {
                    issued.push(index);
                }

                // Check whether the revocationStatusList entry is not included in the previous delta revoked indices
                if RevocationState::Revoked == *entry && !previous_delta.revoked.contains(&index) {
                    revoked.push(index);
                }
            }
        }
        None => {
            // No delta is provided, initial state, so the entire revocation status list is converted to two list of indices
            for (index, entry) in (0u32..).zip(revocation_registry_status_list.iter()) {
                match entry {
                    RevocationState::Active => issued.push(index),
                    RevocationState::Revoked => revoked.push(index),
                }
            }
        }
    }

    Ok(RevocationRegistryEntryData {
        issued,
        revoked,
        current_accumulator: accumulator,
        prev_accumulator: previous_delta
            .as_ref()
            .map_or(String::from("0x"), |prev_delta| prev_delta.accum.clone()),
        timestamp,
    })
}

/// Function to fetch all revocation registry entries before a given timestamp for the given Revocation Registry ID
///
/// # Params
/// - `client`: [LedgerClient] - client connected to the network where contract will be executed
/// - `id`: [RevocationRegistryId] - id of Revocation Registry Definition to resolve
/// - `to_timestamp`: [u64] - timestamp of the status list to be retrieved
///
/// # Returns
///   rev_reg_entries: [Vec<RevRegEntryCreated>] - Revocation Registry Entries
#[allow(irrefutable_let_patterns)]
async fn fetch_revocation_registry_entries(
    client: &LedgerClient,
    id: &RevocationRegistryDefinitionId,
    to_timestamp: u64,
) -> VdrResult<Vec<RevRegEntryCreated>> {
    let parsed_id = ParsedRevocationRegistryDefinitionId::try_from(id)?;
    match (parsed_id.network.as_ref(), client.network()) {
        (Some(schema_network), Some(client_network)) if schema_network != client_network => {
            return Err(VdrError::InvalidRevocationRegistryDefinition(format!("Network of request revocation registry definition id {} does not match to the client network {}", schema_network, client_network)));
        }
        _ => {}
    };

    let rev_registry_events = receive_revocation_registry_history(&client, id).await?;

    Ok(rev_registry_events
        .into_iter()
        .filter_map(|rev_reg_event| {
            // Check if the event is a RevocationRegistryEntryCreatedEvent
            if let RevocationRegistryEvents::RevocationRegistryEntryCreatedEvent(
                rev_reg_entry_created,
            ) = rev_reg_event
            {
                // If the timestamp is less than or equal to `to_timestamp`, include it
                if rev_reg_entry_created.timestamp <= to_timestamp {
                    Some(rev_reg_entry_created) // Include the event
                } else {
                    None // Exclude events with timestamp greater than `to_timestamp`
                }
            } else {
                None // If the event is not a `RevocationRegistryEntryCreatedEvent`, ignore it
            }
        })
        .collect())
}

#[logfn(Trace)]
#[logfn_inputs(Trace)]
async fn receive_revocation_registry_history(
    client: &LedgerClient,
    rev_reg_def_id: &RevocationRegistryDefinitionId,
) -> VdrResult<Vec<RevocationRegistryEvents>> {
    let mut history: Vec<RevocationRegistryEvents> = Vec::new();

    let transaction =
        build_get_revocation_registry_entry_events_query(client, rev_reg_def_id, None, None)
            .await?;
    let logs = client.query_events(&transaction).await?;

    // parse events
    for log in logs {
        let event = parse_revocation_registry_event_response(client, &log)?;
        history.push(event);
    }
    Ok(history)
}

/// Build event query to obtain revocation registry entry associated events from the ledger
///
/// #Params
///  - `client`: [LedgerClient] - client connected to the network where contract will be executed
///  - `rev_reg_def_id`: [RevocationRegistryDefinitionId] - associated Revocation Registry Definition Id
///  - `from_block`: [Block] - start block
///  - `to_block`: [Block] - finish block
///
/// #Returns
///   query: [EventQuery] - prepared event query to send
#[logfn(Info)]
#[logfn_inputs(Debug)]
async fn build_get_revocation_registry_entry_events_query(
    client: &LedgerClient,
    rev_reg_def_id: &RevocationRegistryDefinitionId,
    from_block: Option<&Block>,
    to_block: Option<&Block>,
) -> VdrResult<EventQuery> {
    EventQueryBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_from_block(from_block.cloned())
        .set_to_block(to_block.cloned())
        .set_event_filer(rev_reg_def_id.to_filter())
        .build(client)
}

/// Parse Revocation Registry Event from the event log.
///
/// # Params
/// - `client`: [LedgerClient] - client connected to the network where contract will be executed
/// - `bytes`: [Vec] - result bytes returned from the ledger
///
/// # Returns
///   event: [RevocationRegistryEvents] Parsed Revocation Registry Event object
#[logfn(Info)]
#[logfn_inputs(Debug)]
fn parse_revocation_registry_event_response(
    client: &LedgerClient,
    event: &EventLog,
) -> VdrResult<RevocationRegistryEvents> {
    let contract = client.contract(CONTRACT_NAME)?;

    let event_signature = event.topics.first().ok_or_else(|| {
        VdrError::ContractInvalidResponseData("Unable to get event topic".to_string())
    })?;

    let rev_reg_entry_created_event_signature =
        contract.event(EVENT_REV_REG_ENTRY_CREATED)?.signature();

    if event_signature.eq(&rev_reg_entry_created_event_signature) {
        return parse_rev_reg_entry_created_event_response(client, event)
            .map(RevocationRegistryEvents::RevocationRegistryEntryCreatedEvent);
    }

    Err(VdrError::ContractInvalidResponseData(format!(
        "Unexpected contract event. Event signature: {:?}",
        event_signature
    )))
}

/// Parse RevocationRegistryEntryCreated from the event log.
///
/// # Params
/// - `client`: [LedgerClient] - client connected to the network where contract will be executed
/// - `bytes`: [Vec] - result bytes returned from the ledger
///
/// # Returns
///   event: [RevRegEntryCreated] - Parsed RevocationRegistryEntryCreated event object
#[logfn(Info)]
#[logfn_inputs(Debug)]
fn parse_rev_reg_entry_created_event_response(
    client: &LedgerClient,
    log: &EventLog,
) -> VdrResult<RevRegEntryCreated> {
    EventParser::new()
        .set_contract(CONTRACT_NAME)
        .set_event(EVENT_REV_REG_ENTRY_CREATED)
        .parse::<RevRegEntryCreated>(client, log)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        client::client::test::{
            client, mock_client, CONFIG, DEFAULT_NONCE, TEST_ACCOUNT, TRUSTEE_ACCOUNT,
        },
        contracts::did::types::{
            did::DID,
            did_doc::test::{TEST_ETHR_DID, TEST_ETHR_DID_WITHOUT_NETWORK},
        },
    };
    use rstest::rstest;

    mod build_create_revocation_registry_transaction {
        use crate::{
            contracts::anoncreds::types::{
                credential_definition::test::{
                    CREDENTIAL_DEFINITION_ID, CREDENTIAL_DEFINITION_ID_WITHOUT_NETWORK,
                },
                revocation_registry_definition::test::{
                    revocation_registry_definition, REVOCATION_REGISTRY_DEFINITION_ID,
                    REVOCATION_REGISTRY_DEFINITION_TAG,
                },
                revocation_registry_entry::test::revocation_registry_entry,
            },
            CredentialDefinitionId,
        };

        use super::*;
        use serde_json::Value;

        #[async_std::test]
        async fn build_create_revocation_registry_definition_transaction_test() {
            let client = mock_client();
            let rev_reg_def = revocation_registry_definition(
                &DID::from(TEST_ETHR_DID_WITHOUT_NETWORK),
                &&CredentialDefinitionId::from(CREDENTIAL_DEFINITION_ID_WITHOUT_NETWORK),
                Some(REVOCATION_REGISTRY_DEFINITION_TAG),
            );
            let transaction = build_create_revocation_registry_definition_transaction(
                &client,
                &TEST_ACCOUNT,
                &rev_reg_def,
            )
            .await
            .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(TEST_ACCOUNT.clone()),
                to: CONFIG.contracts.revocation_registry.address.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CONFIG.chain_id,
                data: vec![
                    101, 201, 204, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 226, 219, 108, 141,
                    198, 198, 129, 187, 93, 106, 209, 33, 161, 7, 243, 0, 233, 178, 181, 165, 228,
                    7, 74, 86, 110, 19, 20, 48, 146, 220, 87, 85, 67, 141, 24, 151, 19, 147, 167,
                    9, 100, 136, 80, 1, 146, 59, 15, 146, 90, 38, 142, 252, 84, 206, 140, 139, 17,
                    81, 102, 58, 13, 78, 38, 76, 85, 25, 157, 189, 250, 139, 220, 160, 110, 164,
                    90, 238, 147, 145, 200, 16, 72, 81, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 51, 100, 105, 100, 58, 101, 116, 104, 114, 58, 48, 120, 102, 48,
                    101, 50, 100, 98, 54, 99, 56, 100, 99, 54, 99, 54, 56, 49, 98, 98, 53, 100, 54,
                    97, 100, 49, 50, 49, 97, 49, 48, 55, 102, 51, 48, 48, 101, 57, 98, 50, 98, 53,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 209, 123, 34, 105, 115,
                    115, 117, 101, 114, 73, 100, 34, 58, 34, 100, 105, 100, 58, 101, 116, 104, 114,
                    58, 48, 120, 102, 48, 101, 50, 100, 98, 54, 99, 56, 100, 99, 54, 99, 54, 56,
                    49, 98, 98, 53, 100, 54, 97, 100, 49, 50, 49, 97, 49, 48, 55, 102, 51, 48, 48,
                    101, 57, 98, 50, 98, 53, 34, 44, 34, 114, 101, 118, 111, 99, 68, 101, 102, 84,
                    121, 112, 101, 34, 58, 34, 67, 76, 95, 65, 67, 67, 85, 77, 34, 44, 34, 99, 114,
                    101, 100, 68, 101, 102, 73, 100, 34, 58, 34, 100, 105, 100, 58, 101, 116, 104,
                    114, 58, 48, 120, 102, 48, 101, 50, 100, 98, 54, 99, 56, 100, 99, 54, 99, 54,
                    56, 49, 98, 98, 53, 100, 54, 97, 100, 49, 50, 49, 97, 49, 48, 55, 102, 51, 48,
                    48, 101, 57, 98, 50, 98, 53, 47, 97, 110, 111, 110, 99, 114, 101, 100, 115, 47,
                    118, 48, 47, 67, 76, 65, 73, 77, 95, 68, 69, 70, 47, 100, 105, 100, 58, 101,
                    116, 104, 114, 58, 48, 120, 102, 48, 101, 50, 100, 98, 54, 99, 56, 100, 99, 54,
                    99, 54, 56, 49, 98, 98, 53, 100, 54, 97, 100, 49, 50, 49, 97, 49, 48, 55, 102,
                    51, 48, 48, 101, 57, 98, 50, 98, 53, 58, 70, 49, 68, 67, 108, 97, 70, 69, 122,
                    105, 51, 116, 58, 49, 46, 48, 46, 48, 47, 100, 101, 102, 97, 117, 108, 116, 34,
                    44, 34, 116, 97, 103, 34, 58, 34, 116, 97, 103, 34, 44, 34, 118, 97, 108, 117,
                    101, 34, 58, 123, 34, 109, 97, 120, 67, 114, 101, 100, 78, 117, 109, 34, 58,
                    54, 54, 54, 44, 34, 112, 117, 98, 108, 105, 99, 75, 101, 121, 115, 34, 58, 123,
                    34, 97, 99, 99, 117, 109, 75, 101, 121, 34, 58, 123, 34, 122, 34, 58, 34, 49,
                    32, 48, 66, 66, 46, 46, 46, 51, 56, 54, 34, 125, 125, 44, 34, 116, 97, 105,
                    108, 115, 72, 97, 115, 104, 34, 58, 34, 57, 49, 122, 118, 113, 50, 99, 70, 109,
                    66, 90, 109, 72, 67, 99, 76, 113, 70, 121, 122, 118, 55, 98, 102, 101, 104, 72,
                    72, 53, 114, 77, 104, 100, 65, 71, 53, 119, 84, 106, 113, 121, 50, 80, 69, 34,
                    44, 34, 116, 97, 105, 108, 115, 76, 111, 99, 97, 116, 105, 111, 110, 34, 58,
                    34, 104, 116, 116, 112, 115, 58, 47, 47, 109, 121, 46, 114, 101, 118, 111, 99,
                    97, 116, 105, 111, 110, 115, 46, 116, 97, 105, 108, 115, 47, 116, 97, 105, 108,
                    115, 102, 105, 108, 101, 46, 116, 120, 116, 34, 125, 125, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                ],
                signature: None,
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }

        #[async_std::test]
        async fn build_create_revocation_registry_entry_transaction_test() {
            let client = mock_client();
            let rev_reg_entry = revocation_registry_entry(
                &DID::from(TEST_ETHR_DID_WITHOUT_NETWORK),
                &&RevocationRegistryDefinitionId::from(REVOCATION_REGISTRY_DEFINITION_ID),
            );
            let transaction = build_create_revocation_registry_entry_transaction(
                &client,
                &TEST_ACCOUNT,
                &rev_reg_entry,
            )
            .await
            .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(TEST_ACCOUNT.clone()),
                to: CONFIG.contracts.revocation_registry.address.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CONFIG.chain_id,
                data: vec![
                    240, 62, 44, 129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 226, 219, 108, 141,
                    198, 198, 129, 187, 93, 106, 209, 33, 161, 7, 243, 0, 233, 178, 181, 165, 228,
                    7, 74, 86, 110, 19, 20, 48, 146, 220, 87, 85, 67, 141, 24, 151, 19, 147, 167,
                    9, 100, 136, 80, 1, 146, 59, 15, 146, 90, 38, 142, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 51, 100, 105, 100, 58, 101, 116, 104, 114, 58, 48, 120,
                    102, 48, 101, 50, 100, 98, 54, 99, 56, 100, 99, 54, 99, 54, 56, 49, 98, 98, 53,
                    100, 54, 97, 100, 49, 50, 49, 97, 49, 48, 55, 102, 51, 48, 48, 101, 57, 98, 50,
                    98, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 103, 69, 205, 244, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    12, 99, 117, 114, 114, 101, 110, 116, 65, 99, 99, 117, 109, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 112, 114, 101, 118,
                    65, 99, 99, 117, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                signature: None,
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }

        #[rstest]
        #[case("", TEST_ETHR_DID)]
        #[case(
            REVOCATION_REGISTRY_DEFINITION_TAG,
            "did:ethr:testnet:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        )]
        async fn build_create_revocation_registry_definition_transaction_errors(
            #[case] tag: &str,
            #[case] did: &str,
        ) {
            let client = mock_client();
            let mut rev_reg_def = revocation_registry_definition(
                &DID::from(did),
                &&CredentialDefinitionId::from(CREDENTIAL_DEFINITION_ID),
                Some(tag),
            );
            rev_reg_def.tag = tag.to_string();

            let err = build_create_revocation_registry_definition_transaction(
                &client,
                &TRUSTEE_ACCOUNT,
                &rev_reg_def,
            )
            .await
            .unwrap_err();

            assert!(matches!(
                err,
                VdrError::InvalidRevocationRegistryDefinition { .. }
            ));
        }
    }

    mod build_resolve_revocation_registry_definition_transaction {
        use crate::{
            contracts::anoncreds::types::{
                credential_definition::test::CREDENTIAL_DEFINITION_ID_WITHOUT_NETWORK,
                revocation_registry_definition::test::{
                    revocation_registry_definition, REVOCATION_REGISTRY_DEFINITION_TAG,
                },
            },
            CredentialDefinitionId,
        };

        use super::*;

        #[async_std::test]
        async fn build_resolve_revocation_registry_definition_transaction_test() {
            let client = mock_client();
            let rev_reg_def = revocation_registry_definition(
                &DID::from(TEST_ETHR_DID_WITHOUT_NETWORK),
                &&CredentialDefinitionId::from(CREDENTIAL_DEFINITION_ID_WITHOUT_NETWORK),
                Some(REVOCATION_REGISTRY_DEFINITION_TAG),
            );
            let transaction = build_resolve_revocation_registry_definition_transaction(
                &client,
                &rev_reg_def.id(),
            )
            .await
            .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Read,
                from: None,
                to: CONFIG.contracts.revocation_registry.address.clone(),
                nonce: None,
                chain_id: CONFIG.chain_id,
                data: vec![
                    255, 81, 99, 190, 165, 228, 7, 74, 86, 110, 19, 20, 48, 146, 220, 87, 85, 67,
                    141, 24, 151, 19, 147, 167, 9, 100, 136, 80, 1, 146, 59, 15, 146, 90, 38, 142,
                ],
                signature: None,
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }
    }

    //TODO:
    // mod build_resolve_revocation_registry_events {
    //     use crate::contracts::anoncreds::types::{
    //         credential_definition::test::CREDENTIAL_DEFINITION_ID_WITHOUT_NETWORK,
    //         revocation_registry_definition::test::{
    //             revocation_registry_definition, REVOCATION_REGISTRY_DEFINITION_TAG,
    //         },
    //     };
    //
    //     use super::*;
    //
    //     #[async_std::test]
    //     async fn receive_revocation_registry_history_test() {
    //         let client = client();
    //         // let rev_reg_def = revocation_registry_definition(
    //         //     &DID::from(TEST_ETHR_DID_WITHOUT_NETWORK),
    //         //     &&CredentialDefinitionId::from(CREDENTIAL_DEFINITION_ID_WITHOUT_NETWORK),
    //         //     Some(REVOCATION_REGISTRY_DEFINITION_TAG),
    //         // );
    //         let rev_reg_def_id = RevocationRegistryDefinitionId::from("did:ethr:0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73/anoncreds/v0/REV_REG_DEF/did:ethr:0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73:schema-name:1.0/definition/revocation");
    //         let logs = receive_revocation_registry_history(&client, &rev_reg_def_id).await;
    //         assert!(logs.is_ok());
    //
    //         let logs = logs.unwrap();
    //     }
    // }
}
