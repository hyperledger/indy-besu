use log::{debug, info};

use crate::{
    client::LedgerClient,
    contracts::cl::types::{
        credential_definition::{CredentialDefinition, CredentialDefinitionRecord},
        credential_definition_id::CredentialDefinitionId,
    },
    error::VdrResult,
    types::{Address, Transaction, TransactionBuilder, TransactionParser, TransactionType},
};

const CONTRACT_NAME: &str = "CredentialDefinitionRegistry";
const METHOD_CREATE_CREDENTIAL_DEFINITION: &str = "createCredentialDefinition";
const METHOD_RESOLVE_CREDENTIAL_DEFINITION: &str = "resolveCredentialDefinition";

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
pub async fn build_create_credential_definition_transaction(
    client: &LedgerClient,
    from: &Address,
    id: &CredentialDefinitionId,
    credential_definition: &CredentialDefinition,
) -> VdrResult<Transaction> {
    debug!(
        "{} txn build has started. Sender: {:?}, CredentialDefinition: {:?}",
        METHOD_CREATE_CREDENTIAL_DEFINITION, from, credential_definition
    );

    // TODO: validate credential definition

    let transaction = TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_CREDENTIAL_DEFINITION)
        .add_param(id.into())
        .add_param((&credential_definition.issuer_id).into())
        .add_param((&credential_definition.schema_id).into())
        .add_param(credential_definition.into())
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await?;

    info!(
        "{} txn build has finished. Result: {:?}",
        METHOD_CREATE_CREDENTIAL_DEFINITION, transaction
    );

    Ok(transaction)
}

/// Build transaction to execute CredentialDefinitionRegistry.resolveCredentialDefinition contract
/// method to retrieve an existing Credential Definition by the given id
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `id` id of Credential Definition to resolve
///
/// # Returns
/// Read transaction to submit
pub async fn build_resolve_credential_definition_transaction(
    client: &LedgerClient,
    id: &CredentialDefinitionId,
) -> VdrResult<Transaction> {
    debug!(
        "{} txn build has started. CredentialDefinitionId: {:?}",
        METHOD_RESOLVE_CREDENTIAL_DEFINITION, id
    );

    // TODO: validate credential definition

    let transaction = TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_RESOLVE_CREDENTIAL_DEFINITION)
        .add_param(id.into())
        .set_type(TransactionType::Read)
        .build(client)
        .await?;

    info!(
        "{} txn build has finished. Result: {:?}",
        METHOD_RESOLVE_CREDENTIAL_DEFINITION, transaction
    );

    Ok(transaction)
}

