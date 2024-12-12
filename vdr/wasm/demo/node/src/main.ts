/**
 * Copyright (c) 2024 DSR Corporation, Denver, Colorado.
 * https://www.dsr-corporation.com
 * SPDX-License-Identifier: Apache-2.0
 */

import fs from "fs";
import secp256k1 from "secp256k1";

import { readFileSync } from "fs";
import { resolve } from 'path'
import { LedgerClient, EthrDidRegistry, DidResolver, SchemaRegistry, Endorsement, Schema } from "indy-besu-vdr";

const projectRootPath = resolve('../../../..')
const trustee = {
    address: '0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5',
    secret: Uint8Array.from([139, 187, 177, 179, 69, 175, 86, 181, 96, 165, 178, 11, 212, 176, 237, 28, 216, 204, 153, 88, 161, 98, 98, 188, 117, 17, 132, 83, 203, 84, 109, 247])
}
const identity = {
    address: '0xce70ce892768d46caf120b600dec29ed20198982',
    secret: Uint8Array.from([126, 218, 51, 235, 106, 56, 168, 226, 49, 234, 92, 61, 233, 13, 242, 75, 137, 130, 228, 222, 148, 239, 14, 63, 135, 13, 140, 163, 134, 166, 49, 50])
}
const network = 'test'

function sign(message: Uint8Array, key: Uint8Array) {
    let signature = secp256k1.ecdsaSign(message, key)
    return {
        recovery_id: signature.recid,
        signature: signature.signature
    }
}

function readJson(path: string) {
    const data = readFileSync(path, 'utf8')
    return JSON.parse(data)
}

async function demo() {
    console.log('1. Init client')
    const configPath = `${projectRootPath}/network/config.json`
    const config = readJson(configPath)
    const contractConfigs = [
        {
            address: config.contracts.ethereumDidRegistry.address as string,
            spec: readJson(`${projectRootPath}/${config.contracts.ethereumDidRegistry.specPath}`)
        },
        {
            address: config.contracts.schemaRegistry.address as string,
            spec: readJson(`${projectRootPath}/${config.contracts.schemaRegistry.specPath}`)
        }
    ]


    const client = new LedgerClient(config.chainId, config.nodeAddress, contractConfigs, network, null)
    const status = await client.ping()
    console.log('Status: ' + JSON.stringify(status, null, 2))

    console.log('2. Publish and Modify DID')
    const did = 'did:ethr:' + identity.address
    const serviceAttribute = { "serviceEndpoint": "http://10.0.0.2", "type": "LinkedDomains" }
    const validity = BigInt(1000)
    let endorsingData = await EthrDidRegistry.buildDidSetAttributeEndorsingData(client, did, serviceAttribute, validity)
    let authorSignature = sign(endorsingData.getSigningBytes(), identity.secret)
    endorsingData.setSignature(authorSignature)
    let transaction = await Endorsement.buildEndorsementTransaction(client, trustee.address, endorsingData)
    let transactionSignature = sign(transaction.getSigningBytes(), trustee.secret)
    transaction.setSignature(transactionSignature)
    let txnHash = await client.submitTransaction(transaction)
    let receipt = await client.getReceipt(txnHash)
    console.log('Transaction receipt: ' + receipt)

    console.log('3. Resolve DID Document')
    const didWithMeta = await DidResolver.resolveDid(client, did, null)
    console.log('Resolved DID Document: ' + JSON.stringify(didWithMeta, null, 2))

    console.log('4. Publish Schema')
    const name = (Math.random() + 1).toString(36).substring(7)
    let schema = new Schema(did, name, "1.0.0", ["First Name", "Last Name"])
    let schemaEndorsingData = await SchemaRegistry.buildCreateSchemaEndorsingData(client, schema)
    authorSignature = sign(schemaEndorsingData.getSigningBytes(), identity.secret)
    schemaEndorsingData.setSignature(authorSignature)
    transaction = await Endorsement.buildEndorsementTransaction(client, trustee.address, schemaEndorsingData)
    transactionSignature = sign(transaction.getSigningBytes(), trustee.secret)
    transaction.setSignature(transactionSignature)
    txnHash = await client.submitTransaction(transaction)
    receipt = await client.getReceipt(txnHash)
    console.log('   Schema Transaction receipt: ' + receipt)

    console.log('5. Resolve Schema')
    const resolvedSchema = await SchemaRegistry.resolveSchema(client, schema.getId())
    console.log('   Resolved Schema: ' + resolvedSchema.toString())
}

async function main() {
    await demo()
}

main()
