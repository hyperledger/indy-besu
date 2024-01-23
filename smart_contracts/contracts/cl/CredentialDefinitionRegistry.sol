// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { UniversalDidResolverInterface } from "../did/UniversalDidResolverInterface.sol";
import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";

import { CredentialDefinitionRecord } from "./CredentialDefinitionTypes.sol";
import { CredentialDefinitionRegistryInterface } from "./CredentialDefinitionRegistryInterface.sol";
import { CredentialDefinitionValidator } from "./CredentialDefinitionValidator.sol";
import { CredentialDefinitionAlreadyExist, CredentialDefinitionNotFound } from "./ClErrors.sol";
import { CLRegistry } from "./CLRegistry.sol";
import { SchemaRegistryInterface } from "./SchemaRegistryInterface.sol";

using CredentialDefinitionValidator for string;

contract CredentialDefinitionRegistry is CredentialDefinitionRegistryInterface, ControlledUpgradeable, CLRegistry {
    /**
     * @dev Reference to the contract that manages anoncreds schemas
     */
    SchemaRegistryInterface private _schemaRegistry;

    /**
     * Mapping Credential Definition ID to its Credential Definition Details and Metadata.
     */
    mapping(string id => CredentialDefinitionRecord credentialDefinitionRecord) private _credDefs;

    /**
     * Checks the uniqueness of the credential definition ID
     */
    modifier _uniqueCredDefId(string memory id) {
        if (_credDefs[id].metadata.created != 0) revert CredentialDefinitionAlreadyExist(id);
        _;
    }

    /**
     * Checks that the credential definition exist
     */
    modifier _credDefExist(string memory id) {
        if (_credDefs[id].metadata.created == 0) revert CredentialDefinitionNotFound(id);
        _;
    }

    /**
     * Сhecks that the schema exist
     */
    modifier _schemaExist(string memory id) {
        _schemaRegistry.resolveSchema(id);
        _;
    }

    function initialize(
        address upgradeControlAddress,
        address didResolverAddress,
        address schemaRegistryAddress
    ) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _didResolver = UniversalDidResolverInterface(didResolverAddress);
        _schemaRegistry = SchemaRegistryInterface(schemaRegistryAddress);
    }

    /// @inheritdoc CredentialDefinitionRegistryInterface
    function createCredentialDefinition(
        string calldata id,
        string calldata issuerId,
        string calldata schemaId,
        string calldata credDef
    ) public virtual _uniqueCredDefId(id) _schemaExist(schemaId) _validIssuer(issuerId) {
        id.validateIdSyntax(issuerId, schemaId);

        _credDefs[id].credDef = credDef;
        _credDefs[id].metadata.created = block.timestamp;

        emit CredentialDefinitionCreated(id);
    }

    /// @inheritdoc CredentialDefinitionRegistryInterface
    function resolveCredentialDefinition(
        string calldata id
    ) public view virtual _credDefExist(id) returns (CredentialDefinitionRecord memory credentialDefinitionRecord) {
        return _credDefs[id];
    }
}
