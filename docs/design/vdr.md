# VDR Design

**Disclaimer:** popular packages for working with Ethereum network are very close to `indy-sdk`: their provide tide
modules for the whole flow execution. It may be difficult to reuse only particular functions without full integration.
In the same, time Indy community follows to idea of splitting complex library into components.

> Rust client library to work with Web3, Solidity smart contracts: https://github.com/tomusdrw/rust-web3

## VDR library assumptions

* VDR library firstly will be implemented independently and later integrate into existing indy vdr/sdk/frameworks.
* VDR library will be written in Rust with providing language wrappers.
* VDR library simplifies migration from legacy `sov/indy` formatted did's and id's.
* VDR library simplifies migration from legacy entities (DID Document, Schema, Credential Definition) into
  specifications compatible format.
* VDR library does validation of input and output data.
* VDR library does only Ethereum base account signing of transactions.
* VDR library request builders are tide to the network.
* VDR library will work with RPC Node over HTTP.

## Client

```rust
pub struct LedgerClient {
    chain_id: u64,
    client: Box<dyn Client>,
    contracts: HashMap<String, Box<dyn Contract>>,
    quorum_handler: Option<QuorumHandler>,
}

struct ContractConfig {
    address: String,
    // address of deployed contract
    spec_path: String, // path to JSON file containing compiled contract's ABI specification
}

struct StatusResult {
    status: Status
}

enum Status {
    Ok,
    Err(String)
}

impl LedgerClient {
    /// Create indybesu client interacting with ledger
    ///
    /// # Params
    ///  - `chain_id` - chain id of network (chain ID is part of the transaction signing process to protect against transaction replay attack)
    ///  - `rpc_node` - string - RPC node endpoint
    ///  - `contract_configs` - [ContractSpec] specifications for contracts  deployed on the network
    ///  - `quorum_config` - Option<[QuorumConfig]> quorum configuration. Can be None if quorum is not needed
    ///
    /// # Returns
    ///  client to use for building and sending transactions
    fn new(
        chain_id: u64,
        node_address: String,
        contract_configs: Vec<ContractConfig>,
        quorum_config: Option<QuorumConfig>,
    ) -> LedgerClient {
        unimpltemented!()
    }

    /// Ping Ledger.
    ///
    /// # Returns
    ///  ping status
    pub async fn ping(&self) -> VdrResult<PingStatus> {
        unimpltemented!()
    }

    /// Submit prepared transaction to the ledger
    ///     Depending on the transaction type Write/Read ethereum methods will be used
    ///
    /// #Params
    ///  `transaction` - transaction to submit
    ///
    /// #Returns
    ///  transaction execution result:
    ///    depending on the type it will be either result bytes or block hash
    pub async fn submit_transaction(&self, transaction: &Transaction) -> VdrResult<Vec<u8>> {
        unimpltemented!()
    }

    /// Get receipt for the given block hash
    ///
    /// # Params
    ///  `transaction` - transaction to submit
    ///
    /// # Returns
    ///  receipt for the given block
    pub async fn get_receipt(&self, hash: &[u8]) -> VdrResult<String> {
        unimpltemented!()
    }

    /// Send a prepared query for retrieving log events on the ledger
    ///
    /// #Params
    ///  param: client: Ledger - client (Ethereum client - for example web3::Http)
    ///  param: query: EventQuery - query to send
    ///
    /// #Returns
    ///   logs - list of received events
    pub async fn query_events(&self, query: &EventQuery) -> VdrResult<Vec<RawLog>>;
}

struct SubmitTransactionOptions {}

type Receipt = Vec<u8>;

trait Client: Sync + Send {
    /// Retrieve count of transaction for the given account
    ///
    /// # Params
    /// - `address` address of an account to get number of written transactions
    ///
    /// # Returns
    /// number of transactions
    async fn get_transaction_count(&self, address: &Address) -> VdrResult<[u64; 4]>;

    /// Submit transaction to the ledger
    ///
    /// # Params
    /// - `transaction` transaction to submit
    /// - `transaction` prepared transaction to submit
    ///
    /// # Returns
    /// hash of a block in which transaction included
    async fn submit_transaction(&self, transaction: &[u8]) -> VdrResult<Vec<u8>>;

    /// Submit read transaction to the ledger
    ///
    /// # Params
    /// - `transaction` prepared transaction to submit
    ///
    /// # Returns
    /// result data of transaction execution
    async fn call_transaction(&self, to: &str, transaction: &[u8]) -> VdrResult<Vec<u8>>;

    /// Get the receipt for the given block hash
    ///
    /// # Params
    /// - `hash` hash of a block to get the receipt
    ///
    /// # Returns
    /// receipt as JSON string for the requested block
    async fn get_receipt(&self, hash: &[u8]) -> VdrResult<String>;

    /// Check client connection (passed node is alive and return valid ledger data)
    ///
    /// # Returns
    /// ledger status
    async fn ping(&self) -> VdrResult<PingStatus>;

    /// Get the transaction for the given transaction hash
    ///
    /// # Params
    /// - `hash` hash of a transaction to get
    ///
    /// # Returns
    /// transaction for the requested hash
    async fn get_transaction(&self, hash: &[u8]) -> VdrResult<Option<Transaction>>;

    /// Request log events from the ledger for the given filter
    ///
    /// # Params
    /// - `query` log events filter to submit
    ///
    /// # Returns
    /// received log events
    async fn query_events(&self, query: &EventQuery) -> VdrResult<Vec<RawLog>>;
}

trait Contract: Sync + Send {
    /// Get the address of deployed contract
    ///
    /// # Returns
    /// address of the deployed contract. Should be used to execute contract methods
    fn address(&self) -> &Address;
  
    /// Get the contract function for the given name
    ///
    /// # Params
    /// - `name` name of the function to obtain
    ///
    /// # Returns
    /// Contract function
    fn function(&self, name: &str) -> VdrResult<&Function>;

    /// Get the contract event for the given name
    ///
    /// # Params
    /// - `name` name of the event to obtain
    ///
    /// # Returns
    /// Contract event
    fn event(&self, name: &str) -> VdrResult<&ContractEvent>;
}
```

