// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { UniversalDidResolverInterface } from "../did/UniversalDidResolverInterface.sol";
import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";

import { SchemaAlreadyExist, SchemaNotFound } from "./ClErrors.sol";
import { SchemaRegistryInterface } from "./SchemaRegistryInterface.sol";
import { SchemaRecord } from "./SchemaTypes.sol";
import { SchemaValidator } from "./SchemaValidator.sol";
import { CLRegistry } from "./CLRegistry.sol";

using SchemaValidator for string;

contract SchemaRegistry is SchemaRegistryInterface, ControlledUpgradeable, CLRegistry {
    /**
     * Mapping Schema ID to its Schema Details and Metadata.
     */
    mapping(string id => SchemaRecord SchemaRecord) private _schemas;

    /**
     * Checks the uniqueness of the Schema ID
     */
    modifier _uniqueSchemaId(string memory id) {
        if (_schemas[id].metadata.created != 0) revert SchemaAlreadyExist(id);
        _;
    }

    /**
     * Checks that the Schema exist
     */
    modifier _schemaExist(string memory id) {
        if (_schemas[id].metadata.created == 0) revert SchemaNotFound(id);
        _;
    }

    function initialize(address upgradeControlAddress, address didResolverAddress) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _didResolver = UniversalDidResolverInterface(didResolverAddress);
    }

    /// @inheritdoc SchemaRegistryInterface
    function createSchema(
        string calldata id,
        string calldata issuerId,
        string calldata schema
    ) public virtual _uniqueSchemaId(id) _validIssuer(issuerId) {
        id.validateIdSyntax(issuerId);

        _schemas[id].schema = schema;
        _schemas[id].metadata.created = block.timestamp;

        emit SchemaCreated(id);
    }

    /// @inheritdoc SchemaRegistryInterface
    function resolveSchema(
        string calldata id
    ) public view virtual _schemaExist(id) returns (SchemaRecord memory schemaRecord) {
        return _schemas[id];
    }
}
