import { writeJson } from '../../utils'
import { outFile } from './config'
import {
  accountControl,
  credentialDefinitionRegistry,
  ethereumDidRegistry,
  indyDidRegistry,
  legacyMappingRegistry,
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
    ...legacyMappingRegistry(),
  }
  writeJson(contracts, outFile)
}

if (require.main === module) {
  main()
}
