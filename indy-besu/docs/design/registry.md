## DID/CL Registries approach 

General design for smart contracts oriented for storing objects like DID Document, Schema, Credential Definition.
The main question is whether we need to validate storing object or not? 

### Current Option: Strict object definition

#### Pros

* Only mostly correct data written on the ledger
  * Perform full validation is really difficult in solidity 

#### Cons

* Complexity of smart contracts
    * Smart contracts must include definitions for quite complex type definitions (Did Document)
    * Smart contracts must include validation logic
    * We have to use third party dependency to perform validation
* Inconsistency between spec type definitions and stored objects
    * Solidity does not allow us to define data types exactly matching to the spec
        * As result contracts accept and store objects (DidDocument, Schema, CredentialDefinition) in a modified form
            * DidDocument: VerificationMethod, VerificationRelationship, Service
            * CredentialDefinition: Value
    * On writes, VDR accept spec compatible objects and does their conversion into the form acceptable by contracts
    * On reads, VDR must analyze returned object and convert it into appropriate form
        * Sometimes VDR even need to analyze values of some fields. For instance, depending on the type of the service
          we have to restore its original form
* Inferiority of the performed validation
* Contract size is very big and exceed the limit for public network 
  * Deployment limitation as contracts can be deployed only to gas free networks
* Inefficient from the gas point of view
* Finiteness as we can support only specific object versions and formats without upgrade

##### Validation we do for the moment

* CreateDID:
    * id / DID
        * Check syntax: split string by `:` and check that prefix, method, identifier exist
        * Check that length of `indy` identifier is 21 or 22
    * Verification method:
        * Id, type, controller are not empty
        * Either `publicKeyJwk` or `publicKeyMultibase` specified
    * Verification relationship:
        * Either string or object with correct fields
    * Authentication keys:
        * At least one provided
        * When reference, check that verification method exists
* Schema:
    * IssuerDID exists, owned, and active
    * SchemaId is correctly built from fields
    * Name is not empty
    * Version is not empty
    * Attribute list is not empty
* CredDef:
    * IssuerDID exists, owned, and active
    * Schema exists
    * Type is `CL`
    * Type is not empty
    * Value is not empty

### Option 2: Arbitrary objects as JSON string

#### Pros

* Simplicity: accept objects (Did Document, Schema, CredDef) as JSON strings and store them as is
* Spec compliant objects stored on the ledger
* We can return state proof for spec compliant objects
* Small contract size
* Gas efficient
* Different versions and formats of object can be stored without an upgrade

#### Cons

* Invalid data can be written on the Ledger
  * JSON string may contain not owned data: 
    * VDR must perform validation and be reliable by client  
  * JSON string may contain bad formatted objects
* Duplication:
  * Some fields must be passed as an independent parameters for doing obligatory state checks
    * Like issuer DID for schema and cred def 
