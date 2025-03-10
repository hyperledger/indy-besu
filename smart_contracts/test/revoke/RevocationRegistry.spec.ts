import { expect } from 'chai'
import { keccak256, toUtf8Bytes, toUtf8String, Wallet } from 'ethers'
import { IndyDidRegistry } from '../../contracts-ts'
import { createRevocationRegistryObject } from '../../utils'
import {
  createCredentialDefinition,
  createDid,
  createSchema,
  deployRevocationRegistry,
  TestableCredentialDefinitionRegistry,
  TestableRevocationRegistry,
  TestableRoleControl,
  TestableSchemaRegistry,
} from '../utils/contract-helpers'

import { AnoncredsErrors, AuthErrors, ClErrors, DidErrors } from '../utils/errors'

import { TestAccounts } from '../utils/test-entities'

describe('RevocationRegistry', function () {
  let didRegistry: IndyDidRegistry
  let schemaRegistry: TestableSchemaRegistry
  let credentialDefinitionRegistry: TestableCredentialDefinitionRegistry
  let roleControl: TestableRoleControl
  let testAccounts: TestAccounts
  let schemaId: string
  let credDefId: string
  let issuerAddress: string
  let issuerId: string
  let revocationRegistry: TestableRevocationRegistry

  beforeEach(async function () {
    const {
      revocationRegistry: revocationRegistryInit,
      indyDidRegistry: didRegistryInit,
      schemaRegistry: schemaRegistryInit,
      credentialDefinitionRegistry: credentialDefinitionRegistryInit,
      roleControl: roleControlInit,
      testAccounts: testAccountsInit,
    } = await deployRevocationRegistry()

    didRegistryInit.connect(testAccountsInit.trustee.account)
    schemaRegistryInit.connect(testAccountsInit.trustee.account)
    credentialDefinitionRegistryInit.connect(testAccountsInit.trustee.account)
    revocationRegistryInit.connect(testAccountsInit.trustee.account)

    issuerAddress = testAccountsInit.trustee.account.address
    issuerId = `did:indybesu:mainnet:${issuerAddress}`
    await createDid(didRegistryInit, issuerAddress, issuerId)

    const { id: schemaIdInit } = await createSchema(schemaRegistryInit, issuerAddress, issuerId)
    schemaId = schemaIdInit

    const { id: initCredDefId } = await createCredentialDefinition(
      credentialDefinitionRegistryInit,
      issuerAddress,
      issuerId,
      schemaId,
    )

    didRegistry = didRegistryInit
    testAccounts = testAccountsInit
    schemaRegistry = schemaRegistryInit
    credentialDefinitionRegistry = credentialDefinitionRegistryInit
    roleControl = roleControlInit
    credDefId = initCredDefId
    revocationRegistry = revocationRegistryInit
  })

  describe('Create/Resolve Revocation', function () {
    it('Should  Create revocation', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId })

      const result = await revocationRegistry.createRevocationRegistry(issuerAddress, credDefId, revRegId, revReg)

      expect(result).to.exist
    })

    it('Should  resolve revocation', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId })

      await revocationRegistry.createRevocationRegistry(issuerAddress, credDefId, revRegId, revReg)

      const result = await revocationRegistry.resolveRevocation(revRegId)
      expect(result).to.equal(0)
    })
  })

  describe('Revoke/Suspend/Unrevoke Credential', function () {
    it('Should  suspend and Resolve Revocation Registry', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId })

      await revocationRegistry.createRevocationRegistry(issuerAddress, credDefId, revRegId, revReg)
      await revocationRegistry.suspendCredential(issuerAddress, revRegId)

      const status = await revocationRegistry.resolveRevocation(revRegId)
      expect(status).to.equal(1)
    })

    it('Should unrevoke and Resolve Revocation Registry', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId })

      await revocationRegistry.createRevocationRegistry(issuerAddress, credDefId, revRegId, revReg)
      await revocationRegistry.suspendCredential(issuerAddress, revRegId)
      await revocationRegistry.unrevokeCredential(issuerAddress, revRegId)

      const status = await revocationRegistry.resolveRevocation(revRegId)
      expect(status).to.equal(0)
    })

    it('Should revoke and Resolve Revocation Registry', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId })

      await revocationRegistry.createRevocationRegistry(issuerAddress, credDefId, revRegId, revReg)
      await revocationRegistry.revokeCredential(issuerAddress, revRegId)

      const status = await revocationRegistry.resolveRevocation(revRegId)

      expect(status).to.equal(2)
    })
  })
  describe('Create/Revoke/Suspend/Unrevoke Credential fail', function () {
    it('should fail if trying to create a revocation registry and the revocation registry already exists', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId })

      await revocationRegistry.createRevocationRegistry(issuerAddress, credDefId, revRegId, revReg)

      await expect(revocationRegistry.createRevocationRegistry(issuerAddress, credDefId, revRegId, revReg))
        .to.be.revertedWithCustomError(revocationRegistry.baseInstance, AnoncredsErrors.RevocationAlreadyExist)
        .withArgs(keccak256(toUtf8Bytes(revRegId)))
    })
    it('Should fail if trying to revoke an already revoked Credential', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId })

      await revocationRegistry.createRevocationRegistry(issuerAddress, credDefId, revRegId, revReg)
      await revocationRegistry.revokeCredential(issuerAddress, revRegId)

      await expect(revocationRegistry.revokeCredential(issuerAddress, revRegId))
        .to.be.revertedWithCustomError(revocationRegistry.baseInstance, AnoncredsErrors.CredentialIsAlreadyRevoked)
        .withArgs(keccak256(toUtf8Bytes(revRegId)))
    })

    it('Should fail if trying to suspend an already suspended Credential', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId })

      await revocationRegistry.createRevocationRegistry(issuerAddress, credDefId, revRegId, revReg)
      await revocationRegistry.suspendCredential(issuerAddress, revRegId)

      await expect(revocationRegistry.suspendCredential(issuerAddress, revRegId))
        .to.be.revertedWithCustomError(revocationRegistry.baseInstance, AnoncredsErrors.RevocationIsNotActived)
        .withArgs(keccak256(toUtf8Bytes(revRegId)))
    })

    it('Should fail if trying to unrevoke a non-revoked Credential', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId })

      await revocationRegistry.createRevocationRegistry(issuerAddress, credDefId, revRegId, revReg)

      await expect(revocationRegistry.unrevokeCredential(issuerAddress, revRegId))
        .to.be.revertedWithCustomError(revocationRegistry.baseInstance, AnoncredsErrors.RevocationIsNotRevoked)
        .withArgs(keccak256(toUtf8Bytes(revRegId)))
    })
  })

  describe('Fail by not find the requirements', function () {
    it('Should fail if trying to operate on a Credential with a non-existent Credential Definition', async function () {
      const unknownCredentialDefinitionId = keccak256(toUtf8Bytes('unknown-cred-def-id'))
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId })

      await expect(
        revocationRegistry.createRevocationRegistry(issuerAddress, unknownCredentialDefinitionId, revRegId, revReg),
      ).to.be.revertedWithCustomError(
        credentialDefinitionRegistry.baseInstance,
        AnoncredsErrors.CredentialDefinitionNotFound,
      )
    })

    it('Should fail if trying to operate on a Revacation without required role', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId })

      await revocationRegistry.createRevocationRegistry(issuerAddress, credDefId, revRegId, revReg)
      revocationRegistry.connect(testAccounts.noRole.account)

      await expect(revocationRegistry.revokeCredential(issuerAddress, revRegId)).to.be.revertedWithCustomError(
        roleControl.baseInstance,
        AuthErrors.Unauthorized,
      )
    })
  })
})
