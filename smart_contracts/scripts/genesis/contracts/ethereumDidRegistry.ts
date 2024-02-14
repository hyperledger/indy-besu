import { padLeft } from 'web3-utils'
import { config } from '../config'
import { ContractConfig } from '../contractConfig'
import { buildProxySection, slots } from '../helpers'

export interface EthereumDidRegistryConfig extends ContractConfig {
  data: {
    upgradeControlAddress: string
  }
}

export function ethereumDidRegistry() {
  const { name, address, description, data } = config.ethereumDidRegistry
  const storage: any = {}

  storage[slots['0']] = padLeft(data.upgradeControlAddress, 64)
  return buildProxySection(name, address, description, storage)
}