## Transaction methods

```rust

/// Transaction object
struct Transaction {
    /// type of transaction: write/read
    /// depending on the transaction type different client methods will be executed to submit transaction
    type_: TransactionType,
    /// transaction sender account address
    from: Option<Address>,
    /// transaction recipient address
    to: String,
    /// nonce - count of transaction sent by account
    nonce: Option<u64>,
    /// chain id of the ledger
    chain_id: u64,
    /// transaction payload
    data: Vec<u8>,
    /// transaction signature
    signature: Option<TransactionSignature>,
}

impl Transaction {
    /// Get bytes which needs to be signed for transaction sending 
    ///
    /// # Returns
    ///  bytes to sign
    fn get_signing_bytes(&self) -> VdrResult<Vec<u8>> {
        unimplemented!()
    }

    /// Set transaction signature
    fn set_signature(&mut self, signature_data: SignatureData) {
        unimplemented!()
    }
}

enum TransactionType {
    Read,
    Write
}

struct TransactionEndorsingData {
  to: Address,
  from: Address,
  params: Vec<ContractParam>,
}

impl TransactionEndorsingData {
  fn get_signing_bytes(&self) -> VdrResult<Vec<u8>> {
    unimplemented!()
  }
}

struct EventQuery {
  address: Address,
  from_block: Option<Block>,
  to_block: Option<Block>,
  topic: String,
}

struct SignatureData {
    /// recovery ID using for public key recovery
    pub recovery_id: u64,
    /// ECDSA signature
    pub signature: Vec<u8>,
}
```

## Contracts/Requests methods

### DID IndyBesu
 
#### Writes

```rust
/// Prepare transaction executing `DidRegistry.createDid` smart contract method to create a new DID on the Ledger
///
/// #Params
///  param: client: LedgerClient - Ledger client
///  param: from: string - sender account address
///  param: did: string - DID to create
///  param: did_document: DidDocument - DID Document matching to the specification: https://www.w3.org/TR/did-core/
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_create_did_transaction(
    client: LedgerClient,
    from: String,
    did: String,
    did_document: DidDoc,
) -> Transaction {
    unimplemented!();
}
```

```rust
/// Prepare transaction executing `DidRegistry.updateDid` smart contract method to update an existing DID Document
///
/// #Params
///  param: client: LedgerClient - Ledger client
///  param: from: string - sender account address
///  param: did: string - DID to update
///  param: did_document: DidDocument - DID Document matching to the specification: https://www.w3.org/TR/did-core/
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_update_did_transaction(
  client: LedgerClient,
  from: String,
  did: String,
  did_document: DidDoc,
) -> Transaction;
```

