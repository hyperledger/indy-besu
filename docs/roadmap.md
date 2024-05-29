# Hyperledger Indy Besu ROADMAP

Note: Right now we have finished PoC implementation. Roadmap tasks and their priorities are still in process of defining.

## Phase 0: PoC

* Network configuration:
    * Infrastructure and script for development and local testing
* Network Permission base implementation:
    * Control user permissions
    * Control versions of deployed contracts
    * Control the list of network validator nodes
    * General control of write transactions
* Network identity implementation:
    * Indy2 DID method
      * Basic DID Document validation
    * Anoncreds Registry:
      * Schema and Credential Definition registries with basic validation
* Migration:
    * Design of migration from legacy (Indy) network
* Demo:
    * Flow test and sample scripts to prove the concept
* VDR:
    * Client library preparing and executing Ledger transactions (native part)
    * Support Indy2 DID Method
    * Anoncreds Anoncreds entities (schema, credential definition)
* CI/CD
    * Basic pipelines
* Docs
  * Publish Draft Indy2 DID Method

## Phase 1: MVP

* Network identity implementation:
    * Deprecate Indy2 method in favor of using `did:indy:besu` Indy DID withod extention
    * Anoncreds Registry: implement event approach matching to `did:ethr` design
    * Endorsement support for DID Documents and Anoncreds entities
* Network Permission implementation:
    * Restrict execution of transactions exclusively to users with specific roles
* Network configuration
    * Improve error handling
    * Add transaction explorer
* Demo:
    * Integration into Aries Frameworks Javascript
* Migration:
    * Tooling implementation
    * Mapping of legacy identifiers to ethereum account
* Ready for experiments and testing
* Documentation:
    * Network operators:
        * How to bootstrap a network
        * How to set up a node
        * How to onboard organizations
    * Network users:
        * How to set up public DIDs
        * How to set up schemas, cred defs etc.
        * How to issue and verify credentials
* VDR:
    * Support `did:indy:besu` DID method
    * Support endorsement flow
    * Implement additional validations to replace checks removed from contracts.
    * Wrappers for foreign languages: Python + JavaScript
    * Quorum handling for read and write transactions

## Phase 2: Beta

* Ready for deployments
* IaC:
  * Implement scripts for setting up a network
  * Implement IaC for setting up a network monitor and explorer
* Network configuration:
    * Tooling for configuration of specific networks
    * Add Transaction Author Agreement
* Network Permission:
    * Logic for assigning roles (Trustee) by consensus
* Network identity:
  * Restrict the creation of DID, Schema, and Cred Def exclusively to users with Trustee and Endorser roles
* Aries Frameworks:
  * Support Indy 2.0 in Aries Framework Javascript
  * Support Indy 2.0 in ACA-Py
* Migration:
    * Process finalization
* Anoncreds Registry:
    * Revocation entities support
* Command Line Interface
* CI/CD
    * Advanced: testing and deployment
* VDR
  * `indy-vdr` integration
  * Revocation support
  * Improve client API in more user-friendly way

## Phase 3: New features

* New versions of DID and CL Anoncreds methods
* AnonCreds 2.0 with W3CVC/BBS+
* Tombstones
* Hierarchy of trusted issuers
