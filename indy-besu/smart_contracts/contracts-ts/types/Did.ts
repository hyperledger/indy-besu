import { DidMetadataStruct, DidRecordStruct } from '../../typechain-types/contracts/did/IndyDidRegistry'

export type DidRecord = DidRecordStruct
export type DidMetadata = DidMetadataStruct

export function mapDidMetadata(metadata: DidMetadata) {
  return {
    owner: metadata.owner,
    sender: metadata.sender,
    created: metadata.created,
    updated: metadata.updated,
    deactivated: metadata.deactivated,
  }
}

export function mapDidRecord(record: DidRecord) {
  return {
    document: record.document,
    metadata: mapDidMetadata(record.metadata),
  }
}
