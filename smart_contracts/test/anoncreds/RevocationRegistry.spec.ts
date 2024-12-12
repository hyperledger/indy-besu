/**
 * Copyright (c) 2024 DSR Corporation, Denver, Colorado.
 * https://www.dsr-corporation.com
 * SPDX-License-Identifier: Apache-2.0
 */

import { expect } from 'chai'
import { keccak256, toUtf8Bytes } from 'ethers'
import { IndyDidRegistry } from '../../contracts-ts'
import { RevocationRegistryEntryStruct } from '../../typechain-types/contracts/anoncreds/RevocationRegistry'
import {
  createCredentialDefinitionObject,
  CreateRevocationEntryParams,
  createRevocationRegistryDefinitionObject,
  createRevocationRegistryEntryObject,
  createSchemaObject,
} from '../../utils'
import {
  createCredentialDefinition,
  createDidSigned,
  createSchema,
  createSchemaSigned,
  deployRevocationRegistry,
  TestableCredentialDefinitionRegistry,
  TestableRevocationRegistry,
  TestableRoleControl,
  TestableSchemaRegistry,
  testActorAddress,
  testActorPrivateKey,
} from '../utils/contract-helpers'
import { AuthErrors, ClErrors, DidErrors } from '../utils/errors'
import { TestAccounts } from '../utils/test-entities'

