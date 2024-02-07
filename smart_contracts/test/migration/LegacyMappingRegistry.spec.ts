import { expect } from 'chai'
import { EthereumExtDidRegistry, LegacyMappingRegistry } from '../../contracts-ts'
import {
  createDid,
  deployLegacyMappingRegistry,
  signDidMappingEndorsementData,
  signResourceMappingEndorsementData,
  TestableLegacyMappingRegistry,
  testActorAddress,
  testActorPrivateKey,
} from '../utils/contract-helpers'
import { DidErrors, MigrationErrors } from '../utils/errors'
import { TestAccounts } from '../utils/test-entities'

describe('LegacyMappingRegistry', function () {
  let didRegistry: EthereumExtDidRegistry
  let legacyMappingRegistry: TestableLegacyMappingRegistry
  let testAccounts: TestAccounts
  let issuer: string

  const legacyDid = '2vZAi1riCVGnQMfQAjbThG'
  const legacyVerkey = Uint8Array.from([
    15, 147, 97, 223, 64, 179, 188, 70, 162, 110, 219, 163, 185, 25, 180, 23, 224, 175, 15, 188, 235, 170, 233, 240,
    145, 111, 204, 153, 108, 117, 188, 145,
  ])
  const legacySignature = Uint8Array.from([])
  const legacySchemaId = '2vZAi1riCVGnQMfQAjbThG:2:test_credential:1.0.0'
  const schemaId = 'did:ethr:0xcb799c9bca0d1ce7385726ccbd40b9fc4313e5b1/anoncreds/v0/SCHEMA/test_credential/1.0.0'

  beforeEach(async function () {
    const {
      didRegistry: didRegistryInit,
      legacyMappingRegistry: legacyMappingRegistryInit,
      testAccounts: testAccountsInit,
    } = await deployLegacyMappingRegistry()

    issuer = testAccountsInit.trustee.account.address
    legacyMappingRegistryInit.connect(testAccountsInit.trustee.account)

    const issuerId = `did:ethr:${issuer}`
    await createDid(didRegistryInit, testAccountsInit.trustee.account.address, issuerId)

    legacyMappingRegistry = legacyMappingRegistryInit
    didRegistry = didRegistryInit
    testAccounts = testAccountsInit
  })

  describe('Add/Resolve DID mapping', function () {
    it('Should create DID mapping', async function () {
      await legacyMappingRegistry.createDidMapping(issuer, legacyDid, legacyVerkey, legacySignature)

      const address = await legacyMappingRegistry.didMapping(legacyDid)
      expect(address).to.be.equal(issuer)
    })

    it('Should fail if DID mapping is being created already exists', async function () {
      await legacyMappingRegistry.createDidMapping(issuer, legacyDid, legacyVerkey, legacySignature)

      await expect(
        legacyMappingRegistry.createDidMapping(issuer, legacyDid, legacyVerkey, legacySignature),
      ).to.be.revertedWithCustomError(legacyMappingRegistry.baseInstance, MigrationErrors.DidMappingAlreadyExist)
    })

    it('Should fail if DID mapping is being created with not matching ed25518 key', async function () {
      const ed25519Key = Uint8Array.from([
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 25, 180, 23, 224, 175, 15, 188, 235, 170, 233, 240, 145, 111, 204, 153,
        108, 117, 188, 145,
      ])

      await expect(
        legacyMappingRegistry.createDidMapping(issuer, legacyDid, ed25519Key, legacySignature),
      ).to.be.revertedWithCustomError(legacyMappingRegistry.baseInstance, MigrationErrors.InvalidEd25519Key)
    })

    it('Should fail if DID mapping is being created with not owned account', async function () {
      legacyMappingRegistry.connect(testAccounts.trustee2.account)

      await expect(
        legacyMappingRegistry.createDidMapping(issuer, legacyDid, legacyVerkey, legacySignature),
      ).to.be.revertedWithCustomError(legacyMappingRegistry.baseInstance, DidErrors.NotIdentityOwner)
    })
  })

  describe('Endorse/Resolve DID mapping', function () {
    it('Should endorse DID mapping', async function () {
      const sig = await signDidMappingEndorsementData(
        legacyMappingRegistry,
        testActorAddress,
        testActorPrivateKey,
        legacyDid,
        legacyVerkey,
        legacySignature,
      )
      await legacyMappingRegistry.createDidMappingSigned(
        testActorAddress,
        legacyDid,
        legacyVerkey,
        legacySignature,
        sig,
      )

      const address = await legacyMappingRegistry.didMapping(legacyDid)
      expect(address).to.be.not.equal(issuer)
    })

    it('Should fail if endorsing duplicate DID mapping', async function () {
      // private key does not match to address
      const sig = await signDidMappingEndorsementData(
        legacyMappingRegistry,
        testActorAddress,
        testActorPrivateKey,
        legacyDid,
        legacyVerkey,
        legacySignature,
      )

      await legacyMappingRegistry.createDidMappingSigned(
        testActorAddress,
        legacyDid,
        legacyVerkey,
        legacySignature,
        sig,
      )

      await expect(
        legacyMappingRegistry.createDidMappingSigned(testActorAddress, legacyDid, legacyVerkey, legacySignature, sig),
      ).to.be.revertedWithCustomError(legacyMappingRegistry.baseInstance, MigrationErrors.DidMappingAlreadyExist)
    })

    it('Should fail if endorsing with not owned DID', async function () {
      // private key does not match to address
      const sig = await signDidMappingEndorsementData(
        legacyMappingRegistry,
        testAccounts.trustee2.account.address,
        testActorPrivateKey,
        legacyDid,
        legacyVerkey,
        legacySignature,
      )

      await expect(
        legacyMappingRegistry.createDidMappingSigned(
          testAccounts.trustee2.account.address,
          legacyDid,
          legacyVerkey,
          legacySignature,
          sig,
        ),
      ).to.be.revertedWithCustomError(legacyMappingRegistry.baseInstance, DidErrors.NotIdentityOwner)
    })

    it('Should fail if endorsing invalid signature', async function () {
      const sig = await signDidMappingEndorsementData(
        legacyMappingRegistry,
        testActorAddress,
        testActorPrivateKey,
        '356FbajrLCJxbQbn8GSb3B',
        legacyVerkey,
        legacySignature,
      )

      await expect(
        legacyMappingRegistry.createDidMappingSigned(testActorAddress, legacyDid, legacyVerkey, legacySignature, sig),
      ).to.be.revertedWithCustomError(legacyMappingRegistry.baseInstance, DidErrors.NotIdentityOwner)
    })
  })

  describe('Add/Resolve Resource mapping', function () {
    beforeEach(async function () {
      await legacyMappingRegistry.createDidMapping(issuer, legacyDid, legacyVerkey, legacySignature)
    })

    it('Should create Resource mapping', async function () {
      await legacyMappingRegistry.createResourceMapping(issuer, legacyDid, legacySchemaId, schemaId)

      const resolvedSchemaId = await legacyMappingRegistry.resourceMapping(legacySchemaId)
      expect(resolvedSchemaId).to.be.equal(schemaId)
    })

    it('Should fail if mapping is being created already exists', async function () {
      await legacyMappingRegistry.createResourceMapping(issuer, legacyDid, legacySchemaId, schemaId)

      await expect(legacyMappingRegistry.createResourceMapping(issuer, legacyDid, legacySchemaId, schemaId))
        .to.be.revertedWithCustomError(legacyMappingRegistry.baseInstance, MigrationErrors.ResourceMappingAlreadyExist)
        .withArgs(legacySchemaId)
    })

    it('Should fail if mapping is being created with not existing DID mapping', async function () {
      await expect(
        legacyMappingRegistry.createResourceMapping(issuer, '356FbajrLCJxbQbn8GSb3B', legacySchemaId, schemaId),
      ).to.be.revertedWithCustomError(legacyMappingRegistry.baseInstance, DidErrors.NotIdentityOwner)
    })

    it('Should fail if mapping is being created with not owned DID mapping', async function () {
      await expect(
        legacyMappingRegistry.createResourceMapping(
          testAccounts.trustee2.account.address,
          legacyDid,
          legacySchemaId,
          schemaId,
        ),
      ).to.be.revertedWithCustomError(legacyMappingRegistry.baseInstance, DidErrors.NotIdentityOwner)
    })
  })

  describe('Endorse/Resolve Resource mapping', function () {
    beforeEach(async function () {
      const sig = await signDidMappingEndorsementData(
        legacyMappingRegistry,
        testActorAddress,
        testActorPrivateKey,
        legacyDid,
        legacyVerkey,
        legacySignature,
      )
      await legacyMappingRegistry.createDidMappingSigned(
        testActorAddress,
        legacyDid,
        legacyVerkey,
        legacySignature,
        sig,
      )
    })

    it('Should endorse Resource mapping', async function () {
      const sig = await signResourceMappingEndorsementData(
        legacyMappingRegistry,
        testActorAddress,
        testActorPrivateKey,
        legacyDid,
        legacySchemaId,
        schemaId,
      )
      await legacyMappingRegistry.createResourceMappingSigned(
        testActorAddress,
        legacyDid,
        legacySchemaId,
        schemaId,
        sig,
      )

      const identifier = await legacyMappingRegistry.resourceMapping(legacySchemaId)
      expect(identifier).to.be.equal(schemaId)
    })

    it('Should fail if endorsing duplicate mapping', async function () {
      // private key does not match to address
      const sig = await signResourceMappingEndorsementData(
        legacyMappingRegistry,
        testActorAddress,
        testActorPrivateKey,
        legacyDid,
        legacySchemaId,
        schemaId,
      )

      await legacyMappingRegistry.createResourceMappingSigned(
        testActorAddress,
        legacyDid,
        legacySchemaId,
        schemaId,
        sig,
      )

      await expect(
        legacyMappingRegistry.createResourceMappingSigned(testActorAddress, legacyDid, legacySchemaId, schemaId, sig),
      ).to.be.revertedWithCustomError(legacyMappingRegistry.baseInstance, MigrationErrors.ResourceMappingAlreadyExist)
    })

    it('Should fail if endorsing with not owned DID', async function () {
      // private key does not match to address
      const sig = await signResourceMappingEndorsementData(
        legacyMappingRegistry,
        testAccounts.trustee2.account.address,
        testActorPrivateKey,
        legacyDid,
        legacySchemaId,
        schemaId,
      )

      await expect(
        legacyMappingRegistry.createResourceMappingSigned(
          testAccounts.trustee2.account.address,
          legacyDid,
          legacySchemaId,
          schemaId,
          sig,
        ),
      ).to.be.revertedWithCustomError(legacyMappingRegistry.baseInstance, DidErrors.NotIdentityOwner)
    })
  })
})
