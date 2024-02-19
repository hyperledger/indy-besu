import asyncio
import json
import secrets
import string
import os

from eth_keys import keys
from indy_besu_vdr import *

# Account address to use for sending transactions
trustee = {
    "address": '0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5',
    "secret": '8bbbb1b345af56b560a5b20bd4b0ed1cd8cc9958a16262bc75118453cb546df7'
}
identity = {
    "address": '0xce70ce892768d46caf120b600dec29ed20198982',
    "secret": '7eda33eb6a38a8e231ea5c3de90df24b8982e4de94ef0e3f870d8ca386a63132'
}
network = 'test'
project_root = f"{os.getcwd()}/../../.."

def sign(secret: str, data: bytes):
    signature = keys.PrivateKey(bytearray.fromhex(secret)).sign_msg_hash(data)
    rec_id = int(signature[-1:][0])
    sig = signature[0:-1]
    return SignatureData(rec_id, sig)


def read_config():
    with open(f"{project_root}/network/config.json") as f:
        return json.loads(f.read())


async def demo():
    print("1. Init client")
    config = read_config()

    did_registry_contract = config["contracts"]["ethereumDidRegistry"]
    did_contract_address = did_registry_contract["address"]
    did_contract_spec_path = "{}/{}".format(project_root, did_registry_contract["specPath"])

    schema_registry_contract = config["contracts"]["schemaRegistry"]
    schema_contract_address = schema_registry_contract["address"]
    schema_contract_spec_path = "{}/{}".format(project_root, schema_registry_contract["specPath"])

    contract_configs = [
        ContractConfig(did_contract_address, did_contract_spec_path, None),
        ContractConfig(schema_contract_address, schema_contract_spec_path, None),
    ]
    client = LedgerClient(config["chainId"], config["nodeAddress"], contract_configs, network, None)
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
    schema_id = did + '/anoncreds/v0/SCHEMA/' + name + '/1.0.0'
    schema = {
        "attrNames": ["First Name", "Last Name"],
        "issuerId": did,
        "name": name,
        "version": "1.0.0"
    }
    endorsing_data = await build_create_schema_endorsing_data(client, json.dumps(schema))
    identity_signature = sign(identity["secret"], endorsing_data.get_signing_bytes())
    transaction = await build_create_schema_signed_transaction(client,
                                                               trustee["address"],
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
