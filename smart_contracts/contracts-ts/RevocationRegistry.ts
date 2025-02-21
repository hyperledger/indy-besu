/**
 * Copyright (c) 2024 DSR Corporation, Denver, Colorado.
 * https://www.dsr-corporation.com
 * SPDX-License-Identifier: Apache-2.0
 */

import { AbiCoder, concat, getBytes, keccak256, Signature, solidityPacked, toUtf8Bytes, toUtf8String } from 'ethers'
import {
  RevocationRegistryDefinitionMetadataStruct,
  RevocationRegistryEntryStruct,
} from '../typechain-types/contracts/anoncreds/RevocationRegistry'
import { Contract } from '../utils/contract'

export type RevocationRegistryDefinitionRecord = {
  revRegDef: string
  metadata: RevocationRegistryDefinitionMetadataStruct
}

export class RevocationRegistry extends Contract {
  constructor(sender?: any) {
    super(RevocationRegistry.name, sender)
  }

  public async createRevocationRegistryDefinition(
    identity: string,
    id: string,
    credDefId: string,
    issuerId: string,
    revRegDef: string,
  ) {
    const tx = await this.instance.createRevocationRegistryDefinition(
      identity,
      keccak256(toUtf8Bytes(id)),
      keccak256(toUtf8Bytes(credDefId)),
      issuerId,
      toUtf8Bytes(revRegDef),
    )
    return tx.wait()
  }

  public async createRevocationRegistryDefinitionSigned(
    identity: string,
    id: string,
    credDefId: string,
    issuerId: string,
    revRegDef: string,
    signature: Signature,
  ) {
    const tx = await this.instance.createRevocationRegistryDefinitionSigned(
      identity,
      signature.v,
      signature.r,
      signature.s,
      keccak256(toUtf8Bytes(id)),
      keccak256(toUtf8Bytes(credDefId)),
      issuerId,
      toUtf8Bytes(revRegDef),
    )
    return tx.wait()
  }

  public async createRevocationRegistryEntry(
    identity: string,
    revRegId: string,
    issuerId: string,
    revRegEntry: RevocationRegistryEntryStruct,
  ) {
    const tx = await this.instance.createRevocationRegistryEntry(
      identity,
      keccak256(toUtf8Bytes(revRegId)),
      issuerId,
      revRegEntry,
    )
    return tx.wait()
  }

  public async createRevocationRegistryEntrySigned(
    identity: string,
    revRegDefId: string,
    issuerId: string,
    revRegEntry: RevocationRegistryEntryStruct,
    signature: Signature,
  ) {
    const tx = await this.instance.createRevocationRegistryEntrySigned(
      identity,
      signature.v,
      signature.r,
      signature.s,
      keccak256(toUtf8Bytes(revRegDefId)),
      issuerId,
      revRegEntry,
    )
    return tx.wait()
  }

  public async resolveRevocationRegistryDefinition(id: string): Promise<RevocationRegistryDefinitionRecord> {
    const record = await this.instance.resolveRevocationRegistryDefinition(keccak256(toUtf8Bytes(id)))
    return {
      revRegDef: toUtf8String(getBytes(record.revRegDef)),
      metadata: {
        created: record.metadata.created,
        issuerId: record.metadata.issuerId,
        currentAccumulator: record.metadata.currentAccumulator,
      },
    }
  }

  public async fetchAllRevocationEntries(id: string): Promise<RevocationRegistryEntryStruct[]> {
    const eventLogs = await this.instance.queryFilter(
      this.instance.filters.RevocationRegistryEntryCreated(keccak256(toUtf8Bytes(id))),
    )
    const revRegEntries = eventLogs.map((log) => log.args.revRegEntry.toObject(true))
    return revRegEntries
  }

  public signCreateRevRegDefEndorsementData(
    identity: string,
    privateKey: Uint8Array,
    id: string,
    credDefId: string,
    issuerId: string,
    revRegDef: string,
  ) {
    return this.signEndorsementData(
      privateKey,
      concat([
        identity,
        toUtf8Bytes('createRevocationRegistryDefinition'),
        getBytes(keccak256(toUtf8Bytes(id)), 'hex'),
        getBytes(keccak256(toUtf8Bytes(credDefId)), 'hex'),
        toUtf8Bytes(issuerId),
        getBytes(toUtf8Bytes(revRegDef), 'hex'),
      ]),
    )
  }

  public signCreateRevRegEntryEndorsementData(
    identity: string,
    privateKey: Uint8Array,
    revRegDefId: string,
    issuerId: string,
    revRegEntry: RevocationRegistryEntryStruct,
  ) {
    const revRegEntrySolidityStruct = ['tuple(bytes,bytes,uint32[],uint32[],uint64)']

    return this.signEndorsementData(
      privateKey,
      concat([
        identity,
        toUtf8Bytes('createRevocationRegistryEntry'),
        getBytes(keccak256(toUtf8Bytes(revRegDefId)), 'hex'),
        toUtf8Bytes(issuerId),
        getBytes(new AbiCoder().encode(revRegEntrySolidityStruct, [Object.values(revRegEntry)])),
      ]),
    )
  }
}
