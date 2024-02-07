use log_derive::{logfn, logfn_inputs};

use crate::{
    client::LedgerClient,
    contracts::{
        migration::types::{ed25519_signature::Ed25519Signature, identifier::Identifier},
        types::did::{LegacyDid, LegacyVerkey},
    },
    did_ethr_registry::ETHR_DID_METHOD,
    error::VdrResult,
    types::{
        Address, MethodStringParam, Transaction, TransactionBuilder,
        TransactionEndorsingDataBuilder, TransactionParser, TransactionType,
    },
    SignatureData, TransactionEndorsingData, DID,
};

const CONTRACT_NAME: &str = "LegacyIdentifiersRegistry";
const METHOD_CREATE_DID_MAPPING: &str = "createDidMapping";
const METHOD_CREATE_DID_MAPPING_SIGNED: &str = "createDidMappingSigned";
const METHOD_CREATE_CL_MAPPING: &str = "createClMapping";
const METHOD_CREATE_CL_MAPPING_SIGNED: &str = "createClMappingSigned";
const METHOD_DID_MAPPING: &str = "didMapping";
const METHOD_CL_MAPPING: &str = "clMapping";

/// Build transaction to execute LegacyIdentifiersRegistry.createDidMapping contract method to
///  create a legacy DID identifier mapping
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `identity` transaction sender account address (new account)
/// - `legacy_identifier` identifier of legacy sov/indy DID
/// - `legacy_verkey` Ed25519 verification key of the legacy DID identifier.
/// - `ed25519_signature` ED25519 signature to prove key possession.
///
/// # Returns
/// Write transaction to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_did_mapping_transaction(
    client: &LedgerClient,
    sender: &Address,
    did: &DID,
    legacy_identifier: &LegacyDid,
    legacy_verkey: &LegacyVerkey,
    ed25519_signature: &Ed25519Signature,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    println!("identity {:?}", identity);
    println!("sender {:?}", sender);
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_DID_MAPPING)
        .add_param(&identity)?
        .add_param(legacy_identifier)?
        .add_param(legacy_verkey)?
        .add_param(ed25519_signature)?
        .set_type(TransactionType::Write)
        .set_from(sender)
        .build(client)
        .await
}

/// Prepared data for execution of LegacyIdentifiersRegistry.createDidMapping contract method to endorse a new DID mapping
///
/// #Params
/// - `client` client connected to the network where contract will be executed
/// - `identity` transaction sender account address (new account)
/// - `legacy_identifier` identifier of legacy sov/indy DID
/// - `legacy_verkey` Ed25519 verification key of the legacy DID identifier.
/// - `ed25519_signature` ED25519 signature to prove key possession.
///
/// #Returns
///   data: TransactionEndorsingData - transaction endorsement data to sign
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_did_mapping_endorsing_data(
    client: &LedgerClient,
    identity: &Address,
    legacy_identifier: &LegacyDid,
    legacy_verkey: &LegacyVerkey,
    ed25519_signature: &Ed25519Signature,
) -> VdrResult<TransactionEndorsingData> {
    TransactionEndorsingDataBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_identity(&identity)
        .add_param(MethodStringParam::from(METHOD_CREATE_DID_MAPPING))?
        .add_param(legacy_identifier)?
        .add_param(legacy_verkey)?
        .add_param(ed25519_signature)?
        .build(client)
        .await
}

/// Build transaction to execute LegacyIdentifiersRegistry.createDidMappingSigned contract method to
///  endorse a legacy DID identifier mapping
/// Endorsing version of the method - sender is not identity owner
///
/// #Params
/// - `client` client connected to the network where contract will be executed
/// - `identity` transaction sender account address (new account)
/// - `legacy_identifier` identifier of legacy sov/indy DID
/// - `legacy_verkey` Ed25519 verification key of the legacy DID identifier.
/// - `ed25519_signature` ED25519 signature to prove key possession.
/// - `signature` signature of DID identity owner
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_did_mapping_signed_transaction(
    client: &LedgerClient,
    from: &Address,
    identity: &Address,
    legacy_identifier: &LegacyDid,
    legacy_verkey: &LegacyVerkey,
    ed25519_signature: &Ed25519Signature,
    signature: &SignatureData,
) -> VdrResult<Transaction> {
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_DID_MAPPING_SIGNED)
        .add_param(identity)?
        .add_param(signature.v())?
        .add_param(signature.r())?
        .add_param(signature.s())?
        .add_param(legacy_identifier)?
        .add_param(legacy_verkey)?
        .add_param(ed25519_signature)?
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await
}