```rust
/// Prepare transaction executing `DidRegistry.deactivateDid` smart contract method to deactivate an existing DID
///
/// #Params
///  param: client: LedgerClient - Ledger client
///  param: from: string - sender account address
///  param: did: string - did to deactivate
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_deactivate_did_transaction(
    client: LedgerClient,
    from: String,
    did: String,
) -> Transaction;
```

#### Resolve

```rust
/// Prepare transaction executing `DidRegistry.resolveDid` smart contract method to resolve a DID
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: did - DID to resolve
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_resolve_did_transaction(
    client: LedgerClient,
    did: String,
) -> Transaction;
```

```rust
/// Parse response for of `DidRegistry.resolveDid` smart contract 
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: response: bytes - received response
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_parse_resolve_did_response(
    client: LedgerClient,
    response: bytes,
) -> DidDocumentWithMeta;
```

### DID Ethr

#### Writes

```rust
/// Change owner of ethr DID 
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: did: string - DID to change ownership
///  param: new_owner: string - account address of new owner
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
pub async fn build_did_change_owner_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
    new_owner: &Address,
) -> VdrResult<Transaction>;

/// Prepared data for endorsing EthereumExtDidRegistry.changeOwner contract method
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` DID to change ownership
///  - `new_owner` account address of new owner
///
/// #Returns
///   data: TransactionEndorsingData - transaction endorsement data to sign
pub async fn build_did_change_owner_endorsing_data(
    client: &LedgerClient,
    did: &DID,
    new_owner: &Address,
) -> VdrResult<TransactionEndorsingData>;

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
pub async fn build_did_change_owner_signed_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
    new_owner: &Address,
    signature: &Signature,
) -> VdrResult<Transaction>;

/// An identity can assign multiple delegates to manage signing on their behalf for specific purposes.
/// Function to add a new delegate for a DID
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: did: string - DID to add delegate
///  param: delegate_type: string - type of delegation
///  param: delegate: string - account address of delegate
///  param: validity: Option<u64> - delegate validity time
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
pub async fn build_did_add_delegate_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
    delegate_type: &DelegateType,
    delegate: &Address,
    validity: &Validity,
) -> VdrResult<Transaction>;

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
pub async fn build_did_add_delegate_endorsing_data(
  client: &LedgerClient,
  did: &DID,
  delegate_type: &DelegateType,
  delegate: &Address,
  validity: &Validity,
) -> VdrResult<TransactionEndorsingData>;

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
pub async fn build_did_add_delegate_signed_transaction(
  client: &LedgerClient,
  sender: &Address,
  did: &DID,
  delegate_type: &DelegateType,
  delegate: &Address,
  validity: &Validity,
  signature: &SignatureData,
) -> VdrResult<Transaction>;

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
pub async fn build_did_revoke_delegate_transaction(
  client: &LedgerClient,
  sender: &Address,
  did: &DID,
  delegate_type: &DelegateType,
  delegate: &Address,
) -> VdrResult<Transaction>;

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
pub async fn build_did_revoke_delegate_endorsing_data(
  client: &LedgerClient,
  did: &DID,
  delegate_type: &DelegateType,
  delegate: &Address,
) -> VdrResult<TransactionEndorsingData>;

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
pub async fn build_did_revoke_delegate_signed_transaction(
  client: &LedgerClient,
  sender: &Address,
  did: &DID,
  delegate_type: &DelegateType,
  delegate: &Address,
  signature: &SignatureData,
) -> VdrResult<Transaction>;

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
pub async fn build_did_set_attribute_transaction(
  client: &LedgerClient,
  sender: &Address,
  did: &DID,
  attribute: &DidDocAttribute,
  validity: &Validity,
) -> VdrResult<Transaction>;

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
pub async fn build_did_set_attribute_endorsing_data(
  client: &LedgerClient,
  did: &DID,
  attribute: &DidDocAttribute,
  validity: &Validity,
) -> VdrResult<TransactionEndorsingData>;

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
pub async fn build_did_set_attribute_signed_transaction(
  client: &LedgerClient,
  sender: &Address,
  did: &DID,
  attribute: &DidDocAttribute,
  validity: &Validity,
  signature: &SignatureData,
) -> VdrResult<Transaction>;

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
pub async fn build_did_revoke_attribute_transaction(
  client: &LedgerClient,
  sender: &Address,
  did: &DID,
  attribute: &DidDocAttribute,
) -> VdrResult<Transaction>;

