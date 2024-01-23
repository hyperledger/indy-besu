import { SchemaRecordStruct } from '../../typechain-types/contracts/cl/SchemaRegistryInterface'

export type SchemaRecord = SchemaRecordStruct

export function mapSchemaRecord(record: SchemaRecord) {
  return {
    schema: record.schema,
    metadata: {
      created: record.metadata.created,
    },
  }
}
