# Copyright (c) 2024 DSR Corporation, Denver, Colorado.
# https://www.dsr-corporation.com
# SPDX-License-Identifier: Apache-2.0

import json
import typing

import indy_besu_vdr
from indy_besu_vdr.indy_besu_vdr import *


class Transaction(indy_besu_vdr.indy_besu_vdr.Transaction):
    def get_signing_bytes(self) -> bytes:
        return transaction_get_signing_bytes(self)

    def set_signature(self, signature: SignatureData):
        self.signature = signature

    def to_string(self) -> str:
        return transaction_to_string(self)

    @classmethod
    def from_string(cls, value: str) -> "Transaction":
        return transaction_from_string(value)

    @classmethod
    def init(cls, transaction: indy_besu_vdr.indy_besu_vdr.Transaction) -> "Transaction":
        return Transaction(
            transaction.type,
            transaction._from,
            transaction.to,
            transaction.nonce,
            transaction.chain_id,
            transaction.data,
            transaction.signature,
            transaction.hash,
        )


class TransactionEndorsingData(indy_besu_vdr.indy_besu_vdr.TransactionEndorsingData):
    def get_signing_bytes(self) -> bytes:
        return transaction_endorsing_data_get_signing_bytes(self)

    def set_signature(self, signature: SignatureData):
        self.signature = signature

    def to_string(self) -> str:
        return transaction_endorsing_data_to_string(self)

    @classmethod
    def from_string(cls, value: str) -> "TransactionEndorsingData":
        return transaction_endorsing_data_from_string(value)

    @classmethod
    def init(cls, transaction: indy_besu_vdr.indy_besu_vdr.TransactionEndorsingData) -> "TransactionEndorsingData":
        return TransactionEndorsingData(
            transaction.to,
            transaction._from,
            transaction.nonce,
            transaction.contract,
            transaction.method,
            transaction.endorsing_method,
            transaction.params,
            transaction.signature,
        )


class DidIndyRegistry:
    @staticmethod
    async def build_create_did_transaction(client: LedgerClient, _from: str, did: str, did_doc: dict) -> "Transaction":
        return Transaction.init(await build_create_did_transaction(client, _from, did, json.dumps(did_doc)))

    @staticmethod
    async def build_create_did_endorsing_data(client: LedgerClient, did: str,
                                              did_doc: dict) -> "TransactionEndorsingData":
        return TransactionEndorsingData.init(await build_create_did_endorsing_data(client, did, json.dumps(did_doc)))

    @staticmethod
    async def build_update_did_transaction(client: LedgerClient, _from: str, did: str, did_doc: dict) -> "Transaction":
        return Transaction.init(await build_update_did_transaction(client, _from, did, json.dumps(did_doc)))

    @staticmethod
    async def build_update_did_endorsing_data(client: LedgerClient, did: str,
                                              did_doc: dict) -> "TransactionEndorsingData":
        return TransactionEndorsingData.init(await build_update_did_endorsing_data(client, did, json.dumps(did_doc)))

    @staticmethod
    async def build_deactivate_did_transaction(client: LedgerClient, _from: str, did: str) -> "Transaction":
        return Transaction.init(await build_deactivate_did_transaction(client, _from, did))

    @staticmethod
    async def build_deactivate_did_endorsing_data(client: LedgerClient, did: str) -> "TransactionEndorsingData":
        return TransactionEndorsingData.init(await build_deactivate_did_endorsing_data(client, did))

    @staticmethod
    async def build_resolve_did_transaction(client: LedgerClient, did: str) -> "Transaction":
        return Transaction.init(await build_resolve_did_transaction(client, did))

    @staticmethod
    def parse_resolve_did_result(client: LedgerClient, data: bytes) -> str:
        return parse_resolve_did_result(client, data)


