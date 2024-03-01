## Legacy identifiers support

The idea is using of a basic mapping between legacy DIDs identifiers and ethereum accounts instead of introducing a new
`did:indy2` DID method.  
So that legacy DID can be associated with a new `did:ethr` and we can only use `did:ethr` in the network.

* Create a new `LegacyMappingRegistry` smart contract which will be holding mapping of legacy identifiers to ethereum accounts/new ids:
    ```
    contract LegacyMappingRegistry {
        // Mapping storing indy/sov DID identifiers to the corresponding account address
        mapping(bytes16 legacyIdentifier => address account) public didMapping;
  
        // Mapping storing indy/sov formatted identifiers of schema/credential-definition to the corresponding new form
        mapping(string legacyId => string newId) public resourceMapping;
    
        function createDidMapping(
            address identity,
            string calldata identifier,
            bytes32 ed25519Key,
            bytes calldata ed25519Signature
        )
            // check signature
            // check legacyDid is derived from key
            didMapping[identifier] = msg.sender;
        }
    
        function createDidMappingSigned(
            address identity,
            uint8 sigV,
            bytes32 sigR,
            bytes32 sigS,
            string calldata identifier,
            bytes32 ed25519Key,
            bytes calldata ed25519Signature
        )
            // check signatures
            didMapping[identifier] = identity;
        }
    
        // resolve mapping done through `didMapping(bytes16 identifier)` function available after contract compilation
    
        function createResourceMapping(
            address identity,
            string calldata legacyIssuerIdentifier,
            string calldata legacyIdentifier,
            string calldata newIdentifier
        )
            // fetch issuer did from legacy schema/credDef id 
            // check issuer did is derived from key
            // check msg.sender is owner of issuer did
            // check identity is owner of schema / cred def
            // check signature
            resourceMapping[legacyIdentifier] = newIdentifier;
        }
    
        function createResourceMappingSigned(
            address identity,
            uint8 sigV,
            bytes32 sigR,
            bytes32 sigS,
            string calldata legacyIssuerIdentifier,
            string calldata legacyIdentifier,
            string calldata newIdentifier
        )
            // fetch issuer did from legacy schema/credDef id 
            // check issuer did is derived from key
            // check identity is owner of issuer did
            // check identity is owner of schema / cred def
            // check signatures
            resourceMapping[legacyIdentifier] = newIdentifier;
        }
    
        // resolve mapping done through `resourceMapping(string legacyIdentifier)` function available after contract compilation
    }
    ```
    * Note, that user must pass signature over identifier to prove ownership
* On migration, DID owners willing to preserve resolving of legacy formatted DIDs and id's must do:
    * add mapping between legacy
      identifier and ethereum account representing `did:ethr` identifier by
      executing `LegacyMappingRegistry.createDidMapping(...)` method where he must pass:
        * DID identifier itself
        * Associated public key
        * Ed25519 signature owner identifier proving ownership
            * Signature must be done over `legacyDid` bytes
    * add mapping between legacy schema/credDef id's and new ones executing `LegacyMappingRegistry.createResourceMapping()` method he must pass:
        * Legacy DID
        * Legacy schema/credDef id
        * New id of corresponding schema/credDef
* After migration, clients in order to resolve legacy identifiers:
    * for DID document firstly must resolve ethereum account
      using `LegacyMappingRegistry.didMapping(legacyIdentifier)`, and next resolve DID ether document as it described in the
      corresponding specification.
    * for Schema/Credential Definition firstly must resolve new identifier
      using `LegacyMappingRegistry.resourceMapping(legacyIdentifier)`, and next resolve Schema/Credential Definition as it described in the
      corresponding specification.
