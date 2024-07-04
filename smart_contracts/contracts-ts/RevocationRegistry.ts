import { concat, getBytes, keccak256, Signature, toUtf8Bytes, toUtf8String } from 'ethers'
import { RevocationMetadataStruct } from '../typechain-types/contracts/revoke/RevocationRegistry'
import { Contract } from '../utils/contract'

export type RevocationRecord = {
  revocation: string
  metadata: RevocationMetadataStruct
}

export class RevocationRegistry extends Contract {
  constructor(sender?: any) {
    super(RevocationRegistry.name, sender)
  }

  public async createRevocationRegistry(identity: string, id: string, revocation: string) {
    const tx = await this.instance.createRevocationRegistry(
      identity,
      keccak256(toUtf8Bytes(id)),
      toUtf8Bytes(revocation),
    )
    return tx.wait()
  }

  public async createCredentialDefinitionSigned(
    identity: string,
    id: string,
    revocation: string,
    signature: Signature,
  ) {
    const tx = await this.instance.createRevocationRegistrySigned(
      identity,
      signature.v,
      signature.r,
      signature.s,
      keccak256(toUtf8Bytes(id)),
      toUtf8Bytes(revocation),
    )
    return tx.wait()
  }

  public async resolveRevocation(id: string): Promise<RevocationRecord> {
    const record = await this.instance.resolveRevocation(keccak256(toUtf8Bytes(id)))
    return {
      revocation: toUtf8String(getBytes(record.credDef)),
      metadata: {
        created: record.metadata.created,
        creator: record.metadata.creator,
        updated: record.metadata.updated,
        status: record.metadata.status,
      },
    }
  }

  public signCreateRevocationWithEndorsementData(
    identity: string,
    privateKey: Uint8Array,
    id: string,
    revocation: string,
  ) {
    return this.signEndorsementData(
      privateKey,
      concat([
        identity,
        toUtf8Bytes('createRevocationRegistry'),
        getBytes(keccak256(toUtf8Bytes(id)), 'hex'),
        toUtf8Bytes(revocation),
      ]),
    )
  }

  public async revokeCredential(identity: string, id: string) {
    const tx = await this.instance.revokeCredential(identity, keccak256(toUtf8Bytes(id)))
    return tx.wait()
  }

  public async revokeCredentialSigned(identity: string, signature: Signature, id: string) {
    const tx = await this.instance.revokeCredentialSigned(
      identity,
      signature.v,
      signature.r,
      signature.s,
      keccak256(toUtf8Bytes(id)),
    )
    return tx.wait()
  }

  public async suspendCredential(identity: string, id: string) {
    const tx = await this.instance.suspendCredential(identity, keccak256(toUtf8Bytes(id)))
    return tx.wait()
  }

  public async suspendCredentialSigned(identity: string, signature: Signature, id: string) {
    const tx = await this.instance.suspendCredentialSigned(
      identity,
      signature.v,
      signature.r,
      signature.s,
      keccak256(toUtf8Bytes(id)),
    )
    return tx.wait()
  }

  public async unrevokeCredential(identity: string, id: string) {
    const tx = await this.instance.unrevokeCredential(identity, keccak256(toUtf8Bytes(id)))
    return tx.wait()
  }

  public async unrevokeCredentialSigned(identity: string, signature: Signature, id: string) {
    const tx = await this.instance.unrevokeCredentialSigned(
      identity,
      signature.v,
      signature.r,
      signature.s,
      keccak256(toUtf8Bytes(id)),
    )
    return tx.wait()
  }
}