/// Prepared data for endorsing EthereumExtDidRegistry.revokeAttribute contract method
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` DID to add an attribute
///  - `attribute` attribute to add
///
/// #Returns
///   transaction: Transaction - prepared write transaction object to sign and submit
pub async fn build_did_revoke_attribute_endorsing_data(
  client: &LedgerClient,
  did: &DID,
  attribute: &DidDocAttribute,
) -> VdrResult<TransactionEndorsingData>;

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
pub async fn build_did_revoke_attribute_signed_transaction(
  client: &LedgerClient,
  sender: &Address,
  did: &DID,
  attribute: &DidDocAttribute,
  signature: &SignatureData,
) -> VdrResult<Transaction>;
```

#### Resolve

```rust
/// Build transaction to execute EthereumExtDidRegistry.owners contract method to get
///   an account address owning the DID.
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` target DID
///
/// #Returns
///   transaction: Transaction - prepared read transaction object to submit
pub async fn build_get_did_owner_transaction(
  client: &LedgerClient,
  did: &DID,
) -> VdrResult<Transaction>;

/// Build transaction to execute EthereumExtDidRegistry.changed contract method to get
///   block number when DID was changed last time
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` target DID
///
/// #Returns
///   transaction: Transaction - prepared read transaction object to submit
pub async fn build_get_did_changed_transaction(
  client: &LedgerClient,
  did: &DID,
) -> VdrResult<Transaction>;

/// Build transaction to execute EthereumExtDidRegistry.nonce contract method to get signing
///   nonce needed for endorsement
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `did` target DID
///
/// #Returns
///   transaction: Transaction - prepared read transaction object to submit
pub async fn build_get_identity_nonce_transaction(
  client: &LedgerClient,
  identity: &Address,
) -> VdrResult<Transaction>;

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
pub async fn build_get_did_events_query(
  client: &LedgerClient,
  did: &DID,
  from_block: Option<&Block>,
  to_block: Option<&Block>,
) -> VdrResult<EventQuery>

/// Parse the result of execution EthereumExtDidRegistry.changed contract method to receive
///   a block number when DID was changed last time
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Block number
pub fn parse_did_changed_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Block>;

/// Parse the result of execution EthereumExtDidRegistry.owners contract method to receive
///   an account address owning the DID.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Owner account address
pub fn parse_did_owner_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Address>;

/// Parse the result of execution EthereumExtDidRegistry.nonce contract method to receive
///   a signing nonce needed for endorsement
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Nonce to use for endorsing
pub fn parse_did_nonce_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Nonce>;

/// Parse DidAttributeChangedEvent from the event log.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Parsed DidAttributeChanged event object
pub fn parse_did_attribute_changed_event_response(
  client: &LedgerClient,
  log: &EventLog,
) -> VdrResult<DidAttributeChanged>;

/// Parse DidDelegateChangedEvent from the event log.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Parsed DidDelegateChanged event object
pub fn parse_did_delegate_changed_event_response(
  client: &LedgerClient,
  log: &EventLog,
) -> VdrResult<DidDelegateChanged>;

/// Parse DidOwnerChangedEvent from the event log.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Parsed DidOwnerChanged event object
pub fn parse_did_owner_changed_event_response(
  client: &LedgerClient,
  log: &EventLog,
) -> VdrResult<DidOwnerChanged>;

/// Parse DID associated event from the event log (it can be one of: DidAttributeChanged, DidDelegateChanged, DidOwnerChanged).
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Parsed DID event object
pub fn parse_did_event_response(client: &LedgerClient, event: &EventLog) -> VdrResult<DidEvents>;
```

### Resolve DID

```rust
/// Single step function to resolve a DidDocument with metadata for the given DID
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `did` DID to get a DID Document and metadata
/// - `options` Resolution options
///
/// # Returns
///   Parsed DID event object
pub async fn resolve_did(
  client: &LedgerClient,
  did: &DID,
  options: Option<&DidResolutionOptions>,
) -> VdrResult<DidDocumentWithMeta>
```

### Schema

#### Create Schema

```rust
/// Build transaction to execute SchemaRegistry.createSchema contract method to create a new Schema
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `from` transaction sender account address
/// - `schema` Schema object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:schema
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_create_schema_transaction(
    client: LedgerClient,
    from: String,
    schema: Schema,
) -> Transaction;
```

#### Resolve Schema

```rust
/// Prepare transaction executing `SchemaRegistry.resolveSchema` smart contract method 
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: id - id of Schema to resolve
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_resolve_schema_transaction(
    client: LedgerClient,
    id: String,
) -> Transaction;
```

