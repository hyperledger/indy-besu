use log_derive::{logfn, logfn_inputs};

use crate::{
    client::LedgerClient,
    contracts::{
        did::{
            did_ethr_resolver,
            types::{
                did_doc_attribute::{DelegateType, DidDocAttribute, Validity},
                did_events::{DidAttributeChanged, DidDelegateChanged, DidEvents, DidOwnerChanged},
            },
            DidResolutionOptions,
        },
        DidDocumentWithMeta,
    },
    error::VdrResult,
    types::{
        Address, EventLog, EventParser, EventQuery, EventQueryBuilder, MethodStringParam,
        MethodUintBytesParam, Transaction, TransactionBuilder, TransactionEndorsingDataBuilder,
        TransactionParser, TransactionType,
    },
    Block, Nonce, SignatureData, TransactionEndorsingData, VdrError, DID,
};

const CONTRACT_NAME: &str = "EthereumExtDidRegistry";

const METHOD_DID_CHANGED: &str = "changed";
const METHOD_DID_NONCE: &str = "nonce";
const METHOD_DID_OWNER: &str = "owners";
const METHOD_CHANGE_OWNER: &str = "changeOwner";
const METHOD_CHANGE_OWNER_SIGNED: &str = "changeOwnerSigned";
const METHOD_ADD_DELEGATE: &str = "addDelegate";
const METHOD_ADD_DELEGATE_SIGNED: &str = "addDelegateSigned";
const METHOD_REVOKE_DELEGATE: &str = "revokeDelegate";
const METHOD_REVOKE_DELEGATE_SIGNED: &str = "revokeDelegateSigned";
const METHOD_SET_ATTRIBUTE: &str = "setAttribute";
const METHOD_SET_ATTRIBUTE_SIGNED: &str = "setAttributeSigned";
const METHOD_REVOKE_ATTRIBUTE: &str = "revokeAttribute";
const METHOD_REVOKE_ATTRIBUTE_SIGNED: &str = "revokeAttributeSigned";

const EVENT_DID_ATTRIBUTE_CHANGED: &str = "DIDAttributeChanged";
const EVENT_DID_DELEGATE_CHANGED: &str = "DIDDelegateChanged";
const EVENT_DID_OWNER_CHANGED: &str = "DIDOwnerChanged";

pub const ETHR_DID_METHOD: &str = "ethr";

// TODO: In current implementation all methods accept DID but contract API accept identity account address
//  Should we change it?

/// Build transaction to execute EthereumExtDidRegistry.changeOwner contract method to
///   change the owner of ether DID
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `sender` sender account address (Must be DID owner)
///  - `did` DID to change ownership
///  - `new_owner` account address of new owner
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_change_owner_transaction(
    client: &LedgerClient,
    sender: &Address,
    did: &DID,
    new_owner: &Address,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CHANGE_OWNER)
        .add_param(&identity)?
        .add_param(new_owner)?
        .set_type(TransactionType::Write)
        .set_from(sender)
        .build(client)
        .await
}