class DidEthrRegistry:
    @staticmethod
    async def build_did_change_owner_transaction(client: LedgerClient, _from: str, did: str,
                                                 new_owner: str) -> "Transaction":
        return Transaction.init(await build_did_change_owner_transaction(client, _from, did, new_owner))

    @staticmethod
    async def build_did_change_owner_endorsing_data(client: LedgerClient, did: str,
                                                    new_owner: str) -> "TransactionEndorsingData":
        return TransactionEndorsingData.init(await build_did_change_owner_endorsing_data(client, did, new_owner))

    @staticmethod
    async def build_did_add_delegate_transaction(client: LedgerClient, _from: str, did: str,
                                                 delegate_type: str, delegate: str, validity: int) -> "Transaction":
        return Transaction.init(
            await build_did_add_delegate_transaction(client, _from, did, delegate_type, delegate, validity))

    @staticmethod
    async def build_did_add_delegate_endorsing_data(client: LedgerClient, did: str,
                                                    delegate_type: str, delegate: str,
                                                    validity: int) -> "TransactionEndorsingData":
        return TransactionEndorsingData.init(
            await build_did_add_delegate_endorsing_data(client, did, delegate_type, delegate, validity))

    @staticmethod
    async def build_did_revoke_delegate_transaction(client: LedgerClient, _from: str, did: str,
                                                    delegate_type: str, delegate: str) -> "Transaction":
        return Transaction.init(
            await build_did_revoke_delegate_transaction(client, _from, did, delegate_type, delegate))

    @staticmethod
    async def build_did_revoke_delegate_endorsing_data(client: LedgerClient, did: str,
                                                       delegate_type: str, delegate: str) -> "TransactionEndorsingData":
        return TransactionEndorsingData.init(
            await build_did_revoke_delegate_endorsing_data(client, did, delegate_type, delegate))

    @staticmethod
    async def build_did_set_attribute_transaction(client: LedgerClient, _from: str, did: str,
                                                  attribute: str, validity: int) -> "Transaction":
        return Transaction.init(
            await build_did_set_attribute_transaction(client, _from, did, attribute, validity))

    @staticmethod
    async def build_did_set_attribute_endorsing_data(client: LedgerClient, did: str, attribute: dict,
                                                     validity: int) -> "TransactionEndorsingData":
        return TransactionEndorsingData.init(
            await build_did_set_attribute_endorsing_data(client, did, json.dumps(attribute), validity))

    @staticmethod
    async def build_did_revoke_attribute_transaction(client: LedgerClient, _from: str, did: str,
                                                     attribute: dict) -> "Transaction":
        return Transaction.init(
            await build_did_revoke_attribute_transaction(client, _from, did, json.dumps(attribute)))

    @staticmethod
    async def build_did_revoke_attribute_endorsing_data(client: LedgerClient, did: str,
                                                        attribute: str) -> "TransactionEndorsingData":
        return TransactionEndorsingData.init(
            await build_did_revoke_attribute_endorsing_data(client, did, attribute))

    @staticmethod
    async def build_get_did_owner_transaction(client: LedgerClient, did: str) -> "Transaction":
        return Transaction.init(await build_get_did_owner_transaction(client, did))

    @staticmethod
    async def build_get_did_changed_transaction(client: LedgerClient, did: str) -> "Transaction":
        return Transaction.init(await build_get_did_changed_transaction(client, did))

    @staticmethod
    async def build_get_identity_nonce_transaction(client: LedgerClient, did: str) -> "Transaction":
        return Transaction.init(await build_get_identity_nonce_transaction(client, did))

    @staticmethod
    async def build_get_did_events_query(client: LedgerClient, did: str, from_block: typing.Optional[int],
                                         to_block: typing.Optional[int]) -> "EventQuery":
        return await build_get_did_events_query(client, did, from_block, to_block)

    @staticmethod
    def parse_did_changed_result(client: LedgerClient, data: bytes) -> int:
        return parse_did_changed_result(client, data)

    @staticmethod
    def parse_did_nonce_result(client: LedgerClient, data: bytes) -> int:
        return parse_did_nonce_result(client, data)

    @staticmethod
    def parse_did_owner_result(client: LedgerClient, data: bytes) -> str:
        return parse_did_owner_result(client, data)

    @staticmethod
    def parse_did_attribute_changed_event_response(client: LedgerClient, data: EventLog) -> DidAttributeChanged:
        return parse_did_attribute_changed_event_response(client, data)

    @staticmethod
    def parse_did_delegate_changed_event_response(client: LedgerClient, data: EventLog) -> DidDelegateChanged:
        return parse_did_delegate_changed_event_response(client, data)

    @staticmethod
    def parse_did_owner_changed_event_response(client: LedgerClient, data: EventLog) -> DidOwnerChanged:
        return parse_did_owner_changed_event_response(client, data)

    @staticmethod
    def parse_did_event_response(client: LedgerClient, data: EventLog) -> DidEvents:
        return parse_did_event_response(client, data)


