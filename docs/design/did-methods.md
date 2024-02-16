# DID Methods

Out of box Ledger provides an ability to use one of two supported DID methods: `did:ethr` or `did:indybesu`.

Contracts implementing both methods are deployed on the network and integrated with `Anoncreds Registry`.

Ledger `permission` related modules are implemented in a way to use **account address** but not a DID.

It is up to a User which DID method to use.

> Moreover, users having an appropriate permissions can even deploy contracts adding support for another DID methods
> (need to integrate into `CLRegistry`).

## Ethereum DID method: did:ethr

Ethereum DID Method `did:ethr` described in
the [specification](https://github.com/decentralized-identity/ethr-did-resolver/blob/master/doc/did-method-spec.md).

Example DID: `did:ethr:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266`

## IndyBesu DID method: did:indybesu

The identifier of `indybesu` DID method is an identity address similarly to `did:ethr` method, but there multiple 
significant differences between them:
* API consist of more traditional `create`, `update`, `deactivate` methods
* The associated `Did Document` is stored in the contract storage in complete form
* In order to resolve Did Document you only need to call single method
* DID must be registered by executing one of `create` contract methods
* State proof can be obtained for resolved Did Record

Example:

Example DID: `did:indybesu:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266`

### DID Syntax

| parameter          | value                                                   |
|--------------------|---------------------------------------------------------|
| did                | “did:” method-name “:” namespace “:” method-specific-id |
| method-name        | “indybesu”                                              |
| namespace          | “testnet”/"mainnet"                                     |
| method-specific-id | ethereum-address                                        |
