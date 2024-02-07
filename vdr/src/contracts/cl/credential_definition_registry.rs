use log_derive::{logfn, logfn_inputs};

use crate::{
    client::LedgerClient,
    contracts::cl::types::{
        credential_definition::{CredentialDefinition, CredentialDefinitionCreatedEvent},
        credential_definition_id::CredentialDefinitionId,
    },
    error::VdrResult,
    types::{
        Address, EventParser, EventQueryBuilder, MethodStringParam, Transaction,
        TransactionBuilder, TransactionEndorsingDataBuilder, TransactionParser, TransactionType,
    },
    Block, EventLog, EventQuery, SignatureData, TransactionEndorsingData, VdrError,
};

const CONTRACT_NAME: &str = "CredentialDefinitionRegistry";
const METHOD_CREATE_CREDENTIAL_DEFINITION: &str = "createCredentialDefinition";
const METHOD_CREATE_CREDENTIAL_DEFINITION_SIGNED: &str = "createCredentialDefinitionSigned";
const METHOD_CREDENTIAL_DEFINITION_CREATED: &str = "created";
const EVENT_CREDENTIAL_DEFINITION_CREATED: &str = "CredentialDefinitionCreated";

/// Build transaction to execute CredentialDefinitionRegistry.createCredentialDefinition contract
/// method to create a new Credential Definition
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `from` transaction sender account address
/// - `id` id of credential definition to be created
/// - `credential_definition` Credential Definition object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:credential-definition
///
/// # Returns
/// Write transaction to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_credential_definition_transaction(
    client: &LedgerClient,
    from: &Address,
    id: &CredentialDefinitionId,
    credential_definition: &CredentialDefinition,
) -> VdrResult<Transaction> {
    credential_definition.validate()?;
    let identity = Address::try_from(&credential_definition.issuer_id)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_CREDENTIAL_DEFINITION)
        .add_param(&identity)?
        .add_param(id)?
        .add_param(&credential_definition.schema_id)?
        .add_param(credential_definition)?
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await
}

/// Prepared data for endorsing CredentialDefinitionRegistry.createCredentialDefinition contract method
///
/// #Params
///  - `client` client connected to the network where contract will be executed
/// - `id` id of credential definition to be created
/// - `credential_definition` Credential Definition object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:credential-definition
///
/// #Returns
///   data: TransactionEndorsingData - transaction endorsement data to sign
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_credential_definition_endorsing_data(
    client: &LedgerClient,
    id: &CredentialDefinitionId,
    credential_definition: &CredentialDefinition,
) -> VdrResult<TransactionEndorsingData> {
    credential_definition.validate()?;
    let identity = Address::try_from(&credential_definition.issuer_id)?;
    TransactionEndorsingDataBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_identity(&identity)
        .add_param(&identity)?
        .add_param(MethodStringParam::from(METHOD_CREATE_CREDENTIAL_DEFINITION))?
        .add_param(id)?
        .add_param(&credential_definition.schema_id)?
        .add_param(credential_definition)?
        .build(client)
        .await
}

/// Build transaction to execute CredentialDefinitionRegistry.createCredentialDefinitionSigned contract method to
///   endorse a new Credential Definition
/// Endorsing version of the method - sender is not identity owner
///
/// #Params
///  - `client` client connected to the network where contract will be executed
/// - `id` id of credential definition to be created
/// - `credential_definition` Credential Definition object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:credential-definition
///  - `signature` signature of schema issuer
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_credential_definition_signed_transaction(
    client: &LedgerClient,
    from: &Address,
    id: &CredentialDefinitionId,
    credential_definition: &CredentialDefinition,
    signature: &SignatureData,
) -> VdrResult<Transaction> {
    credential_definition.validate()?;
    let identity = Address::try_from(&credential_definition.issuer_id)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_CREDENTIAL_DEFINITION_SIGNED)
        .add_param(&identity)?
        .add_param(signature.v())?
        .add_param(signature.r())?
        .add_param(signature.s())?
        .add_param(id)?
        .add_param(&credential_definition.schema_id)?
        .add_param(credential_definition)?
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await
}

