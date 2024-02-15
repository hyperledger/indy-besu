use log_derive::{logfn, logfn_inputs};

use crate::{
    client::LedgerClient,
    contracts::cl::types::{
        schema::{Schema, SchemaCreatedEvent},
        schema_id::SchemaId,
    },
    error::VdrResult,
    types::{
        Address, EventParser, EventQueryBuilder, MethodStringParam, Transaction,
        TransactionBuilder, TransactionEndorsingDataBuilder, TransactionParser, TransactionType,
    },
    Block, EventLog, EventQuery, SignatureData, TransactionEndorsingData, VdrError,
};

const CONTRACT_NAME: &str = "SchemaRegistry";
const METHOD_CREATE_SCHEMA: &str = "createSchema";
const METHOD_CREATE_SCHEMA_SIGNED: &str = "createSchemaSigned";
const METHOD_SCHEMA_CREATED: &str = "created";
const EVENT_SCHEMA_CREATED: &str = "SchemaCreated";

/// Build transaction to execute SchemaRegistry.createSchema contract method to create a new Schema
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `from` transaction sender account address
/// - `id` id of schema to be created
/// - `schema` Schema object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:schema
///
/// # Returns
/// Write transaction to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_schema_transaction(
    client: &LedgerClient,
    from: &Address,
    schema: &Schema,
) -> VdrResult<Transaction> {
    schema.validate()?;
    let identity = Address::try_from(&schema.issuer_id)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_SCHEMA)
        .add_param(&identity)?
        .add_param(&schema.id())?
        .add_param(schema)?
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await
}

/// Prepared data for execution of SchemaRegistry.createSchema contract method to endorse a new Schema
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `id` id of schema to be created
///  - `schema` Schema object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:schema
///
/// #Returns
///   data: TransactionEndorsingData - transaction endorsement data to sign
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_schema_endorsing_data(
    client: &LedgerClient,
    schema: &Schema,
) -> VdrResult<TransactionEndorsingData> {
    schema.validate()?;
    let identity = Address::try_from(&schema.issuer_id)?;
    TransactionEndorsingDataBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_identity(&identity)
        .add_param(&identity)?
        .add_param(MethodStringParam::from(METHOD_CREATE_SCHEMA))?
        .add_param(&schema.id())?
        .add_param(schema)?
        .build(client)
        .await
}

/// Build transaction to execute SchemaRegistry.createSchemaSigned contract method to
///   endorse a new Schema
/// Endorsing version of the method - sender is not identity owner
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `id` id of schema to be created
///  - `schema` Schema object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:schema
///  - `signature` signature of schema issuer
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_schema_signed_transaction(
    client: &LedgerClient,
    sender: &Address,
    schema: &Schema,
    signature: &SignatureData,
) -> VdrResult<Transaction> {
    schema.validate()?;
    let identity = Address::try_from(&schema.issuer_id)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_SCHEMA_SIGNED)
        .add_param(&identity)?
        .add_param(signature.v())?
        .add_param(signature.r())?
        .add_param(signature.s())?
        .add_param(&schema.id())?
        .add_param(schema)?
        .set_type(TransactionType::Write)
        .set_from(sender)
        .build(client)
        .await
}

/// Build transaction to execute SchemaRegistry.schemasCreated contract method to get
///   block number when Schema was created
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `id` identifier of target schema
///
/// #Returns
///   transaction: Transaction - prepared read transaction object to submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_get_schema_created_transaction(
    client: &LedgerClient,
    id: &SchemaId,
) -> VdrResult<Transaction> {
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_SCHEMA_CREATED)
        .add_param(id)?
        .set_type(TransactionType::Read)
        .build(client)
        .await
}

/// Build event query to get SchemaRegistry.SchemaCreated event from the ledger
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `id` identifier of target schema
///  - `from_block` start block
///  - `to_block` finish block
///
/// #Returns
///   query: EventQuery - prepared event query to send
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_get_schema_query(
    client: &LedgerClient,
    id: &SchemaId,
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

/// Parse the result of execution SchemaRegistry.schemas contract method to receive
///   block number when a schema was created
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Block when the schema was created
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_schema_created_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Block> {
    TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_SCHEMA_CREATED)
        .parse::<Block>(client, bytes)
}

/// Parse SchemaRegistry.SchemaCreated from the event log.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Parsed Schema object
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_schema_created_event(
    client: &LedgerClient,
    log: &EventLog,
) -> VdrResult<SchemaCreatedEvent> {
    EventParser::new()
        .set_contract(CONTRACT_NAME)
        .set_event(EVENT_SCHEMA_CREATED)
        .parse(client, log)
}

