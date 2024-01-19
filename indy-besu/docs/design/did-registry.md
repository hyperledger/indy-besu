# Indy DID Registry

## Storage format

* DID Records collection:
    * Description: Mapping holding the list of DID's to their DID Document and metadata.
    * Format:
        ```
        mapping(string DID => DidRecord didRecord);
  
        struct DidDocStorage {
             string document;
             DidMetadata metadata;
        }
  
        struct DidMetadata {
            address creator;
            uint256 created;
            uint256 updated;
            bool deactivated;
        }
        ```
    * Example:
      ```
      {
          "did:indy2:testnet:SEp33q43PsdP7nDATyySSH": {
              document: "
                  {
                      "@context": [
                          "https://www.w3.org/ns/did/v1",
                           "https://w3id.org/security/suites/ed25519-2020/v1"
                      ],
                      "id": "did:example:123456789abcdefghi",
                      "verificationMethod": [{
                          "id": "did:example:123456789abcdefghi#key-1",
                          "type": "Ed25519VerificationKey2020",
                          "controller": "did:example:123456789abcdefghi",
                          "publicKeyMultibase": "zH3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
                      }],
                      "authentication": ["#key-1"],
                  }
              ", 
              metadata: {
                  owner: 0x93917cadbace5dfce132b991732c6cda9bcc5b8a,
                  sender: 0x93917cadbace5dfce132b991732c6cda9bcc5b8a,
                  created: 1234,
                  updated: 1234,
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
* sender (address): An address of DID Document sender
* created (timestamp): Time of a block ordered a transaction for DID Doc creation
* updated (timestamp): The updated field is null if an Update operation has never been performed on the DID document.
  Time of a block ordered a transaction changed a DID Doc last time
* deactivated (string): If DID has been deactivated, DID document metadata MUST include this property with the boolean
  value true. By default this is set to false.

## Transactions (Smart Contract's methods)

Contract name: **IndyDidRegistry**

### Create DID

* Method: `createDid`
    * Description: Transaction to create a new DID record (DID Document and corresponding DID Metadata)
    * Parameters:
        * `identity` - Address of DID owner
        * `did` - The new DID
        * `document` - The new DID Document as JSON string
    * Restrictions:
        * DID must not exist
        * Valid DID must be provided
    * Format:
        ```
        IndyDidRegistry.createDid(
          address identity, 
          string calldata did, 
          string calldata document
        )
        ```
    * Example:
        ```
        IndyDidRegistry.createDid(
          "0xa9b7df62c953c4c49deebea05d3c8fee1f47c1f6",
          "did:indy2:testnet:SEp33q43PsdP7nDATyySSH",
          "{ did document as json string }" 
        )
        ```
    * Raised Event:
        * `DIDCreated(did)`

### Update DID

* Method: `updateDid`
    * Description: Transaction to update an existing DidDocStorage entry
    * Restrictions:
        * DID must exist
        * DID must be active
        * Sender must be authorized to perform update (owner or sender)
    * Format:
        ```
        IndyDidRegistry.updateDid(
          string calldata did, 
          string calldata document
        )
        ```
    * Example:
        ```
        IndyDidRegistry.updatedDid(
          "did:indy2:testnet:SEp33q43PsdP7nDATyySSH"
          "{ did document as json string }" 
        )
        ```
    * Raised Event:
        * `DIDUpdated(did)`

### Deactivate DID

* Method: `deactivateDid`
    * Description: Transaction to deactivate an existing DID
    * Parameters:
        * `did` - DID to update
        * `document` - The new DID Document as JSON string
    * Restrictions:
        * DID must exist
        * DID must be active
        * Sender must be authorized to perform deactivation (owner or sender)
    * Format:
        ```
        IndyDidRegistry.deactivateDid( 
          string did
        )
        ```
    * Example:
        ```
        IndyDidRegistry.deactivateDid(
          "did:indy2:testnet:SEp33q43PsdP7nDATyySSH"
        )
        ```
    * Raised Event:
        * `DIDDeactivated(did)`

### Resolve DID Document with Meta

* Method: `resolveDid`
    * Description: Transaction to resolve DidDocStorage entry (DID Document and corresponding DID Doc Metadata)
    * Parameters:
        * `did` - DID to deactivate
    * Restrictions:
        * DID must exist
    * Format:
        ```
        IndyDidRegistry.resolveDid(
          string did,
        ) returns (DidRecord didRecord)
        ```
    * Example:
        ```
        IndyDidRegistry.resolveDid(
          "did:indy2:testnet:SEp33q43PsdP7nDATyySSH"
        )
        ```
    * Raised Event: `None`





