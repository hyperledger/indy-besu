export function createBaseDidDocument(did: string, key?: any) {
  const kid = `${did}#KEY-1`
  return JSON.stringify({
    '@context': ['https://www.w3.org/ns/did/v1'],
    id: did,
    controller: [did],
    verificationMethod: [
      key || {
        id: kid,
        type: 'Ed25519VerificationKey2018',
        controller: did,
        publicKeyMultibase: 'zAKJP3f7BD6W4iWEQ9jwndVTCBq8ua2Utt8EEjJ6Vxsf',
      },
    ],
    authentication: [kid],
    assertionMethod: [],
    capabilityInvocation: [],
    capabilityDelegation: [],
    keyAgreement: [],
    service: [],
    alsoKnownAs: [],
  })
}

interface CreateSchemaParams {
  issuerId: string
  name?: string
  version?: string
  attrNames?: string[]
}

export function createSchemaObject({
  issuerId,
  name = 'BasicIdentity',
  version = '1.0.0',
  attrNames = ['First Name', 'Last Name'],
}: CreateSchemaParams) {
  const id = `${issuerId}/anoncreds/v0/SCHEMA/${name}/${version}`
  return {
    id,
    schema: JSON.stringify({
      id,
      issuerId,
      name,
      version,
      attrNames,
    }),
  }
}

interface CreateCredentialDefinitionParams {
  issuerId: string
  schemaId: string
  credDefType?: string
  tag?: string
  value?: Record<string, string>
}

export function createCredentialDefinitionObject({
  issuerId,
  schemaId,
  credDefType = 'CL',
  tag = 'BasicIdentity',
  value = {
    n: '779...397',
    rctxt: '774...977',
    s: '750..893',
    z: '632...005',
  },
}: CreateCredentialDefinitionParams) {
  const id = `${issuerId}/anoncreds/v0/CLAIM_DEF/${schemaId}/${tag}`
  return {
    id,
    credDef: JSON.stringify({
      id,
      issuerId,
      schemaId,
      credDefType,
      tag,
      value,
    }),
  }
}

interface CreateRevocationRegistryParams {
  version?: string
  tag?: string
  revRegId?: string
  revRegDefType?: string
  credDefId?: string
  schemaId?: string
  issuanceType?: number
  tailsHash?: string
  issuerId: string
}

export function createRevocationRegistryObject({
  version = '1.0.0',
  issuerId,
  tag = 'BasicIdentity',
  revRegDefType = 'CL_ACCUM',
  schemaId = `${issuerId}/anoncreds/v0/SCHEMA/${tag}/${version}`,
  revRegId = `${issuerId}/anoncreds/v0/REV_REG/${schemaId}/auth_cred-def-este4:${revRegDefType}:0`,
  credDefId = `${issuerId}/anoncreds/v0/CLAIM_DEF/${schemaId}/${tag}`,
  issuanceType = 0,
  tailsHash = 'AAAAA',
}: CreateRevocationRegistryParams) {
  return {
    revRegId,
    revReg: JSON.stringify({
      version,
      revRegId,
      type: revRegDefType,
      credentialDefinitionId: credDefId,
      issuanceType,
      tailsHash,
      issuerId,
    }),
  }
}
