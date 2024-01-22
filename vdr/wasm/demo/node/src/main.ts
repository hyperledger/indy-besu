import fs from "fs";
import secp256k1 from "secp256k1";

import { LedgerClient, EthrDidRegistry } from "indy2-vdr";

const chainId = 1337
const nodeAddress = 'http://127.0.0.1:8545'
// set path to the compiled contract
const didEthrRegistryConfig = {
    address: '0x0000000000000000000000000000000000018888',
    specPath: '/Users/indy-besu/smart_contracts/artifacts/contracts/did/EthereumExtDidRegistry.sol/EthereumExtDidRegistry.json'
}

const trustee = {
    address: '0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5',
    secret: Uint8Array.from([ 139, 187, 177, 179, 69, 175, 86, 181, 96, 165, 178, 11, 212, 176, 237, 28, 216, 204, 153, 88, 161, 98, 98, 188, 117, 17, 132, 83, 203, 84, 109, 247 ])
}
const identity = {
    address: '0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5',
    secret: Uint8Array.from([17, 230, 90, 7, 166, 132, 10, 84, 177, 140, 139, 18, 254, 94, 235, 200, 171, 63, 41, 29, 76, 110, 166, 107, 86, 7, 198, 74, 34, 146, 157, 66])
}

async function demo() {
    console.log('1. Init client')
    const contractConfigs = [
        {
            "address": didEthrRegistryConfig.address,
            "spec": JSON.parse(fs.readFileSync(didEthrRegistryConfig.specPath, 'utf8')),
        }
    ]
    const client = new LedgerClient(chainId, nodeAddress, contractConfigs, null)
    const status = await client.ping()
    console.log('Status: ' + JSON.stringify(status, null, 2))

    console.log('2. Publish and Modify DID')

    console.log('2.1 Publish Service DID Attribute')
    const did = 'did:ethr:' + identity.address
    const serviceAttribute = {"serviceEndpoint":"http://10.0.0.2","type":"TestService"}
    let endorsingData = await EthrDidRegistry.buildDidSetAttributeEndorsingData(client, did, serviceAttribute, BigInt(100000))
    let endorsingBytesToSign = endorsingData.getSigningBytes()
    let signature = secp256k1.ecdsaSign(endorsingBytesToSign, identity.secret)
    let authorSignature = {
        recovery_id: signature.recid,
        signature: signature.signature
    }

    let transaction = await EthrDidRegistry.buildDidSetAttributeSignedTransaction(client, trustee.address, did, serviceAttribute, BigInt(100000), authorSignature)
    let bytesToSign = transaction.getSigningBytes()
    signature = secp256k1.ecdsaSign(bytesToSign, trustee.secret)
    let transactionSignature = {
        recovery_id: signature.recid,
        signature: signature.signature
    }
    transaction.setSignature(transactionSignature)
    let txnHash = await client.submitTransaction(transaction)
    let receipt = await client.getReceipt(txnHash)
    console.log('Transaction receipt: ' + receipt)

    console.log('3. Resolve DID Document')
    const didWithMeta = await EthrDidRegistry.resolveDid(client, did, null)
    console.log('Resolved DID Document: ' + JSON.stringify(didWithMeta, null, 2))
}

async function main() {
    await demo()
}

main()
