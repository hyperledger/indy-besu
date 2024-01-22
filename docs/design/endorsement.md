## Endorsement

Not all identity owners may have permissions for writing transactions on the ledger.  
The goal of this document to define a mechanism of doing transaction writes to the ledger by a special parties having an
Endorser role with preserving of original author as an entity owner.

### DID Indy registry

#### Flow

* Author steps:
    * Step 1: Prepares a DID Document object
    * Step 2: Queries `nonce` from the ledger: `IndyDidRegistry.nonce(identity)`
    * Step 3: Execute VDR method to calculate hash need to be signed - contract signed data according
      to [EIP](https://eips.ethereum.org/EIPS/eip-191).
        ```
        keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), nonce[identity], identity, "createDid", did, document))
        // Arguments when calculating hash to validate
        // 1: byte(0x19) - the initial 0x19 byte
        // 2: byte(0) - the version byte
        // 3: address(this) - the validator address
        // 4-8: Application specific data
        //  nonce - nonce to prevent reply attack
        //  identity - author account address
        //  `createDid` original contract method - added to be aligned with did:ethr contract
        //  did - DID to be created
        //  document - DID document as JSON string
        ```
    * Step 4: Performs EcDSA signing using his ethereum identity account keys
    * Step 5: Passes DID Document and Signature to Endorser
* Endorser steps:
    * Step 1: Endorser builds transaction to endorse
      DID: `endorseDid(address identity, string did, string document, uint8 sigV, bytes32 sigR, bytes32 sigS)`
      > Optionally: `identity` can be derived from DidDocument.id instead of passing explicitly
    * Step 2: Endorser does regular EcDSA signing of the **Transaction**
    * Step 3: Endorser submit the signed transaction to the ledger which executes deployed `IndyDidRegistry.endorseDid`
      contract method
* Ethereum:
    * Checks the validity of the transaction level signature (Endorser's signature)
* Contract:
    * Step 1: Get current nonce value of identity
    * Step 2: Calculate the hash of signed data: same as for Author Step 3
    * Step 3: Checks the validity of the provided signature against identity passed as the parameter `ecrecover(...);`
        * `ecrecover` returns an account signed the message

#### Contracts

```
mapping(address => uint) public nonce;

// identity - ethereum address of DID owner
// document - did document
// identitySignature - identity owner signatures (EcDSA and optionally ED25519) ower serialized DID Document
function endorseDid(address identity, string calldata did, string calldata document, uint8 sigV, bytes32 sigR, bytes32 sigS) {
    // sender is endorser when it's not equal to identity
    if (msg.sender == identity) {
        revert InvalidmethodExecution;
    }
    
    // calculate the hash of DiDocument 
    // this hash will be checked agains signatures to verify ownership 
    bytes32 hash = keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), nonce[identity], identity, "createDid", did, document));

    // verify EcDSA identity owner signature ower DID + DidDocument
    checkEcDsaSignature(identity, hash, identitySignature);
    
    nonce[identity]++;
    record[didDocument.did].didDocument = didDocument      
    record[didDocument.did].metadata.owner = identity      
    record[didDocument.did].metadata.sender = msg.sender      
}

function checkEcDsaSignature(address identity, bytes32 hash, EcDSASignature signature) {
    address signer = ecrecover(hash, signature.v, signature.r, signature.s);
    if (signer == address(0)) {
        revert InvalidSignature("Invalid signature provided");
    }
    if (identity != signer) {
        revert InvalidSignature("Signature does not match to the target identity");
    }
}
```

#### VDR

```rust
// Prepare endorsing bytes which need to be signed by an identity owner 
fn prepare_endorse_did_data(
    client: &LedgerClient,
    identity: &Address, 
    did_doc: DidDocument
) -> Vec<u8>;

// Build transaction to endorse DID
fn build_endorse_did_transaction(
    client: &LedgerClient,
    sender: &Address,
    identity: &Address,
    did_doc: &DidDocument,
    signature: &Signature
) -> VdrResult<Transaction> {}
```

### DID Ethr registry

`did:ethr` allows using Ethereum addresses as identifier without prior its registration on the network.

So that DID assume to be written by default -> endorsement is not needed.

Endorsement is needed to modify DID properties, what can be done using the set of existing contract methods:

#### Contracts

```
function setAttributeSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes32 name, bytes memory value, uint validity)

function revokeAttributeSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes32 name, bytes memory value)

function addDelegateSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes32 delegateType, address delegate, uint validity)

function revokeDelegateSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes32 delegateType, address delegate)

function changeOwnerSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, address newOwner)
```

#### VDR

TO BE defined later.

### CL Registry (Schema / Credential Definition)

#### Flow

**Schema endorsing steps**

> In case of Schema and Credential Definition we do not need to add `nonce` as we do not have an update operation.  

* Author steps:
    * Step 1: Author prepares a Schema object
    * Step 2: Execute VDR method to calculate hash need to be signed - contract signed data according
      to [EIP](https://eips.ethereum.org/EIPS/eip-191).
        ```
        keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "createSchema", id, issuerId, schema))
        // Arguments when calculating hash to validate
        // 1: byte(0x19) - the initial 0x19 byte
        // 2: byte(0) - the version byte
        // 3: address(this) - the validator address
        // 4-8: Application specific data
        //  identity - author account address
        //  `createSchema` original contract method - added to be aligned with did:ethr contract
        //  id - id of schema to be created
        //  issuerId - id of schema issuer
        //  schema - schema as JSON string
        ```
    * Step 3: Performs EcDSA signing using his ethereum identity account keys
    * Step 4: Author passes Schema and Signature to Endorser
* Endorser steps:
    * Step 1: Endorser builds transaction to endorse
      DID: `endorseSchema(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, string id, string issuerId, string schema)`
    * Step 2: Endorser does regular EcDSA signing of the **Transaction**
    * Step 3: Endorser submit the signed transaction to the ledger which executes
      deployed `SchemaRegistry.endorseSchema`
      contract method
* Ethereum:
    * Checks the validity of the transaction level signature (Endorser's signature)
* Contract:
    * Step 1: Calculate the hash of signed data: same as for Author Step 3
    * Step 2: Checks the validity of the provided signature against identity passed as the parameter `ecrecover(...);`
        * `ecrecover` returns an account signed the message
    * Step 3: Resolve and check identity owner for the schema `issuerId`

**Credential Definition endorsing steps**:

> Credential Definition endorsing process is the same as for Schema.

* Author steps:
    * Step 1: Author prepares a Credential Definition object
    * Step 2: Execute VDR method to calculate hash need to be signed - contract signed data according
      to [EIP](https://eips.ethereum.org/EIPS/eip-191).
        ```
        keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "createCredentialDefinition", id, issuerId, schemaId, credDef))
        // Arguments when calculating hash to validate
        // 1: byte(0x19) - the initial 0x19 byte
        // 2: byte(0) - the version byte
        // 3: address(this) - the validator address
        // 4-9: Application specific data
        //  identity - author account address
        //  `createSchema` original contract method - added to be aligned with did:ethr contract
        //  id - id of schema to be created
        //  issuerId - id of issuer
        //  schemaId - id of schema
        //  credDef - credential definition as JSON string
        ```
    * Step 3: Performs EcDSA signing using his ethereum identity account keys
    * Step 4: Author passes Credential Definition and Signature to Endorser
* Endorser/Ethereum/Contract steps are similar to the schema steps.

#### Contracts

```
function endorseSchema(
    address identity,
    string calldata id,
    string calldata issuerId,
    string calldata schema,
    uint8 sigV,
    bytes32 sigR,
    bytes32 sigS
) public virtual {
    // validate identity signature
    bytes32 hash = keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "createSchema", id, issuerId, schema));
    checkSignature(identity, hash, sigV, sigR, sigS);

    // store schema
    createSchema(identity, id, issuerId, schema);
}

function endorseCredentialDefinition(
    address identity,
    string memory id,
    string calldata issuerId,
    string calldata schemaId,
    string memory credDef,
    uint8 sigV,
    bytes32 sigR,
    bytes32 sigS
) public virtual {
    // validate identity signature
    bytes32 hash = keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "createCredentialDefinition", id, issuerId, schemaId, credDef));
    checkSignature(identity, hash, sigV, sigR, sigS);

    // store credential definition
    createCredentialDefinition_(identity, id, issuerId, schemaId, credDef);
}
```

#### VDR

```rust
// Prepare schema endorsing bytes which need to be signed by an identity owner 
fn prepare_endorse_schema_data(
    client: &LedgerClient,
    identity: &Address,
    id: &SchemaId,
    issuer_id: &DID,
    schema: &Schema,
) -> Vec<u8>;

// Build transaction to endorse Schema
fn build_endorse_schema_transaction(
    client: &LedgerClient,
    sender: &Address,
    identity: &Address,
    id: &SchemaId,
    issuer_id: &DID,
    schema: &Schema,
    signature: &Signature
) -> VdrResult<Transaction> {}

// Prepare credential definition endorsing bytes which need to be signed by an identity owner 
fn prepare_endorse_credential_definition_data(
    client: &LedgerClient,
    identity: &Address,
    id: &SchemaId,
    issuer_id: &DID,
    schema_id: &SchemaId,
    cred_def: &CredentialDefinition,
) -> Vec<u8>;

// Build transaction to endorse CredentialDefinition
fn build_endorse_credential_definition_transaction(
    client: &LedgerClient,
    sender: &Address,
    identity: &Address,
    id: &SchemaId,
    issuer_id: &DID,
    schema_id: &SchemaId,
    cred_def: &CredentialDefinition,
    signature: &Signature
) -> VdrResult<Transaction> {}
```

