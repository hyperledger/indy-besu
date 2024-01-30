import asyncio
import json
import secrets
import string

from eth_keys import keys
from indy2_vdr import *

# chain id of the running network
chain_id = 1337
# address of an RPC node connected to the network
node_address = 'http://127.0.0.1:8545'
# address of deployed IndyDidRegistry smart contract
did_contact_address = '0x0000000000000000000000000000000000018888'
schema_contact_address = '0x0000000000000000000000000000000000005555'
# Path to the compiled IndyDidRegistry smart contract
did_contact_spec_path = '/Users/indy-besu/smart_contracts/artifacts/contracts/did/EthereumExtDidRegistry.sol/EthereumExtDidRegistry.json'
schema_contact_spec_path = '/Users/indy-besu/smart_contracts/artifacts/contracts/cl/SchemaRegistry.sol/SchemaRegistry.json'
# Account address to use for sending transactions
trustee = {
    "address": '0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5',
    "secret": '8bbbb1b345af56b560a5b20bd4b0ed1cd8cc9958a16262bc75118453cb546df7'
}
identity = {
    "address": '0xce70ce892768d46caf120b600dec29ed20198982',
    "secret": '7eda33eb6a38a8e231ea5c3de90df24b8982e4de94ef0e3f870d8ca386a63132'
}


def sign(secret: str, data: bytes):
    signature = keys.PrivateKey(bytearray.fromhex(secret)).sign_msg_hash(data)
    rec_id = int(signature[-1:][0])
    sig = signature[0:-1]
    return SignatureData(rec_id, sig)


async def demo():
    print("1. Init client")
    contract_configs = [
        ContractConfig(did_contact_address, did_contact_spec_path, None),
        ContractConfig(schema_contact_address, schema_contact_spec_path, None),
    ]
    client = LedgerClient(chain_id, node_address, contract_configs, None)
    status = await client.ping()
    print(' Status: ' + str(status))

    print("2. Publish DID")
    did = 'did:ethr:' + identity['address']
    service_attribute = {"serviceEndpoint": "http://10.0.0.2", "type": "TestService"}
    endorsing_data = await build_did_set_attribute_endorsing_data(client, did, json.dumps(service_attribute), 1000)
    identity_signature = sign(identity["secret"], endorsing_data.get_signing_bytes())
    transaction = await build_did_set_attribute_signed_transaction(client, trustee["address"], did,
                                                                   json.dumps(service_attribute), 1000,
                                                                   identity_signature)
    trustee_signature = sign(trustee["secret"], transaction.get_signing_bytes())
    transaction.set_signature(trustee_signature)
    txn_hash = await client.submit_transaction(transaction)
    print(' Transaction hash: ' + bytes(txn_hash).hex())
    receipt = await client.get_receipt(txn_hash)
    print(' Transaction receipt: ' + str(receipt))

    print("3. Resolve DID Document")
    resolved_did_doc = await resolve_did(client, did, None)
    print(' Resolved DID Document:' + resolved_did_doc)

    print("4. Publish Schema")
    name = ''.join(secrets.choice(string.ascii_uppercase + string.digits) for _ in range(6))
    schema_id = 'did:ethr:test:' + identity["address"] + '/anoncreds/v0/SCHEMA/' + name + '/1.0.0'
    schema = {
        "attrNames": ["First Name", "Last Name"],
        "issuerId": 'did:ethr:test:' + identity["address"],
        "name": name,
        "version": "1.0.0"
    }
    endorsing_data = await build_create_schema_endorsing_data(client, schema_id, json.dumps(schema))
    identity_signature = sign(identity["secret"], endorsing_data.get_signing_bytes())
    transaction = await build_create_schema_signed_transaction(client, trustee["address"], schema_id,
                                                               json.dumps(schema),
                                                               identity_signature)
    trustee_signature = sign(trustee["secret"], transaction.get_signing_bytes())
    transaction.set_signature(trustee_signature)
    txn_hash = await client.submit_transaction(transaction)
    print(' Transaction hash: ' + bytes(txn_hash).hex())
    receipt = await client.get_receipt(txn_hash)
    print(' Transaction receipt: ' + str(receipt))

    print("5. Resolve Schema")
    resolved_schema = await resolve_schema(client, schema_id)
    print(' Resolved Schema:' + resolved_schema)

if __name__ == "__main__":
    asyncio.get_event_loop().run_until_complete(demo())