/// Single step function to resolve a Schema for the given ID
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `id` id of schema to resolve
///
/// # Returns
///   Resolved Schema object
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn resolve_schema(client: &LedgerClient, id: &SchemaId) -> VdrResult<Schema> {
    let transaction = build_get_schema_created_transaction(client, id).await?;
    let response = client.submit_transaction(&transaction).await?;
    let created_block = parse_schema_created_result(client, &response)?;

    let schema_query =
        build_get_schema_query(client, id, Some(&created_block), Some(&created_block)).await?;
    let events = client.query_events(&schema_query).await?;

    if events.is_empty() {
        return Err(VdrError::ClientInvalidResponse(format!(
            "Schema not found for id: {:?}",
            id
        )));
    }

    if events.len() > 1 {
        return Err(VdrError::ClientInvalidResponse(format!(
            "More then one schema resolved for the given id: {:?}",
            id
        )));
    }

    let schema = parse_schema_created_event(client, &events[0])?.schema;

    let schema_id = schema.id();
    if &schema_id != id {
        return Err(VdrError::InvalidSchema(format!(
            "Schema ID {} does not match to requested {}",
            schema_id.to_string(),
            id.to_string()
        )));
    }

    Ok(schema)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        client::client::test::{
            mock_client, CHAIN_ID, DEFAULT_NONCE, SCHEMA_REGISTRY_ADDRESS, TRUSTEE_ACC,
        },
        contracts::{
            cl::types::schema::test::{schema, SCHEMA_ATTRIBUTES, SCHEMA_NAME, SCHEMA_VERSION},
            did::types::{did::DID, did_doc::test::ISSUER_ID},
        },
        utils::init_env_logger,
    };
    use std::sync::RwLock;

    mod build_create_schema_transaction {
        use super::*;
        use rstest::rstest;
        use std::collections::HashSet;

        #[async_std::test]
        async fn build_create_schema_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let (_, schema) = schema(&DID::from(ISSUER_ID), Some(SCHEMA_NAME));
            let transaction = build_create_schema_transaction(&client, &TRUSTEE_ACC, &schema)
                .await
                .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(TRUSTEE_ACC.clone()),
                to: SCHEMA_REGISTRY_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: vec![
                    89, 21, 183, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 226, 219, 108, 141,
                    198, 198, 129, 187, 93, 106, 209, 33, 161, 7, 243, 0, 233, 178, 181, 34, 27,
                    23, 130, 143, 227, 3, 94, 147, 14, 185, 63, 10, 50, 145, 115, 71, 104, 106,
                    145, 232, 190, 123, 84, 240, 64, 217, 94, 167, 52, 119, 152, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 141, 123, 34, 105, 115, 115, 117, 101, 114, 73, 100, 34, 58, 34,
                    100, 105, 100, 58, 101, 116, 104, 114, 58, 116, 101, 115, 116, 110, 101, 116,
                    58, 48, 120, 102, 48, 101, 50, 100, 98, 54, 99, 56, 100, 99, 54, 99, 54, 56,
                    49, 98, 98, 53, 100, 54, 97, 100, 49, 50, 49, 97, 49, 48, 55, 102, 51, 48, 48,
                    101, 57, 98, 50, 98, 53, 34, 44, 34, 110, 97, 109, 101, 34, 58, 34, 70, 49, 68,
                    67, 108, 97, 70, 69, 122, 105, 51, 116, 34, 44, 34, 118, 101, 114, 115, 105,
                    111, 110, 34, 58, 34, 49, 46, 48, 46, 48, 34, 44, 34, 97, 116, 116, 114, 78,
                    97, 109, 101, 115, 34, 58, 91, 34, 70, 105, 114, 115, 116, 32, 78, 97, 109,
                    101, 34, 93, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }

        #[rstest]
        #[case::name_not_provided("", SCHEMA_VERSION, &SCHEMA_ATTRIBUTES)]
        #[case::version_not_provided(SCHEMA_NAME, "", &SCHEMA_ATTRIBUTES)]
        #[case::attributes_not_provided(SCHEMA_NAME, SCHEMA_VERSION, &HashSet::new())]
        async fn build_create_schema_transaction_errors(
            #[case] name: &str,
            #[case] version: &str,
            #[case] attributes: &HashSet<String>,
        ) {
            init_env_logger();
            let client = mock_client();
            let (_, mut schema) = schema(&DID::from(ISSUER_ID), Some(name));
            schema.name = name.to_string();
            schema.version = version.to_string();
            schema.attr_names = attributes.clone();

            let err = build_create_schema_transaction(&client, &TRUSTEE_ACC, &schema)
                .await
                .unwrap_err();

            assert!(matches!(err, VdrError::InvalidSchema { .. }));
        }
    }

    mod build_get_schema_query {
        use super::*;

        #[async_std::test]
        async fn build_get_schema_query_test() {
            init_env_logger();
            let client = mock_client();
            let (id, _) = schema(&DID::from(ISSUER_ID), Some(SCHEMA_NAME));
            let query = build_get_schema_query(&client, &id, None, None)
                .await
                .unwrap();
            let expected_query = EventQuery {
                address: SCHEMA_REGISTRY_ADDRESS.clone(),
                from_block: None,
                to_block: None,
                event_signature: None,
                event_filter: Some(
                    "221b17828fe3035e930eb93f0a32917347686a91e8be7b54f040d95ea7347798".to_string(),
                ),
            };
            assert_eq!(expected_query, query);
        }
    }
    //
    // mod parse_resolve_schema_result {
    //     use super::*;
    //     use crate::contracts::did::types::did::DID;
    //
    //     #[test]
    //     fn parse_resolve_schema_result_test() {
    //         init_env_logger();
    //         let client = mock_client();
    //         let data = vec![
    //             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //             0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //             0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 166, 63, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 134, 123, 34, 97, 116,
    //             116, 114, 78, 97, 109, 101, 115, 34, 58, 91, 34, 70, 105, 114, 115, 116, 32, 78,
    //             97, 109, 101, 34, 44, 34, 76, 97, 115, 116, 32, 78, 97, 109, 101, 34, 93, 44, 34,
    //             105, 115, 115, 117, 101, 114, 73, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110,
    //             100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115,
    //             122, 107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 34,
    //             44, 34, 110, 97, 109, 101, 34, 58, 34, 70, 49, 68, 67, 108, 97, 70, 69, 122, 105,
    //             51, 116, 34, 44, 34, 118, 101, 114, 115, 105, 111, 110, 34, 58, 34, 49, 46, 48, 46,
    //             48, 34, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //             0, 0, 0,
    //         ];
    //         let parsed_schema = parse_resolve_schema_result(&client, &data).unwrap();
    //         let (_, expected_schema) = schema(&DID::from(ISSUER_ID), Some(SCHEMA_NAME));
    //         assert_eq!(expected_schema, parsed_schema);
    //     }
    // }
}
