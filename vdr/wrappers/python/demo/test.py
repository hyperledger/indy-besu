import asyncio
import json
from eth_keys import keys
from indy2_vdr import *

# chain id of the running network
chain_id = 1337
# address of an RPC node connected to the network
node_address = 'http://127.0.0.1:8545'
# address of deployed IndyDidRegistry smart contract
did_contact_address = '0x0000000000000000000000000000000000018888'
# Path to the compiled IndyDidRegistry smart contract
did_contact_spec_path = '/Users/indy-besu/smart_contracts/artifacts/contracts/did/EthereumExtDidRegistry.sol/EthereumExtDidRegistry.json'
# Account address to use for sending transactions
trustee = {
    "address": '0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5',
    "secret": '8bbbb1b345af56b560a5b20bd4b0ed1cd8cc9958a16262bc75118453cb546df7'
}
identity = {
    "address": '0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5',
    "secret": '11e65a07a6840a54b18c8b12fe5eebc8ab3f291d4c6ea66b5607c64a22929d42'
}


def sign(secret: str, data: bytes):
    signature = keys.PrivateKey(bytearray.fromhex(secret)).sign_msg_hash(data)
    rec_id = int(signature[-1:][0])
    sig = signature[0:-1]
    return SignatureData(rec_id, sig)


async def demo():
    print("1. Init client")
    contract_configs = [
        ContractConfig(did_contact_address, did_contact_spec_path, None)
    ]
    client = LedgerClient(chain_id, node_address, contract_configs, None)
    status = await client.ping()
    print('Status: ' + str(status))

    print("2. Publish and Modify DID")
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
    print('Transaction hash: ' + bytes(txn_hash).hex())
    receipt = await client.get_receipt(txn_hash)
    print('Transaction receipt: ' + str(receipt))

    print("3. Resolve DID Document")
    resolved_did_doc = await resolve_did(client, did, None)
    print('Resolved DID Document:' + resolved_did_doc)


if __name__ == "__main__":
    asyncio.get_event_loop().run_until_complete(demo())
