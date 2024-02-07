import { Signature } from 'ethers'
import { Contract } from '../utils/contract'

export class LegacyMappingRegistry extends Contract {
  public static readonly defaultAddress = '0x0000000000000000000000000000000000019999'

  constructor(sender?: any) {
    super(LegacyMappingRegistry.name, sender)
  }

  public async createDidMapping(
    identity: string,
    identifier: string,
    ed25519Key: Uint8Array,
    ed25519Signature: Uint8Array,
  ) {
    const tx = await this.instance.createDidMapping(identity, identifier, ed25519Key, ed25519Signature)
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
      identifier,
      ed25519Key,
      ed25519Signature,
    )
    return tx.wait()
  }

  public async didMapping(id: string): Promise<string> {
    return this.instance.didMapping(id)
  }

  public async createResourceMapping(
    identity: string,
    legacyIssuerIdentifier: string,
    legacyIdentifier: string,
    newIdentifier: string,
  ) {
    const tx = await this.instance.createResourceMapping(
      identity,
      legacyIssuerIdentifier,
      legacyIdentifier,
      newIdentifier,
    )
    return tx.wait()
  }

  public async createResourceMappingSigned(
    identity: string,
    legacyIssuerIdentifier: string,
    legacyIdentifier: string,
    newIdentifier: string,
    signature: Signature,
  ) {
    const tx = await this.instance.createResourceMappingSigned(
      identity,
      signature.v,
      signature.r,
      signature.s,
      legacyIssuerIdentifier,
      legacyIdentifier,
      newIdentifier,
    )
    return tx.wait()
  }

  public async resourceMapping(id: string): Promise<string> {
    return this.instance.resourceMapping(id)
  }
}
