## Endorsement

Not all identity owners may have permissions for writing transactions on the ledger.  
We need to define a mechanism of doing transaction writes by an Endorser with preserving original author as an entity
owner.

### DID Indy registry

#### Flow

* Author steps:
    * Step 1: Author prepares a DID Document object
    * Step 2: Author convert DID Document into contracts representation (which will be stored on the ledger) and encodes
      it into bytes using `abi.encodePacked` (available in solidity as well)
    * Step 3: Author performs EcDSA signing using his ethereum identity account keys
    * Step 4: Author passes DID Document and Signature to Endorser
* Endorser steps:
    * Step 1: Endorser builds transaction to endorse
      DID: `endorseDid(address sender, address identity, DidDocument didDocument, bytes32 identitySignature)`
      > Optionally: `identity` can be derived from DidDocument.id instead of passing explicitly
    * Step 2: Endorser does regular EcDSA signing of the **Transaction**
    * Step 3: Endorser submit the signed transaction to the ledger which executes deployed `IndyDidRegistry.endorseDid`
      contract method
* Ethereum:
    * Checks the validity of the transaction level signature (Endorser's signature)
* Contract:
    * Step 1: Encodes DID Document into bytes using `abi.encodePacked`
    * Step 2: Checks the validity of the provided signature against identity passed as the parameter `ecrecover(...);`
        * `ecrecover` returns an account signed the message

#### Contracts

```
// identity - ethereum address of DID owner
// document - did document
// identitySignature - identity owner signatures (EcDSA and optionally ED25519) ower serialized DID Document
function endorseDid(address identity, DidDocument didDocument, bytes32 identitySignature) {
    // sender is endorser when it's not equal to identity
    if (msg.sender == identity) {
        revert InvalidmethodExecution;
    }
    
    // calculate the hash of DiDocument 
    // this hash will be checked agains signatures to verify ownership 
    bytes32 hash = abi.encodePacked(didDocument);
    
    // verify EcDSA identity owner signature ower DID + DidDocument
    checkEcDsaSignature(identity, hash, identitySignature);
    
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
// Encode DID Document which need to be signed by an identity owner 
fn indy_vdr_encode_did_document(did_doc: DidDocument) -> Vec<u8>;

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

```
function setAttributeSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes32 name, bytes memory value, uint validity)

function revokeAttributeSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes32 name, bytes memory value)

function addDelegateSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes32 delegateType, address delegate, uint validity)

function revokeDelegateSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes32 delegateType, address delegate)

function changeOwnerSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, address newOwner)
```

#### Contracts

Should we extend DidEthrRegistry contract to add roles check? 

> We already extended `DidEthrRegistry` to use UpgradeControl

#### VDR

TO BE defined later.

### CL Registry (Schema / Credential Definition)

#### Flow

**Schema endorsing**

* Author steps:
    * Step 1: Author prepares a valid Schema (schema contains `issuerID` referencing to DID entry)
    * Step 2: Author convert Schema into contracts representation (which will be stored on the ledger) and encodes it
      into bytes using `abi.encodePacked` (available in solidity as well)
        * Step 3: Author performs EcDSA signing using his ethereum identity account keys
        * Step 4: Author passes Schema and Signature to Endorser
* Endorser steps:
    * Step 1: Endorser builds transaction to endorse
      DID: `endorseSchema(address sender, Schema schema, bytes32 identitySignature)`
    * Step 2: Endorser does regular EcDSA signing of the **Transaction**
    * Step 3: Endorser submit the signed transaction to the ledger which executes
      deployed `SchemaRegistry.endorseSchema`
      contract method
* Ethereum:
    * Checks the validity of the transaction level signature (Endorser's signature)
* Contract:
    * Step 1: Encodes DID Document into bytes using `abi.encodePacked`
    * Step 2: Resolve identity owner for the schema `issuerId`
    * Step 3: Checks the validity of the provided signature against identity passed as the parameter `ecrecover(...);`

Credential Definition endorsing process is the same as for Schema.

#### Contracts

```
// schema - CL schema
// identitySignature - signature ower serialized Schema of the schema issuer identity owner
function endorseSchema(Schema schema, EcDsaSignature identitySignature) {
    bytes32 hash = abi.encodePacked(schema);

    // resolver owner of issuerDID
    DidMetadata issuerDidMeta = didResolver.resolveMetadata(schema.issuerId)
    if (msg.sender == identity) {
        revert InvalidmethodExecution;
    }
    
    checkEcDsaSignature(issuerDidMeta.owner, hash, identitySignature);

    _schemas[schema.id].schema = schema;
}

// credDef - CL credential definition
// identitySignature - signature ower serialized credDef of the cred def issuer identity owner
function endorseCredentialDefinition(CredentialDefinition credDef, EcDsaSignature identitySignature) {
    bytes32 hash = abi.encodePacked(credDef);

    // resolver owner of issuerDID
    DidMetadata issuerDidMeta = didResolver.resolveMetadata(credDef.issuerId)
    if (msg.sender == identity) {
        revert InvalidmethodExecution;
    }
    
    checkEcDsaSignature(issuerDidMeta.owner, hash, identitySignature);
    
    _credDefs[credDef.id].credDef = credDef;
}
```

#### VDR

```rust
// Encode Schema which need to be signed by an identity owner 
fn indy_vdr_encode_schema(schema: Schema) -> Vec<u8>;

// Build transaction to endorse Schema
fn build_endorse_schema_transaction(
    client: &LedgerClient,
    sender: &Address,
    schema: &Schema,
    signature: &Signature
) -> VdrResult<Transaction> {}

// Encode CredentialDefinition which need to be signed by an identity owner 
fn indy_vdr_encode_credential_definition(cred_def: CredentialDefinition) -> Vec<u8>;

// Build transaction to endorse CredentialDefinition
fn build_endorse_credential_definition_transaction(
    client: &LedgerClient,
    sender: &Address,
    cred_def: &CredentialDefinition,
    signature: &Signature
) -> VdrResult<Transaction> {}
```
