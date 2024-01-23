# CL Registry

## Schema

### ID Syntax

#### AnonCreds Spec style

| parameter   | value                                                           |
|-------------|-----------------------------------------------------------------|
| id          | “did:” method-name “:” namespace “:” method-specific-id         |
| method-name | “indy2”, “indy”, “sov”, “ethr”                                  |
| namespace   | “testnet”/"mainnet"                                             |
| indy-id     | <issuer_did>/anoncreds/v0/SCHEMA/<schema_name>/<schema_version> |

```
Example: did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu/anoncreds/v0/SCHEMA/BasicIdentity/1.0.0
```

### Storage format

* Schemas collection:
    * Description: Mapping holding the list of Schema ID's to their data and metadata.
    * Format:
        ```
        mapping(string id => SchemaRecord schemaRecord);
  
        struct SchemaRecord {
            string data;
            SchemaMetadata metadata;
        }

        struct SchemaMetadata {
            uint256 created;
        }
        ```
    * Example:
      ```
      {
          "did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu/anoncreds/v0/SCHEMA/BasicIdentity/1.0.0": {
              schema: "{
                  "issuerId": "did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu",
                  "name": "BasicIdentity",
                  "version": "1.0.0",
                  "attrNames": ["First Name", "Last Name"]              
              }", 
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
        * `id` - Id of schema to be created
        * `issuerId` - Id of schema issuer
        * `schema` - AnonCreds schema as JSON string
    * Restrictions:
        * Schema id must be unique.
        * Corresponding issuer DID must exist, be active, and owned by sender.
    * Format:
        ```
        SchemaRegistry.createSchema(
            string calldata id,
            string calldata issuerId,
            string calldata schema
        )
        ```
    * Example:
        ```
        SchemaRegistry.createSchema(
            "did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu/anoncreds/v0/SCHEMA/BasicIdentity/1.0.0",
            "did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu",
            "{
                "issuerId": "did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu",
                "name": "BasicIdentity",
                "version": "1.0.0",
                "attrNames": ["First Name", "Last Name"]
            }"
        )
    * Raised Event:
        * `SchemaCreated(schemaId)`

#### Resolve schema

* Method: `resolveSchema`
    * Description: Transaction to resolve Schema for giving id
    * Parameters:
        * `id` - ID of the Schema to resolve
    * Restrictions:
        * Schema must exist.
    * Format:
        ```
        SchemaRegistry.resolveSchema(
            string id
        ) returns (SchemaRecord sschemaRecord)
        ```
    * Example:
        ```
        SchemaRegistry.resolveSchema(
            "did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu/anoncreds/v0/SCHEMA/BasicIdentity/1.0.0"
        )
    * Raised Event: `None`

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

* Credential Definitions collection:
    * Description: Mapping holding the list of Credential Definition ID's to their data and metadata.
    * Format:
        ```
        mapping(string id => CredentialDefinitionRecord credentialDefinitionRecord);

        struct CredentialDefinitionRecord {
            string credDef;
            CredentialDefinitionMetadata metadata;
        }

        struct CredentialDefinitionMetadata {
            uint256 created;
        }
        ```
    * Example:
      ```
      {
          "did:indy2:sovrin:Gs6cQcvrtWoZKsbBhD3dQJ/anoncreds/v0/CLAIM_DEF/56495/mctc": {
              credDef: "{
                  "issuerId": "did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu",
                  "schemaId": "did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu/anoncreds/v0/SCHEMA/BasicIdentity/1.0.0",
                  "type": "CL",
                  "tag": "BasicIdentity",
                  "value": "{ ... }"
              }", 
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
the [specification](https://hyperledger.github.io/anoncreds-spec/#generating-a-credential-definition-without-revocation-support).

##### CredentialDefinitionMetadata

* `created` - timestamp of credential definition creation.

### Transactions (Smart Contract's methods)

Contract name: **CredentialDefinitionRegistry**

#### Create a new credential definition

* Method: `createCredentialDefinition`
    * Description: Transaction to create a new AnonCreds Credential Definition
    * Parameters:
        * `id` - Id of credential definition to be created
        * `issuerId` - Id of credential definition issuer
        * `schemaId` - Id of credential definition schema
        * `credDef` - AnonCreds credential definition as JSON string
    * Restrictions:
        * Credential Definition must be unique.
        * Corresponding issuer DID must exist, be active, and owned by sender.
        * Corresponding schema must exist.
    * Format:
        ```
        CredentialDefinitionRegistry.createCredentialDefinition(
            string calldata id,
            string calldata issuerId,
            string calldata schemaId,
            string calldata credDef
        )
        ```
    * Example:
        ```
        CredentialDefinitionRegistry.createCredentialDefinition(
            "did:indy2:sovrin:Gs6cQcvrtWoZKsbBhD3dQJ/anoncreds/v0/CLAIM_DEF/56495/BasicIdentity",
            "did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu",
            "did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu/anoncreds/v0/SCHEMA/BasicIdentity/1.0.0",
            "{
                "issuerId": "did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu",
                "schemaId": "did:indy2:mainnet:Y6LRXGU3ZCpm7yzjVRSaGu/anoncreds/v0/SCHEMA/BasicIdentity/1.0.0",
                "type": "CL",
                "tag": "BasicIdentity",
                "value": "{.......}",
            }"
        )
    * Raised Event:
        * `CredentialDefinitionCreated(credentialDefinitionId)`

#### Resolve credential definition

* Method: `resolveCredentialDefinition`
    * Description: Transaction to resolve Credential Definition for giving id
    * Parameters:
        * `id` - Id of credential definition to be resolved
    * Restrictions:
        * Credential Definition must exist.
    * Format:
        ```
        CredentialDefinitionRegistry.resolveCredentialDefinition(
            string calldata id
        ) returns (CredentialDefinitionRecord credentialDefinitionRecord)
        ```
    * Example:
        ```
        CredentialDefinitionRegistry.resolveCredentialDefinition(
           "did:indy2:sovrin:Gs6cQcvrtWoZKsbBhD3dQJ/anoncreds/v0/CLAIM_DEF/56495/BasicIdentity"
        )
    * Raised Event: `None`