/// Prepared data for endorsing EthereumExtDidRegistry.changeOwner contract method
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` DID to change ownership
///  - `new_owner` account address of new owner
///
/// #Returns
///   data: TransactionEndorsingData - transaction endorsement data to sign
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_change_owner_endorsing_data(
    client: &LedgerClient,
    did: &DID,
    new_owner: &Address,
) -> VdrResult<TransactionEndorsingData> {
    let identity = Address::try_from(did)?;
    let nonce = resolve_identity_nonce(client, &identity).await?;
    TransactionEndorsingDataBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_identity(&identity)
        .add_param(&nonce)?
        .add_param(&identity)?
        .add_param(MethodStringParam::from(METHOD_CHANGE_OWNER))?
        .add_param(new_owner)?
        .build(client)
        .await
}

/// Build transaction to execute EthereumExtDidRegistry.changeOwnerSigned contract method to
///   change the owner of ether DID
/// Endorsing version of the method - sender is not identity owner
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `sender` sender account address
///  - `did` DID to change ownership
///  - `new_owner` account address of new owner
///  - `signature` signature of DID identity owner
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_change_owner_signed_transaction(
    client: &LedgerClient,
    sender: &Address,
    did: &DID,
    new_owner: &Address,
    signature: &SignatureData,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_CHANGE_OWNER_SIGNED)
        .add_param(&identity)?
        .add_param(signature.v())?
        .add_param(signature.r())?
        .add_param(signature.s())?
        .add_param(new_owner)?
        .set_type(TransactionType::Write)
        .set_from(sender)
        .build(client)
        .await
}

/// Build transaction to execute EthereumExtDidRegistry.addDelegate contract method to add a delegate.
/// An identity can assign multiple delegates to manage signing on their behalf for specific purposes.
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `sender` sender account address (Must be DID owner)
///  - `did` DID to add a delegate
///  - `delegate_type` type of delegation (`veriKey` or `sigAuth`)
///  - `delegate` account address of delegate
///  - `validity` delegate validity time
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_add_delegate_transaction(
    client: &LedgerClient,
    sender: &Address,
    did: &DID,
    delegate_type: &DelegateType,
    delegate: &Address,
    validity: &Validity,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_ADD_DELEGATE)
        .add_param(&identity)?
        .add_param(delegate_type)?
        .add_param(delegate)?
        .add_param(validity)?
        .set_type(TransactionType::Write)
        .set_from(sender)
        .build(client)
        .await
}

/// Prepared data for endorsing EthereumExtDidRegistry.addDelegate contract method
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` DID to add a delegate
///  - `delegate_type` type of delegation (`veriKey` or `sigAuth`)
///  - `delegate` account address of delegate
///  - `validity` delegate validity time
///
/// #Returns
///   data: TransactionEndorsingData - transaction endorsement data to sign
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_add_delegate_endorsing_data(
    client: &LedgerClient,
    did: &DID,
    delegate_type: &DelegateType,
    delegate: &Address,
    validity: &Validity,
) -> VdrResult<TransactionEndorsingData> {
    let identity = Address::try_from(did)?;
    let nonce = resolve_identity_nonce(client, &identity).await?;
    TransactionEndorsingDataBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_identity(&identity)
        .add_param(&nonce)?
        .add_param(&identity)?
        .add_param(MethodStringParam::from(METHOD_ADD_DELEGATE))?
        .add_param(delegate_type)?
        .add_param(delegate)?
        .add_param(MethodUintBytesParam::from(validity.0))?
        .build(client)
        .await
}

/// Build transaction to execute EthereumExtDidRegistry.addDelegateSigned contract method to add a delegate.
/// An identity can assign multiple delegates to manage signing on their behalf for specific purposes.
///
/// Endorsing version of the method - sender is not identity owner
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `sender` sender account address
///  - `did` DID to add a delegate
///  - `delegate_type` type of delegation (`veriKey` or `sigAuth`)
///  - `delegate` account address of delegate
///  - `validity` delegate validity time
///  - `signature` signature of DID identity owner
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_add_delegate_signed_transaction(
    client: &LedgerClient,
    sender: &Address,
    did: &DID,
    delegate_type: &DelegateType,
    delegate: &Address,
    validity: &Validity,
    signature: &SignatureData,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_ADD_DELEGATE_SIGNED)
        .add_param(&identity)?
        .add_param(signature.v())?
        .add_param(signature.r())?
        .add_param(signature.s())?
        .add_param(delegate_type)?
        .add_param(delegate)?
        .add_param(validity)?
        .set_type(TransactionType::Write)
        .set_from(sender)
        .build(client)
        .await
}

/// Build transaction to execute EthereumExtDidRegistry.revokeDelegate contract method to revoke a delegate.
/// An identity can assign multiple delegates to manage signing on their behalf for specific purposes.
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `sender` sender account address (Must be DID owner)
///  - `did` DID to revoke a delegate
///  - `delegate_type` type of delegation (`veriKey` or `sigAuth`)
///  - `delegate` account address of delegate
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_revoke_delegate_transaction(
    client: &LedgerClient,
    sender: &Address,
    did: &DID,
    delegate_type: &DelegateType,
    delegate: &Address,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_REVOKE_DELEGATE)
        .add_param(&identity)?
        .add_param(delegate_type)?
        .add_param(delegate)?
        .set_type(TransactionType::Write)
        .set_from(sender)
        .build(client)
        .await
}

