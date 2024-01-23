import { Contract } from '../utils/contract'
import { DidRecord, mapDidRecord } from './types'

export class IndyDidRegistry extends Contract {
  public static readonly defaultAddress = '0x0000000000000000000000000000000000003333'

  constructor(sender?: any) {
    super(IndyDidRegistry.name, sender)
  }

  public async createDid(identity: string, did: string, document: string) {
    const tx = await this.instance.createDid(identity, did, document)
    return tx.wait()
  }

  public async updateDid(did: string, document: string) {
    const tx = await this.instance.updateDid(did, document)
    return tx.wait()
  }

  public async deactivateDid(did: string) {
    const tx = await this.instance.deactivateDid(did)
    return tx.wait()
  }

  public async resolveDid(did: string): Promise<DidRecord> {
    const didRecord = await this.instance.resolveDid(did)
    return mapDidRecord(didRecord)
  }
}