class DidResolver:
    @staticmethod
    async def resolve_did(client: LedgerClient, did: str, options: typing.Optional[DidResolutionOptions]) -> str:
        return await resolve_did(client, did, options)


class Schema(indy_besu_vdr.indy_besu_vdr.Schema):
    def __init__(self, issuer_id: "str", name: "str", version: "str", attr_names: "typing.List[str]"):
        super().__init__(issuer_id, name, version, attr_names)

    @property
    def id(self) -> str:
        return schema_get_id(self)

    @classmethod
    def init(cls, schema: indy_besu_vdr.indy_besu_vdr.Schema) -> "Schema":
        return Schema(
            schema.issuer_id,
            schema.name,
            schema.version,
            schema.attr_names,
        )

    def to_string(self) -> str:
        return schema_to_string(self)

    @classmethod
    def from_string(cls, value: str) -> "Schema":
        return Schema.init(schema_from_string(value))


class SchemaRegistry:
    @staticmethod
    async def build_create_schema_transaction(client: LedgerClient, _from: str, schema: dict) -> "Transaction":
        return Transaction.init(await build_create_schema_transaction(client, _from, json.dumps(schema)))

    @staticmethod
    async def build_create_schema_endorsing_data(client: LedgerClient, schema: Schema) -> "TransactionEndorsingData":
        return TransactionEndorsingData.init(await build_create_schema_endorsing_data(client, schema))

    @staticmethod
    async def build_resolve_schema_transaction(client: LedgerClient, id: str) -> "Transaction":
        return Transaction.init(await build_resolve_schema_transaction(client, id))

    @staticmethod
    def parse_resolve_schema_result(client: LedgerClient, data: bytes) -> str:
        return parse_resolve_schema_result(client, data)

    @staticmethod
    async def resolve_schema(client: LedgerClient, id: str) -> Schema:
        return Schema.init(await resolve_schema(client, id))


class CredentialDefinition(indy_besu_vdr.indy_besu_vdr.CredentialDefinition):
    def __init__(self, issuer_id: "str", schema_id: "str", cred_def_type: "str", tag: "str", value: "JsonValue"):
        super().__init__(issuer_id, schema_id, cred_def_type, tag, value)

    @property
    def id(self) -> str:
        return credential_definition_get_id(self)

    @classmethod
    def init(cls, cred_def: indy_besu_vdr.indy_besu_vdr.CredentialDefinition) -> "CredentialDefinition":
        return CredentialDefinition(
            cred_def.issuer_id,
            cred_def.schema_id,
            cred_def.cred_def_type,
            cred_def.tag,
            cred_def.value,
        )

    def to_string(self) -> str:
        return credential_definition_to_string(self)

    @classmethod
    def from_string(cls, value: str) -> "CredentialDefinition":
        return CredentialDefinition.init(credential_definition_from_string(value))


class CredentialDefinitionRegistry:
    @staticmethod
    async def build_create_credential_definition_transaction(client: LedgerClient, _from: str,
                                                             credential_definition: dict) -> "Transaction":
        return Transaction.init(
            await build_create_credential_definition_transaction(client, _from, json.dumps(credential_definition)))

    @staticmethod
    async def build_create_credential_definition_endorsing_data(client: LedgerClient,
                                                                credential_definition: dict) -> "TransactionEndorsingData":
        return TransactionEndorsingData.init(
            await build_create_credential_definition_endorsing_data(client, json.dumps(credential_definition)))

    @staticmethod
    async def build_resolve_credential_definition_transaction(client: LedgerClient, id: str) -> "Transaction":
        return Transaction(await build_resolve_credential_definition_transaction(client, id))

    @staticmethod
    def parse_resolve_credential_definition_result(client: LedgerClient, data: bytes) -> str:
        return parse_resolve_credential_definition_result(client, data)

    @staticmethod
    async def resolve_credential_definition(client: LedgerClient, id: str) -> str:
        return await resolve_credential_definition(client, id)


