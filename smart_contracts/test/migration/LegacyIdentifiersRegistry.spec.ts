import { expect } from 'chai'
import { EthereumExtDidRegistry, LegacyIdentifiersRegistry } from '../../contracts-ts'
import {
  createDid,
  deployLegacyIdentifiersRegistry,
  signClMappingEndorsementData,
  signDidMappingEndorsementData,
  TestableLegacyIdentifiersRegistry,
  testActorAddress,
  testActorPrivateKey,
} from '../utils/contract-helpers'
import { MigrationErrors } from '../utils/errors'
import { TestAccounts } from '../utils/test-entities'

describe('LegacyIdentifiersRegistry', function () {
  let didRegistry: EthereumExtDidRegistry
  let legacyIdentifiersRegistry: TestableLegacyIdentifiersRegistry
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
      legacyIdentifiersRegistry: legacyIdentifiersRegistryInit,
      testAccounts: testAccountsInit,
    } = await deployLegacyIdentifiersRegistry()

    issuer = testAccountsInit.trustee.account.address
    legacyIdentifiersRegistryInit.connect(testAccountsInit.trustee.account)

    const issuerId = `did:ethr:${issuer}`
    await createDid(didRegistryInit, testAccountsInit.trustee.account.address, issuerId)

    legacyIdentifiersRegistry = legacyIdentifiersRegistryInit
    didRegistry = didRegistryInit
    testAccounts = testAccountsInit
  })

  describe('Add/Resolve DID mapping', function () {
    it('Should create DID mapping', async function () {
      await legacyIdentifiersRegistry.createDidMapping(issuer, legacyDid, legacyVerkey, legacySignature)

      const address = await legacyIdentifiersRegistry.resolveNewDid(legacyDid)
      expect(address).to.be.equal(issuer)
    })

    it('Should fail if mapping is being created already exists', async function () {
      await legacyIdentifiersRegistry.createDidMapping(issuer, legacyDid, legacyVerkey, legacySignature)

      await expect(
        legacyIdentifiersRegistry.createDidMapping(issuer, legacyDid, legacyVerkey, legacySignature),
      ).to.be.revertedWithCustomError(legacyIdentifiersRegistry.baseInstance, MigrationErrors.DidMappingAlreadyExist)
    })

    it('Should fail if mapping is being created with not matching ed25518 key', async function () {
      const ed25519Key = Uint8Array.from([
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 25, 180, 23, 224, 175, 15, 188, 235, 170, 233, 240, 145, 111, 204, 153,
        108, 117, 188, 145,
      ])

      await expect(
        legacyIdentifiersRegistry.createDidMapping(issuer, legacyDid, ed25519Key, legacySignature),
      ).to.be.revertedWithCustomError(legacyIdentifiersRegistry.baseInstance, MigrationErrors.InvalidEd25519Key)
    })

    it('Should fail if mapping is being created with not owned account', async function () {
      legacyIdentifiersRegistry.connect(testAccounts.trustee2.account)

      await expect(
        legacyIdentifiersRegistry.createDidMapping(issuer, legacyDid, legacyVerkey, legacySignature),
      ).to.be.revertedWithCustomError(legacyIdentifiersRegistry.baseInstance, MigrationErrors.UnauthorizedSender)
    })
  })

  describe('Endorse/Resolve DID mapping', function () {
    it('Should endorse DID mapping', async function () {
      const sig = await signDidMappingEndorsementData(
        legacyIdentifiersRegistry,
        testActorAddress,
        testActorPrivateKey,
        legacyDid,
        legacyVerkey,
        legacySignature,
      )
      await legacyIdentifiersRegistry.createDidMappingSigned(
        testActorAddress,
        legacyDid,
        legacyVerkey,
        legacySignature,
        sig,
      )

      const address = await legacyIdentifiersRegistry.resolveNewDid(legacyDid)
      expect(address).to.be.not.equal(issuer)
    })

    it('Should fail if endorsing duplicate mapping', async function () {
      // private key does not match to address
      const sig = await signDidMappingEndorsementData(
        legacyIdentifiersRegistry,
        testActorAddress,
        testActorPrivateKey,
        legacyDid,
        legacyVerkey,
        legacySignature,
      )

      await legacyIdentifiersRegistry.createDidMappingSigned(
        testActorAddress,
        legacyDid,
        legacyVerkey,
        legacySignature,
        sig,
      )

      await expect(
        legacyIdentifiersRegistry.createDidMappingSigned(
          testActorAddress,
          legacyDid,
          legacyVerkey,
          legacySignature,
          sig,
        ),
      ).to.be.revertedWithCustomError(legacyIdentifiersRegistry.baseInstance, MigrationErrors.DidMappingAlreadyExist)
    })

    it('Should fail if endorsing with not owned DID', async function () {
      // private key does not match to address
      const sig = await signDidMappingEndorsementData(
        legacyIdentifiersRegistry,
        testAccounts.trustee2.account.address,
        testActorPrivateKey,
        legacyDid,
        legacyVerkey,
        legacySignature,
      )

      await expect(
        legacyIdentifiersRegistry.createDidMappingSigned(
          testAccounts.trustee2.account.address,
          legacyDid,
          legacyVerkey,
          legacySignature,
          sig,
        ),
      ).to.be.revertedWithCustomError(legacyIdentifiersRegistry.baseInstance, MigrationErrors.UnauthorizedSender)
    })

    it('Should fail if endorsing invalid signature', async function () {
      const sig = await signDidMappingEndorsementData(
        legacyIdentifiersRegistry,
        testActorAddress,
        testActorPrivateKey,
        '356FbajrLCJxbQbn8GSb3B',
        legacyVerkey,
        legacySignature,
      )

      await expect(
        legacyIdentifiersRegistry.createDidMappingSigned(
          testActorAddress,
          legacyDid,
          legacyVerkey,
          legacySignature,
          sig,
        ),
      ).to.be.revertedWithCustomError(legacyIdentifiersRegistry.baseInstance, MigrationErrors.UnauthorizedSender)
    })
  })

  describe('Add/Resolve CL mapping', function () {
    beforeEach(async function () {
      await legacyIdentifiersRegistry.createDidMapping(issuer, legacyDid, legacyVerkey, legacySignature)
    })

    it('Should create CL mapping', async function () {
      await legacyIdentifiersRegistry.createClMapping(issuer, legacyDid, legacySchemaId, schemaId)

      const resolvedSchemaId = await legacyIdentifiersRegistry.resolveNewId(legacySchemaId)
      expect(resolvedSchemaId).to.be.equal(schemaId)
    })

    it('Should fail if mapping is being created already exists', async function () {
      await legacyIdentifiersRegistry.createClMapping(issuer, legacyDid, legacySchemaId, schemaId)

      await expect(legacyIdentifiersRegistry.createClMapping(issuer, legacyDid, legacySchemaId, schemaId))
        .to.be.revertedWithCustomError(legacyIdentifiersRegistry.baseInstance, MigrationErrors.ClMappingAlreadyExist)
        .withArgs(legacySchemaId)
    })

    it('Should fail if mapping is being created with not existing DID mapping', async function () {
      await expect(
        legacyIdentifiersRegistry.createClMapping(issuer, '356FbajrLCJxbQbn8GSb3B', legacySchemaId, schemaId),
      ).to.be.revertedWithCustomError(legacyIdentifiersRegistry.baseInstance, MigrationErrors.UnauthorizedSender)
    })

    it('Should fail if mapping is being created with not owned DID mapping', async function () {
      await expect(
        legacyIdentifiersRegistry.createClMapping(
          testAccounts.trustee2.account.address,
          legacyDid,
          legacySchemaId,
          schemaId,
        ),
      ).to.be.revertedWithCustomError(legacyIdentifiersRegistry.baseInstance, MigrationErrors.UnauthorizedSender)
    })
  })

  describe('Endorse/Resolve CL mapping', function () {
    beforeEach(async function () {
      const sig = await signDidMappingEndorsementData(
        legacyIdentifiersRegistry,
        testActorAddress,
        testActorPrivateKey,
        legacyDid,
        legacyVerkey,
        legacySignature,
      )
      await legacyIdentifiersRegistry.createDidMappingSigned(
        testActorAddress,
        legacyDid,
        legacyVerkey,
        legacySignature,
        sig,
      )
    })

    it('Should endorse CL mapping', async function () {
      const sig = await signClMappingEndorsementData(
        legacyIdentifiersRegistry,
        testActorAddress,
        testActorPrivateKey,
        legacyDid,
        legacySchemaId,
        schemaId,
      )
      await legacyIdentifiersRegistry.createClMappingSigned(testActorAddress, legacyDid, legacySchemaId, schemaId, sig)

      const identifier = await legacyIdentifiersRegistry.resolveNewId(legacySchemaId)
      expect(identifier).to.be.equal(schemaId)
    })

    it('Should fail if endorsing duplicate mapping', async function () {
      // private key does not match to address
      const sig = await signClMappingEndorsementData(
        legacyIdentifiersRegistry,
        testActorAddress,
        testActorPrivateKey,
        legacyDid,
        legacySchemaId,
        schemaId,
      )

      await legacyIdentifiersRegistry.createClMappingSigned(testActorAddress, legacyDid, legacySchemaId, schemaId, sig)

      await expect(
        legacyIdentifiersRegistry.createClMappingSigned(testActorAddress, legacyDid, legacySchemaId, schemaId, sig),
      ).to.be.revertedWithCustomError(legacyIdentifiersRegistry.baseInstance, MigrationErrors.ClMappingAlreadyExist)
    })

    it('Should fail if endorsing with not owned DID', async function () {
      // private key does not match to address
      const sig = await signClMappingEndorsementData(
        legacyIdentifiersRegistry,
        testAccounts.trustee2.account.address,
        testActorPrivateKey,
        legacyDid,
        legacySchemaId,
        schemaId,
      )

      await expect(
        legacyIdentifiersRegistry.createClMappingSigned(
          testAccounts.trustee2.account.address,
          legacyDid,
          legacySchemaId,
          schemaId,
          sig,
        ),
      ).to.be.revertedWithCustomError(legacyIdentifiersRegistry.baseInstance, MigrationErrors.UnauthorizedSender)
    })
  })
})
