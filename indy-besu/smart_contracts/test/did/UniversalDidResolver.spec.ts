import { loadFixture } from '@nomicfoundation/hardhat-network-helpers'
import { expect } from 'chai'
import { UniversalDidResolver } from '../../contracts-ts'
import { createBaseDidDocument } from '../../utils'
import { deployUniversalDidResolver, EthereumDIDRegistry } from '../utils/contract-helpers'
import { TestAccounts } from '../utils/test-entities'

describe('UniversalDidResolver', function () {
  const indy2DidDocument = createBaseDidDocument('did:indy2:testnet:SEp33q43PsdP7nDATyySSH')
  let universalDidReolver: UniversalDidResolver
  let testAccounts: TestAccounts

  async function deployUniversalDidResolverFixture() {
    const {
      universalDidReolver: universalDidReolverInit,
      didRegistry: didRegistryInit,
      testAccounts: testAccountsInit,
    } = await deployUniversalDidResolver()

    didRegistryInit.connect(testAccounts.trustee.account)
    await didRegistryInit.createDid(indy2DidDocument)

    return { universalDidReolverInit, testAccountsInit }
  }

  beforeEach(async function () {
    const { universalDidReolverInit, testAccountsInit } = await loadFixture(deployUniversalDidResolverFixture)

    universalDidReolver = universalDidReolverInit
    testAccounts = testAccountsInit

    universalDidReolver.connect(testAccounts.trustee.account)
  })

  describe('Resolve did:indy2', function () {
    it('Should resolve DID document', async function () {
      const document = await universalDidReolver.resolveDocument(indy2DidDocument.id)

      expect(document).to.be.deep.equal(indy2DidDocument)
    })

    it('Should resolve DID metadata', async function () {
      const metadata = await universalDidReolver.resolveMetadata(indy2DidDocument.id)

      expect(metadata).to.contain({
        creator: testAccounts.trustee.account.address,
        deactivated: false,
      })
    })
  })

  describe('Resolve did:ethr', function () {
    it('Should resolve DID metadata', async function () {
      const metadata = await universalDidReolver.resolveMetadata(
        `did:ethr:${testAccounts.trustee.account.address.substring(2)}`,
      )

      expect(metadata).to.contain({
        creator: testAccounts.trustee.account.address,
        deactivated: false,
      })
    })
  })
})
