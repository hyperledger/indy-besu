## Design documents

### Diagrams

- [Components Overview](./design/conmponents.png)
- [Deployed Smart Contracts](./design/contracts.png)
- [Flow](./design/flow.png)

### Modules

- Network Permission modules:
  - [Auth](design/auth.md) - control user permissions
    - role control - manage roles assigned to accounts
    - access control - first level validation: whether to accept write transactions (execute target contract method) from a given account
  - [Upgrading contracts](design/upgradability.md) - control versions of deployed contracts (proposing and approving new versions).
  - [Validators node management](design/network.md) - control the list of network validator nodes
- Identity:
  - [DID Methods](design/did-methods.md) - Supported DID method
    - [IndyBesu DID Registry](design/indybesu-did-registry.md) - `indybesu` DID Registry
  - [Anoncreds Registry](design/cl-registry.md)
- [Client](design/vdr.md) - design of VDR library

### Migration documents

- [Indy Migration](migration/migration.md)

### Development designs

- [Roadmap](./roadmap.md)
- [DID and Anoncreds registers approach](./design/registry.md)
- [Transaction Endorsement](./design/endorsement.md)
- [Contract upgradability](./design/upgradability.md)
- [Legacy identifiers support](./design/legacy-identifiers-support.md)