```rust
/// Parse response for of `SchemaRegistry.resolveSchema` smart contract 
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: response: bytes - received response bytes
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_parse_resolve_schema_response(
    client: LedgerClient,
    response: bytes,
) -> SchemaWithMeta;
```

```rust
/// Single step function to resolve schema from the ledger
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: id - id of Schema to resolve
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_resolve_schema(
    client: LedgerClient,
    id: String,
) -> Schema;
```

### Credential Definition

#### Create Credential Definition

```rust
/// Prepare transaction executing CredentialDefinitionRegistry.createCredentialDefinition smart contract method
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: cred_def - Credential Definition object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:credential-definition 
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_create_credential_definition_transaction(
    client: LedgerClient,
    from: String,
    cred_def: CredentialDefinition,
    options: Option<BuildTxnOptions>,
) -> Transaction;
```

#### Resolve Credential DefinitionCredential Definition

```rust
/// Prepare transaction executing CredentialDefinitionRegistry.resolveCredentialDefinition smart contract method
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: id - id of Credential Definition to resolve
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_resolve_credential_definition_transaction(
    client: LedgerClient,
    id: String,
    options: Option<BuildTransactionOptions>,
) -> Transaction;
```

```rust
/// Parse response for of `CredentialDefinitionRegistry.resolveCredentialDefinition` smart contract 
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: response: bytes - received response bytes
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_parse_resolve_credential_definition_response(
    client: LedgerClient,
    response: bytes,
) -> CredentialDefinitionWithMeta;
```

```rust
/// Single step function to resolve credential definition from the ledger
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: id - id of Credential Definition to resolve
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_resolve_credential_definition(
    client: LedgerClient,
    id: String,
) -> CredentialDefinition;
```

### Auth

#### Assign role

```rust
/// Prepare transaction executing RoleControl.assignRole smart contract method
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: to: string - account address to assign role
///  param: role: string - role to assign
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_assign_role_transaction(
    client: LedgerClient,
    from: String,
    to: String,
    role: String,
    options: Option<BuildTxnOptions>,
) -> Transaction;
```

#### Revoke role

```rust
/// Prepare transaction executing RoleControl.revokeRole smart contract method
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: to: string - account address to assign role
///  param: role: string - role to revoke
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_revoke_role_transaction(
    client: LedgerClient,
    from: String,
    to: String,
    role: String,
    options: Option<BuildTxnOptions>,
) -> Transaction;
```

#### Get role

```rust
/// Prepare transaction executing RoleControl.getRole smart contract method
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: account: string - account address to get role
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_get_role_transaction(
    client: LedgerClient,
    account: String,
    options: Option<BuildTransactionOptions>,
) -> Transaction;
```

```rust
/// Parse response for of `RoleControl.getRole` smart contract 
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: response: bytes - received response bytes
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_parse_get_role_response(
    client: LedgerClient,
    response: bytes,
) -> AccountRole;
```

### Validator

#### Add validator

```rust
/// Prepare transaction executing ValidatorControl.addValidator smart contract method
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: validator: string - address of valdiator to add
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_add_validator_transaction(
    client: LedgerClient,
    from: String,
    validator: String,
    options: Option<BuildTxnOptions>,
) -> Transaction;
```

#### Remove validator

```rust
/// Prepare transaction executing ValidatorControl.removeValidator smart contract method
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: validator: string - address of valdiator to remove
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_remove_validator_transaction(
    client: LedgerClient,
    from: String,
    validator: String,
    options: Option<BuildTxnOptions>,
) -> Transaction;
```

#### Get validators

##### Request builder

```rust
/// Prepare transaction executing ValidatorControl.getValdiators smart contract method
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_get_validators_transaction(
    client: LedgerClient,
    options: Option<BuildTransactionOptions>,
) -> Transaction;
```

```rust
/// Parse response for of `ValidatorControl.getValidators` smart contract 
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: response: bytes - received response bytes
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_parse_get_validators_response(
    client: LedgerClient,
    response: bytes,
) -> ValidatorList;

struct ValidatorList {
    validators: Vec<string>
}
```

### DID Ethr

#### Writes

