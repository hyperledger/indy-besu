/**
 * Copyright (c) 2024 DSR Corporation, Denver, Colorado.
 * https://www.dsr-corporation.com
 * SPDX-License-Identifier: Apache-2.0
 */

import { getBytes } from 'ethers'
import {
  CredentialDefinitionRegistry,
  EthereumExtDidRegistry,
  IndyDidRegistry,
  LegacyMappingRegistry,
  RoleControl,
  SchemaRegistry,
  UniversalDidResolver,
  UpgradeControl,
  ValidatorControl,
} from '../../contracts-ts'
import { Contract, createBaseDidDocument, createSchemaObject } from '../../utils'
import { getTestAccounts, ZERO_ADDRESS } from './test-entities'

export const testActorAddress = '0x2036C6CD85692F0Fb2C26E6c6B2ECed9e4478Dfd'
export const testActorPrivateKey = getBytes('0xa285ab66393c5fdda46d6fbad9e27fafd438254ab72ad5acb681a0e9f20f5d7b')

export class TestableEthereumDIDRegistry extends testableContractMixin(Contract) {
  constructor() {
    super(EthereumExtDidRegistry.name)
  }
}

export class UpgradablePrototype extends testableContractMixin(Contract) {
  public get version(): Promise<string> {
    return this.instance.getVersion()
  }
}

export class TestableIndyDidRegistry extends testableContractMixin(IndyDidRegistry) {}

export class TestableSchemaRegistry extends testableContractMixin(SchemaRegistry) {}

export class TestableCredentialDefinitionRegistry extends testableContractMixin(CredentialDefinitionRegistry) {}

export class TestableRoleControl extends testableContractMixin(RoleControl) {}

export class TestableValidatorControl extends testableContractMixin(ValidatorControl) {}

export class TestableUpgradeControl extends testableContractMixin(UpgradeControl) {}

export class TestableUniversalDidResolver extends testableContractMixin(UniversalDidResolver) {}

export class TestableLegacyMappingRegistry extends testableContractMixin(LegacyMappingRegistry) {}

export async function deployRoleControl() {
  const roleControl = await new TestableRoleControl().deployProxy({ params: [ZERO_ADDRESS] })
  const testAccounts = await getTestAccounts(roleControl)

  return { roleControl, testAccounts }
}

export async function deployIndyDidRegistry() {
  const { roleControl, testAccounts } = await deployRoleControl()

  const indyDidRegistry = await new TestableIndyDidRegistry().deployProxy({
    params: [ZERO_ADDRESS, roleControl.address],
  })

  return { roleControl, indyDidRegistry, testAccounts }
}

export async function deployUniversalDidResolver() {
  const { roleControl, indyDidRegistry, testAccounts } = await deployIndyDidRegistry()
  const ethereumDIDRegistry = await new TestableEthereumDIDRegistry().deploy()

  const universalDidResolver = await new TestableUniversalDidResolver().deployProxy({
    params: [ZERO_ADDRESS, indyDidRegistry.address, ethereumDIDRegistry.address],
  })

  return { roleControl, universalDidResolver, ethereumDIDRegistry, indyDidRegistry, testAccounts }
}

export async function deploySchemaRegistry() {
  const { roleControl, universalDidResolver, indyDidRegistry, testAccounts } = await deployUniversalDidResolver()
  const schemaRegistry = await new TestableSchemaRegistry().deployProxy({
    params: [ZERO_ADDRESS, universalDidResolver.address, roleControl.address],
  })

  return { roleControl, universalDidResolver, indyDidRegistry, schemaRegistry, testAccounts }
}

export async function deployCredentialDefinitionRegistry() {
  const { roleControl, universalDidResolver, indyDidRegistry, schemaRegistry, testAccounts } =
    await deploySchemaRegistry()
  const credentialDefinitionRegistry = await new TestableCredentialDefinitionRegistry().deployProxy({
    params: [ZERO_ADDRESS, universalDidResolver.address, schemaRegistry.address, roleControl.address],
  })

  return {
    roleControl,
    credentialDefinitionRegistry,
    universalDidResolver,
    indyDidRegistry,
    schemaRegistry,
    testAccounts,
  }
}

export async function deployLegacyMappingRegistry() {
  const { roleControl, universalDidResolver, indyDidRegistry, testAccounts } = await deployUniversalDidResolver()
  const legacyMappingRegistry = await new TestableLegacyMappingRegistry().deployProxy({
    params: [ZERO_ADDRESS, universalDidResolver.address, roleControl.address],
  })

  return { roleControl, universalDidResolver, indyDidRegistry, legacyMappingRegistry, testAccounts }
}

export async function createDid(didRegistry: IndyDidRegistry, identity: string, did: string) {
  const didDocument = createBaseDidDocument(did)
  await didRegistry.createDid(identity, didDocument)
  return didDocument
}

export async function createDidSigned(didRegistry: IndyDidRegistry, identity: string, did: string) {
  const didDocument = createBaseDidDocument(did)
  const sig = await didRegistry.signCreateDidEndorsementData(identity, testActorPrivateKey, didDocument)
  await didRegistry.createDidSigned(identity, didDocument, sig)
}

export async function createSchema(schemaRegistry: SchemaRegistry, identity: string, issuerId: string) {
  const { id, schema } = createSchemaObject({ issuerId })
  await schemaRegistry.createSchema(identity, id, issuerId, schema)
  return { id, schema }
}

export async function createSchemaSigned(schemaRegistry: SchemaRegistry, identity: string, issuerId: string) {
  const { id, schema } = createSchemaObject({ issuerId })
  const signature = await schemaRegistry.signCreateSchemaEndorsementData(
    identity,
    testActorPrivateKey,
    id,
    issuerId,
    schema,
  )
  await schemaRegistry.createSchemaSigned(identity, id, issuerId, schema, signature)
  return { id, schema }
}

function testableContractMixin<T extends new (...args: any[]) => Contract>(Base: T) {
  return class extends Base {
    public get baseInstance() {
      return this.instance
    }
  }
}
