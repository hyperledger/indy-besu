import { expect } from 'chai'
import { keccak256, toUtf8Bytes } from 'ethers'
import { IndyDidRegistry } from '../../contracts-ts'
import { createCredentialDefinitionObject, createRevocationRegistryObject } from '../../utils'
import {
  createDid,
  createSchema,
  deployCredentialDefinitionRegistry,
  deployRevocationRegistry,
  TestableCredentialDefinitionRegistry,
  TestableRevocationRegistry,
  TestableRoleControl,
  TestableSchemaRegistry,
  testActorAddress,
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
  let issuerAddress: string
  let issuerId: string
  let sId: string
  let revocationRegistry: TestableRevocationRegistry

  beforeEach(async function () {
    const {
      indyDidRegistry: didRegistryInit,
      schemaRegistry: schemaRegistryInit,
      credentialDefinitionRegistry: credentialDefinitionRegistryInit,
      roleControl: roleControlInit,
      testAccounts: testAccountsInit,
    } = await deployCredentialDefinitionRegistry()

    const { revocationRegistry: revocationRegistryInit } = await deployRevocationRegistry()

    didRegistryInit.connect(testAccountsInit.trustee.account)
    schemaRegistryInit.connect(testAccountsInit.trustee.account)
    credentialDefinitionRegistryInit.connect(testAccountsInit.trustee.account)
    revocationRegistryInit.connect(testAccountsInit.trustee.account)

    issuerAddress = testAccountsInit.trustee.account.address
    issuerId = `did:indybesu:mainnet:${issuerAddress}`

    const CreateDid = await createDid(didRegistryInit, issuerAddress, issuerId)
    // console.log (CreateDid)

    const { id: schemaIdInit } = await createSchema(schemaRegistryInit, issuerAddress, issuerId)
    schemaId = schemaIdInit
    // console.log (schemaId)

    const { id, credDef } = createCredentialDefinitionObject({ issuerId, schemaId })
    // sId = id
    console.log(sId)
    // console.log(credDef)

    const CreatecreDef = await credentialDefinitionRegistryInit.createCredentialDefinition(
      issuerAddress,
      id,
      issuerId,
      schemaId,
      credDef,
    )

    console.log(CreatecreDef)
    // console.log("create revoga:")

    didRegistry = didRegistryInit
    schemaRegistry = schemaRegistryInit
    credentialDefinitionRegistry = credentialDefinitionRegistryInit
    revocationRegistry = revocationRegistryInit
    roleControl = roleControlInit
    testAccounts = testAccountsInit
  })

  describe('Revoke/Suspend/Unrevoke Credential', function () {
    it('Should  resolve revocation', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId })

      const tx = await revocationRegistry.createRevocationRegistry(issuerAddress, issuerId, revReg)
      // console.log(tx)
      // const revRegId = tx.id

      const result = await revocationRegistry.resolveRevocation(revRegId)
      console.log(result)

      // const tx1 = await revocationRegistry.revokeCredential(issuerAddress, revRegId);

      // const result = await revocationRegistry.resolveRevocation(revRegId);
      expect(result.metadata.status).to.be.equal('revoked')
    })

    /*it('Should suspend and resolve Credential', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId})

      await revocationRegistry.createRevocationRegistry(issuerAddress, revRegId, revReg);
      await revocationRegistry.suspendCredential(issuerAddress, revRegId);

      const result = await revocationRegistry.resolveRevocation(revRegId);
      expect(result.metadata.status).to.be.equal('suspended');
    });

    it('Should unrevoke and resolve Credential', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId})

      await revocationRegistry.createRevocationRegistry(issuerAddress, revRegId, revReg);
      await revocationRegistry.revokeCredential(issuerAddress, revRegId);
      await revocationRegistry.unrevokeCredential(issuerAddress, revRegId);

      const result = await revocationRegistry.resolveRevocation(revRegId);
      expect(result.metadata.status).to.be.equal('active');
    });

    it('Should fail if trying to revoke an already revoked Credential', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId})

      await revocationRegistry.createRevocationRegistry(issuerAddress, revRegId, revReg);
      await revocationRegistry.revokeCredential(issuerAddress, revRegId);

      await expect(
        revocationRegistry.revokeCredential(issuerAddress, revRegId),
      ).to.be.revertedWithCustomError(revocationRegistry.baseInstance, AnoncredsErrors.CredentialIsAlreadyRevoked).withArgs(revRegId);
    });

    it('Should fail if trying to suspend an already suspended Credential', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId})

      await revocationRegistry.createRevocationRegistry(issuerAddress, revRegId, revReg);
      await revocationRegistry.suspendCredential(issuerAddress, revRegId);

      await expect(
        revocationRegistry.suspendCredential(issuerAddress, revRegId),
      ).to.be.revertedWithCustomError(revocationRegistry.baseInstance, AnoncredsErrors.RevocationIsNotsuspended).withArgs(revRegId);
    });

    it('Should fail if trying to unrevoke a non-revoked Credential', async function () {
     const { revRegId, revReg } = createRevocationRegistryObject({ issuerId})

      await revocationRegistry.createRevocationRegistry(issuerAddress, revRegId, revReg);

      await expect(
        revocationRegistry.unrevokeCredential(issuerAddress, revRegId),
      ).to.be.revertedWithCustomError(revocationRegistry.baseInstance, AnoncredsErrors.RevocationIsNotRevoked).withArgs(revRegId);
    });

    it('Should fail if trying to operate on a Credential with a non-existent Credential Definition', async function () {
      const unknownCredentialDefinitionId = keccak256(toUtf8Bytes('unknown-cred-def-id'));
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId})

      await expect(
        revocationRegistry.revokeCredential(issuerAddress, revRegId),
      ).to.be.revertedWithCustomError(credentialDefinitionRegistry.baseInstance, AnoncredsErrors.CredentialDefinitionNotFound).withArgs(unknownCredentialDefinitionId);
    });

    it('Should fail if trying to operate on a Credential without required role', async function () {
      const { revRegId, revReg } = createRevocationRegistryObject({ issuerId})

      await revocationRegistry.createRevocationRegistry(issuerAddress, revRegId, revReg);

      revocationRegistry.connect(testAccounts.noRole.account);

      await expect(
        revocationRegistry.revokeCredential(issuerAddress, revRegId),
      ).to.be.revertedWithCustomError(roleControl.baseInstance, AuthErrors.Unauthorized);
    });*/
  })
})