/// Prepared data for endorsing EthereumExtDidRegistry.revokeDelegate contract method
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` DID to add a delegate
///  - `delegate_type` type of delegation (`veriKey` or `sigAuth`)
///  - `delegate` account address of delegate
///
/// #Returns
///   data: TransactionEndorsingData - transaction endorsement data to sign
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_revoke_delegate_endorsing_data(
    client: &LedgerClient,
    did: &DID,
    delegate_type: &DelegateType,
    delegate: &Address,
) -> VdrResult<TransactionEndorsingData> {
    let identity = Address::try_from(did)?;
    let nonce = resolve_identity_nonce(client, &identity).await?;
    TransactionEndorsingDataBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_identity(&identity)
        .add_param(&nonce)?
        .add_param(&identity)?
        .add_param(MethodStringParam::from(METHOD_REVOKE_DELEGATE))?
        .add_param(delegate_type)?
        .add_param(delegate)?
        .build(client)
        .await
}

/// Build transaction to execute EthereumExtDidRegistry.revokeDelegateSigned contract method to revoke a delegate.
/// An identity can assign multiple delegates to manage signing on their behalf for specific purposes.
///
/// Endorsing version of the method - sender is not identity owner
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `sender` sender account address
///  - `did` DID to revoke a delegate
///  - `delegate_type` type of delegation (`veriKey` or `sigAuth`)
///  - `delegate` account address of delegate
///  - `signature` signature of DID identity owner
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_revoke_delegate_signed_transaction(
    client: &LedgerClient,
    sender: &Address,
    did: &DID,
    delegate_type: &DelegateType,
    delegate: &Address,
    signature: &SignatureData,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_REVOKE_DELEGATE_SIGNED)
        .add_param(&identity)?
        .add_param(signature.v())?
        .add_param(signature.r())?
        .add_param(signature.s())?
        .add_param(delegate_type)?
        .add_param(delegate)?
        .set_type(TransactionType::Write)
        .set_from(sender)
        .build(client)
        .await
}

/// Build transaction to execute EthereumExtDidRegistry.setAttribute contract method to add
///   a non ledger DID associated attribute.
/// An identity may need to publish some information that is only needed off-chain but
///   still requires the security benefits of using a blockchain.
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `sender` sender account address (Must be DID owner)
///  - `did` DID to add an attribute
///  - `attribute` attribute to add
///  - `validity` attribute validity time
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_set_attribute_transaction(
    client: &LedgerClient,
    sender: &Address,
    did: &DID,
    attribute: &DidDocAttribute,
    validity: &Validity,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_SET_ATTRIBUTE)
        .add_param(&identity)?
        .add_param(&attribute.name()?)?
        .add_param(&attribute.value()?)?
        .add_param(validity)?
        .set_type(TransactionType::Write)
        .set_from(sender)
        .build(client)
        .await
}

/// Prepared data for endorsing EthereumExtDidRegistry.setAttribute contract method
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` DID to add an attribute
///  - `attribute` attribute to add
///  - `validity` attribute validity time
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_set_attribute_endorsing_data(
    client: &LedgerClient,
    did: &DID,
    attribute: &DidDocAttribute,
    validity: &Validity,
) -> VdrResult<TransactionEndorsingData> {
    let identity = Address::try_from(did)?;
    let nonce = resolve_identity_nonce(client, &identity).await?;
    TransactionEndorsingDataBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_identity(&identity)
        .add_param(&nonce)?
        .add_param(&identity)?
        .add_param(MethodStringParam::from(METHOD_SET_ATTRIBUTE))?
        .add_param(&attribute.name()?)?
        .add_param(&attribute.value()?)?
        .add_param(MethodUintBytesParam::from(validity.0))?
        .build(client)
        .await
}

/// Build transaction to execute EthereumExtDidRegistry.setAttributeSigned contract method to add
///   a non ledger DID associated attribute.
/// An identity may need to publish some information that is only needed off-chain but
///   still requires the security benefits of using a blockchain.
///
/// Endorsing version of the method - sender is not identity owner
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `sender` sender account address
///  - `did` DID to add an attribute
///  - `attribute` attribute to add
///  - `validity` attribute validity time
///  - `signature` signature of DID identity owner
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_set_attribute_signed_transaction(
    client: &LedgerClient,
    sender: &Address,
    did: &DID,
    attribute: &DidDocAttribute,
    validity: &Validity,
    signature: &SignatureData,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_SET_ATTRIBUTE_SIGNED)
        .add_param(&identity)?
        .add_param(signature.v())?
        .add_param(signature.r())?
        .add_param(signature.s())?
        .add_param(&attribute.name()?)?
        .add_param(&attribute.value()?)?
        .add_param(validity)?
        .set_type(TransactionType::Write)
        .set_from(sender)
        .build(client)
        .await
}

