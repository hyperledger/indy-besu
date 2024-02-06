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
    /// Create indy2 client interacting with ledger
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
) -> VdrResult<Transaction>;

/// Prepared data for execution of SchemaRegistry.createSchema contract method to endorse a new Schema
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `id` id of schema to be created
///  - `schema` Schema object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:schema
///
/// #Returns
///   data: TransactionEndorsingData - transaction endorsement data to sign
pub async fn build_create_schema_endorsing_data(
  client: &LedgerClient,
  id: &SchemaId,
  schema: &Schema,
) -> VdrResult<TransactionEndorsingData>;

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
pub async fn build_create_schema_signed_transaction(
  client: &LedgerClient,
  sender: &Address,
  id: &SchemaId,
  schema: &Schema,
  signature: &SignatureData,
) -> VdrResult<Transaction>
```

#### Resolve Schema

```rust
/// Build transaction to execute SchemaRegistry.schemasCreated contract method to get
///   block number when Schema was created
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `id` identifier of target schema
///
/// #Returns
///   transaction: Transaction - prepared read transaction object to submit
pub async fn build_get_schema_created_transaction(
  client: &LedgerClient,
  id: &SchemaId,
) -> VdrResult<Transaction>;

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
#[logfn(Ifo)]
pub async fn build_get_schema_query(
  client: &LedgerClient,
  id: &SchemaId,
  from_block: Option<&Block>,
  to_block: Option<&Block>,
) -> VdrResult<EventQuery>
```

```rust
/// Parse the result of execution SchemaRegistry.schemas contract method to receive
///   block number when a schema was created
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Block when the schema was created
pub fn parse_schema_created_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Block>;

/// Parse SchemaRegistry.SchemaCreated from the event log.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Parsed Schema object
pub fn parse_schema_created_event(client: &LedgerClient, log: &EventLog) -> VdrResult<SchemaCreatedEvent>
```

```rust
/// Single step function to resolve a Schema for the given ID
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `id` id of schema to resolve
///
/// # Returns
///   Resolved Schema object
pub async fn resolve_schema(client: &LedgerClient, id: &SchemaId) -> VdrResult<Schema>;
```

### Credential Definition

#### Create Credential Definition

```rust
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
) -> VdrResult<Transaction>;

/// Prepared data for endorsing CredentialDefinitionRegistry.createCredentialDefinition contract method
///
/// #Params
///  - `client` client connected to the network where contract will be executed
/// - `id` id of credential definition to be created
/// - `credential_definition` Credential Definition object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:credential-definition
///
/// #Returns
///   data: TransactionEndorsingData - transaction endorsement data to sign
pub async fn build_create_credential_definition_endorsing_data(
  client: &LedgerClient,
  id: &CredentialDefinitionId,
  credential_definition: &CredentialDefinition,
) -> VdrResult<TransactionEndorsingData>;

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
pub async fn build_create_credential_definition_signed_transaction(
  client: &LedgerClient,
  from: &Address,
  id: &CredentialDefinitionId,
  credential_definition: &CredentialDefinition,
  signature: &SignatureData,
) -> VdrResult<Transaction>
```

#### Resolve Credential DefinitionCredential Definition

```rust
/// Build transaction to execute CredentialDefinitionRegistry.credDefs contract method to get
///   block number when a Credential Definition was created
///
/// #Params
///  - `client` client connected to the network where contract will be executed
///  - `id` identifier of target credential definition
///
/// #Returns
///   transaction: Transaction - prepared read transaction object to submit
pub async fn build_get_credential_definition_created_transaction(
  client: &LedgerClient,
  id: &CredentialDefinitionId,
) -> VdrResult<Transaction>;

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
pub async fn build_get_credential_definition_query(
  client: &LedgerClient,
  id: &CredentialDefinitionId,
  from_block: Option<&Block>,
  to_block: Option<&Block>,
) -> VdrResult<EventQuery>
```

```rust
/// Parse the result of execution CredentialDefinitionRegistry.credDefs contract method to receive
///   block number when a credential definition was created
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Block when the credential definition was created
pub fn parse_credential_definition_created_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Block>;

/// Parse CredentialDefinitionRegistry.CredentialDefinitionCreated from the event log.
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
///   Parsed Credential Definition event object
pub fn parse_credential_definition_created_event(client: &LedgerClient, log: &EventLog) -> VdrResult<CredentialDefinitionCreatedEvent>
```

```rust
/// Single step function to resolve a Credential Definition for the given ID
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `id` id of schema to resolve
///
/// # Returns
///   Resolved Credential Definition object
pub async fn resolve_credential_definition(client: &LedgerClient, id: &CredentialDefinitionId) -> VdrResult<CredentialDefinition>;
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