```rust
/// Change owner of ethr DID 
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: did: string - DID to change ownership
///  param: new_owner: string - account addres sof new owner
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
pub async fn build_did_change_owner_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
    new_owner: &Address,
) -> VdrResult<Transaction>;

/// Endorsing version of method to change owner for ethr DID 
pub async fn build_did_change_owner_signed_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
    new_owner: &Address,
    signature: &Signature,
) -> VdrResult<Transaction>;

/// An identity can assign multiple delegates to manage signing on their behalf for specific purposes.
/// Function to add a new delegate for a DID
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: did: string - DID to add delegate
///  param: delegate_type: string - type of delegation
///  param: delegate: string - account address of delegate
///  param: validity: Option<u64> - delegate validity time
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
pub async fn build_did_add_delegate_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
    delegate_type: &DelegateType,
    delegate: &Address,
    validity: Option<u64>,
) -> VdrResult<Transaction>;

/// Endorsing version of method to add a delegate for ethr DID
pub async fn build_did_add_delegate_signed_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
    delegate_type: &DelegateType,
    delegate: &Address,
    validity: Option<u64>,
    signature: &Signature,
) -> VdrResult<Transaction>;

/// An identity can assign multiple delegates to manage signing on their behalf for specific purposes.
/// Function to remove a delegate for a DID
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: did: string - DID to remove delegate
///  param: delegate_type: string - type of delegation
///  param: delegate: number - account address of delegate
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
pub async fn build_did_revoke_delegate_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
    delegate_type: &DelegateType,
    delegate: &Address,
) -> VdrResult<Transaction>;

/// Endorsing version of method to remove a delegate for ethr DID
pub async fn build_did_revoke_delegate_signed_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
    delegate_type: &DelegateType,
    delegate: &Address,
    signature: &Signature,
) -> VdrResult<Transaction>;

/// An identity may need to publish some information that is only needed off-chain but still requires the security benefits of using a blockchain.
/// Function to add an attribute associated with the DID
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: did: string - DID to add attribute
///  param: attribute: DidDocAttribute - attribute to add
///  param: validity: Option<u64> - attribute validity time
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
pub async fn build_did_set_attribute_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
    attribute: &DidDocAttribute,
    validity: Option<Validity>,
) -> VdrResult<Transaction>;

/// Endorsing version of method to add an attribute for ethr DID
pub async fn build_did_set_attribute_signed_transaction(
    client: &LedgerClient,
    from: &Address,
    did: &DID,
    attribute: &DidDocAttribute,
    validity: Option<Validity>,
    signature: &Signature,
) -> VdrResult<Transaction>;

/// An identity may need to publish some information that is only needed off-chain but still requires the security benefits of using a blockchain.
/// Function to remove an attribute associated with the DID
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: did: string - DID to add attribute
///  param: attribute: DidDocAttribute - attribute to add
///  param: validity: Option<u64> - attribute validity time
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
pub async fn build_did_revoke_attribute_transaction(
    client: &LedgerClient,
    _from: &Address,
    did: &DID,
    attribute: &DidDocAttribute,
) -> VdrResult<Transaction>;

/// Endorsing version of method to remove an attribute for ethr DID
pub async fn build_did_revoke_attribute_signed_transaction(
    client: &LedgerClient,
    from: &Address,
    _did: &DID,
    attribute: &DidDocAttribute,
    signature: &Signature,
) -> VdrResult<Transaction>;
```

#### Resolve

```rust
/// Build a transaction to query a block when a DID was changed lst time.
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: did: string - DID to get the chnaged block number
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
pub async fn build_did_changed_transaction(
    client: &LedgerClient,
    did: &DID,
) -> VdrResult<Transaction>;

/// Parse response of `EthrDidRegistry.changed` smart contract 
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: response: bytes - received response
///
/// #Returns
///   block - block number when an identity was changed last time (0 - mean that identity has never been changed)
pub fn parse_did_changed_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Block>;

/// Build an query to retrieve log events raised for the given DID  
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: did: string - DID to query log events
///  param: block: Option<number> - Specific block number to retrieve events 
///
/// #Returns
///   query: EventQuery - prepared event query to submit
pub async fn build_get_did_event_query(
    client: &LedgerClient,
    did: &DID,
    block: Option<&Block>,
) -> VdrResult<EventQuery>;

/// Parse log response of query for DID events
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: log: RawLog - received log events
///
/// #Returns
///   events - parsed events raised for the DID
pub fn parse_did_event_response(client: &LedgerClient, log: &RawLog) -> VdrResult<DIDEvents>;

/// Single step function to resolve DID DDocument for teh given DID
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: did: string - DID to get a DID Document 
///
/// #Returns
///   DidDocument - Received DID DDocument
pub async fn resolve_did(client: &LedgerClient, did: &DID) -> VdrResult<DidDocument>;
```
