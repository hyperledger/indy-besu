use log::{debug, info};

use crate::{
    client::LedgerClient,
    contracts::did::types::{
        did::DID,
        did_doc::{DidDocument, DidRecord},
    },
    error::VdrResult,
    types::{Address, Transaction, TransactionBuilder, TransactionParser, TransactionType},
};

const CONTRACT_NAME: &str = "IndyDidRegistry";
const METHOD_CREATE_DID: &str = "createDid";
const METHOD_UPDATE_DID: &str = "updateDid";
const METHOD_DEACTIVATE_DID: &str = "deactivateDid";
const METHOD_RESOLVE_DID: &str = "resolveDid";

/// Build transaction to execute IndyDidRegistry.createDid contract method to create a new DID
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `from` transaction sender account address
/// - `identity` DID owner account address
/// - `did` DID to be created
/// - `did_doc` DID Document matching to the specification: https://www.w3.org/TR/did-core/
///
/// # Returns
/// Write transaction to sign and submit
pub async fn build_create_did_transaction(
    client: &LedgerClient,
    from: &Address,
    identity: &Address,
    did: &DID,
    did_doc: &DidDocument,
) -> VdrResult<Transaction> {
    debug!(
        "{} txn build has started. Sender: {:?}, DidDocument: {:?}",
        METHOD_CREATE_DID, from, did_doc
    );

    let transaction = TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_DID)
        .add_param(identity.try_into()?)
        .add_param(did.into())
        .add_param(did_doc.into())
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await?;

    info!(
        "{} txn build has finished. Result: {:?}",
        METHOD_CREATE_DID, transaction
    );

    Ok(transaction)
}

/// Build transaction to execute IndyDidRegistry.updateDid contract method to update DID document for an existing DID
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `from` transaction sender account address
/// - `did` DID to be updated
/// - `did_doc` new DID Document matching to the specification: https://www.w3.org/TR/did-core/
///
/// # Returns
/// Write transaction to sign and submit
pub async fn build_update_did_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
    did_doc: &DidDocument,
) -> VdrResult<Transaction> {
    debug!(
        "{} txn build has started. Sender: {:?}, DidDocument: {:?}",
        METHOD_UPDATE_DID, from, did_doc
    );

    let transaction = TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_UPDATE_DID)
        .add_param(did.into())
        .add_param(did_doc.into())
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await?;

    info!(
        "{} txn build has finished. Result: {:?}",
        METHOD_UPDATE_DID, transaction
    );

    Ok(transaction)
}

/// Build transaction to execute IndyDidRegistry.deactivateDid contract method to deactivate an existing DID
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `from` transaction sender account address
/// - `did` DID to deactivate
///
/// # Returns
/// Write transaction to sign and submit
pub async fn build_deactivate_did_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
) -> VdrResult<Transaction> {
    debug!(
        "{} txn build has started. Sender: {:?}, Did: {:?}",
        METHOD_DEACTIVATE_DID, from, did
    );

    let transaction = TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_DEACTIVATE_DID)
        .add_param(did.into())
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await?;

    info!(
        "{} txn build has finished. Result: {:?}",
        METHOD_DEACTIVATE_DID, transaction
    );

    Ok(transaction)
}

/// Build transaction to execute IndyDidRegistry.resolveDid contract method to receive a DID Document associated with the DID
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `did` target DID to receive DID Document
///
/// # Returns
/// Read transaction to submit
pub async fn build_resolve_did_transaction(
    client: &LedgerClient,
    did: &DID,
) -> VdrResult<Transaction> {
    debug!(
        "{} txn build has started. Did: {:?}",
        METHOD_RESOLVE_DID, did
    );

    let transaction = TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_RESOLVE_DID)
        .add_param(did.into())
        .set_type(TransactionType::Read)
        .build(client)
        .await?;

    info!(
        "{} txn build has finished. Result: {:?}",
        METHOD_RESOLVE_DID, transaction
    );

    Ok(transaction)
}