/// Build transaction to execute CredentialDefinitionRegistry.credDefs contract method to get
///   block number when a Credential Definition was created
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `id` identifier of target credential definition
///
/// #Returns
///   transaction: Transaction - prepared read transaction object to submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_get_credential_definition_created_transaction(
    client: &LedgerClient,
    id: &CredentialDefinitionId,
) -> VdrResult<Transaction> {
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREDENTIAL_DEFINITION_CREATED)
        .add_param(id)?
        .set_type(TransactionType::Read)
        .build(client)
        .await
}

/// Build event query to get CredentialDefinitionRegistry.CredentialDefinitionCreated event from the ledger
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `id` identifier of target credential definition
///  - `from_block` start block
///  - `to_block` finish block
///
/// #Returns
///   query: EventQuery - prepared event query to send
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_get_credential_definition_query(
    client: &LedgerClient,
    id: &CredentialDefinitionId,
    from_block: Option<&Block>,
    to_block: Option<&Block>,
) -> VdrResult<EventQuery> {
    EventQueryBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_from_block(from_block.cloned())
        .set_to_block(to_block.cloned())
        .set_event_filer(id.to_filter())
        .build(client)
}

/// Parse the result of execution CredentialDefinitionRegistry.credDefs contract method to receive
///   block number when a credential definition was created
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Block when the credential definition was created
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_credential_definition_created_result(
    client: &LedgerClient,
    bytes: &[u8],
) -> VdrResult<Block> {
    TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREDENTIAL_DEFINITION_CREATED)
        .parse::<Block>(client, bytes)
}

/// Parse CredentialDefinitionRegistry.CredentialDefinitionCreated from the event log.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Parsed Credential Definition event object
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_credential_definition_created_event(
    client: &LedgerClient,
    log: &EventLog,
) -> VdrResult<CredentialDefinitionCreatedEvent> {
    EventParser::new()
        .set_contract(CONTRACT_NAME)
        .set_event(EVENT_CREDENTIAL_DEFINITION_CREATED)
        .parse(client, log)
}

