## Endorsement

Not all identity owners may have permissions for writing transactions on the ledger.  
The goal of this document to define a mechanism of doing transaction writes to the ledger by a special parties having an
Endorser role with preserving of original author as an entity owner.

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

Endorsing for schemas and credential definition is designed to match existing `did:ethr` API.

> In case of Schema and Credential Definition we do not need to add `nonce` as we do not have an update operation.  

* Author steps:
    * Step 1: Author prepares a Schema object
    * Step 2: Execute VDR method to calculate hash need to be signed - contract signed data according
      to [EIP](https://eips.ethereum.org/EIPS/eip-191).
        ```
        keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "createSchema", id, schema))
        // Arguments when calculating hash to validate
        // 1: byte(0x19) - the initial 0x19 byte
        // 2: byte(0) - the version byte
        // 3: address(this) - the validator address
        // 4-7: Application specific data
        //  identity - author account address
        //  `createSchema` original contract method - added to be aligned with did:ethr contract
        //  id - id of schema to be created
        //  schema - schema as JSON string
        ```
    * Step 3: Performs EcDSA signing using his ethereum identity account keys
    * Step 4: Author passes Schema and Signature to Endorser
* Endorser steps:
    * Step 1: Endorser builds transaction to endorse
      DID: `endorseSchema(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes32 id, bytes schema)`
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
        keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "createCredentialDefinition", id, schemaId, credDef))
        // Arguments when calculating hash to validate
        // 1: byte(0x19) - the initial 0x19 byte
        // 2: byte(0) - the version byte
        // 3: address(this) - the validator address
        // 4-8: Application specific data
        //  identity - author account address
        //  `createSchema` original contract method - added to be aligned with did:ethr contract
        //  id - id of schema to be created
        //  schemaId - id of schema
        //  credDef - credential definition as JSON string
        ```
    * Step 3: Performs EcDSA signing using his ethereum identity account keys
    * Step 4: Author passes Credential Definition and Signature to Endorser
* Endorser/Ethereum/Contract steps are similar to the schema steps.

#### Contracts

```
function createSchemaSigned(
    address identity,
    uint8 sigV,
    bytes32 sigR,
    bytes32 sigS
    bytes32 id,
    bytes schema,

) public virtual {
    // validate identity signature
    bytes32 hash = keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "createSchema", id, schema));
    checkSignature(identity, hash, sigV, sigR, sigS);

    // store schema
    createSchema(identity, id, schema);
}

function endorseCredentialDefinition(
    address identity,
    uint8 sigV,
    bytes32 sigR,
    bytes32 sigS,
    byets32 id,
    byets32 schemaId,
    byets credDef
) public virtual {
    // validate identity signature
    bytes32 hash = keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "createCredentialDefinition", id, schemaId, credDef));
    checkSignature(identity, hash, sigV, sigR, sigS);

    // store credential definition
    createCredentialDefinition_(identity, id, schemaId, credDef);
}
```

#### VDR

```rust
// Prepare schema endorsing bytes which need to be signed by an identity owner 
pub async fn build_create_schema_endorsing_data(
    client: &LedgerClient,
    schema: &Schema,
) -> VdrResult<TransactionEndorsingData>;

// Build transaction to endorse Schema
pub async fn build_create_schema_signed_transaction(
    client: &LedgerClient,
    sender: &Address,
    schema: &Schema,
    signature: &SignatureData,
) -> VdrResult<Transaction>;

// Prepare credential definition endorsing bytes which need to be signed by an identity owner 
pub async fn build_create_credential_definition_endorsing_data(
    client: &LedgerClient,
    id: &CredentialDefinitionId,
    credential_definition: &CredentialDefinition,
) -> VdrResult<TransactionEndorsingData>;

// Build transaction to endorse CredentialDefinition
pub async fn build_create_credential_definition_signed_transaction(
    client: &LedgerClient,
    from: &Address,
    id: &CredentialDefinitionId,
    credential_definition: &CredentialDefinition,
    signature: &SignatureData,
) -> VdrResult<Transaction>;
```

