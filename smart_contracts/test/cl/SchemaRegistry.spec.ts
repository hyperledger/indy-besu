import { expect } from 'chai'
import { keccak256, toUtf8Bytes } from 'ethers'
import { EthereumExtDidRegistry, SchemaRegistry } from '../../contracts-ts'
import { createSchemaObject } from '../../utils'
import {
  createDid,
  deploySchemaRegistry,
  signSchemaEndorsementData,
  TestableSchemaRegistry,
  testActorAddress,
  testActorPrivateKey,
} from '../utils/contract-helpers'
import { ClErrors } from '../utils/errors'
import { TestAccounts } from '../utils/test-entities'

describe('SchemaRegistry', function () {
  let didRegistry: EthereumExtDidRegistry
  let schemaRegistry: TestableSchemaRegistry
  let testAccounts: TestAccounts
  let issuer: string

  beforeEach(async function () {
    const {
      didRegistry: didRegistryInit,
      schemaRegistry: schemaRegistryInit,
      testAccounts: testAccountsInit,
    } = await deploySchemaRegistry()

    issuer = testAccountsInit.trustee.account.address
    didRegistryInit.connect(testAccountsInit.trustee.account)
    schemaRegistryInit.connect(testAccountsInit.trustee.account)

    const issuerId = `did:ethr:mainnet:${issuer}`
    await createDid(didRegistryInit, testAccountsInit.trustee.account.address, issuerId)

    didRegistry = didRegistryInit
    testAccounts = testAccountsInit
    schemaRegistry = schemaRegistryInit
  })

  describe('Add/Resolve Schema', function () {
    it('Should create Schema', async function () {
      const { id, schema } = createSchemaObject({ issuer })

      await schemaRegistry.createSchema(issuer, id, schema)

      const created = await schemaRegistry.created(id)
      expect(created).to.be.not.equal(0)
    })

    it('Should return zero created block a non-existing schema', async function () {
      const { id } = createSchemaObject({ issuer })

      const created = await schemaRegistry.created(id)
      expect(created).to.be.equal(0)
    })

    it('Should fail if Schema is being created already exists', async function () {
      const { id, schema } = createSchemaObject({ issuer })

      await schemaRegistry.createSchema(issuer, id, schema)

      await expect(schemaRegistry.createSchema(issuer, id, schema))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.SchemaAlreadyExist)
        .withArgs(keccak256(toUtf8Bytes(id)))
    })

    it('Should fail if Schema is being created with not owned Issuer DID', async function () {
      const { id, schema } = createSchemaObject({ issuer })

      await expect(schemaRegistry.createSchema(testAccounts.trustee2.account.address, id, schema))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.UnauthorizedIssuer)
        .withArgs(testAccounts.trustee2.account.address, testAccounts.trustee.account.address)
    })
  })

  describe('Endorse/Resolve Schema with did:ethr Issuer', function () {
    it('Should endorse Schema', async function () {
      const { id, schema } = createSchemaObject({ issuer: testActorAddress })

      const sig = await signSchemaEndorsementData(schemaRegistry, testActorAddress, testActorPrivateKey, id, schema)

      await schemaRegistry.createSchemaSigned(testActorAddress, id, schema, sig)
      const created = await schemaRegistry.created(id)
      expect(created).to.be.not.equal(0)
    })

    it('Should fail if Schema is being endorsed with not owned Issuer DID', async function () {
      const { id, schema } = createSchemaObject({ issuer: testAccounts.trustee2.account.address })

      const sig = await signSchemaEndorsementData(
        schemaRegistry,
        testAccounts.trustee2.account.address,
        testActorPrivateKey,
        id,
        schema,
      )
      await expect(schemaRegistry.createSchemaSigned(testAccounts.trustee2.account.address, id, schema, sig))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, ClErrors.UnauthorizedIssuer)
        .withArgs(testAccounts.trustee2.account.address, testActorAddress)
    })

    it('Should fail if Schema is being endorsed with invalid signature', async function () {
      const { id, schema } = createSchemaObject({ issuer: testActorAddress })

      const sig = await signSchemaEndorsementData(
        schemaRegistry,
        testActorAddress,
        testActorPrivateKey,
        'different id passed into signature',
        schema,
      )
      await expect(schemaRegistry.createSchemaSigned(testActorAddress, id, schema, sig)).to.be.revertedWithCustomError(
        schemaRegistry.baseInstance,
        ClErrors.UnauthorizedIssuer,
      )
    })
  })
})
