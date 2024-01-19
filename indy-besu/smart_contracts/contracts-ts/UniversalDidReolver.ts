import { Contract } from '../utils'
import { DidMetadata, mapDidMetadata } from './types'

export class UniversalDidResolver extends Contract {
  public static readonly defaultAddress = '0x000000000000000000000000000000000019999'

  constructor(sender?: any) {
    super(UniversalDidResolver.name, sender)
  }

  public async resolveDocument(id: string): Promise<string> {
    return this.instance.resolveDocument(id)
  }

  public async resolveMetadata(id: string): Promise<DidMetadata> {
    const metadata = await this.instance.resolveMetadata(id)
    return mapDidMetadata(metadata)
  }
}
