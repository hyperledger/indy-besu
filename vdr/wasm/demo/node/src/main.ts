import fs from "fs";
import secp256k1 from "secp256k1";
import bs58 from "bs58";
import { randomBytes } from 'crypto'

import { LedgerClient, IndyDidRegistry, EthrDidRegistry } from "indy2-vdr";

const chainId = 1337
const nodeAddress = 'http://127.0.0.1:8545'
// set path to the compiled contract
const didIndyRegistryConfig = {
    address: '0x0000000000000000000000000000000000003333',
    specPath: '/Users/indy-besu/smart_contracts/artifacts/contracts/did/IndyDidRegistry.sol/IndyDidRegistry.json'
}
const didEthrRegistryConfig = {
    address: '0x0000000000000000000000000000000000018888',
    specPath: '/Users/indy-besu/smart_contracts/artifacts/contracts/did/EthereumExtDidRegistry.sol/EthereumExtDidRegistry.json'
}

const account = '0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5'
const identity = '0xb9059400dcd05158ffd8ca092937989dd27b3bdc'
const secret = Uint8Array.from([ 139, 187, 177, 179, 69, 175, 86, 181, 96, 165, 178, 11, 212, 176, 237, 28, 216, 204, 153, 88, 161, 98, 98, 188, 117, 17, 132, 83, 203, 84, 109, 247 ])

async function didIndyDemo(client: LedgerClient) {
    console.log('2. Publish DID Document')
    const did = 'did:indy2:' + bs58.encode(randomBytes(16))
    const kid = did + '#KEY-1'
    const didDoc = {
        "@context": [ "https://www.w3.org/ns/did/v1" ],
        "id": did,
        "verificationMethod": [
            {
                "controller": did,
                "id": kid,
                "publicKeyMultibase": "zAKJP3f7BD6W4iWEQ9jwndVTCBq8ua2Utt8EEjJ6Vxsf",
                "type": "Ed25519VerificationKey2018"
            }
        ],
        "authentication": [ kid ]
    }
    console.log('DID Document: ' + JSON.stringify(didDoc, null, 2))
    let transaction = await IndyDidRegistry.buildCreateDidTransaction(client, account, identity, did, didDoc)
    const bytesToSign = transaction.getSigningBytes()
    const signature = secp256k1.ecdsaSign(bytesToSign, secret)
    transaction.setSignature({
        recovery_id: signature.recid,
        signature: signature.signature
    })

    const txnHash = await client.submitTransaction(transaction)
    console.log('Transaction hash: ' + txnHash)

    const receipt = await client.getReceipt(txnHash)
    console.log('Transaction receipt: ' + receipt)

    console.log('3. Resolve DID Document')
    transaction = await IndyDidRegistry.buildResolveDidTransaction(client, didDoc.id)
    const response = await client.submitTransaction(transaction)
    const resolvedDidDoc = IndyDidRegistry.parseResolveDidResult(client, response)
    console.log('Resolved DID Document: ' + JSON.stringify(resolvedDidDoc, null, 2))
}

async function didEthrDemo(client: LedgerClient) {
    const did = 'did:ethr:' + account

    console.log('2. Publish Sevice DID Attribute')
    const serviceAttribute = {"serviceEndpoint":"http://10.0.0.2","type":"TestService"}
    let transaction = await EthrDidRegistry.buildDidSetAttributeTransaction(client, account, did, serviceAttribute, BigInt(100000))
    let bytesToSign = transaction.getSigningBytes()
    let signature = secp256k1.ecdsaSign(bytesToSign, secret)
    transaction.setSignature({
        recovery_id: signature.recid,
        signature: signature.signature
    })
    let txnHash = await client.submitTransaction(transaction)
    let receipt = await client.getReceipt(txnHash)
    console.log('Transaction receipt: ' + receipt)

    console.log('2. Publish Key DID Attribute')
    const keyAttribute = {"publicKeyBase58":"FbQWLPRhTH95MCkQUeFYdiSoQt8zMwetqfWoxqPgaq7x","purpose":"enc","type":"X25519KeyAgreementKey2020"}
    transaction = await EthrDidRegistry.buildDidSetAttributeTransaction(client, account, did, keyAttribute, BigInt(100000))
    bytesToSign = transaction.getSigningBytes()
    signature = secp256k1.ecdsaSign(bytesToSign, secret)
    transaction.setSignature({
        recovery_id: signature.recid,
        signature: signature.signature
    })
    txnHash = await client.submitTransaction(transaction)
    receipt = await client.getReceipt(txnHash)
    console.log('Transaction receipt: ' + receipt)

    console.log('3. Resolve DID Document')
    const didWithMeta = await EthrDidRegistry.resolveDid(client, did, null)
    console.log('Resolved DID Document: ' + JSON.stringify(didWithMeta, null, 2))
}

async function main() {
    console.log('1. Init client')
    const contractConfigs = [
        {
            "address": didIndyRegistryConfig.address,
            "spec": JSON.parse(fs.readFileSync(didIndyRegistryConfig.specPath, 'utf8')),
        },
        {
            "address": didEthrRegistryConfig.address,
            "spec": JSON.parse(fs.readFileSync(didEthrRegistryConfig.specPath, 'utf8')),
        }
    ]
    const client = new LedgerClient(chainId, nodeAddress, contractConfigs, null)
    const status = await client.ping()
    console.log('Status: ' + JSON.stringify(status, null, 2))

    console.log('DID Indy demo')
    await didIndyDemo(client)

    console.log('DID Ethr demo')
    await didEthrDemo(client)
}

main()
