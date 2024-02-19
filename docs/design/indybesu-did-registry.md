# Indy DID Registry

## Storage format

* DID Records collection:
    * Description: Mapping holding the list of DID's to their DID Document and metadata.
    * Format:
        ```
        mapping(address identity => DidRecord didRecord);
  
        struct DidDocStorage {
             bytes document;
             DidMetadata metadata;
        }
  
        struct DidMetadata {
            address owner;
            uint256 created;
            uint256 updated;
            uint256 versionId;
            bool deactivated;
        }
        ```
    * Example:
      ```
      {
          "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266": {
              document: bytes("
                  {
                      "@context": [
                          "https://www.w3.org/ns/did/v1",
                           "https://w3id.org/security/suites/ed25519-2020/v1"
                      ],
                      "id": "did:indybesu:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
                      "verificationMethod": [{
                          "id": "did:indybesu:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266#key-1",
                          "type": "Ed25519VerificationKey2020",
                          "controller": "did:indybesu:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
                          "publicKeyMultibase": "zH3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
                      }],
                      "authentication": ["did:indybesu:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266#key-1"],
                  }
              "), 
              metadata: {
                  owner: 0x93917cadbace5dfce132b991732c6cda9bcc5b8a,
                  created: 1234,
                  updated: 1234,
                  versionId: 1234,
                  deactivated: false
              }, 
          },
          ...
      }
      ```

### Types definition

#### DidDocument

DID Document must match to the [specification](https://www.w3.org/TR/did-core/).

#### DID Document metadata

Each DID Document MUST have a metadata section when a representation is produced. It can have the following properties:

* owner (address): An address of DID owner
* created (timestamp): Time of a block ordered a transaction for DID Doc creation
* updated (timestamp): The updated field is null if an Update operation has never been performed on the DID document
  Time of a block ordered a transaction changed a DID Doc last time
* versionId (number): Block number when DID was created or updated
* deactivated (string): If DID has been deactivated, DID document metadata MUST include this property with the boolean
  value true. By default, this is set to false.

## Transactions (Smart Contract's methods)

Contract name: **IndyDidRegistry**

### Create DID

* Method: `createDid`
    * Description: Transaction to create a new DID record (DID Document and corresponding DID Metadata)
    * Parameters:
        * `identity` - Address of DID owner
        * `document` - DID Document JSON as bytes
    * Restrictions:
        * DID must not exist
        * Valid DID must be provided
        * Sender must be equal to identity
        * Sender must have either TRUSTEE or ENDORSER or STEWARD role assigned 
    * Format:
        ```
        IndyDidRegistry.createDid(
          address identity, 
          bytes document
        )
        ```
    * Example:
        ```
        IndyDidRegistry.createDid(
          "0xa9b7df62c953c4c49deebea05d3c8fee1f47c1f6",
          "{ did document as json bytes }" 
        )
        ```
    * Raised Event:
        * `DIDCreated(identity)`

### Update DID

* Method: `updateDid`
    * Description: Transaction to update an existing DidDocStorage entry
    * Parameters:
        * `identity` - Address of DID owner
        * `document` - DID Document JSON as bytes
    * Restrictions:
        * DID must exist
        * DID must be active
        * Sender must be equal to identity
        * Sender must be either identity owner or have a TRUSTEE role assigned
    * Format:
        ```
        IndyDidRegistry.updateDid(
          address identity, 
          bytes calldata document
        )
        ```
    * Example:
        ```
        IndyDidRegistry.updatedDid(
          "0xa9b7df62c953c4c49deebea05d3c8fee1f47c1f6"
          "{ did document as json bytes }" 
        )
        ```
    * Raised Event:
        * `DIDUpdated(identity)`

### Deactivate DID

* Method: `deactivateDid`
    * Description: Transaction to deactivate an existing DID
    * Parameters:
        * `identity` - Address of DID owner
    * Restrictions:
        * DID must exist
        * DID must be active
        * Sender must be equal to identity
        * Sender must be either identity owner or have a TRUSTEE role assigned
    * Format:
        ```
        IndyDidRegistry.deactivateDid( 
          address identity
        )
        ```
    * Example:
        ```
        IndyDidRegistry.deactivateDid(
          "0xa9b7df62c953c4c49deebea05d3c8fee1f47c1f6"
        )
        ```
    * Raised Event:
        * `DIDDeactivated(identity)`

### Resolve DID Document with Meta

* Method: `resolveDid`
    * Description: Transaction to resolve DidDocStorage entry (DID Document and corresponding DID Doc Metadata)
    * Parameters:
        * `identity` - Address of the DID identity to be resolved
    * Restrictions:
        * DID must exist
    * Format:
        ```
        IndyDidRegistry.resolveDid(
          address identity,
        ) returns (DidRecord didRecord)
        ```
    * Example:
        ```
        IndyDidRegistry.resolveDid(
          "0xa9b7df62c953c4c49deebea05d3c8fee1f47c1f6"
        )
        ```
    * Raised Event: `None`





