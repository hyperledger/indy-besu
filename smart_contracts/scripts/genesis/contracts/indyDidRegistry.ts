import { padLeft } from 'web3-utils'
import { ContractConfig } from '../contractConfig'
import { buildProxySection, slots } from '../helpers'

export interface IndyDidRegistryConfig extends ContractConfig {
  data: {
    upgradeControlAddress: string
  }
}

export function indyDidRegistry(config: IndyDidRegistryConfig) {
  const { name, address, description, data } = config
  const storage: any = {}

  storage[slots['0']] = padLeft(data.upgradeControlAddress, 64)
  return buildProxySection(name, address, description, storage)
}
