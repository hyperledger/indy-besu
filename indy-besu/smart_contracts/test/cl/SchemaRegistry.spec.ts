import { expect } from 'chai'
import { IndyDidRegistry, SchemaRegistry } from '../../contracts-ts'
import { createSchemaObject } from '../../utils'
import { createDid, deploySchemaRegistry, TestableSchemaRegistry } from '../utils/contract-helpers'
import { ClErrors } from '../utils/errors'
import { TestAccounts } from '../utils/test-entities'

describe('SchemaRegistry', function () {
  let didRegistry: IndyDidRegistry
  let schemaRegistry: TestableSchemaRegistry
  let testAccounts: TestAccounts
  const issuerId = 'did:indy2:mainnet:SEp33q43PsdP7nDATyySSH'

  beforeEach(async function () {
    const {
      indyDidRegistry: didRegistryInit,
      schemaRegistry: schemaRegistryInit,
      testAccounts: testAccountsInit,
    } = await deploySchemaRegistry()

    didRegistryInit.connect(testAccountsInit.trustee.account)
    schemaRegistryInit.connect(testAccountsInit.trustee.account)
    await createDid(didRegistryInit, testAccountsInit.trustee.account.address, issuerId)

    didRegistry = didRegistryInit
    testAccounts = testAccountsInit
    schemaRegistry = schemaRegistryInit
  })

  describe('Add/Resolve Schema', function () {
    it('Should create and resolve Schema', async function () {
      const { id, schema } = createSchemaObject({ issuerId })

      await schemaRegistry.createSchema(id, issuerId, schema)
      const result = await schemaRegistry.resolveSchema(id)

      expect(result.schema).to.be.deep.equal(schema)
    })

    it('Should fail if resolving a non-existing schema', async function () {
      const { id, schema } = createSchemaObject({ issuerId })

      await expect(schemaRegistry.resolveSchema(id))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.SchemaNotFound)
        .withArgs(id)
    })

    it('Should fail if Schema is being created already exists', async function () {
      const { id, schema } = createSchemaObject({ issuerId })

      await schemaRegistry.createSchema(id, issuerId, schema)

      await expect(schemaRegistry.createSchema(id, issuerId, schema))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.SchemaAlreadyExist)
        .withArgs(id)
    })

    it('Should fail if Schema is being created with non-existing Issuer', async function () {
      const unknownIssuerId = 'did:indy2:mainnet:GEzcdDLhCpGCYRHW82kjHd'
      const { id, schema } = createSchemaObject({ issuerId: unknownIssuerId })

      await expect(schemaRegistry.createSchema(id, unknownIssuerId, schema))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.IssuerNotFound)
        .withArgs(unknownIssuerId)
    })

    it('Should fail if Schema is being created with inactive Issuer', async function () {
      didRegistry.deactivateDid(issuerId)

      const { id, schema } = createSchemaObject({ issuerId })

      await expect(schemaRegistry.createSchema(id, issuerId, schema))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.IssuerHasBeenDeactivated)
        .withArgs(issuerId)
    })

    it('Should fail if Schema is being created with invalid Schema ID', async function () {
      const { schema } = createSchemaObject({ issuerId })
      const id = 'SEp33q43PsdP7nDATyySSH:2:BasicSchema:1.0.0'

      await expect(schemaRegistry.createSchema(id, issuerId, schema))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.InvalidSchemaId)
        .withArgs(id)
    })

    it('Should fail if Schema is being created with not owned Issuer DID', async function () {
      const issuerId2 = 'did:indy2:mainnet:SEp33q43PsdP7nDATyyDDA'
      const { id, schema } = createSchemaObject({ issuerId })

      didRegistry.connect(testAccounts.trustee2.account)
      schemaRegistry.connect(testAccounts.trustee2.account)

      await createDid(didRegistry, testAccounts.trustee2.account.address, issuerId2)
      await expect(schemaRegistry.createSchema(id, issuerId, schema))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.UnauthorizedIssuer)
        .withArgs(testAccounts.trustee2.account.address)
    })
  })

  describe('Add/Resolve Schema with did:ethr Issuer', function () {
    it('Should create and resolve Schema', async function () {
      const ethrIssuerId = `did:ethr:${testAccounts.trustee.account.address}`

      const { id, schema } = createSchemaObject({ issuerId: ethrIssuerId })

      await schemaRegistry.createSchema(id, ethrIssuerId, schema)
      const result = await schemaRegistry.resolveSchema(id)

      expect(result.schema).to.be.deep.equal(schema)
    })

    it('Should fail if Schema is being created with not owned Issuer DID', async function () {
      const ethrIssuerId = `did:ethr:${testAccounts.trustee2.account.address}`

      const { id, schema } = createSchemaObject({ issuerId: ethrIssuerId })

      await expect(schemaRegistry.createSchema(id, ethrIssuerId, schema))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.UnauthorizedIssuer)
        .withArgs(testAccounts.trustee.account.address)
    })

    it('Should fail if Schema is being created with invalid Issuer ID', async function () {
      const invalidIssuerId = 'did:ethr:ab$ddfgh354345'
      const { id, schema } = createSchemaObject({ issuerId: invalidIssuerId })

      await expect(schemaRegistry.createSchema(id, invalidIssuerId, schema))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.InvalidIssuerId)
        .withArgs(invalidIssuerId)
    })
  })
})
