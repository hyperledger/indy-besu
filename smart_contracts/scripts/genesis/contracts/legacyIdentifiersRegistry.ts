import { padLeft } from 'web3-utils'
import { config } from '../config'
import { ContractConfig } from '../contractConfig'
import { buildProxySection, slots } from '../helpers'

export interface LegacyIdentifiersRegistryConfig extends ContractConfig {
  data: {
    ethereumDidRegistry: string
    upgradeControlAddress: string
  }
}

export function legacyIdentifiersRegistry() {
  const { name, address, description, data } = config.legacyIdentifiers
  const storage: any = {}

  // address of upgrade control contact stored in slot 0
  storage[slots['0']] = padLeft(data.upgradeControlAddress, 64)
  // address of DID resolver contact stored in slot 1
  storage[slots['1']] = padLeft(data.ethereumDidRegistry, 64)
  return buildProxySection(name, address, description, storage)
}
