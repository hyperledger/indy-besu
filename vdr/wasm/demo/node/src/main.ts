import fs from "fs";
import secp256k1 from "secp256k1";

import { LedgerClient, EthrDidRegistry, SchemaRegistry } from "indy2-vdr";

const chainId = 1337
const nodeAddress = 'http://127.0.0.1:8545'
// set path to the compiled contract
const didEthrRegistryConfig = {
    address: '0x0000000000000000000000000000000000018888',
    specPath: '/Users/indy-besu/smart_contracts/artifacts/contracts/did/EthereumExtDidRegistry.sol/EthereumExtDidRegistry.json'
}
const schemaRegistryConfig = {
    address: '0x0000000000000000000000000000000000005555',
    specPath: '/Users/indy-besu/smart_contracts/artifacts/contracts/cl/SchemaRegistry.sol/SchemaRegistry.json'
}


const trustee = {
    address: '0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5',
    secret: Uint8Array.from([ 139, 187, 177, 179, 69, 175, 86, 181, 96, 165, 178, 11, 212, 176, 237, 28, 216, 204, 153, 88, 161, 98, 98, 188, 117, 17, 132, 83, 203, 84, 109, 247 ])
}
const identity = {
    address: '0xce70ce892768d46caf120b600dec29ed20198982',
    secret: Uint8Array.from([126, 218, 51, 235, 106, 56, 168, 226, 49, 234, 92, 61, 233, 13, 242, 75, 137, 130, 228, 222, 148, 239, 14, 63, 135, 13, 140, 163, 134, 166, 49, 50])
}

function sign(message: Uint8Array, key: Uint8Array) {
    let signature = secp256k1.ecdsaSign(message, key)
    return {
        recovery_id: signature.recid,
        signature: signature.signature
    }
}

async function demo() {
    console.log('1. Init client')
    const contractConfigs = [
        {
            "address": didEthrRegistryConfig.address,
            "spec": JSON.parse(fs.readFileSync(didEthrRegistryConfig.specPath, 'utf8')),
        },
        {
            "address": schemaRegistryConfig.address,
            "spec": JSON.parse(fs.readFileSync(schemaRegistryConfig.specPath, 'utf8')),
        }
    ]
    const client = new LedgerClient(chainId, nodeAddress, contractConfigs, null)
    const status = await client.ping()
    console.log('Status: ' + JSON.stringify(status, null, 2))

    console.log('2. Publish and Modify DID')
    const did = 'did:ethr:' + identity.address
    const serviceAttribute = {"serviceEndpoint":"http://10.0.0.2","type":"TestService"}
    let endorsingData = await EthrDidRegistry.buildDidSetAttributeEndorsingData(client, did, serviceAttribute, BigInt(100000))
    let authorSignature = sign(endorsingData.getSigningBytes(), identity.secret)
    let transaction = await EthrDidRegistry.buildDidSetAttributeSignedTransaction(client, trustee.address, did, serviceAttribute, BigInt(100000), authorSignature)
    let transactionSignature = sign(transaction.getSigningBytes(), trustee.secret)
    transaction.setSignature(transactionSignature)
    let txnHash = await client.submitTransaction(transaction)
    let receipt = await client.getReceipt(txnHash)
    console.log('Transaction receipt: ' + receipt)

    console.log('3. Resolve DID Document')
    const didWithMeta = await EthrDidRegistry.resolveDid(client, did, null)
    console.log('Resolved DID Document: ' + JSON.stringify(didWithMeta, null, 2))

    console.log('4. Publish Schema')
    const name  = (Math.random() + 1).toString(36).substring(7)
    const schemaId = `did:ethr:test:${identity.address}/anoncreds/v0/SCHEMA/${name}/1.0.0`
    const schema = {
        "attrNames": [ "First Name", "Last Name" ],
        "issuerId": `did:ethr:test:${identity.address}`,
        "name": name,
        "version": "1.0.0"
    }
    let schemaEndorsingData = await SchemaRegistry.buildCreateSchemaEndorsingData(client, schemaId, schema)
    authorSignature =  sign(schemaEndorsingData.getSigningBytes(), identity.secret)
    transaction = await SchemaRegistry.buildCreateSchemaSignedTransaction(client, trustee.address, schemaId, schema, authorSignature)
    transactionSignature = sign(transaction.getSigningBytes(), trustee.secret)
    transaction.setSignature(transactionSignature)
    txnHash = await client.submitTransaction(transaction)
    receipt = await client.getReceipt(txnHash)
    console.log('   Schema Transaction receipt: ' + receipt)

    console.log('5. Resolve Schema')
    const resolvedSchema = await SchemaRegistry.resolveSchema(client, schemaId)
    console.log('   Resolved Schema: ' + JSON.stringify(resolvedSchema, null, 2))
}

async function main() {
    await demo()
}

main()
