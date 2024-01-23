import { Contract } from '../utils/contract'
import { mapSchemaRecord, SchemaRecord } from './types'

export class SchemaRegistry extends Contract {
  public static readonly defaultAddress = '0x0000000000000000000000000000000000005555'

  constructor(sender?: any) {
    super(SchemaRegistry.name, sender)
  }

  public async createSchema(id: string, issuerId: string, schema: string) {
    const tx = await this.instance.createSchema(id, issuerId, schema)
    return tx.wait()
  }

  public async resolveSchema(id: string): Promise<SchemaRecord> {
    const result = await this.instance.resolveSchema(id)
    return mapSchemaRecord(result)
  }
}
