import environment from '../environment'
import { Actor } from './utils/actor'
import { ROLES } from '../contracts-ts'
import { createCredentialDefinitionObject, createSchemaObject, createRevocationRegistryObject } from '../utils'

async function demo() {
  let receipt: any

  const trustee = await new Actor(environment.accounts.account1).init()
  const faber = await new Actor().init()
  const alice = await new Actor().init()

  console.log('1. Trustee assign ENDORSER role to Faber')
  receipt = await trustee.roleControl.assignRole(ROLES.ENDORSER, faber.address)
  console.log(`Role ${ROLES.ENDORSER} assigned to account ${faber.address}. Receipt: ${JSON.stringify(receipt)}`)

  console.log('2. Faber creates DID Document')
  receipt = await faber.didRegistry.createDid(faber.address, faber.didDocument)
  console.log(`Did Document created for DID ${faber.did}. Receipt: ${JSON.stringify(receipt)}`)

  console.log('3. Faber creates Test Schema')
  const { id: schemaId, schema } = createSchemaObject({ issuerId: faber.did })
  receipt = await faber.schemaRegistry.createSchema(faber.address, schemaId, faber.did, schema)
  console.log(`Schema created for id ${schemaId}. Receipt: ${JSON.stringify(receipt)}`)

  console.log('4. Faber resolves Test Schema to ensure its written')
  const resolvedSchema = await faber.schemaRegistry.resolveSchema(schemaId)
  console.log(`Schema resolved for ${schemaId}. Schema: ${resolvedSchema.schema}`)

  console.log('5. Faber create Test Credential Definition')
  const { id: credentialDefinitionId, credDef: credentialDefinition } = createCredentialDefinitionObject({
    issuerId: faber.did,
    schemaId: schemaId,
  })
  receipt = await faber.credentialDefinitionRegistry.createCredentialDefinition(
    faber.address,
    credentialDefinitionId,
    faber.did,
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

  console.log("7. Alice resolves Faber's Did Document")
  const faberDidDocument = await alice.didRegistry.resolveDid(faber.address)
  console.log(`Did Document resolved for ${faber.did}. DID Document: ${faberDidDocument?.document}`)

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

  // Adicionando operações de revogação

  console.log('10. Faber cria o Registro de Revogação')
  const { revRegId, revReg } = createRevocationRegistryObject({
    issuerId: faber.did,
    schemaId: schemaId,
    credDefId: credentialDefinitionId,
  })
  receipt = await faber.revocationRegistry.createRevocationRegistry(
    faber.address,
    revRegId,
    faber.did,
    revReg
  )
  console.log(`Revocation Registry created for id ${revRegId}. Receipt: ${JSON.stringify(receipt)}`)

  console.log('11. Alice resolve o Registro de Revogação')
  const resolvedRevocationRegistry = await alice.revocationRegistry.resolveRevocationRegistry(revRegId)
  console.log(`Revocation Registry resolved for ${revRegId}. Revocation Registry: ${resolvedRevocationRegistry.revReg}`)

  console.log('12. Faber revoga uma credencial')
  receipt = await faber.revocationRegistry.revokeCredential(faber.address, revRegId, 'credentialId')
  console.log(`Credential revoked. Receipt: ${JSON.stringify(receipt)}`)

  console.log('13. Alice verifica se a credencial foi revogada')
  const isRevoked = await alice.revocationRegistry.isCredentialRevoked(revRegId, 'credentialId')
  console.log(`Credential is revoked: ${isRevoked}`)
}

if (require.main === module) {
  demo()
}

module.exports = exports = demo
