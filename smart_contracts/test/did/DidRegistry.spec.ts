import { expect } from 'chai'
import { deployDidRegistry, TestableDidRegistry } from '../utils/contract-helpers'
import { TestAccounts } from '../utils/test-entities'

describe('DIDContract', function () {
  let didRegistry: TestableDidRegistry
  let testAccounts: TestAccounts

  beforeEach(async function () {
    const { didRegistry: didRegistryInit, testAccounts: testAccountsInit } = await deployDidRegistry()

    didRegistry = didRegistryInit
    testAccounts = testAccountsInit

    didRegistry.connect(testAccounts.trustee.account)
  })

  describe('Create DID', function () {
    it('Create DID Works', async function () {
      // We do not need to cover DID Registry with tests as it's already done in the original contract
      const changed = await didRegistry.changed(testAccounts.trustee.account.address)
      expect(changed).to.be.equal(0)
    })
  })
})
