import bs58 from 'bs58'
import { Signature } from 'ethers'
import { Contract } from '../utils/contract'

export class LegacyIdentifiersRegistry extends Contract {
  public static readonly defaultAddress = '0x0000000000000000000000000000000000019999'

  constructor(sender?: any) {
    super(LegacyIdentifiersRegistry.name, sender)
  }

  public async createDidMapping(
    identity: string,
    identifier: string,
    ed25519Key: Uint8Array,
    ed25519Signature: Uint8Array,
  ) {
    const tx = await this.instance.createDidMapping(identity, bs58.decode(identifier), ed25519Key, ed25519Signature)
    return tx.wait()
  }

  public async createDidMappingSigned(
    identity: string,
    identifier: string,
    ed25519Key: Uint8Array,
    ed25519Signature: Uint8Array,
    signature: Signature,
  ) {
    const tx = await this.instance.createDidMappingSigned(
      identity,
      signature.v,
      signature.r,
      signature.s,
      bs58.decode(identifier),
      ed25519Key,
      ed25519Signature,
    )
    return tx.wait()
  }

  public async resolveNewDid(id: string): Promise<string> {
    return this.instance.didMapping(bs58.decode(id))
  }

  public async createClMapping(
    identity: string,
    legacyIssuerIdentifier: string,
    legacyIdentifier: string,
    newIdentifier: string,
  ) {
    const tx = await this.instance.createClMapping(
      identity,
      bs58.decode(legacyIssuerIdentifier),
      legacyIdentifier,
      newIdentifier,
    )
    return tx.wait()
  }

  public async createClMappingSigned(
    identity: string,
    legacyIssuerIdentifier: string,
    legacyIdentifier: string,
    newIdentifier: string,
    signature: Signature,
  ) {
    const tx = await this.instance.createClMappingSigned(
      identity,
      signature.v,
      signature.r,
      signature.s,
      bs58.decode(legacyIssuerIdentifier),
      legacyIdentifier,
      newIdentifier,
    )
    return tx.wait()
  }

  public async resolveNewId(id: string): Promise<string> {
    return this.instance.clMapping(id)
  }
}
