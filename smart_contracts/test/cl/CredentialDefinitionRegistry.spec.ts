import { expect } from 'chai'
import { keccak256, toUtf8Bytes } from 'ethers'
import { EthereumExtDidRegistry } from '../../contracts-ts'
import { createCredentialDefinitionObject } from '../../utils'
import {
  createDid,
  createSchema,
  createSchemaSigned,
  deployCredentialDefinitionRegistry,
  signCredDefEndorsementData,
  TestableCredentialDefinitionRegistry,
  TestableSchemaRegistry,
  testActorAddress,
  testActorPrivateKey,
} from '../utils/contract-helpers'
import { ClErrors } from '../utils/errors'
import { TestAccounts } from '../utils/test-entities'

describe('CredentialDefinitionRegistry', function () {
  let didRegistry: EthereumExtDidRegistry
  let schemaRegistry: TestableSchemaRegistry
  let credentialDefinitionRegistry: TestableCredentialDefinitionRegistry
  let testAccounts: TestAccounts
  let schemaId: string
  let issuer: string

  beforeEach(async function () {
    const {
      didRegistry: didRegistryInit,
      schemaRegistry: schemaRegistryInit,
      credentialDefinitionRegistry: credentialDefinitionRegistryInit,
      testAccounts: testAccountsInit,
    } = await deployCredentialDefinitionRegistry()

    issuer = testAccountsInit.trustee.account.address
    didRegistryInit.connect(testAccountsInit.trustee.account)
    schemaRegistryInit.connect(testAccountsInit.trustee.account)
    credentialDefinitionRegistryInit.connect(testAccountsInit.trustee.account)

    const issuerId = `did:ethr:mainnet:${issuer}`
    await createDid(didRegistryInit, testAccountsInit.trustee.account.address, issuerId)

    const { id } = await createSchema(schemaRegistryInit, issuer)

    didRegistry = didRegistryInit
    testAccounts = testAccountsInit
    schemaRegistry = schemaRegistryInit
    credentialDefinitionRegistry = credentialDefinitionRegistryInit
    schemaId = id
  })

  describe('Add/Resolve Credential Definition', function () {
    it('Should create Credential Definition', async function () {
      const { id, credDef } = createCredentialDefinitionObject({ issuer, schemaId })

      await credentialDefinitionRegistry.createCredentialDefinition(issuer, id, schemaId, credDef)

      const created = await credentialDefinitionRegistry.created(id)
      expect(created).to.be.not.equal(0)
    })

    it('Should return zero created block a non-existing credential definition', async function () {
      const { id } = createCredentialDefinitionObject({ issuer, schemaId })

      const created = await credentialDefinitionRegistry.created(id)
      expect(created).to.be.equal(0)
    })

    it('Should fail if Credential Definition is being already exists', async function () {
      const { id, credDef } = createCredentialDefinitionObject({ issuer, schemaId })

      await credentialDefinitionRegistry.createCredentialDefinition(issuer, id, schemaId, credDef)

      await expect(credentialDefinitionRegistry.createCredentialDefinition(issuer, id, schemaId, credDef))
        .to.be.revertedWithCustomError(
          credentialDefinitionRegistry.baseInstance,
          ClErrors.CredentialDefinitionAlreadyExist,
        )
        .withArgs(keccak256(toUtf8Bytes(id)))
    })

    it('Should fail if Credential Definition is being created with non-existing Schema', async function () {
      const unknownSchemaId = 'did:indy2:mainnet:SEp33q43PsdP7nDATyySSH/anoncreds/v0/SCHEMA/Test/1.0.0'
      const { id, credDef } = createCredentialDefinitionObject({ issuer, schemaId: unknownSchemaId })

      await expect(credentialDefinitionRegistry.createCredentialDefinition(issuer, id, unknownSchemaId, credDef))
        .to.be.revertedWithCustomError(credentialDefinitionRegistry.baseInstance, ClErrors.SchemaNotFound)
        .withArgs(keccak256(toUtf8Bytes(unknownSchemaId)))
    })

    it('Should fail if Credential Definition is being created with not owned Issuer DID', async function () {
      const issuerId2 = 'did:indy2:mainnet:SEp33q43PsdP7nDATyyDDA'
      const { id, credDef } = createCredentialDefinitionObject({ issuer, schemaId })

      await expect(
        credentialDefinitionRegistry.createCredentialDefinition(
          testAccounts.trustee2.account.address,
          id,
          schemaId,
          credDef,
        ),
      )
        .to.be.revertedWithCustomError(credentialDefinitionRegistry.baseInstance, ClErrors.UnauthorizedIssuer)
        .withArgs(testAccounts.trustee2.account.address, testAccounts.trustee.account.address)
    })
  })

  describe('Endorse/Resolve Credential Definition with did:ethr Issuer', function () {
    it('Should endorse and resolve Credential Definition', async function () {
      const { id: ethSchemaId } = await createSchemaSigned(schemaRegistry, testActorAddress)

      const { id, credDef } = createCredentialDefinitionObject({ issuer: testActorAddress, schemaId: ethSchemaId })
      const signature = await signCredDefEndorsementData(
        credentialDefinitionRegistry,
        testActorAddress,
        testActorPrivateKey,
        id,
        ethSchemaId,
        credDef,
      )

      await credentialDefinitionRegistry.createCredentialDefinitionSigned(
        testActorAddress,
        id,
        ethSchemaId,
        credDef,
        signature,
      )

      const created = await credentialDefinitionRegistry.created(id)
      expect(created).to.be.not.equal(0)
    })

    it('Should fail if Credential Definition is being endorsed with not owned Issuer DID', async function () {
      const { id: ethSchemaId } = await createSchemaSigned(schemaRegistry, testActorAddress)
      const { id, credDef } = createCredentialDefinitionObject({ issuer: testActorAddress, schemaId: ethSchemaId })

      const signature = await signCredDefEndorsementData(
        credentialDefinitionRegistry,
        testAccounts.trustee2.account.address,
        testActorPrivateKey,
        id,
        ethSchemaId,
        credDef,
      )
      await expect(
        credentialDefinitionRegistry.createCredentialDefinitionSigned(
          testAccounts.trustee2.account.address,
          id,
          ethSchemaId,
          credDef,
          signature,
        ),
      )
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.UnauthorizedIssuer)
        .withArgs(testAccounts.trustee2.account.address, testActorAddress)
    })

    it('Should fail if Schema is being endorsed with invalid signature', async function () {
      const { id: ethSchemaId } = await createSchemaSigned(schemaRegistry, testActorAddress)
      const { id, credDef } = createCredentialDefinitionObject({ issuer: testActorAddress, schemaId: ethSchemaId })

      const signature = await signCredDefEndorsementData(
        credentialDefinitionRegistry,
        testActorAddress,
        testActorPrivateKey,
        'different id passed into signature',
        ethSchemaId,
        credDef,
      )
      await expect(
        credentialDefinitionRegistry.createCredentialDefinitionSigned(
          testActorAddress,
          id,
          schemaId,
          credDef,
          signature,
        ),
      ).to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.UnauthorizedIssuer)
    })
  })
})
