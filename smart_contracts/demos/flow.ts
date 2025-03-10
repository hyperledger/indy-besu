/**
 * Copyright (c) 2024 DSR Corporation, Denver, Colorado.
 * https://www.dsr-corporation.com
 * SPDX-License-Identifier: Apache-2.0
 */

import environment from '../environment'
import { Actor } from './utils/actor'
import { ROLES } from '../contracts-ts'
import { createCredentialDefinitionObject, createRevocationRegistryObject, createSchemaObject } from '../utils'

async function demo() {
  let receipt: any
  let verify: any

  const trustee = await new Actor(environment.accounts.account1).init()
  const faber = await new Actor().init()
  const alice = await new Actor().init()

  console.log('1. Trustee assign ENDORSER role to Faber')
  receipt = await trustee.roleControl.assignRole(ROLES.ENDORSER, faber.address)
  console.log(`Role ${ROLES.ENDORSER} assigned to account ${faber.address}. Receipt: ${JSON.stringify(receipt)}`)

  const issuerAddress = faber.address
  const issuerId = `did:indybesu:mainnet:${issuerAddress}`

  console.log('2. Faber creates DID Document')
  receipt = await faber.didRegistry.createDid(issuerAddress, faber.didDocument)
  console.log(`Did Document created for DID ${issuerId}. Receipt: ${JSON.stringify(receipt)}`)

  console.log('3. Faber creates Test Schema')
  const { id: schemaId, schema } = createSchemaObject({ issuerId })
  receipt = await faber.schemaRegistry.createSchema(issuerAddress, schemaId, issuerId, schema)
  console.log(`Schema created for id ${schemaId}. Receipt: ${JSON.stringify(receipt)}`)

  console.log('4. Faber resolves Test Schema to ensure its written')
  const resolvedSchema = await faber.schemaRegistry.resolveSchema(schemaId)
  console.log(`Schema resolved for ${schemaId}. Schema: ${resolvedSchema.schema}`)

  console.log('5. Faber create Test Credential Definition')
  const { id: credentialDefinitionId, credDef: credentialDefinition } = createCredentialDefinitionObject({
    issuerId,
    schemaId,
  })
  receipt = await faber.credentialDefinitionRegistry.createCredentialDefinition(
    issuerAddress,
    credentialDefinitionId,
    issuerId,
    schemaId,
    credentialDefinition,
  )
  console.log(`Credential Definition created for id ${credentialDefinitionId}. Receipt: ${JSON.stringify(receipt)}`)

  console.log('6. Trustee resolves Test Credential Definition to ensure its written')
  const resolvedCredentialDefinition = await faber.credentialDefinitionRegistry.resolveCredentialDefinition(
    credentialDefinitionId,
  )
  console.log(
    `Credential Definition resolved for ${credentialDefinitionId}. Credential Definition: ${resolvedCredentialDefinition.credDef}`,
  )

  console.log("7. ALice resolves Faber's Did Document")
  const faberDidDocument = await alice.didRegistry.resolveDid(issuerAddress)
  console.log(`Did Document resolved for ${issuerId}. DID Document: ${faberDidDocument?.document}`)

  console.log('8. Alice resolves Test Schema')
  const testSchema = await alice.schemaRegistry.resolveSchema(schemaId)
  console.log(`Schema resolved for ${schemaId}. Schema: ${testSchema.schema}`)

  console.log('9. Alice resolves Test Credential Definition')
  const testCredentialDefinition = await alice.credentialDefinitionRegistry.resolveCredentialDefinition(
    credentialDefinitionId,
  )
  console.log(
    `Credential Definition resolved for ${credentialDefinitionId}. Credential Definition: ${testCredentialDefinition.credDef}`,
  )

  // Revocations

  console.log('10. Faber creates Test Revocation Registry')
  const { revRegId, revReg } = createRevocationRegistryObject({
    issuerId,
  })

  receipt = await faber.revocationRegistry.createRevocationRegistry(
    issuerAddress,
    credentialDefinitionId,
    revRegId,
    revReg,
  )
  console.log(`Revocation Registry created for id ${revRegId}. Receipt: ${JSON.stringify(receipt)}`)

  console.log('11. Alice resolves the Revocation Registry')
  const resolvedRevocationRegistry = await alice.revocationRegistry.resolveRevocation(revRegId)
  const replacer = (key, value) => (typeof value === 'bigint' ? value.toString() : value)
  verify = JSON.stringify(resolvedRevocationRegistry, replacer, 2)
  if (verify === '"0"') {
    verify = 'Revocation Registry actived'
  } else {
    verify = 'Revocation Registry is Not actived'
  }
  console.log(`Revocation Registry resolved for ${revRegId}. Revocation Registry: ${verify}`)

  console.log('12. Faber revokes a credential')
  receipt = await faber.revocationRegistry.revokeCredential(issuerAddress, revRegId)
  console.log(`Credential revoked. Receipt: ${receipt}`)

  console.log('12. Alice checks if the credential is revoked')
  const isRevoked = await alice.revocationRegistry.resolveRevocation(revRegId)
  verify = JSON.stringify(isRevoked, replacer, 2)
  if (verify === '"2"') {
    console.log(`Credential is revoked!`)
  } else {
    console.log(`Credential is not revoked!`)
  }
}

if (require.main === module) {
  demo()
}

module.exports = exports = demo
