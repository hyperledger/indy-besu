import { getBytes, keccak256, Signature, toUtf8Bytes, toUtf8String } from 'ethers'
import { SchemaCreatedEvent } from '../typechain-types/contracts/cl/SchemaRegistryInterface'
import { Contract } from '../utils/contract'

export class SchemaRegistry extends Contract {
  public static readonly defaultAddress = '0x0000000000000000000000000000000000005555'

  constructor(sender?: any) {
    super(SchemaRegistry.name, sender)
  }

  public async createSchema(identity: string, id: string, schema: string) {
    const tx = await this.instance.createSchema(identity, keccak256(toUtf8Bytes(id)), toUtf8Bytes(schema))
    return tx.wait()
  }

  public async createSchemaSigned(identity: string, id: string, schema: string, signature: Signature) {
    const tx = await this.instance.createSchemaSigned(
      identity,
      signature.v,
      signature.r,
      signature.s,
      keccak256(toUtf8Bytes(id)),
      toUtf8Bytes(schema),
    )
    return tx.wait()
  }

  public async created(id: string): Promise<number> {
    return this.instance.created(keccak256(toUtf8Bytes(id)))
  }

  public async resolveSchema(schemaId: string): Promise<string> {
    const filer = await this.instance.filters.SchemaCreated(keccak256(toUtf8Bytes(schemaId)))
    const events = await this.instance.queryFilter(filer)
    return toUtf8String(getBytes(events[0].args[2]))
  }
}
