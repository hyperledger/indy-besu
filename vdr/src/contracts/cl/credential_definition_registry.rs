use log_derive::{logfn, logfn_inputs};

use crate::{
    client::LedgerClient,
    contracts::cl::types::{
        credential_definition::{CredentialDefinition, CredentialDefinitionRecord},
        credential_definition_id::CredentialDefinitionId,
    },
    error::VdrResult,
    types::{
        Address, MethodStringParam, Transaction, TransactionBuilder,
        TransactionEndorsingDataBuilder, TransactionParser, TransactionType,
    },
    SignatureData, TransactionEndorsingData, VdrError,
};

const CONTRACT_NAME: &str = "CredentialDefinitionRegistry";
const METHOD_CREATE_CREDENTIAL_DEFINITION: &str = "createCredentialDefinition";
const METHOD_CREATE_CREDENTIAL_DEFINITION_SIGNED: &str = "createCredentialDefinitionSigned";
const METHOD_RESOLVE_CREDENTIAL_DEFINITION: &str = "resolveCredentialDefinition";

/// Build transaction to execute CredentialDefinitionRegistry.createCredentialDefinition contract
/// method to create a new Credential Definition
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `from` transaction sender account address
/// - `credential_definition` Credential Definition object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:credential-definition
///
/// # Returns
/// Write transaction to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_credential_definition_transaction(
    client: &LedgerClient,
    from: &Address,
    credential_definition: &CredentialDefinition,
) -> VdrResult<Transaction> {
    // TODO: validate credential definition
    let identity = Address::try_from(&credential_definition.issuer_id)?;
    let id = credential_definition.id();
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_CREDENTIAL_DEFINITION)
        .add_param(&identity)?
        .add_param(&id)?
        .add_param(&credential_definition.issuer_id)?
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
/// - `credential_definition` Credential Definition object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:credential-definition
///
/// #Returns
///   data: TransactionEndorsingData - transaction endorsement data to sign
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_credential_definition_endorsing_data(
    client: &LedgerClient,
    credential_definition: &CredentialDefinition,
) -> VdrResult<TransactionEndorsingData> {
    let identity = Address::try_from(&credential_definition.issuer_id)?;
    let id = credential_definition.id();
    TransactionEndorsingDataBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_identity(&identity)
        .add_param(&identity)?
        .add_param(&MethodStringParam::from(
            METHOD_CREATE_CREDENTIAL_DEFINITION,
        ))?
        .add_param(&id)?
        .add_param(&credential_definition.issuer_id)?
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
    credential_definition: &CredentialDefinition,
    signature: &SignatureData,
) -> VdrResult<Transaction> {
    // TODO: validate credential definition
    let identity = Address::try_from(&credential_definition.issuer_id)?;
    let id = credential_definition.id();
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_CREDENTIAL_DEFINITION_SIGNED)
        .add_param(&identity)?
        .add_param(&signature.v())?
        .add_param(&signature.r())?
        .add_param(&signature.s())?
        .add_param(&id)?
        .add_param(&credential_definition.issuer_id)?
        .add_param(&credential_definition.schema_id)?
        .add_param(credential_definition)?
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await
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
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_resolve_credential_definition_transaction(
    client: &LedgerClient,
    id: &CredentialDefinitionId,
) -> VdrResult<Transaction> {
    // TODO: validate credential definition
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_RESOLVE_CREDENTIAL_DEFINITION)
        .add_param(id)?
        .set_type(TransactionType::Read)
        .build(client)
        .await
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
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_resolve_credential_definition_result(
    client: &LedgerClient,
    bytes: &[u8],
) -> VdrResult<CredentialDefinitionRecord> {
    // TODO: validate credential definition
    TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_RESOLVE_CREDENTIAL_DEFINITION)
        .parse::<CredentialDefinitionRecord>(client, bytes)
}

