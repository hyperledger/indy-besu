import bs58 from 'bs58'
import { concat, getAddress, getBytes, keccak256, Signature, SigningKey, toUtf8Bytes } from 'ethers'
import {
  CredentialDefinitionRegistry,
  EthereumExtDidRegistry,
  RoleControl,
  SchemaRegistry,
  UpgradeControl,
  ValidatorControl,
} from '../../contracts-ts'
import { LegacyMappingRegistry } from '../../contracts-ts/LegacyMappingRegistry'
import { Contract, createSchemaObject } from '../../utils'
import { getTestAccounts, ZERO_ADDRESS } from './test-entities'

export const testActorAddress = '0x2036C6CD85692F0Fb2C26E6c6B2ECed9e4478Dfd'
export const testActorPrivateKey = getBytes('0xa285ab66393c5fdda46d6fbad9e27fafd438254ab72ad5acb681a0e9f20f5d7b')

export class UpgradablePrototype extends testableContractMixin(Contract) {
  public get version(): Promise<string> {
    return this.instance.getVersion()
  }
}

export class TestableDidRegistry extends testableContractMixin(EthereumExtDidRegistry) {}

export class TestableSchemaRegistry extends testableContractMixin(SchemaRegistry) {}

export class TestableCredentialDefinitionRegistry extends testableContractMixin(CredentialDefinitionRegistry) {}

export class TestableRoleControl extends testableContractMixin(RoleControl) {}

export class TestableValidatorControl extends testableContractMixin(ValidatorControl) {}

export class TestableUpgradeControl extends testableContractMixin(UpgradeControl) {}

export class TestableLegacyMappingRegistry extends testableContractMixin(LegacyMappingRegistry) {}

function testableContractMixin<T extends new (...args: any[]) => Contract>(Base: T) {
  return class extends Base {
    public get baseInstance() {
      return this.instance
    }
  }
}

export async function deployRoleControl() {
  const roleControl = await new RoleControl().deployProxy({ params: [ZERO_ADDRESS] })
  const testAccounts = await getTestAccounts(roleControl)

  return { roleControl, testAccounts }
}

export async function deployDidRegistry() {
  const { testAccounts } = await deployRoleControl()

  const didRegistry = await new TestableDidRegistry().deployProxy({
    params: [ZERO_ADDRESS],
  })

  return { didRegistry, testAccounts }
}

export async function deploySchemaRegistry() {
  const { didRegistry, testAccounts } = await deployDidRegistry()
  const schemaRegistry = await new TestableSchemaRegistry().deployProxy({
    params: [ZERO_ADDRESS, didRegistry.address],
  })

  return { didRegistry, schemaRegistry, testAccounts }
}

export async function deployCredentialDefinitionRegistry() {
  const { didRegistry, schemaRegistry, testAccounts } = await deploySchemaRegistry()
  const credentialDefinitionRegistry = await new TestableCredentialDefinitionRegistry().deployProxy({
    params: [ZERO_ADDRESS, didRegistry.address, schemaRegistry.address],
  })

  return { credentialDefinitionRegistry, didRegistry, schemaRegistry, testAccounts }
}

export async function deployLegacyMappingRegistry() {
  const { didRegistry, testAccounts } = await deployDidRegistry()
  const legacyMappingRegistry = await new TestableLegacyMappingRegistry().deployProxy({
    params: [ZERO_ADDRESS, didRegistry.address],
  })

  return { didRegistry, legacyMappingRegistry, testAccounts }
}

export async function createDid(didRegistry: EthereumExtDidRegistry, identity: string, did: string) {
  // DID assume to be created by default
  return did
}

export async function signEndorsementData(privateKey: Uint8Array, contract: string, data: string) {
  const dataToSign = concat(['0x1900', getAddress(contract), data])
  return new SigningKey(privateKey).sign(keccak256(dataToSign))
}

export async function signSchemaEndorsementData(
  schemaRegistry: SchemaRegistry,
  identity: string,
  privateKey: Uint8Array,
  id: string,
  schema: string,
) {
  return signEndorsementData(
    privateKey,
    schemaRegistry.address!,
    concat([identity, toUtf8Bytes('createSchema'), getBytes(keccak256(toUtf8Bytes(id)), 'hex'), toUtf8Bytes(schema)]),
  )
}

export async function createSchema(schemaRegistry: SchemaRegistry, issuer: string) {
  const { id, schema } = createSchemaObject({ issuer })
  await schemaRegistry.createSchema(issuer, id, schema)
  return { id, schema }
}

export async function createSchemaSigned(schemaRegistry: SchemaRegistry, issuer: string) {
  const { id, schema } = createSchemaObject({ issuer })
  const signature = await signSchemaEndorsementData(schemaRegistry, issuer, testActorPrivateKey, id, schema)
  await schemaRegistry.createSchemaSigned(issuer, id, schema, signature)
  return { id, schema }
}

export async function signCredDefEndorsementData(
  credentialDefinitionRegistry: CredentialDefinitionRegistry,
  identity: string,
  privateKey: Uint8Array,
  id: string,
  schemaId: string,
  credDef: string,
) {
  return signEndorsementData(
    privateKey,
    credentialDefinitionRegistry.address!,
    concat([
      identity,
      toUtf8Bytes('createCredentialDefinition'),
      getBytes(keccak256(toUtf8Bytes(id)), 'hex'),
      getBytes(keccak256(toUtf8Bytes(schemaId)), 'hex'),
      toUtf8Bytes(credDef),
    ]),
  )
}

export async function signDidMappingEndorsementData(
  legacyMappingRegistry: LegacyMappingRegistry,
  identity: string,
  privateKey: Uint8Array,
  identifier: string,
  ed25519Key: Uint8Array,
  ed25519Signature: Uint8Array,
) {
  return signEndorsementData(
    privateKey,
    legacyMappingRegistry.address!,
    concat([identity, toUtf8Bytes('createDidMapping'), toUtf8Bytes(identifier), ed25519Key, ed25519Signature]),
  )
}

export async function signResourceMappingEndorsementData(
  legacyMappingRegistry: LegacyMappingRegistry,
  identity: string,
  privateKey: Uint8Array,
  legacyIssuerIdentifier: string,
  legacyIdentifier: string,
  newIdentifier: string,
) {
  return signEndorsementData(
    privateKey,
    legacyMappingRegistry.address!,
    concat([
      identity,
      toUtf8Bytes('createResourceMapping'),
      toUtf8Bytes(legacyIssuerIdentifier),
      toUtf8Bytes(legacyIdentifier),
      toUtf8Bytes(newIdentifier),
    ]),
  )
}
