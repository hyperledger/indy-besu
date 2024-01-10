# VDR Design

**Disclaimer:** popular packages for working with Ethereum network are very close to `indy-sdk`: their provide tide
modules for the whole flow execution. It may be difficult to reuse only particular functions without full integration.
In the same, time Indy community follows to idea of splitting complex library into components.

> Rust client library to work with Web3, Solidity smart contracts: https://github.com/tomusdrw/rust-web3

## VDR library assumptions

* VDR library firstly will be implemented independently and later integrate into existing indy vdr/sdk/frameworks.
* VDR library will be written in Rust with providing language wrappers.
* VDR library does conversion of legacy `sov/indy` formatted did's and id's into `indy2` format.
* VDR library does conversion of ledger formatted entities (DID Document, Schema, Credential Definition) into
  specifications compatible format.
* VDR library does only basic validation within request builders.
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
  address: String, // address of deployed contract
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
}

trait Contract: Sync + Send {
  /// Get the address of deployed contract
  ///
  /// # Returns
  /// address of the deployed contract. Should be used to execute contract methods
  fn address(&self) -> &Address;

  /// Encode data required for the execution of a contract method
  ///
  /// # Params
  /// - `method` method to execute
  /// - `params` data to pass/encode for contract execution
  ///
  /// # Returns
  /// encoded data to set into transaction
  fn encode_input(&self, method: &str, params: &[ContractParam]) -> VdrResult<Vec<u8>>;

  /// Decode the value (bytes) returned as the result of the execution of a contract method
  ///
  /// # Params
  /// - `method` method to execute
  /// - `output` data to decode (returned as result of sending call transaction)
  ///
  /// # Returns
  /// contract execution result in decoded form
  fn decode_output(&self, method: &str, output: &[u8]) -> VdrResult<ContractOutput>;
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
  nonce: Option<[u64; 4]>,
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

struct SignatureData {
  /// recovery ID using for public key recovery
  pub recovery_id: u64,
  /// ECDSA signature
  pub signature: Vec<u8>,
}
```

## Contracts/Requests methods

### DID Document

#### Create DID

```rust
// Probably we do no even need it
struct BuildTxnOptions {}
```

```rust
/// Prepare transaction executing `IndyDidRegistry.createDid` smart contract method to create a new DID on the Ledger
///
/// #Params
///  param: client: LedgerClient - Ledger client
///  param: from: string - sender account address
///  param: did_document: DidDocument - DID Document matching to the specification: https://www.w3.org/TR/did-core/
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_create_did_transaction(
    client: LedgerClient,
    from: String,
    did_document: DidDoc,
    options: Option<BuildTxnOptions>,
) -> Transaction {
    unimplemented!();
}
```

#### Update DID

```rust
/// Prepare transaction executing `IndyDidRegistry.updateDid` smart contract method to update an existing DID Document
///
/// #Params
///  param: client: LedgerClient - Ledger client
///  param: from: string - sender account address
///  param: did_document: DidDocument - DID Document matching to the specification: https://www.w3.org/TR/did-core/
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_update_did_transaction(
    client: LedgerClient,
    from: String,
    did_document: DidDoc,
    options: Option<BuildTxnOptions>,
) -> Transaction;
```

#### Deactivate DID

```rust
/// Prepare transaction executing `IndyDidRegistry.deactivateDid` smart contract method to deactivate an existing DID
///
/// #Params
///  param: client: LedgerClient - Ledger client
///  param: from: string - sender account address
///  param: did: string - did to deactivate
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_deactivate_did_transaction(
    client: LedgerClient,
    from: String,
    did: String,
    options: Option<BuildTxnOptions>,
) -> Transaction;
```

#### Resolve DID

```rust
/// Prepare transaction executing `IndyDidRegistry.resolveDid` smart contract method to resolve a DID
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: did - DID to resolve
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_resolve_did_transaction(
    client: LedgerClient,
    did: String,
    options: Option<BuildTransactionOptions>,
) -> Transaction;
```

```rust
/// Parse response for of `IndyDidRegistry.resolveDid` smart contract 
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

### Schema

#### Create Schema

```rust
/// Prepare transaction executing SchemaRegistry.createSchema smart contract method
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: from: string - sender account address
///  param: schema - Schema object matching to the specification - https://hyperledger.github.io/anoncreds-spec/#term:schema
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_create_schema_transaction(
    client: LedgerClient,
    from: String,
    schema: Schema,
    options: Option<BuildTxnOptions>,
) -> Transaction;
```

#### Resolve Schema

```rust
/// Prepare transaction executing `SchemaRegistry.resolveSchema` smart contract method 
///
/// #Params
///  param: client: Ledger - client (Ethereum client - for example web3::Http)
///  param: id - id of Schema to resolve
///  param: options: Option<BuildTxnOptions> - (Optional) extra data required for transaction preparation
///
/// #Returns
///   transaction: Transaction - prepared transaction object 
fn indy_vdr_build_resolve_schema_transaction(
    client: LedgerClient,
    id: String,
    options: Option<BuildTransactionOptions>,
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
