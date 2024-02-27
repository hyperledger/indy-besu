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

### Indy Ledger Objects: Glossary

#### DID Document
A DID Document conform to the [Decentralized Identifiers (DIDs) Core specification](https://https://www.w3.org/TR/did-core/).

> #### SCHEMA
> A SCHEMA object is a template that defines a set of attribute (names) which are going to be used by issuers for issuance of Verifiable Credentials within a Hyperledger Indy network. SCHEMAs have a name, version and can be written to the ledger by any entity with proper permissions. Schemas can be read from a Hyperledger Indy Node by any client.
>
> SCHEMAs define the list of attribute (names) of issued credentials based on a CRED_DEF (see below).
> 
> #### CRED_DEF
> A CRED_DEF (short for “credential definition”) object contains data required for credential issuance as well as credential validation and can be read by any Hyperledger Indy client. A > CRED_DEF object references a SCHEMA, references a DID of the issuer and can be written by any issuer who intends to issue credentials based on that specific SCHEMA to the ledger and has the proper permissions in doing so. A public key of the issuer is included within the CRED_DEF which allows validation of the credentials signed by the issuer’s private key. When credentials are issued by using the issuers CRED_DEF, the attribute (names) of the SCHEMA have to be used.

(c) [Indy DID Method](https://hyperledger.github.io/indy-did-method/#indy-ledger-objects-glossary)

#### Indy Besu VDR

Hyperledger Indy Besu VDR (for "Verifiable Data Registry") is an open source implementation of an Indy client/resolver for both DIDs and other Indy objects. The repository is called indy-besu contains Indy Besu VDR [here](https://github.com/hyperledger/indy-besu/tree/main/vdr).

### Target System(s)

The `did:indybesu` DID method applies to all DIDs which are anchored to a Hyperledger Indy Besu Ledger and which comply with these specific conventions.

### Indy Besu DID Method Identifiers

> The did:indybesu Method DID identifier has four components that are concatenated to make a DID specification conformant identifier. The components are:

> - **DID**: the hardcoded string `did:` to indicate the identifier is a DID
> - **DID Indy-Besu Method**: the hardcoded string `indybesu:` indicating that the identifier uses this DID Method specification.
> - **DID Indy-Besu Namespace**: a string that identifies the name of the primary Indy Besu ledger, followed by a `:`. The namespace string may optionally have a secondary ledger name prefixed by a `:` following the primary name. If there is no secondary ledger element, the DID resides on the primary ledger, else it resides on the secondary ledger. By convention, the primary is a production ledger while the secondary ledgers are non-production ledgers (e.g. staging, test, development) associated with the primary ledger. Examples include, `sovrin`, `sovrin:staging` and `idunion`.
- **Namespace Identifier**: The identifier of `indybesu` DID method is an identity address similarly to `did:ethr` method, but there multiple 
significant differences between them:
  - API consist of more traditional `create`, `update`, `deactivate` methods
  - The associated `Did Document` is stored in the contract storage in complete form
  - In order to resolve Did Document you only need to call single method
  - DID must be registered by executing one of `create` contract methods
  - State proof can be obtained for resolved Did Record

Example:

Example DID: `did:indybesu:testnet:0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266`

### DID Syntax

| parameter          | value                                                   |
|--------------------|---------------------------------------------------------|
| did                | “did:” method-name “:” namespace “:” method-specific-id |
| method-name        | “indybesu”                                              |
| namespace          | *(%x61-7A / DIGIT)                                      |
| method-specific-id | ethereum-address                                        |

> ### Other Indy Ledger Object Identifiers

> Indy Besu ledgers may hold object types other than DIDs, and each of the other object types must also be resolvable to a specific Indy network instance. The identifiers for these objects are used in data structures that are exchanged by Indy Besu clients (e.g. Aries Agents)--verifiable credentials, presentation requests, presentations and so on. Transitioning to the `did:indybesu` DID Method requires transitioning Indy Besu clients/resolvers to use the identifiers defined in this section.

> ### DID URLs for Indy Object Identifiers

> The structure of identifiers for all non-DID Indy Besu ledger objects is the following DID URL structure, based on the DID of the object's DID controller:
> - `<did>/<object-family>/<object-family-version>/<object-type>/<object-type-identifier>`

> The components of the DID URL are:

> - `<did>` the `did:indybesu` DID of the object-owning controller
> - `<object-family>` family of the object
> - `<object-family-version>` version of the object family
> - `<object-type>` one of [[ref: SCHEMA]], [[ref: CRED_DEF]], [[ref: REV_REG_DEF]], [[ref: REV_REG_ENTRY]], [[ref: ATTRIB]]
> - `<object-type-identifier>` an object type unique identifier defined by Indy by object type.

> The data returned from resolving such DID URLs is the ledger object and relevant state proof; the same data returned from the Indy Node read object transactions, such as the GET_SCHEMA transaction, and dependent on the type of the object.

> Since indy allows special characters within the names of the different ledger objects, percent encoding according to [Section 2 of RFC3986](https://datatracker.ietf.org/doc/html/rfc3986#section-2) has to be applied to access these objects via DID URLs.

> While there are no restrictions regarding the used characters, we strongly encourage avoiding special characters in the names of ledger objects.

> The following sections cover each ledger object type, providing:

> - an example DID URL identifier,
> - a link to an example object residing on the Sovrin MainNet Indy ledger (courtesy of [indyscan.io](https://indyscan.io)),
> - the appropriate object family and version,
> - the format of the response when resolving the DID URL,
> - the pre-`did:indybesu` identifier for each object, and
> - notes about the elements of the pre-`did:indybesu` identifier.

> This first version of the `did:indybesu` DID Method will use an `<object-family>` value of `anoncreds` and an `<object-family-version>` of `v0` to match the
pre-specification, open source version of anoncreds as implemented in the [indy-sdk](https://github.com/hyperledger/indy-sdk/tree/master/docs/design/002-anoncreds).
Later versions of the `did:indybesu` specification will use a higher `<object-family-version>` as the AnonCreds standardization work proceeds
and the required dependency on Hyperledger Indy is removed. In this initial version, the DID URLs are closely aligned with the existing object identifiers.

> #### Schema

DID URL: `did:indybesu:testnet:0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5/anoncreds/v0/SCHEMA/F1DClaFEzi3t/1.0.0`

> - Object Family: `anoncreds`
> - Family Version: `v0`
- Object Type: `SCHEMA`
- Name (identifier), example `F1DClaFEzi3t`: The client-defined schema name
> - Schema Version, example `4.3.4`: The client-defined version of the [[ref: SCHEMA]]

> #### Cred Def

DID URL: `did:indybesu:testnet:0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5/anoncreds/v0/CLAIM_DEF/did:indybesu:testnet:0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5/anoncreds/v0/SCHEMA/F1DClaFEzi3t/1.0.0/default`

> - Object Family: `anoncreds`
> - Family Version: `v0`
- Object Type: `CLAIM_DEF`
- Schema ID, example `did:indybesu:testnet:0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5/anoncreds/v0/SCHEMA/F1DClaFEzi3t/1.0.0`: A unique identifier for the schema upon which the CredDef is defined. 
- Tag, example `default`: The client-defined cred def name.

> We recommend that AnonCred credential issuers use a unique Name item per Cred Def, and not rely on the embedded Schema ID
remaining in the DID URL for a Cred Def in future versions of the `did:indybesu` method.

> ### Finding Indy Ledgers

//todo

## Security Considerations

Hyperledger Indy is a public, permissioned distributed ledger that uses QBFT to establish a consensus between upfront well-authenticated nodes. The security mechanisms by Hyperledger Indy Besu and Hyperledger Besu guarantee the correct processing of requests and transactions according to the rules, which are themselves part of the consensus on the ledger. In particular, this enables the creation and update of schemas, credential definitions and DIDs by their owners by authenticating with the corresponding public keys stored on the ledger.

//todo

The following sections describe how the `did:indybesu` DID Method adheres to the security considerations outlined in the [DID Core 1.0 specification](https://w3c.github.io/did-core) and in accordance with RFC3552.

> ### Privacy Considerations

> Given that Indy is a publicly readable, immutable ledger, no personally identifiable information, including DIDs where a person is the DID Subject, should be placed on the network. As this DID method does not offer yet any means to delete or deactivate personal information (e.g. in the sense of GDPR), it is important to enforce these rules by organizational means, for example through an Endorser Transaction Agreement or other contractual agreements.

The further privacy properties are stated according to [Section 5 of RFC6973](https://tools.ietf.org/html/rfc6973#section-5).

> #### Surveillance

> The DIDs and their resolved DID Documents are  public readable and therefore the content and the changes of the data are inherently suspectable to surveillance.
Furthermore, authors of read and write requests can be surveilled by their communication to the ledger nodes. Clients sending write requests can be observed and identified by their author DIDs and signatures. However, read requests are unsigned and only need to be sent to one node, therefore offering choice and better protection from surveillance for the client.

> #### Stored Data Compromise

> The compromise of stored data on the ledger is prevented by the distributed, signed and consensus-based storage of the data. The stored data of an individual ledger node shall be protected by implementing best practice in securing the IT infrastructure, like ISO27001 and Information Security Management systems (ISMS).

> #### Unsolicited Traffic

> DID Documents can be resolved from a DID, however the DID subject can choose to include or exclude service endpoints that expose itself to unsolicited traffic. The nodes of the ledger itself are exposed to any unwanted traffic as explained in the Denial-of-Service section.

> #### Correlation

> The Hyperledger Indy ecosystem with Anoncreds 1 was designed to prevent correlation by design. No DIDs nor any data or credentials of natural persons are stored on the ledger, the revocation system guarantees a high degree of anonymity.
However, DIDs on the ledger, used by organizations, enable correlation by the pseudonymous DID itself or through data like service endpoints from the resolved DID Document as described in the DID-Core specification. Ledger nodes are prohibited to collect any metadata of (read) requests to the ledger to prevent correlation.

> #### Identification

> The Hyperledger Indy ecosystem prevents identification of natural persons as they do not have a DID on the ledger. Identification of DIDs through the data of the resolved DID Document is possible and usually desired as issuing or verifying organizations want to authentically disclose their identity.

> #### Secondary Use

> As all data written to the ledger is inherently public, clients sending write requests should be aware of possible secondary use and cautiously decide whether data appropriate data is published. No personal data shall be send to the ledger.

> #### Disclosure

> Disclosure of data send to the ledger is not an issue, as written data is public anyway. Ledger nodes are prohibited to collect any metadata of (read) requests to the ledger to prevent disclosure.

> #### Exclusion

> Any read request to the ledger nodes in unauthorized preventing exclusion to the ledger data.
