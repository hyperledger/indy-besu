# DID Methods

Out of box Ledger provides an ability to use one of two supported DID methods: `did:ethr` or `did:indy`.

Contracts implementing both methods are deployed on the network and integrated with `CL Registry`.

Ledger `permission` related modules are implemented in a way to use **account address** but not a DID.

It is up to a User which DID method to use.

> Moreover, users having an appropriate permissions can even deploy contracts adding support for another DID methods
> (need to integrate into `CLRegistry`).

## Ethereum DID method: did:ethr

Ethereum DID Method `did:ethr` described in
the [specification](https://github.com/decentralized-identity/ethr-did-resolver/blob/master/doc/did-method-spec.md).

Example DID: `did:ethr:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266`

## Indy2 DID method: did:indy2 - Indy/Sov DID methods adoption

New `indy2` DID method represented in a form compatible with `indy` and `sov` DID methods used in legacy Indy based
networks.

Users having `indy/sov` DID's (like `did:sov:2wJPyULfLLnYTEFYzByfUR`) can keep using their `id`
part (`2wJPyULfLLnYTEFYzByfUR`) for preserving the trust.

Example:

* Legacy DID: `did:sov:2wJPyULfLLnYTEFYzByfUR`
* New DID will be stored on the Ledger: `did:indy2:2wJPyULfLLnYTEFYzByfUR`

### DID Syntax

| parameter          | value                                                   |
|--------------------|---------------------------------------------------------|
| did                | “did:” method-name “:” namespace “:” method-specific-id |
| method-name        | “indy2”, “indy”, “sov”                                  |
| namespace          | “testnet”/"mainnet"                                     |
| method-specific-id | indy-id                                                 |
| indy-id            | Base58(Truncate_msb(16(SHA256(publicKey))))             |

The `indy-id` is received by deriving from the initial ED25519 verkey the same was as it is described in
the [Sovrin DID Method Specification](https://sovrin-foundation.github.io/sovrin/spec/did-method-spec-template.html#namespace-specific-identifier-nsi)
.

#### Ownership proving

In case of `did:ethr` ownership is proven by validation of transaction level signature. When an identity decide to
use `indy2` DID method we need to add provide an additional validation to prove an identifier ownership.

The process will be similar to endorsement flow:

* Author steps:
    * Step 1: Author prepares a DID Document object (`indy2` DID method is used)
    * Step 2: Author convert DID Document into contracts representation (which will be stored on the ledger) and encodes
      it into bytes using `abi.encodePacked` (available in solidity as well)
        * Same bytes are signed as in case of endorsement flow
    * Step 3: Author performs Ed25519 signing using secret key associated with the identifier
        * The correspondent verification key must be presented in the DidDocument
    * Step 4: Author passes Ed25519 Signature as contract parameter
* Contract:
    * Step 1: Encodes DID Document into bytes using `abi.encodePacked`
    * Step 2: Find Ed25519 Verification Method
      * Issue: We need to decode the key from string representation. Maybe we should pass key bytes as contract parameter instead? 
    * Step 3: Check that DID identifier can be derived from verification key
    * Step 4: Checks the validity of the provided Ed25519 signature against DidDocument bytes and verification key

```
// legacyDidPosessionSignature - is optional parameter
function createDid(DidDocument didDocument, bytes32 legacyDidPosessionSignature) {
    ...
    bytes32 hash = abi.encodePacked(didDocument);

    // if DID is of `indy` method additional ED25518 signature must be provided and checked 
    if (didDocument.did.isIndyMethod()) {
        if (legacyDidPosessionSignature == address(0)) {
            revert InvalidSignature("Ed25518 signature must be provided");
        }
        string ed25519Key = didDocument.find(key => key.type == ED25519).publicKeyBase58 
        checkDidIdentifierMatchesToKey(ed25519Key, didDocument.did)
        checkEd25518Signature(hash, ed25519Key, legacyDidPosessionSignature)
    }
}
```
