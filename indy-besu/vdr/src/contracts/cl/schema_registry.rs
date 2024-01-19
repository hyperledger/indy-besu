use log::{debug, info};

use crate::{
    client::LedgerClient,
    contracts::cl::types::{
        schema::{Schema, SchemaRecord},
        schema_id::SchemaId,
    },
    error::VdrResult,
    types::{Address, Transaction, TransactionBuilder, TransactionParser, TransactionType},
};

const CONTRACT_NAME: &str = "SchemaRegistry";
const METHOD_CREATE_SCHEMA: &str = "createSchema";
const METHOD_RESOLVE_SCHEMA: &str = "resolveSchema";

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
pub async fn build_create_schema_transaction(
    client: &LedgerClient,
    from: &Address,
    id: &SchemaId,
    schema: &Schema,
) -> VdrResult<Transaction> {
    debug!(
        "{} txn build has started. Sender: {:?}, schema: {:?}",
        METHOD_CREATE_SCHEMA, from, schema
    );

    // TODO: validate schema

    let transaction = TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_SCHEMA)
        .add_param(id.into())
        .add_param((&schema.issuer_id).into())
        .add_param(schema.into())
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await?;

    info!(
        "{} txn build has finished. Result: {:?}",
        METHOD_CREATE_SCHEMA, transaction
    );

    Ok(transaction)
}

/// Build transaction to execute SchemaRegistry.resolveSchema contract method to retrieve an existing Schema by the given id
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `id` id of Schema to resolve
///
/// # Returns
/// Read transaction to submit
pub async fn build_resolve_schema_transaction(
    client: &LedgerClient,
    id: &SchemaId,
) -> VdrResult<Transaction> {
    debug!(
        "{} txn build has started. Schema ID: {:?}",
        METHOD_RESOLVE_SCHEMA, id
    );

    let transaction = TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_RESOLVE_SCHEMA)
        .add_param(id.into())
        .set_type(TransactionType::Read)
        .build(client)
        .await?;

    info!(
        "{} txn build has finished. Result: {:?}",
        METHOD_RESOLVE_SCHEMA, transaction
    );

    Ok(transaction)
}

/// Parse the result of execution SchemaRegistry.resolveSchema contract method to receive a Schema associated with the id
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
/// parsed Schema
pub fn parse_resolve_schema_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Schema> {
    debug!(
        "{} result parse has started. Bytes to parse: {:?}",
        METHOD_RESOLVE_SCHEMA, bytes
    );

    let schema = TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_RESOLVE_SCHEMA)
        .parse::<SchemaRecord>(client, bytes)?
        .schema;

    // TODO: validate schema

    info!(
        "{} result parse has finished. Result: {:?}",
        METHOD_RESOLVE_SCHEMA, schema
    );

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
            cl::types::schema::test::{schema, SCHEMA_NAME},
            did::types::{did::DID, did_doc::test::ISSUER_ID},
        },
        utils::init_env_logger,
    };
    use std::sync::RwLock;

    mod build_create_schema_transaction {
        use super::*;

        #[async_std::test]
        async fn build_create_schema_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let (id, schema) = schema(&DID::from(ISSUER_ID), Some(SCHEMA_NAME));
            let transaction = build_create_schema_transaction(&client, &TRUSTEE_ACC, &id, &schema)
                .await
                .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(TRUSTEE_ACC.clone()),
                to: SCHEMA_REGISTRY_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: vec![
                    54, 206, 23, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 64, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 79, 100, 105, 100, 58, 105, 110, 100, 121, 50, 58, 116, 101, 115, 116,
                    110, 101, 116, 58, 51, 76, 112, 106, 115, 122, 107, 103, 84, 109, 69, 51, 113,
                    84, 104, 103, 101, 50, 53, 70, 90, 119, 47, 97, 110, 111, 110, 99, 114, 101,
                    100, 115, 47, 118, 48, 47, 83, 67, 72, 69, 77, 65, 47, 70, 49, 68, 67, 108, 97,
                    70, 69, 122, 105, 51, 116, 47, 49, 46, 48, 46, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 100, 105, 100, 58, 105, 110, 100,
                    121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122,
                    107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 134, 123, 34, 97, 116, 116, 114, 78, 97, 109, 101, 115, 34, 58, 91, 34, 70,
                    105, 114, 115, 116, 32, 78, 97, 109, 101, 34, 44, 34, 76, 97, 115, 116, 32, 78,
                    97, 109, 101, 34, 93, 44, 34, 105, 115, 115, 117, 101, 114, 73, 100, 34, 58,
                    34, 100, 105, 100, 58, 105, 110, 100, 121, 50, 58, 116, 101, 115, 116, 110,
                    101, 116, 58, 51, 76, 112, 106, 115, 122, 107, 103, 84, 109, 69, 51, 113, 84,
                    104, 103, 101, 50, 53, 70, 90, 119, 34, 44, 34, 110, 97, 109, 101, 34, 58, 34,
                    70, 49, 68, 67, 108, 97, 70, 69, 122, 105, 51, 116, 34, 44, 34, 118, 101, 114,
                    115, 105, 111, 110, 34, 58, 34, 49, 46, 48, 46, 48, 34, 125, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }
    }

    mod build_resolve_schema_transaction {
        use super::*;

        #[async_std::test]
        async fn build_resolve_schema_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let (id, _) = schema(&DID::from(ISSUER_ID), Some(SCHEMA_NAME));
            let transaction = build_resolve_schema_transaction(&client, &id)
                .await
                .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Read,
                from: None,
                to: SCHEMA_REGISTRY_ADDRESS.clone(),
                nonce: None,
                chain_id: CHAIN_ID,
                data: vec![
                    189, 127, 197, 235, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 100, 105, 100, 58, 105,
                    110, 100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106,
                    115, 122, 107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90,
                    119, 47, 97, 110, 111, 110, 99, 114, 101, 100, 115, 47, 118, 48, 47, 83, 67,
                    72, 69, 77, 65, 47, 70, 49, 68, 67, 108, 97, 70, 69, 122, 105, 51, 116, 47, 49,
                    46, 48, 46, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }
    }

    mod parse_resolve_schema_result {
        use super::*;
        use crate::contracts::did::types::did::DID;

        #[test]
        fn parse_resolve_schema_result_test() {
            init_env_logger();
            let client = mock_client();
            let data = vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 166, 63, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 134, 123, 34, 97, 116,
                116, 114, 78, 97, 109, 101, 115, 34, 58, 91, 34, 70, 105, 114, 115, 116, 32, 78,
                97, 109, 101, 34, 44, 34, 76, 97, 115, 116, 32, 78, 97, 109, 101, 34, 93, 44, 34,
                105, 115, 115, 117, 101, 114, 73, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110,
                100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115,
                122, 107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 34,
                44, 34, 110, 97, 109, 101, 34, 58, 34, 70, 49, 68, 67, 108, 97, 70, 69, 122, 105,
                51, 116, 34, 44, 34, 118, 101, 114, 115, 105, 111, 110, 34, 58, 34, 49, 46, 48, 46,
                48, 34, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0,
            ];
            let parsed_schema = parse_resolve_schema_result(&client, &data).unwrap();
            let (_, expected_schema) = schema(&DID::from(ISSUER_ID), Some(SCHEMA_NAME));
            assert_eq!(expected_schema, parsed_schema);
        }
    }
}
