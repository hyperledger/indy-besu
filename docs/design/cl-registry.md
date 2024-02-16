# AnonCreds Registry

## Schema

### ID Syntax

#### AnonCreds Spec style

| parameter   | value                                                           |
|-------------|-----------------------------------------------------------------|
| id          | “did:” method-name “:” namespace “:” method-specific-id         |
| method-name | “indybesu”, “ethr”                                              |
| namespace   | “testnet”/"mainnet"                                             |
| indy-id     | <issuer_did>/anoncreds/v0/SCHEMA/<schema_name>/<schema_version> |

```
Example: did:indybesu:mainnet:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266/anoncreds/v0/SCHEMA/BasicIdentity/1.0.0
```

### Storage format

* Schemas collection:
    * Description: Mapping holding the list of Schema ID's to their data and metadata.
    * Format:
        ```
        mapping(bytes32 id => SchemaRecord schemaRecord);
  
        struct SchemaRecord {
            bytes data;
            SchemaMetadata metadata;
        }

        struct SchemaMetadata {
            uint256 created;
        }
        ```
    * Example:
      ```
      {
          "0x8ae64c08cf45da3364623a7235a9e7d132fdc8e9f6e63858b53a90d9db32c3af": {
              schema: [1,2,3,4,5,6,7,8,9,....], 
              metadata: {
                  created: 1234
              }, 
          },
          ...
      }
      ```

#### Types definition

##### Schema

Schema must match to
the [specification](https://hyperledger.github.io/anoncreds-spec/#schema-publisher-publish-schema-object).

##### Schema Metadata

* `created` - timestamp of schema creation.

### Transactions (Smart Contract's methods)

Contract name: **SchemaRegistry**

#### Create a new schema

* Method: `createSchema`
    * Description: Transaction to create a new AnonCreds Schema
    * Parameters:
        * `identity` - Account address of schema issuer
        * `id` - Keccak hash of Schema id to be created
        * `issuerId` - DID of Schema issuer
        * `schema` - AnonCreds schema JSON as bytes.
    * Restrictions:
        * Schema id must be unique.
        * Sender is equal to identity.
        * Corresponding issuer DID must exist, be active, and owned by identity.
        * Sender must have either TRUSTEE or ENDORSER or STEWARD role assigned.
    * Format:
        ```
        SchemaRegistry.createSchema(
            address identity,
            bytes32 id,
            string calldata issuerId,
            bytes calldata schema
        )
        ```
    * Example:
        ```
        SchemaRegistry.createSchema(
            "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
            "0x8ae64c08cf45da3364623a7235a9e7d132fdc8e9f6e63858b53a90d9db32c3af",
            "did:ethr:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
            [1,2,3,4,5,6,7,8,9,....]
        )
    * Raised Event:
        * `SchemaCreated(id, identity)`

#### Resolve schema

* Method: `resolveSchema`
    * Description: Transaction to resolve Schema for giving id
    * Parameters:
        * `id` - Keccak hash of Schema id to be resolved
    * Restrictions:
        * Schema must exist.
    * Format:
        ```
        SchemaRegistry.resolveSchema(
            bytes32 id
        ) returns (SchemaRecord sschemaRecord)
        ```
    * Example:
        ```
        SchemaRegistry.resolveSchema(
            "0x8ae64c08cf45da3364623a7235a9e7d132fdc8e9f6e63858b53a90d9db32c3af"
        )
    * Raised Event: `None`

## Credential Definition

### ID Syntax

#### AnonCreds Spec style

| parameter   | value                                                   |
|-------------|---------------------------------------------------------|
| id          | “did:” method-name “:” namespace “:” method-specific-id |
| method-name | “indybesu”, “ethr”                                      |
| namespace   | “testnet”/"mainnet"                                     |
| indy-id     | <issuer_did>/anoncreds/v0/CLAIM_DEF/<schema_id>/<name>  |

```
Example: did:indybesu:sovrin:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266/anoncreds/v0/CLAIM_DEF/56495/mctc
```

### Storage format

* Credential Definitions collection:
    * Description: Mapping holding the list of Credential Definition ID's to their data and metadata.
    * Format:
        ```
        mapping(bytes32 id => CredentialDefinitionRecord credentialDefinitionRecord);

        struct CredentialDefinitionRecord {
            bytes credDef;
            CredentialDefinitionMetadata metadata;
        }

        struct CredentialDefinitionMetadata {
            uint256 created;
        }
        ```
    * Example:
      ```
      {
          "0x8ae64c08cf45da3364623a7235a9e7d132fdc8e9f6e63858b53a90d9db32c3af": {
              credDef: [1,2,3,4,5,6,7,8,9,....], 
              metadata: {
                  created: 1234
              }, 
          },
          ...
      }
      ```

#### Types definition

##### CredentialDefinitionData

Schema must match to
the [specification](https://hyperledger.github.io/anoncreds-spec/#generating-a-credential-definition-without-revocation-support)
.

##### CredentialDefinitionMetadata

* `created` - timestamp of credential definition creation.

### Transactions (Smart Contract's methods)

Contract name: **CredentialDefinitionRegistry**

#### Create a new credential definition

* Method: `createCredentialDefinition`
    * Description: Transaction to create a new AnonCreds Credential Definition
    * Parameters:
        * `identity` - Account address of credential definition issuer
        * `id` - Keccak hash of Credential Definition id to be created
        * `issuerId` - DID of Credential Definition issuer
        * `schemaId` - Keccak hash of Schema id
        * `credDef` - AnonCreds Credential Definition JSON as bytes
    * Restrictions:
        * Credential Definition must be unique.
        * Sender mus tbe equal to identity.
        * Corresponding issuer DID must exist, be active, and owned by identity.
        * Corresponding schema must exist.
        * Sender must have either TRUSTEE or ENDORSER or STEWARD role assigned.
    * Format:
        ```
        CredentialDefinitionRegistry.createCredentialDefinition(
            address identity,
            bytes32 id,
            string calldata issuerId,
            bytes32 schemaId,
            bytes calldata credDef
        )
        ```
    * Example:
        ```
        CredentialDefinitionRegistry.createCredentialDefinition(
            "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
            "0x8ae64c08cf45da3364623a7235a9e7d132fdc8e9f6e63858b53a90d9db32c3af",
            did:ethr:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
            0x32ds23fd23445da3364623a7235a9e7d132fdc8e9f6e63858b53adshg234je2f2",
            [1,2,3,4,5,6,7,8,9,....]
        )
    * Raised Event:
        * `CredentialDefinitionCreated(id, identity)`

#### Resolve credential definition

* Method: `resolveCredentialDefinition`
    * Description: Transaction to resolve Credential Definition for giving id
    * Parameters:
        * `id` - Keccak hash of the Credential Definition to be resolved
    * Restrictions:
        * Credential Definition must exist.
    * Format:
        ```
        CredentialDefinitionRegistry.resolveCredentialDefinition(
            bytes32 id
        ) returns (CredentialDefinitionRecord credentialDefinitionRecord)
        ```
    * Example:
        ```
        CredentialDefinitionRegistry.resolveCredentialDefinition(
           "0x8ae64c08cf45da3364623a7235a9e7d132fdc8e9f6e63858b53a90d9db32c3af"
        )
    * Raised Event: `None`


