# CL Registry

## Design

Smart contracts for schema and credential definition registries are designed to follow an approach used for DID:ethr
method:

* Contract Events are used for storing data
* Contracts hold mapping for more efficient events look up
* Gas efficient data types are used in contract method
* Provided methods for delegating signing
    * Nonce is not needed because only `create` methods provided  (no `update` method)

## Schema

### ID Syntax

#### AnonCreds Spec style

| parameter   | value                                                           |
|-------------|-----------------------------------------------------------------|
| id          | “did:” method-name “:” namespace “:” method-specific-id         |
| method-name | “ethr”                                                          |
| namespace   | “testnet”/"mainnet"                                             |
| indy-id     | <issuer_did>/anoncreds/v0/SCHEMA/<schema_name>/<schema_version> |

```
Example: did:ethr:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu/anoncreds/v0/SCHEMA/BasicIdentity/1.0.0
```

### Storage format

* Created schemas mapping:
    * Description: Mapping to track created schemas by their id to the block number when it was created.
    * Format:
        ```
        mapping(bytes32 id => uint block);
        ```
    * Example:
      ```
      {
          "0x8ae64c08cf45da3364623a7235a9e7d132fdc8e9f6e63858b53a90d9db32c3af": 110,
          ...
      }
      ```

### Transactions (Smart Contract's methods)

Contract name: **SchemaRegistry**

#### Create a new schema

