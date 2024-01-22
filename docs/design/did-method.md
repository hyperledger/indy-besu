# DID Method

The extended version of Ethr DID method (`did:ethr`) in used on the ledger.
The core specification of `did:ethr` can be found [here](https://github.com/decentralized-identity/ethr-did-resolver/blob/master/doc/did-method-spec.md) in great details is used on the Ledger.

Example DID: `did:ethr:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266`

The following extension logic is applied to DID ethr smart contract to integrate it with permission related modules:
* Control contract upgrade versions
* Control account roles executing write transactions