/// Build transaction to execute SchemaRegistry.didMappings contract method to get
///   new identity DID for legacy DID identifier
///
/// #Params
/// - `client` client connected to the network where contract will be executed
/// - `legacy_identifier` identifier of legacy sov/indy DID
///
/// #Returns
///   transaction: Transaction - prepared read transaction object to submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_get_did_mapping_transaction(
    client: &LedgerClient,
    legacy_identifier: &LegacyDid,
) -> VdrResult<Transaction> {
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_DID_MAPPING)
        .add_param(legacy_identifier)?
        .set_type(TransactionType::Read)
        .build(client)
        .await
}

/// Parse the result of execution SchemaRegistry.didMappings contract method to receive
///   new identity DID for legacy DID identifier
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Identity DID
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_did_mapping_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<DID> {
    let address = TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_DID_MAPPING)
        .parse::<Address>(client, bytes)?;
    Ok(DID::build(ETHR_DID_METHOD, None, address.as_ref()))
}

/// Build transaction to execute LegacyIdentifiersRegistry.createClMapping contract method to
///  create mapping of legacy schema/credential definition identifier to new one.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `identity` transaction sender account address (new account)
/// - `legacy_issuer_identifier` identifier of legacy sov/indy DID
/// - `legacy_identifier` legacy identifier.
/// - `new_identifier` new identifier.
///
/// # Returns
/// Write transaction to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_cl_mapping_transaction(
    client: &LedgerClient,
    identity: &Address,
    legacy_issuer_identifier: &LegacyDid,
    legacy_identifier: &Identifier,
    new_identifier: &Identifier,
) -> VdrResult<Transaction> {
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_CL_MAPPING)
        .add_param(identity)?
        .add_param(legacy_issuer_identifier)?
        .add_param(legacy_identifier)?
        .add_param(new_identifier)?
        .set_type(TransactionType::Write)
        .set_from(identity)
        .build(client)
        .await
}

/// Prepared data for execution of LegacyIdentifiersRegistry.createClMapping contract method to
///     endorse a new CL mapping.
///
/// #Params
/// - `client` client connected to the network where contract will be executed
/// - `identity` transaction sender account address (new account)
/// - `legacy_issuer_identifier` identifier of legacy sov/indy DID
/// - `legacy_identifier` legacy identifier.
/// - `new_identifier` new identifier.
///
/// #Returns
///   data: TransactionEndorsingData - transaction endorsement data to sign
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_cl_mapping_endorsing_data(
    client: &LedgerClient,
    identity: &Address,
    legacy_issuer_identifier: &LegacyDid,
    legacy_identifier: &Identifier,
    new_identifier: &Identifier,
) -> VdrResult<TransactionEndorsingData> {
    TransactionEndorsingDataBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_identity(&identity)
        .add_param(MethodStringParam::from(METHOD_CREATE_CL_MAPPING))?
        .add_param(legacy_issuer_identifier)?
        .add_param(legacy_identifier)?
        .add_param(new_identifier)?
        .build(client)
        .await
}

/// Build transaction to execute LegacyIdentifiersRegistry.createDidMappingSigned contract method to
///  endorse a legacy DID identifier mapping
/// Endorsing version of the method - sender is not identity owner
///
/// #Params
/// - `client` client connected to the network where contract will be executed
/// - `identity` transaction sender account address (new account)
/// - `legacy_issuer_identifier` identifier of legacy sov/indy DID
/// - `legacy_identifier` legacy identifier.
/// - `new_identifier` new identifier.
/// - `signature` signature of DID identity owner
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_create_cl_mapping_signed_transaction(
    client: &LedgerClient,
    from: &Address,
    identity: &Address,
    legacy_issuer_identifier: &LegacyDid,
    legacy_identifier: &Identifier,
    new_identifier: &Identifier,
    signature: &SignatureData,
) -> VdrResult<Transaction> {
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CREATE_CL_MAPPING_SIGNED)
        .add_param(identity)?
        .add_param(signature.v())?
        .add_param(signature.r())?
        .add_param(signature.s())?
        .add_param(legacy_issuer_identifier)?
        .add_param(legacy_identifier)?
        .add_param(new_identifier)?
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await
}

/// Build transaction to execute LegacyIdentifiersRegistry.clMappings contract method to get
///   new identifier for a legacy Schema/CredentialDefinition
///
/// #Params
/// - `client` client connected to the network where contract will be executed
/// - `legacy_identifier` identifier of legacy Schema/CredentialDefinition
///
/// #Returns
///   transaction: Transaction - prepared read transaction object to submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_get_cl_mapping_transaction(
    client: &LedgerClient,
    legacy_identifier: &Identifier,
) -> VdrResult<Transaction> {
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CL_MAPPING)
        .add_param(legacy_identifier)?
        .set_type(TransactionType::Read)
        .build(client)
        .await
}

/// Parse the result of execution SchemaRegistry.clMappings contract method to receive
///   new identifier for a legacy Schema/CredentialDefinition
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   New identifier
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_cl_mapping_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Identifier> {
    TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CL_MAPPING)
        .parse::<Identifier>(client, bytes)
}