/// Build transaction to execute EthereumExtDidRegistry.revokeAttribute contract method to revoke
///   a non ledger DID associated attribute.
/// An identity may need to publish some information that is only needed off-chain but
///   still requires the security benefits of using a blockchain.
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `sender` sender account address (Must be DID owner)
///  - `did` DID to revoke an attribute
///  - `attribute` attribute to add
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_revoke_attribute_transaction(
    client: &LedgerClient,
    sender: &Address,
    did: &DID,
    attribute: &DidDocAttribute,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_REVOKE_ATTRIBUTE)
        .add_param(&identity)?
        .add_param(&attribute.name()?)?
        .add_param(&attribute.value()?)?
        .set_type(TransactionType::Write)
        .set_from(sender)
        .build(client)
        .await
}

/// Prepared data for endorsing EthereumExtDidRegistry.revokeAttribute contract method
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` DID to add an attribute
///  - `attribute` attribute to add
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_revoke_attribute_endorsing_data(
    client: &LedgerClient,
    did: &DID,
    attribute: &DidDocAttribute,
) -> VdrResult<TransactionEndorsingData> {
    let identity = Address::try_from(did)?;
    let nonce = resolve_identity_nonce(client, &identity).await?;
    TransactionEndorsingDataBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_identity(&identity)
        .add_param(&nonce)?
        .add_param(&identity)?
        .add_param(MethodStringParam::from(METHOD_REVOKE_ATTRIBUTE))?
        .add_param(&attribute.name()?)?
        .add_param(&attribute.value()?)?
        .build(client)
        .await
}

/// Build transaction to execute EthereumExtDidRegistry.revokeAttributeSigned contract method to revoke
///   a non ledger DID associated attribute.
/// An identity may need to publish some information that is only needed off-chain but
///   still requires the security benefits of using a blockchain.
///
/// Endorsing version of the method - sender is not identity owner
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `sender` sender account address
///  - `did` DID to revoke an attribute
///  - `attribute` attribute to add
///  - `signature` signature of DID identity owner
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_did_revoke_attribute_signed_transaction(
    client: &LedgerClient,
    sender: &Address,
    did: &DID,
    attribute: &DidDocAttribute,
    signature: &SignatureData,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_REVOKE_ATTRIBUTE_SIGNED)
        .add_param(&identity)?
        .add_param(signature.v())?
        .add_param(signature.r())?
        .add_param(signature.s())?
        .add_param(&attribute.name()?)?
        .add_param(&attribute.value()?)?
        .set_type(TransactionType::Write)
        .set_from(sender)
        .build(client)
        .await
}

/// Build transaction to execute EthereumExtDidRegistry.owners contract method to get
///   an account address owning the DID.
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` target DID
///
/// #Returns
///   transaction: Transaction - prepared read transaction object to submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_get_did_owner_transaction(
    client: &LedgerClient,
    did: &DID,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_DID_OWNER)
        .add_param(&identity)?
        .set_type(TransactionType::Read)
        .build(client)
        .await
}

/// Build transaction to execute EthereumExtDidRegistry.changed contract method to get
///   block number when DID was changed last time
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` target DID
///
/// #Returns
///   transaction: Transaction - prepared read transaction object to submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_get_did_changed_transaction(
    client: &LedgerClient,
    did: &DID,
) -> VdrResult<Transaction> {
    let identity = Address::try_from(did)?;
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_DID_CHANGED)
        .add_param(&identity)?
        .set_type(TransactionType::Read)
        .build(client)
        .await
}

/// Build transaction to execute EthereumExtDidRegistry.nonce contract method to get signing
///   nonce needed for endorsement
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` target DID
///
/// #Returns
///   transaction: Transaction - prepared read transaction object to submit
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_get_identity_nonce_transaction(
    client: &LedgerClient,
    identity: &Address,
) -> VdrResult<Transaction> {
    TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_DID_NONCE)
        .add_param(identity)?
        .set_type(TransactionType::Read)
        .build(client)
        .await
}

