import fs from "fs";
import secp256k1 from "secp256k1";

import { readFileSync } from "fs";
import { resolve } from 'path'
import { LedgerClient, EthrDidRegistry, DidResolver, SchemaRegistry } from "indy-besu-vdr";

const chainId = 1337
const nodeAddress = 'http://127.0.0.1:8545'


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

function readContractsConfigs(): {address: string, spec: string}[] {
    const projectRootPath = resolve('../../../..')
    const configPath = `${projectRootPath}/config.json`

    const data = readFileSync(configPath, 'utf8')
    const parsed_data = JSON.parse(data)

    const ethDidRegistry = parsed_data.contracts.ethereum_did_registry
    const schemaRegistry = parsed_data.contracts.schema_registry

    return [
        {
            address: ethDidRegistry.address as string,
            spec: `${projectRootPath}/${ethDidRegistry.spec_path}`
        },
        {
            address: schemaRegistry.address as string,
            spec: `${projectRootPath}/${schemaRegistry.spec_path}`
        }
    ]
}

async function demo() {
    console.log('1. Init client')
    const contractConfigs = readContractsConfigs()
    const client = new LedgerClient(chainId, nodeAddress, contractConfigs, null)
    const status = await client.ping()
    console.log('Status: ' + JSON.stringify(status, null, 2))

    console.log('2. Publish and Modify DID')
    const did = 'did:ethr:' + identity.address
    const serviceAttribute = {"serviceEndpoint":"http://10.0.0.2","type":"TestService"}
    const validity = BigInt(1000)
    let endorsingData = await EthrDidRegistry.buildDidSetAttributeEndorsingData(client, did, serviceAttribute, validity)
    let authorSignature = sign(endorsingData.getSigningBytes(), identity.secret)
    let transaction = await EthrDidRegistry.buildDidSetAttributeSignedTransaction(client, trustee.address, did, serviceAttribute, validity, authorSignature)
    let transactionSignature = sign(transaction.getSigningBytes(), trustee.secret)
    transaction.setSignature(transactionSignature)
    let txnHash = await client.submitTransaction(transaction)
    let receipt = await client.getReceipt(txnHash)
    console.log('Transaction receipt: ' + receipt)

    console.log('3. Resolve DID Document')
    const didWithMeta = await DidResolver.resolveDid(client, did, null)
    console.log('Resolved DID Document: ' + JSON.stringify(didWithMeta, null, 2))

    console.log('4. Publish Schema')
    const name  = (Math.random() + 1).toString(36).substring(7)
    const schemaId = `${did}/anoncreds/v0/SCHEMA/${name}/1.0.0`
    const schema = {
        "attrNames": [ "First Name", "Last Name" ],
        "issuerId": did,
        "name": name,
        "version": "1.0.0"
    }
    let schemaEndorsingData = await SchemaRegistry.buildCreateSchemaEndorsingData(client, schema)
    authorSignature =  sign(schemaEndorsingData.getSigningBytes(), identity.secret)
    transaction = await SchemaRegistry.buildCreateSchemaSignedTransaction(client, trustee.address, schema, authorSignature)
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
