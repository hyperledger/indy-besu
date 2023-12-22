import asyncio
import json
import os
from eth_keys import keys

import base58

from indy2_vdr import *

chain_id = 1337
node_address = 'http://127.0.0.1:8545'
did_contact_address = '0x0000000000000000000000000000000000003333'
# set path to the compiled contract
did_contact_spec_path = '/Users/indy-besu/smart_contracts/artifacts/contracts/did/IndyDidRegistry.sol/IndyDidRegistry.json'
account = '0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5'
account_key = '8bbbb1b345af56b560a5b20bd4b0ed1cd8cc9958a16262bc75118453cb546df7'


async def demo():
    print("1. Init client")
    contract_configs = [
        ContractConfig(did_contact_address, did_contact_spec_path, None)
    ]
    client = LedgerClient(chain_id, node_address, contract_configs)
    status = await client.ping()
    print('Status: ' + str(status))

    print("2. Publish DID Document")
    did = 'did:indy2:testnet:' + str(base58.b58encode(os.urandom(16)).decode())
    kid = did + '#KEY-1'
    did_doc = {
        "@context": ["https://www.w3.org/ns/did/v1"],
        "id": did,
        "controller": [],
        "verificationMethod": [
            {
                "id": kid,
                "type": "EcdsaSecp256k1VerificationKey2019",
                "controller": did,
                "publicKeyMultibase": "zQ3shnKp9QFbmV6Xj4YkoCg23DryaxNMTCJikSezYwLibafef"
            }
        ],
        "authentication": [kid],
        "assertionMethod": [],
        "capabilityInvocation": [],
        "capabilityDelegation": [],
        "keyAgreement": [],
        "service": [],
        "alsoKnownAs": []
    }

    print('DID Document: ' + json.dumps(did_doc))

    transaction = await build_create_did_transaction(client, account, json.dumps(did_doc))
    bytes_to_sign = transaction.get_signing_bytes()

    signature = keys.PrivateKey(bytearray.fromhex(account_key)).sign_msg_hash(bytes_to_sign)
    rec_id = int(signature[-1:][0])
    sig = signature[0:-1]
    transaction.set_signature(SignatureData(rec_id, sig))

    txn_hash = await client.submit_transaction(transaction)
    print('Transaction hash: ' + bytes(txn_hash).hex())
    receipt = await client.get_receipt(txn_hash)
    print('Transaction receipt: ' + str(receipt))

    print("3. Resolve DID Document")
    transaction = await build_resolve_did_transaction(client, did)
    response = await client.submit_transaction(transaction)
    resolved_did_doc = parse_resolve_did_result(client, response)
    print('Resolved DID Document:' + resolved_did_doc)


if __name__ == "__main__":
    asyncio.get_event_loop().run_until_complete(demo())