* Method: `createSchema`
    * Description: Transaction to create a new AnonCreds Schema matching to the [specification](https://hyperledger.github.io/anoncreds-spec/#schema-publisher-publish-schema-object)
    * Parameters:
        * `identity` - Account address of schema issuer
        * `id` - KECCAK256 hash of schema id to be created
        * `schema` - AnonCreds Schema object as bytes 
    * Restrictions:
        * Schema id must be unique.
        * Corresponding issuer account must exist and owned by sender.
    * Format:
        ```
        SchemaRegistry.createSchema(
            address identity,
            bytes32 id,
            bytes schema
        )
        ```
    * Raised Event:
       ```
       SchemaCreated(bytes32 indexed id, address identity, bytes schema)`
       ```
    * Example:
        ```
        SchemaRegistry.createSchema(
            "0x173CC02518a355040F5Faee93D3AAAb1259F010c",
            "0x8ae64c08cf45da3364623a7235a9e7d132fdc8e9f6e63858b53a90d9db32c3af",
            [34, 123,  92,  34, 105, 100,  92,  34,  58,  92,  34, 100, ...]
        )

* Method: `createSchemaSigned`
    * Description: Transaction to endorse a new AnonCreds Schema (off-chain author signature)
    * Parameters:
        * `identity` - Account address of schema issuer
        * `sigV` - Part of EcDSA signature.
        * `sigR` - Part of EcDSA signature.
        * `sigS` - Part of EcDSA signature.
        * `id` - KECCAK256 hash of schema id to be created
        * `schema` - AnonCreds schema object as bytes
    * Restrictions:
        * Schema id must be unique.
        * Corresponding issuer account must exist and owned by sender.
    * Format:
        ```
        SchemaRegistry.createSchemaSigned(
            address identity,
            uint8 sigV,
            bytes32 sigR,
            bytes32 sigS,
            bytes32 id,
            bytes schema
        )
        ```
    * Raised Event:
       ```
       SchemaCreated(bytes32 indexed id, address identity, bytes schema)`
       ```
    * Example:
        ```
        SchemaRegistry.createSchemaSigned(
            "0x173CC02518a355040F5Faee93D3AAAb1259F010c",
            27,
            [1, 2, 3, 4, 5, 6, 7, 8, ...],
            [11, 21, 33, 44, 55, 73, ...],
            "0x8ae64c08cf45da3364623a7235a9e7d132fdc8e9f6e63858b53a90d9db32c3af",
            [34, 123,  92,  34, 105, 100,  92,  34,  58,  92,  34, 100, ...]
        )

#### Resolve schema

In order to resolve a Schema the following steps must be done:

* Call `SchemaRegistry.created(bytes32 id)` contract method passing KECCAK256 hash of target Schema id to get the block number when the Schema was created.
    * Schemas are stored in the transaction logs.
    * Query log events from the whole transaction history is very inefficient lookup mechanism.
* Query ledger for `SchemaCreated` events specifying following data:
  * `address`: Address of `SchemaRegistry
  * `topics`: KECCAK256 hash of target Schema id as the second topic
  * `from_block`: block number when the Schema was created
  * `to_block`: block number when the Schema was created
* If result is empty, schema does not exist.
* If result contains more than one event, its unexpected case and ledger history is broken 

## Credential Definition

### ID Syntax

#### AnonCreds Spec style

| parameter   | value                                                   |
|-------------|---------------------------------------------------------|
| id          | “did:” method-name “:” namespace “:” method-specific-id |
| method-name | “indy2”, “indy”, “sov”, “ethr”                          |
| namespace   | “testnet”/"mainnet"                                     |
| indy-id     | <issuer_did>/anoncreds/v0/CLAIM_DEF/<schema_id>/<name>  |

```
Example: did:indy2:sovrin:Gs6cQcvrtWoZKsbBhD3dQJ/anoncreds/v0/CLAIM_DEF/56495/mctc
```

### Storage format

* Created credential definitions mapping:
    * Description: Mapping to track created credential definitions by their id to the block number when it was created.
    * Format:
        ```
        mapping(bytes32 id => uint block);
        ```
    * Example:
      ```
      {
          "0x8ae64c08cf45da3364623a7235a9e7d132fdc8e9f6e63858b53a90d9db32c3af": 110,
          ...
      }
      ```

### Transactions (Smart Contract's methods)

Contract name: **CredentialDefinitionRegistry**

#### Create a new credential definition

* Method: `createCredentialDefinition`
    * Description: Transaction to create a new AnonCreds Credential Definition matching to the [specification](https://hyperledger.github.io/anoncreds-spec/#generating-a-credential-definition-without-revocation-support)
    * Parameters:
        * `identity` - Account address of credential definition issuer
        * `id` - KECCAK256 hash of credential definition id to be created
        * `schemaId` - KECCAK256 hash of schema id to be created
        * `credDef` - AnonCreds Credential Definition object as bytes
    * Restrictions:
        * Credential Definition must be unique.
        * Corresponding issuer DID must exist and owned by sender.
        * Corresponding schema must exist.
    * Format:
        ```
        CredentialDefinitionRegistry.createCredentialDefinition(
            address identity,
            bytes32 id,
            bytes32 schemaId,
            bytes credDef
        )
        ```
    * Raised Event:
       ```
       CredentialDefinitionCreated(bytes32 indexed id, address identity, bytes credDef)`
       ```
    * Example:
        ```
        CredentialDefinitionCreated.createCredentialDefinition(
            "0x173CC02518a355040F5Faee93D3AAAb1259F010c",
            "0x76943530d3587e81f029e8ce20edb64f9254350d81c59ecf6b7e3ed553e9a8f6",
            "0x8ae64c08cf45da3364623a7235a9e7d132fdc8e9f6e63858b53a90d9db32c3af",
            [34, 123,  92,  34, 105, 100,  92,  34,  58,  92,  34, 100, ...]
        )

* Method: `createCredentialDefinitionSigned`
    * Description: Transaction to endorse a new AnonCreds Credential Definition (off-chain author signature)
    * Parameters:
        * `identity` - Account address of credential definition issuer
        * `sigV` - Part of EcDSA signature.
        * `sigR` - Part of EcDSA signature.
        * `sigS` - Part of EcDSA signature.
        * `id` - KECCAK256 hash of credential definition id to be created
        * `schemaId` - KECCAK256 hash of schema id
        * `credDef` - AnonCreds credential definition object as bytes
    * Restrictions:
        * Credential Definition must be unique.
        * Corresponding issuer DID must exist and owned by sender.
        * Corresponding schema must exist.
    * Format:
        ```
        CredentialDefinitionRegistry.createCredentialDefinitionSigned(
            address identity,
            uint8 sigV,
            bytes32 sigR,
            bytes32 sigS,
            bytes32 id,
            bytes32 schemaId,
            bytes credDef
        )
        ```
    * Raised Event:
       ```
       CredentialDefinitionCreated(bytes32 indexed id, address identity, bytes credDef)`
       ```
    * Example:
        ```
        CredentialDefinitionRegistry.createCredentialDefinitionSigned(
            "0x173CC02518a355040F5Faee93D3AAAb1259F010c",
            27,
            [1, 2, 3, 4, 5, 6, 7, 8, ...],
            [11, 21, 33, 44, 55, 73, ...],
            "0x76943530d3587e81f029e8ce20edb64f9254350d81c59ecf6b7e3ed553e9a8f6",
            "0x8ae64c08cf45da3364623a7235a9e7d132fdc8e9f6e63858b53a90d9db32c3af",
            [34, 123,  92,  34, 105, 100,  92,  34,  58,  92,  34, 100, ...]
        )

#### Resolve credential definition

In order to resolve a Credential Definition the following steps must be done:

* Call `CredentialDefinitionRegistry.created(bytes32 id)` contract method passing KECCAK256 hash of target Credential Definition id to get the block number when the Credential Definition was created.
    * Credential Definitions are stored in the transaction logs.
    * Query log events from the whole transaction history is very inefficient lookup mechanism.
* Query ledger for `CredentialDefinitionCreated` events specifying following data:
    * `address`: Address of `CredentialDefinitionRegistry`
    * `topics`: KECCAK256 hash of target Credential Definition id as the second topic
    * `from_block`: block number when the Credential Definition was created
    * `to_block`: block number when the Credential Definition was created
* If result is empty, Credential Definition does not exist.
* If result contains more than one event, its unexpected case and ledger history is broken