/// Parse the result of execution IndyDidRegistry.resolveDid contract method to receive a DID Document associated with the DID
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
/// parsed DID Document
pub fn parse_resolve_did_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<DidDocument> {
    debug!(
        "{} result parse has started. Bytes to parse: {:?}",
        METHOD_RESOLVE_DID, bytes
    );

    let document = TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_RESOLVE_DID)
        .parse::<DidRecord>(client, bytes)?
        .document;

    info!(
        "{} result parse has finished. Result: {:?}",
        METHOD_RESOLVE_DID, document
    );

    Ok(document)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        client::client::test::{
            mock_client, CHAIN_ID, DEFAULT_NONCE, DID_REGISTRY_ADDRESS, TRUSTEE_ACC,
        },
        contracts::did::types::{
            did::DID,
            did_doc::test::{did_doc, ISSUER_ID},
        },
        utils::init_env_logger,
    };
    use std::sync::RwLock;

    mod build_create_did_transaction {
        use super::*;
        use crate::{
            client::client::test::{mock_client, IDENTITY_ACC},
            contracts::{
                did::types::did_doc::test::service, StringOrVector, VerificationMethod,
                VerificationMethodOrReference,
            },
            VerificationKeyType,
        };

        #[async_std::test]
        async fn build_create_did_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let did = DID::from(ISSUER_ID);
            let did_doc = did_doc(Some(ISSUER_ID));
            let transaction =
                build_create_did_transaction(&client, &TRUSTEE_ACC, &IDENTITY_ACC, &did, &did_doc)
                    .await
                    .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(TRUSTEE_ACC.clone()),
                to: DID_REGISTRY_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: vec![
                    30, 113, 85, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 5, 148, 0, 220, 208,
                    81, 88, 255, 216, 202, 9, 41, 55, 152, 157, 210, 123, 59, 220, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 100, 105, 100, 58, 105, 110, 100, 121,
                    50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122, 107,
                    103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 25,
                    123, 34, 64, 99, 111, 110, 116, 101, 120, 116, 34, 58, 91, 34, 104, 116, 116,
                    112, 115, 58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 110,
                    115, 47, 100, 105, 100, 47, 118, 49, 34, 93, 44, 34, 97, 108, 115, 111, 75,
                    110, 111, 119, 110, 65, 115, 34, 58, 91, 93, 44, 34, 97, 115, 115, 101, 114,
                    116, 105, 111, 110, 77, 101, 116, 104, 111, 100, 34, 58, 91, 93, 44, 34, 97,
                    117, 116, 104, 101, 110, 116, 105, 99, 97, 116, 105, 111, 110, 34, 58, 91, 34,
                    100, 105, 100, 58, 105, 110, 100, 121, 50, 58, 116, 101, 115, 116, 110, 101,
                    116, 58, 51, 76, 112, 106, 115, 122, 107, 103, 84, 109, 69, 51, 113, 84, 104,
                    103, 101, 50, 53, 70, 90, 119, 35, 75, 69, 89, 45, 49, 34, 93, 44, 34, 99, 97,
                    112, 97, 98, 105, 108, 105, 116, 121, 68, 101, 108, 101, 103, 97, 116, 105,
                    111, 110, 34, 58, 91, 93, 44, 34, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121,
                    73, 110, 118, 111, 99, 97, 116, 105, 111, 110, 34, 58, 91, 93, 44, 34, 99, 111,
                    110, 116, 114, 111, 108, 108, 101, 114, 34, 58, 91, 93, 44, 34, 105, 100, 34,
                    58, 34, 100, 105, 100, 58, 105, 110, 100, 121, 50, 58, 116, 101, 115, 116, 110,
                    101, 116, 58, 51, 76, 112, 106, 115, 122, 107, 103, 84, 109, 69, 51, 113, 84,
                    104, 103, 101, 50, 53, 70, 90, 119, 34, 44, 34, 107, 101, 121, 65, 103, 114,
                    101, 101, 109, 101, 110, 116, 34, 58, 91, 93, 44, 34, 115, 101, 114, 118, 105,
                    99, 101, 34, 58, 91, 93, 44, 34, 118, 101, 114, 105, 102, 105, 99, 97, 116,
                    105, 111, 110, 77, 101, 116, 104, 111, 100, 34, 58, 91, 123, 34, 99, 111, 110,
                    116, 114, 111, 108, 108, 101, 114, 34, 58, 34, 100, 105, 100, 58, 105, 110,
                    100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115,
                    122, 107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119,
                    34, 44, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110, 100, 121, 50,
                    58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122, 107,
                    103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 35, 75, 69,
                    89, 45, 49, 34, 44, 34, 112, 117, 98, 108, 105, 99, 75, 101, 121, 77, 117, 108,
                    116, 105, 98, 97, 115, 101, 34, 58, 34, 122, 65, 75, 74, 80, 51, 102, 55, 66,
                    68, 54, 87, 52, 105, 87, 69, 81, 57, 106, 119, 110, 100, 86, 84, 67, 66, 113,
                    56, 117, 97, 50, 85, 116, 116, 56, 69, 69, 106, 74, 54, 86, 120, 115, 102, 34,
                    44, 34, 116, 121, 112, 101, 34, 58, 34, 69, 100, 50, 53, 53, 49, 57, 86, 101,
                    114, 105, 102, 105, 99, 97, 116, 105, 111, 110, 75, 101, 121, 50, 48, 49, 56,
                    34, 125, 93, 125, 0, 0, 0, 0, 0, 0, 0,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }

        #[async_std::test]
        async fn build_create_did_transaction_with_two_keys_and_service_test() {
            init_env_logger();
            let client = mock_client();

            let did = DID::from("did:indy2:testnet:3LpjszkgTmE3qThge25FZw");
            let did_doc = DidDocument {
                context: StringOrVector::Vector(vec!["https://www.w3.org/ns/did/v1".to_string()]),
                id: DID::from("did:indy2:testnet:3LpjszkgTmE3qThge25FZw"),
                controller: StringOrVector::Vector(vec![]),
                verification_method: vec![
                    VerificationMethod {
                        id: "did:indy2:testnet:3LpjszkgTmE3qThge25FZw#KEY-1".to_string(),
                        type_: VerificationKeyType::Ed25519VerificationKey2018,
                        controller: "did:indy2:testnet:3LpjszkgTmE3qThge25FZw".to_string(),
                        public_key_multibase: Some("8rnQ4gvtEYi59DMAzN7FyCVatVATkFo7wPXVMy38WmvG".to_string()),
                        public_key_jwk: None,
                    },
                    VerificationMethod {
                        id: "did:indy2:testnet:3LpjszkgTmE3qThge25FZw#KEY-2".to_string(),
                        type_: VerificationKeyType::EcdsaSecp256k1VerificationKey2019,
                        controller: "did:indy2:testnet:3LpjszkgTmE3qThge25FZw".to_string(),
                        public_key_multibase: Some("NaqS2qSLZTJcuKLvFAoBSeRFXeivDfyoUqvSs8DQ4ajydz4KbUvT6vdJyz8i9gJEqGjFkCN27niZhoAbQLgk3imn".to_string()),
                        public_key_jwk: None,
                    },
                ],
                authentication: vec![
                    VerificationMethodOrReference::String("did:indy2:testnet:3LpjszkgTmE3qThge25FZw#KEY-1".to_string()),
                    VerificationMethodOrReference::String("did:indy2:testnet:3LpjszkgTmE3qThge25FZw#KEY-2".to_string()),
                ],
                assertion_method: vec![],
                capability_invocation: vec![],
                capability_delegation: vec![],
                key_agreement: vec![],
                service: vec![
                    service("did:indy2:testnet:3LpjszkgTmE3qThge25FZw#SERVICE-1")
                ],
                also_known_as: Some(vec![]),
            };
            let transaction =
                build_create_did_transaction(&client, &TRUSTEE_ACC, &IDENTITY_ACC, &did, &did_doc)
                    .await
                    .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(TRUSTEE_ACC.clone()),
                to: DID_REGISTRY_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: vec![
                    30, 113, 85, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 5, 148, 0, 220, 208,
                    81, 88, 255, 216, 202, 9, 41, 55, 152, 157, 210, 123, 59, 220, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 100, 105, 100, 58, 105, 110, 100, 121,
                    50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122, 107,
                    103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 203,
                    123, 34, 64, 99, 111, 110, 116, 101, 120, 116, 34, 58, 91, 34, 104, 116, 116,
                    112, 115, 58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 110,
                    115, 47, 100, 105, 100, 47, 118, 49, 34, 93, 44, 34, 97, 108, 115, 111, 75,
                    110, 111, 119, 110, 65, 115, 34, 58, 91, 93, 44, 34, 97, 115, 115, 101, 114,
                    116, 105, 111, 110, 77, 101, 116, 104, 111, 100, 34, 58, 91, 93, 44, 34, 97,
                    117, 116, 104, 101, 110, 116, 105, 99, 97, 116, 105, 111, 110, 34, 58, 91, 34,
                    100, 105, 100, 58, 105, 110, 100, 121, 50, 58, 116, 101, 115, 116, 110, 101,
                    116, 58, 51, 76, 112, 106, 115, 122, 107, 103, 84, 109, 69, 51, 113, 84, 104,
                    103, 101, 50, 53, 70, 90, 119, 35, 75, 69, 89, 45, 49, 34, 44, 34, 100, 105,
                    100, 58, 105, 110, 100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51,
                    76, 112, 106, 115, 122, 107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50,
                    53, 70, 90, 119, 35, 75, 69, 89, 45, 50, 34, 93, 44, 34, 99, 97, 112, 97, 98,
                    105, 108, 105, 116, 121, 68, 101, 108, 101, 103, 97, 116, 105, 111, 110, 34,
                    58, 91, 93, 44, 34, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 73, 110, 118,
                    111, 99, 97, 116, 105, 111, 110, 34, 58, 91, 93, 44, 34, 99, 111, 110, 116,
                    114, 111, 108, 108, 101, 114, 34, 58, 91, 93, 44, 34, 105, 100, 34, 58, 34,
                    100, 105, 100, 58, 105, 110, 100, 121, 50, 58, 116, 101, 115, 116, 110, 101,
                    116, 58, 51, 76, 112, 106, 115, 122, 107, 103, 84, 109, 69, 51, 113, 84, 104,
                    103, 101, 50, 53, 70, 90, 119, 34, 44, 34, 107, 101, 121, 65, 103, 114, 101,
                    101, 109, 101, 110, 116, 34, 58, 91, 93, 44, 34, 115, 101, 114, 118, 105, 99,
                    101, 34, 58, 91, 123, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110,
                    100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115,
                    122, 107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119,
                    35, 83, 69, 82, 86, 73, 67, 69, 45, 49, 34, 44, 34, 115, 101, 114, 118, 105,
                    99, 101, 69, 110, 100, 112, 111, 105, 110, 116, 34, 58, 34, 49, 50, 55, 46, 48,
                    46, 48, 46, 49, 58, 53, 53, 53, 53, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34,
                    68, 73, 68, 67, 111, 109, 109, 83, 101, 114, 118, 105, 99, 101, 34, 125, 93,
                    44, 34, 118, 101, 114, 105, 102, 105, 99, 97, 116, 105, 111, 110, 77, 101, 116,
                    104, 111, 100, 34, 58, 91, 123, 34, 99, 111, 110, 116, 114, 111, 108, 108, 101,
                    114, 34, 58, 34, 100, 105, 100, 58, 105, 110, 100, 121, 50, 58, 116, 101, 115,
                    116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122, 107, 103, 84, 109, 69, 51,
                    113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 34, 44, 34, 105, 100, 34, 58, 34,
                    100, 105, 100, 58, 105, 110, 100, 121, 50, 58, 116, 101, 115, 116, 110, 101,
                    116, 58, 51, 76, 112, 106, 115, 122, 107, 103, 84, 109, 69, 51, 113, 84, 104,
                    103, 101, 50, 53, 70, 90, 119, 35, 75, 69, 89, 45, 49, 34, 44, 34, 112, 117,
                    98, 108, 105, 99, 75, 101, 121, 77, 117, 108, 116, 105, 98, 97, 115, 101, 34,
                    58, 34, 56, 114, 110, 81, 52, 103, 118, 116, 69, 89, 105, 53, 57, 68, 77, 65,
                    122, 78, 55, 70, 121, 67, 86, 97, 116, 86, 65, 84, 107, 70, 111, 55, 119, 80,
                    88, 86, 77, 121, 51, 56, 87, 109, 118, 71, 34, 44, 34, 116, 121, 112, 101, 34,
                    58, 34, 69, 100, 50, 53, 53, 49, 57, 86, 101, 114, 105, 102, 105, 99, 97, 116,
                    105, 111, 110, 75, 101, 121, 50, 48, 49, 56, 34, 125, 44, 123, 34, 99, 111,
                    110, 116, 114, 111, 108, 108, 101, 114, 34, 58, 34, 100, 105, 100, 58, 105,
                    110, 100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106,
                    115, 122, 107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90,
                    119, 34, 44, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110, 100, 121,
                    50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106, 115, 122, 107,
                    103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90, 119, 35, 75, 69,
                    89, 45, 50, 34, 44, 34, 112, 117, 98, 108, 105, 99, 75, 101, 121, 77, 117, 108,
                    116, 105, 98, 97, 115, 101, 34, 58, 34, 78, 97, 113, 83, 50, 113, 83, 76, 90,
                    84, 74, 99, 117, 75, 76, 118, 70, 65, 111, 66, 83, 101, 82, 70, 88, 101, 105,
                    118, 68, 102, 121, 111, 85, 113, 118, 83, 115, 56, 68, 81, 52, 97, 106, 121,
                    100, 122, 52, 75, 98, 85, 118, 84, 54, 118, 100, 74, 121, 122, 56, 105, 57,
                    103, 74, 69, 113, 71, 106, 70, 107, 67, 78, 50, 55, 110, 105, 90, 104, 111, 65,
                    98, 81, 76, 103, 107, 51, 105, 109, 110, 34, 44, 34, 116, 121, 112, 101, 34,
                    58, 34, 69, 99, 100, 115, 97, 83, 101, 99, 112, 50, 53, 54, 107, 49, 86, 101,
                    114, 105, 102, 105, 99, 97, 116, 105, 111, 110, 75, 101, 121, 50, 48, 49, 57,
                    34, 125, 93, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }
    }

    mod build_resolve_did_transaction {
        use super::*;

        #[async_std::test]
        async fn build_resolve_did_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let transaction = build_resolve_did_transaction(&client, &DID::from(ISSUER_ID))
                .await
                .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Read,
                from: None,
                to: DID_REGISTRY_ADDRESS.clone(),
                nonce: None,
                chain_id: CHAIN_ID,
                data: vec![
                    54, 51, 133, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 100, 105, 100, 58, 105,
                    110, 100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 51, 76, 112, 106,
                    115, 122, 107, 103, 84, 109, 69, 51, 113, 84, 104, 103, 101, 50, 53, 70, 90,
                    119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }
    }

    mod parse_resolve_did_result {
        use super::*;

        #[test]
        fn parse_resolve_did_result_with_metadata_test() {
            init_env_logger();
            let client = mock_client();
            let issuer_did = "did:indy2:testnet:Q6Wvnm4v6ENzRC2mkUPkYR";

            let data = vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 5, 148, 0,
                220, 208, 81, 88, 255, 216, 202, 9, 41, 55, 152, 157, 210, 123, 59, 220, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 226, 219, 108, 141, 198, 198, 129, 187, 93, 106,
                209, 33, 161, 7, 243, 0, 233, 178, 181, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 166, 60, 159, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 166, 60, 159,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 2, 25, 123, 34, 64, 99, 111, 110, 116, 101, 120, 116, 34, 58, 91,
                34, 104, 116, 116, 112, 115, 58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114,
                103, 47, 110, 115, 47, 100, 105, 100, 47, 118, 49, 34, 93, 44, 34, 97, 108, 115,
                111, 75, 110, 111, 119, 110, 65, 115, 34, 58, 91, 93, 44, 34, 97, 115, 115, 101,
                114, 116, 105, 111, 110, 77, 101, 116, 104, 111, 100, 34, 58, 91, 93, 44, 34, 97,
                117, 116, 104, 101, 110, 116, 105, 99, 97, 116, 105, 111, 110, 34, 58, 91, 34, 100,
                105, 100, 58, 105, 110, 100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58,
                81, 54, 87, 118, 110, 109, 52, 118, 54, 69, 78, 122, 82, 67, 50, 109, 107, 85, 80,
                107, 89, 82, 35, 75, 69, 89, 45, 49, 34, 93, 44, 34, 99, 97, 112, 97, 98, 105, 108,
                105, 116, 121, 68, 101, 108, 101, 103, 97, 116, 105, 111, 110, 34, 58, 91, 93, 44,
                34, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 73, 110, 118, 111, 99, 97, 116,
                105, 111, 110, 34, 58, 91, 93, 44, 34, 99, 111, 110, 116, 114, 111, 108, 108, 101,
                114, 34, 58, 91, 93, 44, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110,
                100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 81, 54, 87, 118, 110, 109,
                52, 118, 54, 69, 78, 122, 82, 67, 50, 109, 107, 85, 80, 107, 89, 82, 34, 44, 34,
                107, 101, 121, 65, 103, 114, 101, 101, 109, 101, 110, 116, 34, 58, 91, 93, 44, 34,
                115, 101, 114, 118, 105, 99, 101, 34, 58, 91, 93, 44, 34, 118, 101, 114, 105, 102,
                105, 99, 97, 116, 105, 111, 110, 77, 101, 116, 104, 111, 100, 34, 58, 91, 123, 34,
                99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 34, 58, 34, 100, 105, 100, 58,
                105, 110, 100, 121, 50, 58, 116, 101, 115, 116, 110, 101, 116, 58, 81, 54, 87, 118,
                110, 109, 52, 118, 54, 69, 78, 122, 82, 67, 50, 109, 107, 85, 80, 107, 89, 82, 34,
                44, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 105, 110, 100, 121, 50, 58, 116,
                101, 115, 116, 110, 101, 116, 58, 81, 54, 87, 118, 110, 109, 52, 118, 54, 69, 78,
                122, 82, 67, 50, 109, 107, 85, 80, 107, 89, 82, 35, 75, 69, 89, 45, 49, 34, 44, 34,
                112, 117, 98, 108, 105, 99, 75, 101, 121, 77, 117, 108, 116, 105, 98, 97, 115, 101,
                34, 58, 34, 122, 65, 75, 74, 80, 51, 102, 55, 66, 68, 54, 87, 52, 105, 87, 69, 81,
                57, 106, 119, 110, 100, 86, 84, 67, 66, 113, 56, 117, 97, 50, 85, 116, 116, 56, 69,
                69, 106, 74, 54, 86, 120, 115, 102, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 69,
                100, 50, 53, 53, 49, 57, 86, 101, 114, 105, 102, 105, 99, 97, 116, 105, 111, 110,
                75, 101, 121, 50, 48, 49, 56, 34, 125, 93, 125, 0, 0, 0, 0, 0, 0, 0,
            ];
            let parsed_did_doc = parse_resolve_did_result(&client, &data).unwrap();
            assert_eq!(did_doc(Some(issuer_did)), parsed_did_doc);
        }
    }
}