/// Single step function to resolve a Credential Definition for the given ID
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `id` id of credential definition to resolve
///
/// # Returns
///   Resolved Credential Definition object
pub async fn resolve_credential_definition(
    client: &LedgerClient,
    id: &CredentialDefinitionId,
) -> VdrResult<CredentialDefinition> {
    let transaction = build_resolve_credential_definition_transaction(client, id).await?;
    let response = client.submit_transaction(&transaction).await?;
    println!("crededf {:?}", response);
    if response.is_empty() {
        return Err(VdrError::ClientInvalidResponse(format!(
            "Credential Definition not found for id: {:?}",
            id
        )));
    }
    let cred_def_record = parse_resolve_credential_definition_result(client, &response)?;
    Ok(cred_def_record.credential_definition)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        client::client::test::{
            mock_client, CHAIN_ID, CRED_DEF_REGISTRY_ADDRESS, DEFAULT_NONCE, TEST_ACCOUNT,
        },
        contracts::{
            cl::types::{
                credential_definition::test::{credential_definition, CREDENTIAL_DEFINITION_TAG},
                schema::test::SCHEMA_ID,
                schema_id::SchemaId,
            },
            did::types::{did::DID, did_doc::test::TEST_ETHR_DID},
        },
    };
    use std::sync::RwLock;

    mod build_create_credential_definition_transaction {
        use super::*;

        #[async_std::test]
        async fn build_create_credential_definition_transaction_test() {
            let client = mock_client();
            let cred_def = credential_definition(
                &DID::from(TEST_ETHR_DID),
                &SchemaId::from(SCHEMA_ID),
                Some(CREDENTIAL_DEFINITION_TAG),
            );
            let transaction =
                build_create_credential_definition_transaction(&client, &TEST_ACCOUNT, &cred_def)
                    .await
                    .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(TEST_ACCOUNT.clone()),
                to: CRED_DEF_REGISTRY_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: vec![
                    182, 196, 9, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 226, 219, 108, 141,
                    198, 198, 129, 187, 93, 106, 209, 33, 161, 7, 243, 0, 233, 178, 181, 109, 71,
                    26, 149, 232, 163, 135, 235, 109, 104, 137, 85, 62, 141, 209, 156, 9, 33, 105,
                    94, 200, 254, 71, 119, 190, 195, 248, 17, 17, 141, 239, 177, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160,
                    34, 27, 23, 130, 143, 227, 3, 94, 147, 14, 185, 63, 10, 50, 145, 115, 71, 104,
                    106, 145, 232, 190, 123, 84, 240, 64, 217, 94, 167, 52, 119, 152, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 59, 100, 105, 100, 58, 101, 116, 104, 114, 58, 116, 101,
                    115, 116, 110, 101, 116, 58, 48, 120, 102, 48, 101, 50, 100, 98, 54, 99, 56,
                    100, 99, 54, 99, 54, 56, 49, 98, 98, 53, 100, 54, 97, 100, 49, 50, 49, 97, 49,
                    48, 55, 102, 51, 48, 48, 101, 57, 98, 50, 98, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
    }

    mod build_resolve_credential_definition_transaction {
        use super::*;

        #[async_std::test]
        async fn build_resolve_credential_definition_transaction_test() {
            let client = mock_client();
            let cred_def = credential_definition(
                &DID::from(TEST_ETHR_DID),
                &SchemaId::from(SCHEMA_ID),
                Some(CREDENTIAL_DEFINITION_TAG),
            );
            let transaction =
                build_resolve_credential_definition_transaction(&client, &cred_def.id())
                    .await
                    .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Read,
                from: None,
                to: CRED_DEF_REGISTRY_ADDRESS.clone(),
                nonce: None,
                chain_id: CHAIN_ID,
                data: vec![
                    159, 136, 157, 181, 109, 71, 26, 149, 232, 163, 135, 235, 109, 104, 137, 85,
                    62, 141, 209, 156, 9, 33, 105, 94, 200, 254, 71, 119, 190, 195, 248, 17, 17,
                    141, 239, 177,
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
            let client = mock_client();
            let data = vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 203, 143, 197, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 42, 123, 34, 105, 115,
                115, 117, 101, 114, 73, 100, 34, 58, 34, 100, 105, 100, 58, 101, 116, 104, 114, 58,
                116, 101, 115, 116, 110, 101, 116, 58, 48, 120, 102, 48, 101, 50, 100, 98, 54, 99,
                56, 100, 99, 54, 99, 54, 56, 49, 98, 98, 53, 100, 54, 97, 100, 49, 50, 49, 97, 49,
                48, 55, 102, 51, 48, 48, 101, 57, 98, 50, 98, 53, 34, 44, 34, 115, 99, 104, 101,
                109, 97, 73, 100, 34, 58, 34, 100, 105, 100, 58, 101, 116, 104, 114, 58, 116, 101,
                115, 116, 110, 101, 116, 58, 48, 120, 102, 48, 101, 50, 100, 98, 54, 99, 56, 100,
                99, 54, 99, 54, 56, 49, 98, 98, 53, 100, 54, 97, 100, 49, 50, 49, 97, 49, 48, 55,
                102, 51, 48, 48, 101, 57, 98, 50, 98, 53, 47, 97, 110, 111, 110, 99, 114, 101, 100,
                115, 47, 118, 48, 47, 83, 67, 72, 69, 77, 65, 47, 70, 49, 68, 67, 108, 97, 70, 69,
                122, 105, 51, 116, 47, 49, 46, 48, 46, 48, 34, 44, 34, 99, 114, 101, 100, 68, 101,
                102, 84, 121, 112, 101, 34, 58, 34, 67, 76, 34, 44, 34, 116, 97, 103, 34, 58, 34,
                100, 101, 102, 97, 117, 108, 116, 34, 44, 34, 118, 97, 108, 117, 101, 34, 58, 123,
                34, 110, 34, 58, 34, 55, 55, 57, 46, 46, 46, 51, 57, 55, 34, 44, 34, 114, 99, 116,
                120, 116, 34, 58, 34, 55, 55, 52, 46, 46, 46, 57, 55, 55, 34, 44, 34, 115, 34, 58,
                34, 55, 53, 48, 46, 46, 56, 57, 51, 34, 44, 34, 122, 34, 58, 34, 54, 51, 50, 46,
                46, 46, 48, 48, 53, 34, 125, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0,
            ];
            let parsed_cred_def =
                parse_resolve_credential_definition_result(&client, &data).unwrap();
            let expected_cred_def = credential_definition(
                &DID::from(TEST_ETHR_DID),
                &SchemaId::from(SCHEMA_ID),
                Some(CREDENTIAL_DEFINITION_TAG),
            );
            assert_eq!(expected_cred_def, parsed_cred_def.credential_definition);
        }
    }
}
