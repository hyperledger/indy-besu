import { getBytes, keccak256, Signature, toUtf8Bytes, toUtf8String } from 'ethers'
import { CredentialDefinitionCreatedEvent } from '../typechain-types/contracts/cl/CredentialDefinitionRegistry'
import { Contract } from '../utils/contract'

export class CredentialDefinitionRegistry extends Contract {
  public static readonly defaultAddress = '0x0000000000000000000000000000000000004444'

  constructor(sender?: any) {
    super(CredentialDefinitionRegistry.name, sender)
  }

  public async createCredentialDefinition(identity: string, id: string, schemaId: string, credDef: string) {
    const tx = await this.instance.createCredentialDefinition(
      identity,
      keccak256(toUtf8Bytes(id)),
      keccak256(toUtf8Bytes(schemaId)),
      toUtf8Bytes(credDef),
    )
    return tx.wait()
  }

  public async createCredentialDefinitionSigned(
    identity: string,
    id: string,
    schemaId: string,
    credDef: string,
    signature: Signature,
  ) {
    const tx = await this.instance.createCredentialDefinitionSigned(
      identity,
      signature.v,
      signature.r,
      signature.s,
      keccak256(toUtf8Bytes(id)),
      keccak256(toUtf8Bytes(schemaId)),
      toUtf8Bytes(credDef),
    )
    return tx.wait()
  }

  public async created(id: string): Promise<number> {
    return this.instance.created(keccak256(toUtf8Bytes(id)))
  }

  public async resolveCredentialDefinition(id: string): Promise<string> {
    const filer = await this.instance.filters.CredentialDefinitionCreated(keccak256(toUtf8Bytes(id)))
    const events = await this.instance.queryFilter(filer)
    return toUtf8String(getBytes(events[0].args[2]))
  }
}
