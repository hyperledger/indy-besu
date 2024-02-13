import { readFileSync } from 'fs'
import { resolve } from 'path'

import {
  AccountControlConfig,
  CredentialDefinitionsConfig,
  EthereumDidRegistryConfig,
  IndybesuDidRegistryConfig,
  LegacyMappingRegistryConfig,
  RolesConfig,
  SchemasConfig,
  UniversalDidResolverConfig,
  UpgradeControlConfig,
  ValidatorsConfig,
} from './contracts'

export const compiledContractsFolder = 'compiled-contracts'
export const inFile = 'config.json'
export const outFile = 'ContractsGenesis.json'

export interface Config {
  accountControl: AccountControlConfig
  credentialDefinitionRegistry: CredentialDefinitionsConfig
  indybesuDidRegistry: IndybesuDidRegistryConfig
  ethereumDidRegistry: EthereumDidRegistryConfig
  roleControl: RolesConfig
  schemaRegistry: SchemasConfig
  universalDidResolver: UniversalDidResolverConfig
  upgradeControl: UpgradeControlConfig
  validatorControl: ValidatorsConfig
  legacyMapping: LegacyMappingRegistryConfig
}

const contractsAddresses: { [key: string]: string } = {}

export function readContractsConfig() {
  const configPath = resolve('..', inFile)

  const data = readFileSync(configPath, 'utf8')
  const parsed_data = JSON.parse(data)
  const contracts = parsed_data.contracts

  for (const key of Object.keys(contracts)) {
    contractsAddresses[key] = contracts[key].address
  }
}

function getContractAddress(contractName: string): string {
  if (Object.keys(contractsAddresses).length === 0) {
    throw new Error("the 'readContractsConfig()' function must be called")
  }

  if (contractsAddresses[contractName] === undefined) {
    throw new Error(`contract '${contractName}' not found`)
  }

  return contractsAddresses[contractName]
}

export const config: Config = {
  get ethereumDidRegistry(): EthereumDidRegistryConfig {
    return {
      name: 'EthereumExtDidRegistry',
      address: getContractAddress('ethereum_did_registry'),
      description: 'Ethereum registry for ERC-1056 ethr did methods',
    }
  },

  get accountControl(): AccountControlConfig {
    return {
      name: 'AccountControl',
      address: getContractAddress('account_control'),
      description: 'Account permissioning smart contract',
      data: {
        roleControlContractAddress: getContractAddress('role_control'),
        upgradeControlAddress: getContractAddress('upgrade_control'),
      },
    }
  },

  get credentialDefinitionRegistry(): CredentialDefinitionsConfig {
    return {
      name: 'CredentialDefinitionRegistry',
      address: getContractAddress('cred_def_registry'),
      description: 'Smart contract to manage credential definitions',
      data: {
        credentialDefinitions: [],
        ethereumDidRegistry: getContractAddress('ethereum_did_registry'),
        schemaRegistryAddress: getContractAddress('schema_registry'),
        upgradeControlAddress: getContractAddress('upgrade_control'),
      },
    }
  },

  get indyDidValidator(): IndyDidValidatorConfig {
    return {
      name: 'IndyDidValidator',
      address: getContractAddress('did_validator'),
      description: 'Library to validate DID',
    }
  },

  get indyDidRegistry(): IndyDidRegistryConfig {
    return {
      name: 'IndyDidRegistry',
      address: getContractAddress('did_registry'),
      description: 'Smart contract to manage DIDs',
      libraries: { 'contracts/did/IndyDidValidator.sol:IndyDidValidator': getContractAddress('did_validator') },
      data: {
        dids: [],
        upgradeControlAddress: getContractAddress('upgrade_control'),
      },
    }
  },

  get ethereumDidRegistry(): EthereumDidRegistryConfig {
    return {
      name: 'EthereumExtDidRegistry',
      address: getContractAddress('ethereum_did_registry'),
      description: 'Ethereum registry for ERC-1056 ethr did methods',
    }
  },

  get roleControl(): RolesConfig {
    return {
      name: 'RoleControl',
      address: getContractAddress('role_control'),
      description: 'Smart contract to manage account roles',
      data: {
        accounts: [
          {
            account: '0xfe3b557e8fb62b89f4916b721be55ceb828dbd73',
            role: 1,
          },
          {
            account: '0x627306090abaB3A6e1400e9345bC60c78a8BEf57',
            role: 1,
          },
          {
            account: '0xf17f52151EbEF6C7334FAD080c5704D77216b732',
            role: 1,
          },
          {
            account: '0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5',
            role: 1,
          },
          {
            account: '0xca843569e3427144cead5e4d5999a3d0ccf92b8e',
            role: 1,
          },
        ],
        roleOwners: {
          '1': '1',
          '2': '1',
          '3': '1',
        },
      },
    }
  },

  get schemaRegistry(): SchemasConfig {
    return {
      name: 'SchemaRegistry',
      address: getContractAddress('schema_registry'),
      description: 'Smart contract to manage schemas',
      data: {
        schemas: [],
        ethereumDidRegistry: getContractAddress('ethereum_did_registry'),
        upgradeControlAddress: getContractAddress('upgrade_control'),
      },
    }
  },

  get universalDidResolver(): UniversalDidResolverConfig {
    return {
      name: 'UniversalDidResolver',
      address: getContractAddress('universal_did_resolver'),
      description: 'Smart contract to resolve DIDs from various DID registries',
      data: {
        etheriumDidRegistryAddress: getContractAddress('ethereum_did_registry'),
        didRegistryAddress: getContractAddress('did_registry'),
        upgradeControlAddress: getContractAddress('upgrade_control'),
      },
    }
  },

  get upgradeControl(): UpgradeControlConfig {
    return {
      name: 'UpgradeControl',
      address: getContractAddress('upgrade_control'),
      description: 'Smart contract to manage proxy contract upgrades',
      data: {
        roleControlContractAddress: getContractAddress('role_control'),
      },
    }
  },

  get validatorControl(): ValidatorsConfig {
    return {
      name: 'ValidatorControl',
      address: getContractAddress('validator_control'),
      description: 'Smart contract to manage validator nodes',
      data: {
        validators: [
          {
            account: '0xed9d02e382b34818e88b88a309c7fe71e65f419d',
            validator: '0x93917cadbace5dfce132b991732c6cda9bcc5b8a',
          },
          {
            account: '0xb30f304642de3fee4365ed5cd06ea2e69d3fd0ca',
            validator: '0x27a97c9aaf04f18f3014c32e036dd0ac76da5f18',
          },
          {
            account: '0x0886328869e4e1f401e1052a5f4aae8b45f42610',
            validator: '0xce412f988377e31f4d0ff12d74df73b51c42d0ca',
          },
          {
            account: '0xf48de4a0c2939e62891f3c6aca68982975477e45',
            validator: '0x98c1334496614aed49d2e81526d089f7264fed9c',
          },
        ],
        roleControlContractAddress: getContractAddress('role_control'),
        upgradeControlAddress: getContractAddress('upgrade_control'),
      },
    }
  },

  get legacyMapping(): LegacyMappingRegistryConfig {
    return {
      name: 'LegacyMappingRegistry',
      address: getContractAddress('legacy_mapping_registry'),
      description: 'Smart contract to store mapping of legacy identifiers to new one',
      data: {
        ethereumDidRegistry: contractsAddresses.ethereumDidRegistry,
        upgradeControlAddress: contractsAddresses.upgradeControl,
      },
    }
  },
}