/// Build event query to obtain log DID associated events from the ledger
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` target DID
///  - `from_block` start block
///  - `to_block` finish block
///
/// #Returns
///   query: EventQuery - prepared event query to send
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn build_get_did_events_query(
    client: &LedgerClient,
    did: &DID,
    from_block: Option<&Block>,
    to_block: Option<&Block>,
) -> VdrResult<EventQuery> {
    let address = Address::try_from(did)?;
    EventQueryBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_from_block(from_block.cloned())
        .set_to_block(to_block.cloned())
        .set_event_filer(address.to_filter())
        .build(client)
}

/// Parse the result of execution EthereumExtDidRegistry.changed contract method to receive
///   a block number when DID was changed last time
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Block number
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_did_changed_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Block> {
    TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_DID_CHANGED)
        .parse::<Block>(client, bytes)
}

/// Parse the result of execution EthereumExtDidRegistry.owners contract method to receive
///   an account address owning the DID.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Owner account address
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_did_owner_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Address> {
    TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_DID_OWNER)
        .parse::<Address>(client, bytes)
}

/// Parse the result of execution EthereumExtDidRegistry.nonce contract method to receive
///   a signing nonce needed for endorsement
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Nonce to use for endorsing
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_did_nonce_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Nonce> {
    TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_DID_NONCE)
        .parse::<Nonce>(client, bytes)
}

/// Parse DidAttributeChangedEvent from the event log.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Parsed DidAttributeChanged event object
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_did_attribute_changed_event_response(
    client: &LedgerClient,
    log: &EventLog,
) -> VdrResult<DidAttributeChanged> {
    EventParser::new()
        .set_contract(CONTRACT_NAME)
        .set_event(EVENT_DID_ATTRIBUTE_CHANGED)
        .parse::<DidAttributeChanged>(client, log)
}

/// Parse DidDelegateChangedEvent from the event log.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Parsed DidDelegateChanged event object
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_did_delegate_changed_event_response(
    client: &LedgerClient,
    log: &EventLog,
) -> VdrResult<DidDelegateChanged> {
    EventParser::new()
        .set_contract(CONTRACT_NAME)
        .set_event(EVENT_DID_DELEGATE_CHANGED)
        .parse::<DidDelegateChanged>(client, log)
}

/// Parse DidOwnerChangedEvent from the event log.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Parsed DidOwnerChanged event object
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_did_owner_changed_event_response(
    client: &LedgerClient,
    log: &EventLog,
) -> VdrResult<DidOwnerChanged> {
    EventParser::new()
        .set_contract(CONTRACT_NAME)
        .set_event(EVENT_DID_OWNER_CHANGED)
        .parse::<DidOwnerChanged>(client, log)
}

/// Parse DID associated event from the event log (it can be one of: DidAttributeChanged, DidDelegateChanged, DidOwnerChanged).
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Parsed DID event object
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub fn parse_did_event_response(client: &LedgerClient, event: &EventLog) -> VdrResult<DidEvents> {
    let contract = client.contract(CONTRACT_NAME)?;

    let event_signature = event.topics.first().ok_or_else(|| {
        VdrError::ContractInvalidResponseData("Unable to get event topic".to_string())
    })?;

    let did_attribute_changed_event_signature =
        contract.event(EVENT_DID_ATTRIBUTE_CHANGED)?.signature();
    let did_delegate_changed_event_signature =
        contract.event(EVENT_DID_DELEGATE_CHANGED)?.signature();
    let did_owner_changed_event_signature = contract.event(EVENT_DID_OWNER_CHANGED)?.signature();

    if event_signature.eq(&did_attribute_changed_event_signature) {
        return parse_did_attribute_changed_event_response(client, event)
            .map(DidEvents::AttributeChangedEvent);
    }

    if event_signature.eq(&did_delegate_changed_event_signature) {
        return parse_did_delegate_changed_event_response(client, event)
            .map(DidEvents::DelegateChanged);
    }

    if event_signature.eq(&did_owner_changed_event_signature) {
        return parse_did_owner_changed_event_response(client, event).map(DidEvents::OwnerChanged);
    }

    Err(VdrError::ContractInvalidResponseData(format!(
        "Unexpected contract event. Event signature: {:?}",
        event_signature
    )))
}

#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn resolve_identity_nonce(client: &LedgerClient, identity: &Address) -> VdrResult<Nonce> {
    let transaction = build_get_identity_nonce_transaction(client, identity).await?;
    let response = client.submit_transaction(&transaction).await?;
    parse_did_nonce_result(client, &response)
}

/// Single step function to resolve a DidDocument with metadata for the given DID
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `did` DID to get a DID Document and metadata
/// - `options` Resolution options
///
/// # Returns
///   Parsed DID event object
#[logfn(Info)]
#[logfn_inputs(Debug)]
pub async fn resolve_did(
    client: &LedgerClient,
    did: &DID,
    options: Option<&DidResolutionOptions>,
) -> VdrResult<DidDocumentWithMeta> {
    did_ethr_resolver::resolve_did(client, did, options).await
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        client::client::test::{
            mock_client, CHAIN_ID, DEFAULT_NONCE, ETHR_DID_REGISTRY_ADDRESS, IDENTITY_ACC,
            TRUSTEE_ACC,
        },
        contracts::{
            did::types::{
                did::DID,
                did_doc::test::{SERVICE_ENDPOINT, SERVICE_TYPE},
                did_doc_attribute::{
                    PublicKeyAttribute, PublicKeyPurpose, PublicKeyType, ServiceAttribute,
                },
            },
            ServiceEndpoint,
        },
        utils::init_env_logger,
    };
    use std::sync::RwLock;

    fn did() -> DID {
        DID::from(format!("did:ethr:{}", IDENTITY_ACC.as_ref()).as_str())
    }

    pub fn service() -> DidDocAttribute {
        DidDocAttribute::Service(ServiceAttribute {
            type_: SERVICE_TYPE.to_string(),
            service_endpoint: ServiceEndpoint::String(SERVICE_ENDPOINT.to_string()),
        })
    }

    pub fn public_key() -> DidDocAttribute {
        DidDocAttribute::PublicKey(PublicKeyAttribute {
            purpose: PublicKeyPurpose::Enc,
            type_: PublicKeyType::X25519KeyAgreementKey2020,
            public_key_hex: None,
            public_key_base64: None,
            public_key_base58: Some("FbQWLPRhTH95MCkQUeFYdiSoQt8zMwetqfWoxqPgaq7x".to_string()),
            public_key_pem: None,
        })
    }

    pub fn public_key_2() -> DidDocAttribute {
        DidDocAttribute::PublicKey(PublicKeyAttribute {
            purpose: PublicKeyPurpose::Enc,
            type_: PublicKeyType::Ed25519VerificationKey2020,
            public_key_base58: Some("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".to_string()),
            ..PublicKeyAttribute::default()
        })
    }

    pub fn public_key_3() -> DidDocAttribute {
        DidDocAttribute::PublicKey(PublicKeyAttribute {
            purpose: PublicKeyPurpose::VeriKey,
            type_: PublicKeyType::EcdsaSecp256k1VerificationKey2020,
            public_key_hex: Some(
                "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71".to_string(),
            ),
            ..PublicKeyAttribute::default()
        })
    }

    pub fn validity() -> Validity {
        Validity::from(1000)
    }

    mod build_did_change_owner_transaction {
        use super::*;

        #[async_std::test]
        async fn build_did_change_owner_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let transaction =
                build_did_change_owner_transaction(&client, &IDENTITY_ACC, &did(), &TRUSTEE_ACC)
                    .await
                    .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(IDENTITY_ACC.clone()),
                to: ETHR_DID_REGISTRY_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: vec![
                    240, 13, 75, 93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 5, 148, 0, 220, 208,
                    81, 88, 255, 216, 202, 9, 41, 55, 152, 157, 210, 123, 59, 220, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 240, 226, 219, 108, 141, 198, 198, 129, 187, 93, 106, 209,
                    33, 161, 7, 243, 0, 233, 178, 181,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }
    }

    mod build_did_add_delegate_transaction {
        use super::*;

        #[async_std::test]
        async fn build_did_add_delegate_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let transaction = build_did_add_delegate_transaction(
                &client,
                &IDENTITY_ACC,
                &did(),
                &DelegateType::VeriKey,
                &TRUSTEE_ACC,
                &validity(),
            )
            .await
            .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(IDENTITY_ACC.clone()),
                to: ETHR_DID_REGISTRY_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: vec![
                    167, 6, 141, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 5, 148, 0, 220, 208,
                    81, 88, 255, 216, 202, 9, 41, 55, 152, 157, 210, 123, 59, 220, 118, 101, 114,
                    105, 75, 101, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 226, 219, 108, 141,
                    198, 198, 129, 187, 93, 106, 209, 33, 161, 7, 243, 0, 233, 178, 181, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 3, 232,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }
    }

    mod build_did_revoke_delegate_transaction {
        use super::*;

        #[async_std::test]
        async fn build_did_revoke_delegate_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let transaction = build_did_revoke_delegate_transaction(
                &client,
                &IDENTITY_ACC,
                &did(),
                &DelegateType::VeriKey,
                &TRUSTEE_ACC,
            )
            .await
            .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(IDENTITY_ACC.clone()),
                to: ETHR_DID_REGISTRY_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: vec![
                    128, 178, 159, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 5, 148, 0, 220,
                    208, 81, 88, 255, 216, 202, 9, 41, 55, 152, 157, 210, 123, 59, 220, 118, 101,
                    114, 105, 75, 101, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 226, 219, 108,
                    141, 198, 198, 129, 187, 93, 106, 209, 33, 161, 7, 243, 0, 233, 178, 181,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }
    }

    mod build_did_set_attribute_transaction {
        use super::*;

        #[async_std::test]
        async fn build_did_set_service_attribute_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let transaction = build_did_set_attribute_transaction(
                &client,
                &IDENTITY_ACC,
                &did(),
                &service(),
                &validity(),
            )
            .await
            .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(IDENTITY_ACC.clone()),
                to: ETHR_DID_REGISTRY_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: vec![
                    122, 212, 176, 164, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 5, 148, 0, 220,
                    208, 81, 88, 255, 216, 202, 9, 41, 55, 152, 157, 210, 123, 59, 220, 100, 105,
                    100, 47, 115, 118, 99, 47, 83, 101, 114, 118, 105, 99, 101, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 232, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 18, 104, 116, 116, 112, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46,
                    99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }

        #[async_std::test]
        async fn build_did_set_key_attribute_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let transaction = build_did_set_attribute_transaction(
                &client,
                &IDENTITY_ACC,
                &did(),
                &public_key(),
                &validity(),
            )
            .await
            .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(IDENTITY_ACC.clone()),
                to: ETHR_DID_REGISTRY_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: vec![
                    122, 212, 176, 164, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 5, 148, 0, 220,
                    208, 81, 88, 255, 216, 202, 9, 41, 55, 152, 157, 210, 123, 59, 220, 100, 105,
                    100, 47, 112, 117, 98, 47, 88, 50, 53, 53, 49, 57, 47, 101, 110, 99, 47, 98,
                    97, 115, 101, 53, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 232,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 32, 216, 211, 241, 73, 138, 202, 20, 247, 254, 92, 152, 102, 20,
                    103, 236, 224, 41, 108, 66, 163, 228, 133, 29, 248, 18, 225, 230, 17, 163, 84,
                    230, 43,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }
    }

    mod build_did_revoke_attribute_transaction {
        use super::*;

        #[async_std::test]
        async fn build_did_revoke_attribute_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let transaction =
                build_did_revoke_attribute_transaction(&client, &IDENTITY_ACC, &did(), &service())
                    .await
                    .unwrap();
            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(IDENTITY_ACC.clone()),
                to: ETHR_DID_REGISTRY_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: vec![
                    0, 192, 35, 218, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 5, 148, 0, 220, 208,
                    81, 88, 255, 216, 202, 9, 41, 55, 152, 157, 210, 123, 59, 220, 100, 105, 100,
                    47, 115, 118, 99, 47, 83, 101, 114, 118, 105, 99, 101, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 104, 116, 116,
                    112, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                signature: RwLock::new(None),
                hash: None,
            };
            assert_eq!(expected_transaction, transaction);
        }
    }
}
