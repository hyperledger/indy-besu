## Legacy identifiers support

The idea is using of a basic mapping between legacy DIDs identifiers and ethereum accounts instead of introducing a new
`did:indy2` DID method.  
So that legacy DID can be associated with a new `did:ethr` and we can only use `did:ethr` in the network.

* Create a new `LegacyIdMappingRegistry` smart contract which will be holding mapping of legacy identifiers to ethereum accounts/new ids:
    ```
    contract LegacyIdMappingRegistry {
        // did:sov:<indentifier> -> ethr account address
        mapping(string => address) public didMappings;
  
        // legacy formated ids of schemas and cred defs -> new id
        mapping(string => string) public clMappings;
    
        function createDidMapping(string legacyDid, bytes32 key, bytes32 ed25518Signature) {
            // check signature
            // check legacyDid is derived from key
            didMappings[legacyDid] = msg.sender;
        }
    
        function endorseDidMapping(address identity, string legacyDid, bytes32 key, bytes32 ed25518Signature, bytes32 ecdsaSignature) {
            // check signatures
            didMappings[legacyDid] = identity;
        }
    
        // resolve mapping done through `didMappings(string)` function available after contract compilation
    
        function createClMapping(string legacyId, string id, bytes32 key, bytes32 signature) {
            // fetch issuer did from legacy schema/credDef id 
            // check issuer did is derived from key
            // check msg.sender is owner of issuer did
            // check identity is owner of schema / cred def
            // check signature
            clMappings[legacyId] = id;
        }
    
        function endorseClMapping(address identity, string legacyId, string id, bytes32 key, bytes32 ed25518Signature, bytes32 ecdsaSignature) {
            // fetch issuer did from legacy schema/credDef id 
            // check issuer did is derived from key
            // check identity is owner of issuer did
            // check identity is owner of schema / cred def
            // check signatures
            clMappings[legacyId] = id;
        }
    
        // resolve mapping done through `clMappings(string)` function available after contract compilation
    }
    ```
    * Note, that user must pass signature over identifier to prove ownership
* On migration, DID owners willing to preserve resolving of legacy formatted DIDs and id's must do:
    * add mapping between legacy
      identifier and ethereum account representing `did:ethr` identifier by
      executing `LegacyIdMappingRegistry.createDidMapping(...)` method where he must pass:
        * DID identifier itself
        * Associated public key
        * Ed25519 signature owner identifier proving ownership
            * Signature must be done over the following hash value: `keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), msg.sender, "createDidMapping", legacyDid, identity, key))`
    * add mapping between legacy schema/credDef id's and new ones executing `LegacyIdMappingRegistry.createClMapping()` method he must pass:
        * Legacy schema/credDef id
        * New id of corresponding schema/credDef
        * Associated public key
        * Ed25519 signature owner identifier proving ownership
            * Signature must be done over the following hash value: `keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), msg.sender, "createClMapping", legacyId, id, key))`
* After migration, clients in order to resolve legacy identifiers:
    * for DID document firstly must resolve ethereum account
      using `LegacyIdMappingRegistry.didMappings(identifier)`, and next resolve DID ether document as it described in the
      corresponding specification.
    * for Schema/Credential Definition firstly must resolve new identifier
      using `LegacyIdMappingRegistry.clMappings(identifier)`, and next resolve Schema/Credential Definition as it described in the
      corresponding specification.
    