/// Single step function to resolve a Credential Definition for the given ID
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `id` id of schema to resolve
///
/// # Returns
///   Resolved Credential Definition object
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn resolve_credential_definition(
    client: &LedgerClient,
    id: &CredentialDefinitionId,
) -> VdrResult<CredentialDefinition> {
    let transaction = build_get_credential_definition_created_transaction(client, id).await?;
    let response = client.submit_transaction(&transaction).await?;
    let created_block = parse_credential_definition_created_result(client, &response)?;

    let schema_query = build_get_credential_definition_query(
        client,
        id,
        Some(&created_block),
        Some(&created_block),
    )
    .await?;
    let events = client.query_events(&schema_query).await?;

    if events.len() != 1 {
        return Err(VdrError::ClientInvalidResponse(
            format!("Unable to resolve schema: Unexpected amount of schema created events received for id: {:?}", id)
        ));
    }

    let cred_def = parse_credential_definition_created_event(client, &events[0])?.cred_def;
    cred_def.matches_id(id)?;

    Ok(cred_def)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        client::client::test::{
            mock_client, CHAIN_ID, CRED_DEF_REGISTRY_ADDRESS, DEFAULT_NONCE, TRUSTEE_ACC,
        },
        contracts::{
            cl::types::{
                credential_definition::test::{credential_definition, CREDENTIAL_DEFINITION_TAG},
                schema::test::SCHEMA_ID,
                schema_id::SchemaId,
            },
            did::types::{did::DID, did_doc::test::ISSUER_ID},
        },
        utils::init_env_logger,
    };
    use std::sync::RwLock;

    mod build_create_credential_definition_transaction {
        use super::*;
        use serde_json::Value;

        #[async_std::test]
        async fn build_create_credential_definition_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let (id, cred_def) = credential_definition(
                &DID::from(ISSUER_ID),
                &SchemaId::from(SCHEMA_ID),
                Some(CREDENTIAL_DEFINITION_TAG),
            );
            let transaction = build_create_credential_definition_transaction(
                &client,
                &TRUSTEE_ACC,
                &id,
                &cred_def,
            )
            .await
            .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(TRUSTEE_ACC.clone()),
                to: CRED_DEF_REGISTRY_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: vec![
                    187, 199, 66, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 226, 219, 108, 141,
                    198, 198, 129, 187, 93, 106, 209, 33, 161, 7, 243, 0, 233, 178, 181, 190, 177,
                    72, 242, 21, 171, 224, 191, 86, 212, 4, 12, 89, 70, 109, 83, 153, 187, 19, 51,
                    18, 37, 31, 233, 114, 33, 60, 132, 133, 72, 249, 229, 34, 27, 23, 130, 143,
                    227, 3, 94, 147, 14, 185, 63, 10, 50, 145, 115, 71, 104, 106, 145, 232, 190,
                    123, 84, 240, 64, 217, 94, 167, 52, 119, 152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                    42, 123, 34, 105, 115, 115, 117, 101, 114, 73, 100, 34, 58, 34, 100, 105, 100,
                    58, 101, 116, 104, 114, 58, 116, 101, 115, 116, 110, 101, 116, 58, 48, 120,
                    102, 48, 101, 50, 100, 98, 54, 99, 56, 100, 99, 54, 99, 54, 56, 49, 98, 98, 53,
                    100, 54, 97, 100, 49, 50, 49, 97, 49, 48, 55, 102, 51, 48, 48, 101, 57, 98, 50,
                    98, 53, 34, 44, 34, 115, 99, 104, 101, 109, 97, 73, 100, 34, 58, 34, 100, 105,
                    100, 58, 101, 116, 104, 114, 58, 116, 101, 115, 116, 110, 101, 116, 58, 48,
                    120, 102, 48, 101, 50, 100, 98, 54, 99, 56, 100, 99, 54, 99, 54, 56, 49, 98,
                    98, 53, 100, 54, 97, 100, 49, 50, 49, 97, 49, 48, 55, 102, 51, 48, 48, 101, 57,
                    98, 50, 98, 53, 47, 97, 110, 111, 110, 99, 114, 101, 100, 115, 47, 118, 48, 47,
                    83, 67, 72, 69, 77, 65, 47, 70, 49, 68, 67, 108, 97, 70, 69, 122, 105, 51, 116,
                    47, 49, 46, 48, 46, 48, 34, 44, 34, 99, 114, 101, 100, 68, 101, 102, 84, 121,
                    112, 101, 34, 58, 34, 67, 76, 34, 44, 34, 116, 97, 103, 34, 58, 34, 100, 101,
                    102, 97, 117, 108, 116, 34, 44, 34, 118, 97, 108, 117, 101, 34, 58, 123, 34,
                    110, 34, 58, 34, 55, 55, 57, 46, 46, 46, 51, 57, 55, 34, 44, 34, 114, 99, 116,
                    120, 116, 34, 58, 34, 55, 55, 52, 46, 46, 46, 57, 55, 55, 34, 44, 34, 115, 34,
                    58, 34, 55, 53, 48, 46, 46, 56, 57, 51, 34, 44, 34, 122, 34, 58, 34, 54, 51,
                    50, 46, 46, 46, 48, 48, 53, 34, 125, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }

        #[async_std::test]
        async fn build_create_credential_definition_transaction_no_tag() {
            init_env_logger();
            let client = mock_client();
            let (id, mut cred_def) = credential_definition(
                &DID::from(ISSUER_ID),
                &SchemaId::from(SCHEMA_ID),
                Some(CREDENTIAL_DEFINITION_TAG),
            );
            cred_def.tag = "".to_string();

            let err = build_create_credential_definition_transaction(
                &client,
                &TRUSTEE_ACC,
                &id,
                &cred_def,
            )
            .await
            .unwrap_err();
            assert!(matches!(err, VdrError::InvalidCredentialDefinition { .. }));
        }

        #[async_std::test]
        async fn build_create_credential_definition_transaction_no_value() {
            init_env_logger();
            let client = mock_client();
            let (id, mut cred_def) = credential_definition(
                &DID::from(ISSUER_ID),
                &SchemaId::from(SCHEMA_ID),
                Some(CREDENTIAL_DEFINITION_TAG),
            );
            cred_def.value = Value::Null;

            let err = build_create_credential_definition_transaction(
                &client,
                &TRUSTEE_ACC,
                &id,
                &cred_def,
            )
            .await
            .unwrap_err();
            assert!(matches!(err, VdrError::InvalidCredentialDefinition { .. }));
        }
    }

    mod build_resolve_credential_definition_transaction {
        use super::*;

        #[async_std::test]
        async fn build_get_credential_definition_query_test() {
            init_env_logger();
            let client = mock_client();
            let (id, _) = credential_definition(
                &DID::from(ISSUER_ID),
                &SchemaId::from(SCHEMA_ID),
                Some(CREDENTIAL_DEFINITION_TAG),
            );
            let query = build_get_credential_definition_query(&client, &id, None, None)
                .await
                .unwrap();
            let expected_query = EventQuery {
                address: CRED_DEF_REGISTRY_ADDRESS.clone(),
                from_block: None,
                to_block: None,
                event_signature: None,
                event_filter: Some(
                    "beb148f215abe0bf56d4040c59466d5399bb133312251fe972213c848548f9e5".to_string(),
                ),
            };
            assert_eq!(expected_query, query);
        }
    }

    // mod parse_resolve_credential_definition_result {
    //     use super::*;
    //
    //     #[test]
    //     fn parse_resolve_credential_definition_result_test() {
    //         init_env_logger();
    //         let client = mock_client();
    //         let data = vec![
    //             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //             0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //             0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 166, 63, 232, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 123, 34, 99, 114,
    //             101, 100, 68, 101, 102, 84, 121, 112, 101, 34, 58, 34, 67, 76, 34, 44, 34, 105,
    //             115, 115, 117, 101, 114, 73, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110, 100,
    //             121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122,
    //             107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 34, 44, 34,
    //             115, 99, 104, 101, 109, 97, 73, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110, 100,
    //             121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122,
    //             107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 47, 97,
    //             110, 111, 110, 99, 114, 101, 100, 115, 47, 118, 48, 47, 83, 67, 72, 69, 77, 65, 47,
    //             70, 49, 68, 67, 108, 97, 70, 69, 122, 105, 51, 116, 47, 49, 46, 48, 46, 48, 34, 44,
    //             34, 116, 97, 103, 34, 58, 34, 100, 101, 102, 97, 117, 108, 116, 34, 44, 34, 118,
    //             97, 108, 117, 101, 34, 58, 123, 34, 110, 34, 58, 34, 55, 55, 57, 46, 46, 46, 51,
    //             57, 55, 34, 44, 34, 114, 99, 116, 120, 116, 34, 58, 34, 55, 55, 52, 46, 46, 46, 57,
    //             55, 55, 34, 44, 34, 115, 34, 58, 34, 55, 53, 48, 46, 46, 56, 57, 51, 34, 44, 34,
    //             122, 34, 58, 34, 54, 51, 50, 46, 46, 46, 48, 48, 53, 34, 125, 125, 0, 0, 0, 0, 0,
    //             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //         ];
    //         let parsed_cred_def =
    //             parse_resolve_credential_definition_result(&client, &data).unwrap();
    //         let (_, expected_cred_def) = credential_definition(
    //             &DID::from(ISSUER_ID),
    //             &SchemaId::from(SCHEMA_ID),
    //             Some(CREDENTIAL_DEFINITION_TAG),
    //         );
    //         assert_eq!(expected_cred_def, parsed_cred_def);
    //     }
    // }
}