describe('RevocationRegistry', function () {
  let didRegistry: IndyDidRegistry
  let schemaRegistry: TestableSchemaRegistry
  let credentialDefinitionRegistry: TestableCredentialDefinitionRegistry
  let revocationRegistry: TestableRevocationRegistry
  let roleControl: TestableRoleControl
  let testAccounts: TestAccounts
  let schemaId: string
  let credDefId: string
  let issuerAddress: string
  let issuerId: string

  beforeEach(async function () {
    const {
      indyDidRegistry: didRegistryInit,
      schemaRegistry: schemaRegistryInit,
      credentialDefinitionRegistry: credentialDefinitionRegistryInit,
      roleControl: roleControlInit,
      testAccounts: testAccountsInit,
      revocationRegistry: revocationRegistryInit,
    } = await deployRevocationRegistry()

    didRegistryInit.connect(testAccountsInit.trustee.account)
    schemaRegistryInit.connect(testAccountsInit.trustee.account)
    credentialDefinitionRegistryInit.connect(testAccountsInit.trustee.account)
    revocationRegistryInit.connect(testAccountsInit.trustee.account)

    issuerAddress = testAccountsInit.trustee.account.address
    issuerId = `did:ethr:${testAccountsInit.trustee.account.address}`
    const { id: createdSchemaId } = await createSchema(schemaRegistryInit, issuerAddress, issuerId)

    schemaId = createdSchemaId

    const { id: createdCredDefId } = await createCredentialDefinition(
      credentialDefinitionRegistryInit,
      issuerAddress,
      issuerId,
      schemaId,
    )

    didRegistry = didRegistryInit
    testAccounts = testAccountsInit
    schemaRegistry = schemaRegistryInit
    credentialDefinitionRegistry = credentialDefinitionRegistryInit
    revocationRegistry = revocationRegistryInit
    roleControl = roleControlInit
    credDefId = createdCredDefId
  })

  describe('Add/Resolve Revocation Degistry Definition with did:ethr Issuer', function () {
    it('Should create and resolve Revocation Registry Definition', async function () {
      const ethrIssuerId = `did:ethr:${issuerAddress}`
      const { id, revRegDef } = createRevocationRegistryDefinitionObject({ issuerId: ethrIssuerId, credDefId })

      await revocationRegistry.createRevocationRegistryDefinition(issuerAddress, id, credDefId, ethrIssuerId, revRegDef)
      const result = await revocationRegistry.resolveRevocationRegistryDefinition(id)

      expect(result.revRegDef).to.be.deep.equal(revRegDef)
    })

    it('Should fail if Revocation Registry Definition is being created with not owned Issuer DID', async function () {
      const ethrIssuerId = `did:ethr:${testAccounts.trustee2.account.address}`
      const { id, revRegDef } = createRevocationRegistryDefinitionObject({ issuerId: ethrIssuerId, credDefId })

      await expect(
        revocationRegistry.createRevocationRegistryDefinition(
          testAccounts.trustee2.account.address,
          id,
          credDefId,
          ethrIssuerId,
          revRegDef,
        ),
      ).to.be.revertedWithCustomError(revocationRegistry.baseInstance, DidErrors.NotIdentityOwner)
    })

    it('Should fail if Revocation Registry Definition is being created with invalid Issuer ID', async function () {
      const invalidIssuerId = 'did:ethr:ab$ddfgh354345'
      const { id, revRegDef } = createRevocationRegistryDefinitionObject({ issuerId: invalidIssuerId, credDefId })

      await expect(
        revocationRegistry.createRevocationRegistryDefinition(issuerAddress, id, credDefId, invalidIssuerId, revRegDef),
      )
        .to.be.revertedWithCustomError(revocationRegistry.baseInstance, ClErrors.InvalidIssuerId)
        .withArgs(invalidIssuerId)
    })

    it('Should fail if Revocation Registry Definition is being created for nonexistent Credential Definition', async function () {
      const ethrIssuerId = `did:ethr:${issuerAddress}`
      const { id, revRegDef } = createRevocationRegistryDefinitionObject({ issuerId: ethrIssuerId, credDefId })

      const invalidCredDefId = 'invalid credDefId'

      await expect(
        revocationRegistry.createRevocationRegistryDefinition(
          issuerAddress,
          id,
          invalidCredDefId,
          ethrIssuerId,
          revRegDef,
        ),
      )
        .to.be.revertedWithCustomError(credentialDefinitionRegistry.baseInstance, ClErrors.CredentialDefinitionNotFound)
        .withArgs(keccak256(toUtf8Bytes(invalidCredDefId)))
    })
  })

  describe('Endorse/Resolve Revocation Registry Definition with did:ethr Issuer', function () {
    it('Should endorse and resolve Revocation Registry Definition with did:ethr', async function () {
      const authorDid = `did:ethr:${testActorAddress}`
      const { id: ethSchemaId } = await createSchemaSigned(schemaRegistry, testActorAddress, authorDid)

      const { id: credentialDefinitionId, credDef } = createCredentialDefinitionObject({
        issuerId: authorDid,
        schemaId: ethSchemaId,
      })
      const signature = credentialDefinitionRegistry.signCreateCredDefEndorsementData(
        testActorAddress,
        testActorPrivateKey,
        credentialDefinitionId,
        authorDid,
        ethSchemaId,
        credDef,
      )

      await credentialDefinitionRegistry.createCredentialDefinitionSigned(
        testActorAddress,
        credentialDefinitionId,
        authorDid,
        ethSchemaId,
        credDef,
        signature,
      )

      const { id, revRegDef } = createRevocationRegistryDefinitionObject({
        issuerId: authorDid,
        credDefId: credentialDefinitionId,
      })

      const revRegSig = revocationRegistry.signCreateRevRegDefEndorsementData(
        testActorAddress,
        testActorPrivateKey,
        id,
        credentialDefinitionId,
        authorDid,
        revRegDef,
      )

      await revocationRegistry.createRevocationRegistryDefinitionSigned(
        testActorAddress,
        id,
        credentialDefinitionId,
        authorDid,
        revRegDef,
        revRegSig,
      )
      const result = await revocationRegistry.resolveRevocationRegistryDefinition(id)

      expect(result.revRegDef).to.be.deep.equal(revRegDef)
    })

    it('Should fail if Revocation Registry Definition is being endorsed with not owned Issuer DID', async function () {
      const authorDid = `did:ethr:${testActorAddress}`
      const { id: ethSchemaId } = await createSchemaSigned(schemaRegistry, testActorAddress, authorDid)

      const { id: credentialDefinitionId, credDef } = createCredentialDefinitionObject({
        issuerId: authorDid,
        schemaId: ethSchemaId,
      })
      const signature = credentialDefinitionRegistry.signCreateCredDefEndorsementData(
        testActorAddress,
        testActorPrivateKey,
        credentialDefinitionId,
        authorDid,
        ethSchemaId,
        credDef,
      )

      await credentialDefinitionRegistry.createCredentialDefinitionSigned(
        testActorAddress,
        credentialDefinitionId,
        authorDid,
        ethSchemaId,
        credDef,
        signature,
      )

      const { id, revRegDef } = createRevocationRegistryDefinitionObject({
        issuerId: authorDid,
        credDefId: credentialDefinitionId,
      })

      const revRegSig = revocationRegistry.signCreateRevRegDefEndorsementData(
        testAccounts.trustee2.account.address,
        testActorPrivateKey,
        id,
        credentialDefinitionId,
        authorDid,
        revRegDef,
      )

      await expect(
        revocationRegistry.createRevocationRegistryDefinitionSigned(
          testAccounts.trustee2.account.address,
          id,
          credentialDefinitionId,
          authorDid,
          revRegDef,
          revRegSig,
        ),
      ).to.be.revertedWithCustomError(revocationRegistry.baseInstance, DidErrors.NotIdentityOwner)
    })

    it('Should fail if Revocation Registry Definition is being endorsed with invalid signature', async function () {
      const authorDid = `did:ethr:${testActorAddress}`
      const { id: ethSchemaId } = await createSchemaSigned(schemaRegistry, testActorAddress, authorDid)

      const { id: credentialDefinitionId, credDef } = createCredentialDefinitionObject({
        issuerId: authorDid,
        schemaId: ethSchemaId,
      })
      const signature = credentialDefinitionRegistry.signCreateCredDefEndorsementData(
        testActorAddress,
        testActorPrivateKey,
        credentialDefinitionId,
        authorDid,
        ethSchemaId,
        credDef,
      )

      await credentialDefinitionRegistry.createCredentialDefinitionSigned(
        testActorAddress,
        credentialDefinitionId,
        authorDid,
        ethSchemaId,
        credDef,
        signature,
      )

      const { id, revRegDef } = createRevocationRegistryDefinitionObject({
        issuerId: authorDid,
        credDefId: credentialDefinitionId,
      })

      const revRegSig = revocationRegistry.signCreateRevRegDefEndorsementData(
        testAccounts.trustee2.account.address,
        testActorPrivateKey,
        'different id passed into signature',
        credentialDefinitionId,
        authorDid,
        revRegDef,
      )

      await expect(
        revocationRegistry.createRevocationRegistryDefinitionSigned(
          testActorAddress,
          id,
          credentialDefinitionId,
          authorDid,
          revRegDef,
          revRegSig,
        ),
      ).to.be.revertedWithCustomError(revocationRegistry.baseInstance, DidErrors.NotIdentityOwner)
    })
  })

  describe('Add/Resolve All Revocation Registry Entry with did:ethr Issuer', function () {
    it('Should successfully add Revocation Registry Entry', async function () {
      const ethrIssuerId = `did:ethr:${issuerAddress}`
      const { id, revRegDef } = createRevocationRegistryDefinitionObject({ issuerId: ethrIssuerId, credDefId })

      await revocationRegistry.createRevocationRegistryDefinition(issuerAddress, id, credDefId, ethrIssuerId, revRegDef)

      const revocationRegistryEntryParams: CreateRevocationEntryParams = {
        currentAccumulator: '0x20',
        prevAccumulator: '0x',
        issued: [2, 3],
        revoked: [0, 1],
        timestamp: 1731067598,
      }

      const revocationRegistryEntry = createRevocationRegistryEntryObject(revocationRegistryEntryParams)

      await revocationRegistry.createRevocationRegistryEntry(issuerAddress, id, issuerId, revocationRegistryEntry)

      const result = await revocationRegistry.resolveRevocationRegistryDefinition(id)

      expect(result.metadata.currentAccumulator).to.be.deep.equal(revocationRegistryEntryParams.currentAccumulator)

      const entries = await revocationRegistry.fetchAllRevocationEntries(id)
      expect(entries[0]).to.be.deep.equal(revocationRegistryEntryParams)
    })

    it('Should fail to add Revocation Registry Entry with incompatible previous Accumulator', async function () {
      const ethrIssuerId = `did:ethr:${issuerAddress}`
      const { id, revRegDef } = createRevocationRegistryDefinitionObject({ issuerId: ethrIssuerId, credDefId })

      await revocationRegistry.createRevocationRegistryDefinition(issuerAddress, id, credDefId, ethrIssuerId, revRegDef)

      let revocationRegistryEntryParams: CreateRevocationEntryParams = {
        currentAccumulator: '0x20',
        prevAccumulator: '0x',
        issued: [2, 3],
        revoked: [0, 1],
        timestamp: 1731067598,
      }

      let revocationRegistryEntry = createRevocationRegistryEntryObject(revocationRegistryEntryParams)

      await revocationRegistry.createRevocationRegistryEntry(issuerAddress, id, issuerId, revocationRegistryEntry)

      revocationRegistryEntryParams = {
        currentAccumulator: '0x30',
        prevAccumulator: '0x30',
        issued: [2, 3],
        revoked: [0, 1],
        timestamp: 1731067598,
      }

      revocationRegistryEntry = createRevocationRegistryEntryObject(revocationRegistryEntryParams)

      await expect(
        revocationRegistry.createRevocationRegistryEntry(issuerAddress, id, ethrIssuerId, revocationRegistryEntry),
      )
        .to.be.revertedWithCustomError(revocationRegistry.baseInstance, ClErrors.AccumulatorMismatch)
        .withArgs(revocationRegistryEntryParams.prevAccumulator)
    })

    it('Should fail if attempting to create Revocation Registry Entry for nonexistent Revocation Registry Definition', async function () {
      const ethrIssuerId = `did:ethr:${issuerAddress}`
      const id = 'invalid revRegDefId'

      const revocationRegistryEntry = createRevocationRegistryEntryObject({})

      await expect(
        revocationRegistry.createRevocationRegistryEntry(issuerAddress, id, ethrIssuerId, revocationRegistryEntry),
      )
        .to.be.revertedWithCustomError(revocationRegistry.baseInstance, ClErrors.RevocationRegistryDefinitionNotFound)
        .withArgs(keccak256(toUtf8Bytes(id)))
    })

    it('Should fail if attempting to create Revocation Registry Entry for not owned Revocation Registry Definition', async function () {
      const ethrIssuerId = `did:ethr:${issuerAddress}`
      const { id, revRegDef } = createRevocationRegistryDefinitionObject({ issuerId: ethrIssuerId, credDefId })

      await revocationRegistry.createRevocationRegistryDefinition(issuerAddress, id, credDefId, ethrIssuerId, revRegDef)

      const revocationRegistryEntryParams: CreateRevocationEntryParams = {
        currentAccumulator: '0x20',
        prevAccumulator: '0x',
        issued: [2, 3],
        revoked: [0, 1],
        timestamp: 1731067598,
      }

      revocationRegistry.connect(testAccounts.trustee2.account)
      const notRevRegDefIssuerAddress = testAccounts.trustee2.account.address
      const notRevRegDefIssuerId = `did:ethr:${notRevRegDefIssuerAddress}`

      const revocationRegistryEntry = createRevocationRegistryEntryObject(revocationRegistryEntryParams)

      await expect(
        revocationRegistry.createRevocationRegistryEntry(
          notRevRegDefIssuerAddress,
          id,
          notRevRegDefIssuerId,
          revocationRegistryEntry,
        ),
      )
        .to.be.revertedWithCustomError(revocationRegistry.baseInstance, ClErrors.NotRevocationRegistryDefinitionIssuer)
        .withArgs(notRevRegDefIssuerId)
    })
  })
})
