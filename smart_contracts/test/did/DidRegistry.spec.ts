import { expect } from 'chai'
import { createBaseDidDocument } from '../../utils/entity-factories'
import {
  deployIndyDidRegistry,
  TestableIndyDidRegistry,
  testActorAddress,
  testActorPrivateKey,
} from '../utils/contract-helpers'
import { DidError } from '../utils/errors'
import { TestAccounts } from '../utils/test-entities'

describe('IndyDidRegistry', function () {
  let didRegistry: TestableIndyDidRegistry
  let testAccounts: TestAccounts
  let identity: string
  let did: string
  let didDocument: string

  beforeEach(async function () {
    const { indyDidRegistry: didRegistryInit, testAccounts: testAccountsInit } = await deployIndyDidRegistry()

    didRegistry = didRegistryInit
    testAccounts = testAccountsInit

    identity = testAccounts.trustee.account.address
    did = `did:indybesu:testnet:${testAccounts.trustee.account}`
    didDocument = createBaseDidDocument(did)

    didRegistry.connect(testAccounts.trustee.account)
  })

  describe('Create DID', function () {
    it('Should create and resolve DID document', async function () {
      await didRegistry.createDid(identity, didDocument)

      const { document } = await didRegistry.resolveDid(identity)

      expect(document).to.be.deep.equal(didDocument)
    })

    it('Should fail if resolving DID does not exist', async function () {
      await expect(didRegistry.resolveDid(identity))
        .to.revertedWithCustomError(didRegistry.baseInstance, DidError.DidNotFound)
        .withArgs(identity)
    })

    it('Should fail if the DID being created already exists', async function () {
      await didRegistry.createDid(identity, didDocument)

      await expect(didRegistry.createDid(identity, didDocument))
        .to.be.revertedWithCustomError(didRegistry.baseInstance, DidError.DidAlreadyExist)
        .withArgs(identity)
    })
  })

  describe('Update DID', function () {
    it('Should update DID document', async function () {
      await didRegistry.createDid(identity, didDocument)

      await didRegistry.updateDid(identity, didDocument)

      const { document } = await didRegistry.resolveDid(identity)

      expect(document).to.be.deep.equal(didDocument)
    })

    it('Should fail if the DID creator is not an update txn sender', async function () {
      await didRegistry.createDid(identity, didDocument)

      didRegistry.connect(testAccounts.trustee2.account)
      await expect(didRegistry.updateDid(identity, didDocument)).to.revertedWithCustomError(
        didRegistry.baseInstance,
        DidError.NotIdentityOwner,
      )
    })

    it('Should fail if the DID being updated does not exists', async function () {
      await expect(didRegistry.updateDid(identity, didDocument))
        .to.revertedWithCustomError(didRegistry.baseInstance, DidError.DidNotFound)
        .withArgs(identity)
    })

    it('Should fail if the DID being updated is deactivated', async function () {
      await didRegistry.createDid(identity, didDocument)
      await didRegistry.deactivateDid(identity)

      await expect(didRegistry.updateDid(identity, didDocument))
        .to.revertedWithCustomError(didRegistry.baseInstance, DidError.DidHasBeenDeactivated)
        .withArgs(identity)
    })
  })

  describe('Deactivate DID', function () {
    it('Should deactivate DID document', async function () {
      await didRegistry.createDid(identity, didDocument)
      await didRegistry.deactivateDid(identity)

      const didStorage = await didRegistry.resolveDid(identity)

      expect(didStorage.metadata.deactivated).is.true
    })

    it('Should fail if the DID has already been deactivated', async function () {
      await didRegistry.createDid(identity, didDocument)
      await didRegistry.deactivateDid(identity)

      await expect(didRegistry.deactivateDid(identity))
        .to.revertedWithCustomError(didRegistry.baseInstance, DidError.DidHasBeenDeactivated)
        .withArgs(identity)
    })

    it('Should fail if the DID being deactivated does not exists', async function () {
      await expect(didRegistry.deactivateDid(identity))
        .to.revertedWithCustomError(didRegistry.baseInstance, DidError.DidNotFound)
        .withArgs(identity)
    })

    it('Should fail if the DID creator is not an deactivate txn sender', async function () {
      await didRegistry.createDid(identity, didDocument)

      didRegistry.connect(testAccounts.trustee2.account)
      await expect(didRegistry.deactivateDid(identity)).to.revertedWithCustomError(
        didRegistry.baseInstance,
        DidError.NotIdentityOwner,
      )
    })
  })

  describe('Endorse DID', function () {
    it('Should endorse and resolve DID document', async function () {
      const authorDid = `did:indybesu:testnet:${testActorAddress}`
      const authorDidDocument = createBaseDidDocument(authorDid)

      let sig = await didRegistry.signCreateDidEndorsementData(testActorAddress, testActorPrivateKey, authorDidDocument)
      await didRegistry.createDidSigned(testActorAddress, authorDidDocument, sig)

      let didRecord = await didRegistry.resolveDid(testActorAddress)
      expect(didRecord.document).to.be.deep.equal(authorDidDocument)

      const updatedDidDocument = createBaseDidDocument(authorDid, {
        id: 'kid',
        type: 'Ed25519VerificationKey2018',
        controller: authorDid,
        publicKeyMultibase: 'key',
      })

      sig = await didRegistry.signUpdateDidEndorsementData(testActorAddress, testActorPrivateKey, updatedDidDocument)
      await didRegistry.updateDidSigned(testActorAddress, updatedDidDocument, sig)

      didRecord = await didRegistry.resolveDid(testActorAddress)
      expect(didRecord.document).to.be.deep.equal(updatedDidDocument)
    })
  })
})
