import { expect } from 'chai'
import { IndyDidRegistry } from '../../contracts-ts'
import { createCredentialDefinitionObject } from '../../utils'
import {
  createDid,
  createSchema,
  deployCredentialDefinitionRegistry,
  TestableCredentialDefinitionRegistry,
  TestableSchemaRegistry,
} from '../utils/contract-helpers'
import { ClErrors } from '../utils/errors'
import { TestAccounts } from '../utils/test-entities'

describe('CredentialDefinitionRegistry', function () {
  let didRegistry: IndyDidRegistry
  let schemaRegistry: TestableSchemaRegistry
  let credentialDefinitionRegistry: TestableCredentialDefinitionRegistry
  let testAccounts: TestAccounts
  let schemaId: string
  const issuerId = 'did:indy2:mainnet:SEp33q43PsdP7nDATyySSH'

  beforeEach(async function () {
    const {
      indyDidRegistry: didRegistryInit,
      schemaRegistry: schemaRegistryInit,
      credentialDefinitionRegistry: credentialDefinitionRegistryInit,
      testAccounts: testAccountsInit,
    } = await deployCredentialDefinitionRegistry()

    didRegistryInit.connect(testAccountsInit.trustee.account)
    schemaRegistryInit.connect(testAccountsInit.trustee.account)
    credentialDefinitionRegistryInit.connect(testAccountsInit.trustee.account)
    await createDid(didRegistryInit, testAccountsInit.trustee.account.address, issuerId)
    const { id } = await createSchema(schemaRegistryInit, issuerId)

    didRegistry = didRegistryInit
    testAccounts = testAccountsInit
    schemaRegistry = schemaRegistryInit
    credentialDefinitionRegistry = credentialDefinitionRegistryInit
    schemaId = id
  })

  describe('Add/Resolve Credential Definition', function () {
    it('Should create and resolve Credential Definition', async function () {
      const { id, credDef } = createCredentialDefinitionObject({ issuerId, schemaId })

      await credentialDefinitionRegistry.createCredentialDefinition(id, issuerId, schemaId, credDef)
      const result = await credentialDefinitionRegistry.resolveCredentialDefinition(id)

      expect(result.credDef).to.be.deep.equal(credDef)
    })

    it('Should fail if resolving Credential Definition does not exist', async function () {
      const { id } = createCredentialDefinitionObject({ issuerId, schemaId })

      await expect(credentialDefinitionRegistry.resolveCredentialDefinition(id))
        .to.be.revertedWithCustomError(credentialDefinitionRegistry.baseInstance, ClErrors.CredentialDefinitionNotFound)
        .withArgs(id)
    })

    it('Should fail if Credential Definition is being already exists', async function () {
      const { id, credDef } = createCredentialDefinitionObject({ issuerId, schemaId })

      await credentialDefinitionRegistry.createCredentialDefinition(id, issuerId, schemaId, credDef)

      await expect(credentialDefinitionRegistry.createCredentialDefinition(id, issuerId, schemaId, credDef))
        .to.be.revertedWithCustomError(
          credentialDefinitionRegistry.baseInstance,
          ClErrors.CredentialDefinitionAlreadyExist,
        )
        .withArgs(id)
    })

    it('Should fail if Credential Definition is being created with non-existing Issuer', async function () {
      const unknownIssuerId = 'did:indy2:mainnet:GEzcdDLhCpGCYRHW82kjHd'
      const { id, credDef } = createCredentialDefinitionObject({ issuerId: unknownIssuerId, schemaId })

      await expect(credentialDefinitionRegistry.createCredentialDefinition(id, unknownIssuerId, schemaId, credDef))
        .to.be.revertedWithCustomError(credentialDefinitionRegistry.baseInstance, ClErrors.IssuerNotFound)
        .withArgs(unknownIssuerId)
    })

    it('Should fail if Credential Definition is being created with inactive Issuer', async function () {
      didRegistry.deactivateDid(issuerId)

      const { id, credDef } = createCredentialDefinitionObject({ issuerId, schemaId })

      await expect(credentialDefinitionRegistry.createCredentialDefinition(id, issuerId, schemaId, credDef))
        .to.be.revertedWithCustomError(credentialDefinitionRegistry.baseInstance, ClErrors.IssuerHasBeenDeactivated)
        .withArgs(issuerId)
    })

    it('Should fail if Credential Definition is being created with non-existing Schema', async function () {
      const unknownSchemaId = 'did:indy2:mainnet:SEp33q43PsdP7nDATyySSH/anoncreds/v0/SCHEMA/Test/1.0.0'
      const { id, credDef } = createCredentialDefinitionObject({ issuerId, schemaId: unknownSchemaId })

      await expect(credentialDefinitionRegistry.createCredentialDefinition(id, issuerId, unknownSchemaId, credDef))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.SchemaNotFound)
        .withArgs(unknownSchemaId)
    })

    it('Should fail if Credential Definition is being created with not owned Issuer DID', async function () {
      const issuerId2 = 'did:indy2:mainnet:SEp33q43PsdP7nDATyyDDA'
      const { id, credDef } = createCredentialDefinitionObject({ issuerId, schemaId })

      didRegistry.connect(testAccounts.trustee2.account)
      credentialDefinitionRegistry.connect(testAccounts.trustee2.account)

      await createDid(didRegistry, testAccounts.trustee2.account.address, issuerId2)
      await expect(credentialDefinitionRegistry.createCredentialDefinition(id, issuerId, schemaId, credDef))
        .to.be.revertedWithCustomError(credentialDefinitionRegistry.baseInstance, ClErrors.UnauthorizedSender)
        .withArgs(testAccounts.trustee2.account.address)
    })

    it('Should fail if Credential Definition is being with invalid ID', async function () {
      const { credDef } = createCredentialDefinitionObject({ issuerId, schemaId })
      const id = 'Gs6cQcvrtWoZKsbBhD3dQJ:3:CL:140384:mctc'

      await expect(credentialDefinitionRegistry.createCredentialDefinition(id, issuerId, schemaId, credDef))
        .to.be.revertedWithCustomError(
          credentialDefinitionRegistry.baseInstance,
          ClErrors.InvalidCredentialDefinitionId,
        )
        .withArgs(id)
    })
  })

  describe('Add/Resolve Credential Definition with did:ethr Issuer', function () {
    it('Should create and resolve Credential Definition', async function () {
      const ethrIssuerId = `did:ethr:${testAccounts.trustee.account.address}`
      const { id, credDef } = createCredentialDefinitionObject({ issuerId: ethrIssuerId, schemaId })

      await credentialDefinitionRegistry.createCredentialDefinition(id, ethrIssuerId, schemaId, credDef)
      const result = await credentialDefinitionRegistry.resolveCredentialDefinition(id)

      expect(result.credDef).to.be.deep.equal(credDef)
    })

    it('Should fail if Credential Definition is being created with not owned Issuer DID', async function () {
      const ethrIssuerId = `did:ethr:${testAccounts.trustee2.account.address}`
      const { id, credDef } = createCredentialDefinitionObject({ issuerId: ethrIssuerId, schemaId })

      await expect(credentialDefinitionRegistry.createCredentialDefinition(id, ethrIssuerId, schemaId, credDef))
        .to.be.revertedWithCustomError(credentialDefinitionRegistry.baseInstance, ClErrors.UnauthorizedSender)
        .withArgs(testAccounts.trustee.account.address)
    })

    it('Should fail if Credential Definition is being created with invalid Issuer ID', async function () {
      const invalidIssuerId = 'did:ethr:ab$ddfgh354345'
      const { id, credDef } = createCredentialDefinitionObject({ issuerId: invalidIssuerId, schemaId })

      await expect(credentialDefinitionRegistry.createCredentialDefinition(id, invalidIssuerId, schemaId, credDef))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.InvalidIssuerId)
        .withArgs(invalidIssuerId)
    })
  })
})
