import { Contract } from '../utils/contract'
import { CredentialDefinitionRecord, mapCredentialDefinitionRecord } from './types'

export class CredentialDefinitionRegistry extends Contract {
  public static readonly defaultAddress = '0x0000000000000000000000000000000000004444'

  constructor(sender?: any) {
    super(CredentialDefinitionRegistry.name, sender)
  }

  public async createCredentialDefinition(id: string, issuerId: string, schemaId: string, credDef: string) {
    const tx = await this.instance.createCredentialDefinition(id, issuerId, schemaId, credDef)
    return tx.wait()
  }

  public async resolveCredentialDefinition(id: string): Promise<CredentialDefinitionRecord> {
    const result = await this.instance.resolveCredentialDefinition(id)
    return mapCredentialDefinitionRecord(result)
  }
}
