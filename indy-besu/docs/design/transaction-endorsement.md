## Endorsement

#### Case

Create identity and associated DID/DidDocument/Schema/CredDef but submit it to the ledger from a different account
having required permissions.

#### DID Indy registry

##### Flow

* DID creation using endorsement flow:
    * Contract method:
      ```
      // identity - ethereum address of DID owner (author)
      // document - did document
      // identitySignature - identity owner EcDSA signature ower DID Document
      function endorseDid(address identity, DidDocument didDocument, bytes32 identitySignature)
      ```
    * Author steps:
        * Step 1: Author builds a valid DID Document
        * Step 2: Author encodes DID Document
        * Step 3: Author does EcDSA signing using account keys
        * Step 4: Author passes DID Document and Author Signature to Endorser
    * Endorser steps:
        * Step 1: Endorser builds transaction to `ENDORSE_DID` passing Author identity, DidDoc, and signature
        * Step 2: Endorser does EcDSA signing of the **Transaction** using account keys
        * Step 3: Endorser submit the signed transaction execution `IndyDidRegistry.endorseDid` contract method
    * Ethereum:
        * Checks the validity of the transaction level signature (Endorser's signature)
    * Contract:
        * Step 1: Encodes DID Document
        * Step 2: Checks the validity of the provided signature against identity passed as the parameter
* Schema and Credential Definition endorsement flows consist of the same steps with signing of the associated Schema and
  Credential Definition objects

##### Contracts

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

##### VDR

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

#### DID Ethr registry

`did:ethr` allows using Ethereum addresses as identifier without prior its registration on the network.

* DID creation using endorsement flow: **NOT NEEDED** as DID is written by default

#### CL Registry (Schema / Credential Definition)

##### Flow

* Schema endorsing:
    * Contract method:
      ```
      // identity - ethereum address of DID owner (author)
      // document - did document
      // identitySignature - identity owner EcDSA signature ower DID Document
      function endorseSchema(Schema schema, EcDsaSignature identitySignature)
      ```
    * Author steps:
        * Step 1: Author builds a valid Schema (issuerID has `did:ethr` method)
        * Step 2: Author encodes Schema
        * Step 3: Author does EcDSA signing using account keys
        * Step 4: Author passes Schema and Author Signature to Endorser
    * Endorser steps:
        * Step 1: Endorser builds transaction to `ENDORSE_SCHEMA` passing Schema and signature
        * Step 2: Endorser does EcDSA signing of the **Transaction** using account keys
        * Step 3: Endorser submit the signed transaction execution `SchemaRegistry.endorseSchema` contract method
    * Ethereum:
        * Checks the validity of the transaction level signature (Endorser's signature)
    * Contract:
        * Step 1: Encodes Schema
        * Step 2: Resolve identity owner for the schema `issuerId`
        * Step 3: Checks the validity of the provided signature against resolved identity
* Credential Definition endorsing process is the same as for Schema

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

##### VDR

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
