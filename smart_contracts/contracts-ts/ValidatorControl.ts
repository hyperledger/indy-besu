import { Contract } from '../utils/contract'

export class ValidatorControl extends Contract {
  constructor(sender?: any) {
    super(ValidatorControl.name, sender)
  }

  public async getValidators() {
    return this.instance.getValidators()
  }

  public async addValidator(address: string) {
    const tx = await this.instance.addValidator(address)
    return tx.wait()
  }

  public async removeValidator(address: string) {
    return this.instance.removeValidator(address)
  }
}