class LegacyMapping:
    @staticmethod
    async def build_create_did_mapping_transaction(client: LedgerClient, _from: str, did: str, legacy_identifier: str,
                                                   legacy_verkey: str, ed25519_signature: bytes) -> "Transaction":
        return Transaction.init(
            await build_create_did_mapping_transaction(client, _from, did, legacy_identifier, legacy_verkey,
                                                       ed25519_signature))

    @staticmethod
    async def build_create_did_mapping_endorsing_data(client: LedgerClient, did: str, legacy_identifier: str,
                                                      legacy_verkey: str,
                                                      ed25519_signature: bytes) -> "TransactionEndorsingData":
        return TransactionEndorsingData.init(
            await build_create_did_mapping_endorsing_data(client, did, legacy_identifier, legacy_verkey,
                                                          ed25519_signature))

    @staticmethod
    async def build_get_did_mapping_transaction(client: LedgerClient, legacy_identifier: str) -> "Transaction":
        return Transaction.init(await build_get_did_mapping_transaction(client, legacy_identifier))

    @staticmethod
    def parse_did_mapping_result(client: LedgerClient, data: bytes) -> str:
        return parse_did_mapping_result(client, data)

    @staticmethod
    async def build_create_resource_mapping_transaction(client: LedgerClient, _from: str, did: str,
                                                        legacy_issuer_identifier: str, legacy_identifier: str,
                                                        new_identifier: str) -> "Transaction":
        return Transaction.init(
            await build_create_resource_mapping_transaction(client, _from, did, legacy_issuer_identifier,
                                                            legacy_identifier,
                                                            new_identifier))

    @staticmethod
    async def build_create_resource_mapping_endorsing_data(client: LedgerClient, did: str,
                                                           legacy_issuer_identifier: str, legacy_identifier: str,
                                                           new_identifier: str) -> "TransactionEndorsingData":
        return TransactionEndorsingData.init(
            await build_create_resource_mapping_endorsing_data(client, did, legacy_issuer_identifier, legacy_identifier,
                                                               new_identifier))

    @staticmethod
    async def build_get_resource_mapping_transaction(client: LedgerClient, legacy_identifier: str) -> "Transaction":
        return Transaction.init(await build_get_resource_mapping_transaction(client, legacy_identifier))

    @staticmethod
    def parse_resource_mapping_result(client: LedgerClient, data: bytes) -> str:
        return parse_resource_mapping_result(client, data)


class Endorsement:
    @staticmethod
    async def build_endorsement_transaction(client: LedgerClient, _from: str,
                                            endorsement_data: "TransactionEndorsingData") -> "Transaction":
        return Transaction.init(await build_endorsement_transaction(client, _from, endorsement_data))


class RoleControl:
    @staticmethod
    async def build_assign_role_transaction(client: LedgerClient, _from: str,
                                            role: int, account: str) -> "Transaction":
        return Transaction.init(await build_assign_role_transaction(client, _from, role, account))

    @staticmethod
    async def build_revoke_role_transaction(client: LedgerClient, _from: str,
                                            role: int, account: str) -> "Transaction":
        return Transaction.init(await build_revoke_role_transaction(client, _from, role, account))

    @staticmethod
    async def build_has_role_transaction(client: LedgerClient, role: int, account: str) -> "Transaction":
        return Transaction.init(await build_has_role_transaction(client, role, account))

    @staticmethod
    async def build_get_role_transaction(client: LedgerClient, account: str) -> "Transaction":
        return Transaction.init(await build_get_role_transaction(client, account))

    @staticmethod
    def parse_has_role_result(client: LedgerClient, data: bytes) -> bool:
        return parse_has_role_result(client, data)

    @staticmethod
    def parse_get_role_result(client: LedgerClient, data: bytes) -> int:
        return parse_get_role_result(client, data)


class ValidatorControl:
    @staticmethod
    async def build_add_validator_transaction(client: LedgerClient, _from: str,
                                              validator_address: str) -> "Transaction":
        return Transaction.init(await build_add_validator_transaction(client, _from, validator_address))

    @staticmethod
    async def build_remove_validator_transaction(client: LedgerClient, _from: str,
                                                 validator_address: str) -> "Transaction":
        return Transaction.init(await build_remove_validator_transaction(client, _from, validator_address))

    @staticmethod
    async def build_get_validators_transaction(client: LedgerClient) -> "Transaction":
        return Transaction.init(await build_get_validators_transaction(client))

    @staticmethod
    def parse_get_validators_result(client: LedgerClient) -> str:
        return parse_get_validators_result(client)
