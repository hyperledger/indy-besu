import { padLeft } from 'web3-utils'
import { ContractConfig } from '../contractConfig'
import { buildProxySection, slots } from '../helpers'

export interface RevocationConfig extends ContractConfig {
  data: {
    credDefRegistryAddress: string
    upgradeControlAddress: string
    roleControlContractAddress: string
  }
}

export function revocationRegistry(config: RevocationConfig) {
  const { name, address, description, data } = config
  const storage: any = {}

  // address of upgrade control contact stored in slot 0
  storage[slots['0']] = padLeft(data.upgradeControlAddress, 64)
  // address of Credential Definition registry contact stored in slot 2
  storage[slots['1']] = padLeft(data.credDefRegistryAddress, 64)
  // address of Role control contact stored in slot 3
  storage[slots['2']] = padLeft(data.roleControlContractAddress, 64)

  return buildProxySection(name, address, description, storage)
}
