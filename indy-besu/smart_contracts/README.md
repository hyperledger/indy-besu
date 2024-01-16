# Indy smart contracts

### Prerequisites to run

* `node` > `v18.15.0`
* `yarn`

### Install dependencies

```
> yarn install
```

### Compile contracts

```
> yarn compile
```

The following folders should be generated as the result:

* `artifacts` - completed contract specification
* `typechain-types` - typescript bindings for contracts

### Run tests

```
> yarn test
```

### Main Contracts

* `contracts/auth/AccountControl.sol` - contract to manage permissions for account transactions
    * [AccountControl TS contract wrapper class](./contracts-ts/AccountControl.ts)
* `contracts/auth/RoleControl.sol` - contract to manage (assign/revoke) account roles.
    * [RoleControl TS contract wrapper class](./contracts-ts/RoleControl.ts)
* `contracts/cl/CredentialDefinitionRegistry` - contract to manage (create/resolve) credential definitions
    * [CredentialDefinitionRegistry TS contract wrapper class](./contracts-ts/CredentialDefinitionRegistry.ts)
* `contracts/cl/SchemaRegistry` - contract to manager (create/resolve) schemas
    * [SchemaRegistry TS contract wrapper class](./contracts-ts/SchemaRegistry.ts)
* `contracts/did/DidRegistry` - contract to manage (create/update/deactivate/resolve) DID doucments
    * [DidRegistry TS contract wrapper class](./contracts-ts/DidRegistry.ts)
* `contracts/network/ValidatorControl.sol` - contract to manage network validator nodes.
    * [ValidatorControl TS contract wrapper class](./contracts-ts/ValidatorControl.ts)

### Demos

You can find sample scripts demonstrating the usage of deployed contracts in the [demo folder](./demos).

* [Account management](./demos/account-control.ts) - deploy/read/writer transactions.
    ```
    > yarn demo/account
    ```
* [Demo flow](./demos/flow.ts) - create/resolve DID/Schema/Credential Definition using `did:indy2` method.
    ```
    > yarn demo/flow
    ```
* [Demo flow](./demos/flow-with-did-ethr.ts) - create/resolve DID/Schema/Credential Definition using `did:ethr` method.
    ```
    > yarn demo/flow-with-did-ethr
    ```
* [Roles management](./demos/role-control.ts) - get/assign/revoke role to/from account.
    ```
    > yarn demo/roles
    ```
* [Upgrade management](./demos/upgrade-control.ts) - propose/approve upgradable contract implementation.
    ```
    > yarn demo/upgrade
    ```
* [Validators management](./demos/validator-control.ts) - get list of current validators.
    ```
    > yarn demo/validators
    ```

### Helper Scripts

* `genesis` - helper scripts to generate genesis blocks for injecting contracts.

  > Find more details regarding the scripts in the [genesis section](#inject-contracts-into-network-genesis) of this document.

## Inject contracts into network genesis

### Prerequisites

* `socl` tool must be installed on the machine.

This section describes how to inject smart contracts into the genesis state of the network.

1. Prepare the [input file](scripts/genesis/config.ts) with the initial state of each contract.

2. Compile contracts:
   ```
   yarn solc-compile
   ```

* `artifacts` and `compiled-contracts` folders with compiled contracts will be generated as the result of the execution.

3. Execute script generating the contracts content for the network genesis file:
   > yarn genesis/generate

* `ContractsGenesis.json` file will be generated as the result

4. Put the whole block into the `alloc` section of the network genesis file.

5. **If you changed the default address of `ValidatorControl` contract**: Set address of `ValidatorControl` contract
   into `validatorcontractaddress` field of the `qbft` section of the genesis file.

6. **If you changed the default address of `AccountControl` contract**: Set address of `AccountControl` contract
   into `permissions-accounts-contract-address` field of the `config.toml`
   file.
