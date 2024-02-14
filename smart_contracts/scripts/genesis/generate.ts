import { writeJson } from '../../utils'
import { outFile } from './config'
import {
  accountControl,
  credentialDefinitionRegistry,
  ethereumDidRegistry,
  indyDidRegistry,
  roleControl,
  schemaRegistry,
  universalDidResolver,
  upgradeControl,
  validatorControl,
} from './contracts'

function main() {
  const contracts = {
    ...accountControl(),
    ...roleControl(),
    ...validatorControl(),
    ...upgradeControl(),
    ...indyDidRegistry(),
    ...ethereumDidRegistry(),
    ...universalDidResolver(),
    ...schemaRegistry(),
    ...credentialDefinitionRegistry(),
  }
  writeJson(contracts, outFile)
}

if (require.main === module) {
  main()
}
