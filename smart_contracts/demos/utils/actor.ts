import { readFileSync } from 'fs'
import { resolve } from 'path'
import {
  RoleControl,
  IndyDidRegistry,
  SchemaRegistry,
  CredentialDefinitionRegistry,
  ValidatorControl,
  UpgradeControl,
  EthereumExtDidRegistry,
} from '../../contracts-ts'
import { Account, AccountInfo } from '../../utils'

export class Actor {
  public account: Account
  public roleControl!: RoleControl
  public validatorControl!: ValidatorControl
  public didRegistry!: IndyDidRegistry
  public ethereumDIDRegistry!: EthereumExtDidRegistry
  public schemaRegistry!: SchemaRegistry
  public credentialDefinitionRegistry!: CredentialDefinitionRegistry
  public upgradeControl!: UpgradeControl

  constructor(accountInfo?: AccountInfo) {
    this.account = accountInfo ? new Account(accountInfo) : new Account()
  }

  public async init() {
    const addresses = this.readContractsAddresses()

    this.roleControl = await new RoleControl(this.account).getInstance(addresses['role_control'])
    this.validatorControl = await new ValidatorControl(this.account).getInstance(addresses['validator_control'])
    this.didRegistry = await new IndyDidRegistry(this.account).getInstance(addresses['indy_did_registry'])
    this.ethereumDIDRegistry = await new EthereumExtDidRegistry(this.account).getInstance(
      addresses['ethereum_did_registry'],
    )
    this.schemaRegistry = await new SchemaRegistry(this.account).getInstance(addresses['schema_registry'])
    this.credentialDefinitionRegistry = await new CredentialDefinitionRegistry(this.account).getInstance(
      addresses['cred_def_registry'],
    )
    this.upgradeControl = await new UpgradeControl(this.account).getInstance(addresses['upgrade_control'])
    return this
  }

  public get address() {
    return this.account.address
  }

  public get did() {
    return this.account.did
  }

  public get didEthr() {
    return this.account.didEthr
  }

  public get didDocument() {
    return this.account.didDocument
  }

  private readContractsAddresses(): { [key: string]: string } {
    const configPath = resolve('..', 'config.json')

    const data = readFileSync(configPath, 'utf8')
    const contracts = JSON.parse(data).contracts

    const result = {}

    for (const contractName of Object.keys(contracts)) {
      result[contractName] = contracts[contractName].address
    }

    return result
  }
}
