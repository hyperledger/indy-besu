import { writeJson } from '../../utils'
import { outFile } from './config'
import {
  accountControl,
  credentialDefinitionRegistry,
  ethereumDidRegistry,
  roleControl,
  schemaRegistry,
  upgradeControl,
  validatorControl,
} from './contracts'

function main() {
  const contracts = {
    ...accountControl(),
    ...roleControl(),
    ...validatorControl(),
    ...upgradeControl(),
    ...ethereumDidRegistry(),
    ...schemaRegistry(),
    ...credentialDefinitionRegistry(),
  }
  writeJson(contracts, outFile)
}

if (require.main === module) {
  main()
}
