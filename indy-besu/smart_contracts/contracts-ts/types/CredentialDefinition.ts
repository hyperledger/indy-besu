import { CredentialDefinitionRecordStruct } from '../../typechain-types/contracts/cl/CredentialDefinitionRegistryInterface'

export type CredentialDefinitionRecord = CredentialDefinitionRecordStruct

export function mapCredentialDefinitionRecord(data: CredentialDefinitionRecordStruct) {
  return {
    credDef: data.credDef,
    metadata: {
      created: data.metadata.created,
    },
  }
}