/// Parse the result of execution CredentialDefinitionRegistry.resolveCredentialDefinition contract
/// method to receive a Credential Definition associated with the id
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
/// parsed Credential Definition
pub fn parse_resolve_credential_definition_result(
    client: &LedgerClient,
    bytes: &[u8],
) -> VdrResult<CredentialDefinition> {
    debug!(
        "{} result parse has started. Bytes to parse: {:?}",
        METHOD_RESOLVE_CREDENTIAL_DEFINITION, bytes
    );

    let credential_definition = TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_RESOLVE_CREDENTIAL_DEFINITION)
        .parse::<CredentialDefinitionRecord>(client, bytes)?
        .credential_definition;

    // TODO: validate credential definition

    info!(
        "{} result parse has finished. Result: {:?}",
        METHOD_RESOLVE_CREDENTIAL_DEFINITION, credential_definition
    );

    Ok(credential_definition)
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
                    76, 197, 98, 212, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 64, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 160, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 2, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 100, 105, 100, 58, 105, 110, 100, 121, 50, 58,
                    116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122, 107, 103,
                    84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 47, 97, 110, 111,
                    110, 99, 114, 101, 100, 115, 47, 118, 48, 47, 67, 76, 65, 73, 77, 95, 68, 69,
                    70, 47, 100, 105, 100, 58, 105, 110, 100, 121, 50, 58, 116, 101, 115, 116, 110,
                    101, 116, 58, 51, 76, 112, 106, 115, 122, 107, 103, 84, 109, 69, 51, 113, 84,
                    104, 103, 101, 50, 53, 70, 90, 119, 47, 97, 110, 111, 110, 99, 114, 101, 100,
                    115, 47, 118, 48, 47, 83, 67, 72, 69, 77, 65, 47, 70, 49, 68, 67, 108, 97, 70,
                    69, 122, 105, 51, 116, 47, 49, 46, 48, 46, 48, 47, 100, 101, 102, 97, 117, 108,
                    116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 100, 105, 100, 58, 105, 110,
                    100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115,
                    122, 107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 79, 100, 105, 100, 58, 105, 110, 100, 121, 50, 58, 116, 101, 115, 116,
                    110, 101, 116, 58, 51, 76, 112, 106, 115, 122, 107, 103, 84, 109, 69, 51, 113,
                    84, 104, 103, 101, 50, 53, 70, 90, 119, 47, 97, 110, 111, 110, 99, 114, 101,
                    100, 115, 47, 118, 48, 47, 83, 67, 72, 69, 77, 65, 47, 70, 49, 68, 67, 108, 97,
                    70, 69, 122, 105, 51, 116, 47, 49, 46, 48, 46, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 123, 34, 99, 114, 101, 100, 68, 101,
                    102, 84, 121, 112, 101, 34, 58, 34, 67, 76, 34, 44, 34, 105, 115, 115, 117,
                    101, 114, 73, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110, 100, 121, 50, 58,
                    116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122, 107, 103,
                    84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 34, 44, 34, 115,
                    99, 104, 101, 109, 97, 73, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110, 100,
                    121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122,
                    107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 47, 97,
                    110, 111, 110, 99, 114, 101, 100, 115, 47, 118, 48, 47, 83, 67, 72, 69, 77, 65,
                    47, 70, 49, 68, 67, 108, 97, 70, 69, 122, 105, 51, 116, 47, 49, 46, 48, 46, 48,
                    34, 44, 34, 116, 97, 103, 34, 58, 34, 100, 101, 102, 97, 117, 108, 116, 34, 44,
                    34, 118, 97, 108, 117, 101, 34, 58, 123, 34, 110, 34, 58, 34, 55, 55, 57, 46,
                    46, 46, 51, 57, 55, 34, 44, 34, 114, 99, 116, 120, 116, 34, 58, 34, 55, 55, 52,
                    46, 46, 46, 57, 55, 55, 34, 44, 34, 115, 34, 58, 34, 55, 53, 48, 46, 46, 56,
                    57, 51, 34, 44, 34, 122, 34, 58, 34, 54, 51, 50, 46, 46, 46, 48, 48, 53, 34,
                    125, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }
    }

    mod build_resolve_credential_definition_transaction {
        use super::*;

        #[async_std::test]
        async fn build_resolve_credential_definition_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let (id, _) = credential_definition(
                &DID::from(ISSUER_ID),
                &SchemaId::from(SCHEMA_ID),
                Some(CREDENTIAL_DEFINITION_TAG),
            );
            let transaction = build_resolve_credential_definition_transaction(&client, &id)
                .await
                .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Read,
                from: None,
                to: CRED_DEF_REGISTRY_ADDRESS.clone(),
                nonce: None,
                chain_id: CHAIN_ID,
                data: vec![
                    97, 112, 196, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 100, 105, 100, 58, 105,
                    110, 100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106,
                    115, 122, 107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90,
                    119, 47, 97, 110, 111, 110, 99, 114, 101, 100, 115, 47, 118, 48, 47, 67, 76,
                    65, 73, 77, 95, 68, 69, 70, 47, 100, 105, 100, 58, 105, 110, 100, 121, 50, 58,
                    116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122, 107, 103,
                    84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 47, 97, 110, 111,
                    110, 99, 114, 101, 100, 115, 47, 118, 48, 47, 83, 67, 72, 69, 77, 65, 47, 70,
                    49, 68, 67, 108, 97, 70, 69, 122, 105, 51, 116, 47, 49, 46, 48, 46, 48, 47,
                    100, 101, 102, 97, 117, 108, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }
    }

    mod parse_resolve_credential_definition_result {
        use super::*;

        #[test]
        fn parse_resolve_credential_definition_result_test() {
            init_env_logger();
            let client = mock_client();
            let data = vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 166, 63, 232, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 123, 34, 99, 114,
                101, 100, 68, 101, 102, 84, 121, 112, 101, 34, 58, 34, 67, 76, 34, 44, 34, 105,
                115, 115, 117, 101, 114, 73, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110, 100,
                121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122,
                107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 34, 44, 34,
                115, 99, 104, 101, 109, 97, 73, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110, 100,
                121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122,
                107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 47, 97,
                110, 111, 110, 99, 114, 101, 100, 115, 47, 118, 48, 47, 83, 67, 72, 69, 77, 65, 47,
                70, 49, 68, 67, 108, 97, 70, 69, 122, 105, 51, 116, 47, 49, 46, 48, 46, 48, 34, 44,
                34, 116, 97, 103, 34, 58, 34, 100, 101, 102, 97, 117, 108, 116, 34, 44, 34, 118,
                97, 108, 117, 101, 34, 58, 123, 34, 110, 34, 58, 34, 55, 55, 57, 46, 46, 46, 51,
                57, 55, 34, 44, 34, 114, 99, 116, 120, 116, 34, 58, 34, 55, 55, 52, 46, 46, 46, 57,
                55, 55, 34, 44, 34, 115, 34, 58, 34, 55, 53, 48, 46, 46, 56, 57, 51, 34, 44, 34,
                122, 34, 58, 34, 54, 51, 50, 46, 46, 46, 48, 48, 53, 34, 125, 125, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let parsed_cred_def =
                parse_resolve_credential_definition_result(&client, &data).unwrap();
            let (_, expected_cred_def) = credential_definition(
                &DID::from(ISSUER_ID),
                &SchemaId::from(SCHEMA_ID),
                Some(CREDENTIAL_DEFINITION_TAG),
            );
            assert_eq!(expected_cred_def, parsed_cred_def);
        }
    }
}
