// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { UniversalDidResolverInterface } from "../did/UniversalDidResolverInterface.sol";
import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";

import { SchemaAlreadyExist, SchemaNotFound } from "./AnoncredsErrors.sol";
import { SchemaRegistryInterface } from "./SchemaRegistryInterface.sol";
import { SchemaRecord } from "./SchemaTypes.sol";
import { AnoncredsRegistry } from "./AnoncredsRegistry.sol";
import { RoleControlInterface } from "../auth/RoleControl.sol";

contract SchemaRegistry is SchemaRegistryInterface, ControlledUpgradeable, AnoncredsRegistry {
    /**
     * Mapping Schema ID to its Schema Details and Metadata.
     */
    mapping(bytes32 id => SchemaRecord SchemaRecord) private _schemas;

    /**
     * Checks the uniqueness of the Schema ID
     */
    modifier _uniqueSchemaId(bytes32 id) {
        if (_schemas[id].metadata.created != 0) revert SchemaAlreadyExist(id);
        _;
    }

    /**
     * Checks that the Schema exist
     */
    modifier _schemaExist(bytes32 id) {
        if (_schemas[id].metadata.created == 0) revert SchemaNotFound(id);
        _;
    }

    function initialize(
        address upgradeControlAddress,
        address didResolverAddress,
        address roleControlContractAddress
    ) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _didResolver = UniversalDidResolverInterface(didResolverAddress);
        _roleControl = RoleControlInterface(roleControlContractAddress);
    }

    /// @inheritdoc SchemaRegistryInterface
    function createSchema(
        address identity,
        bytes32 id,
        string calldata issuerId,
        bytes calldata schema
    ) public virtual {
        _createSchema(identity, msg.sender, id, issuerId, schema);
    }

    /// @inheritdoc SchemaRegistryInterface
    function createSchemaSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        string calldata issuerId,
        bytes calldata schema
    ) public virtual {
        bytes32 hash = keccak256(
            abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "createSchema", id, issuerId, schema)
        );
        _createSchema(identity, ecrecover(hash, sigV, sigR, sigS), id, issuerId, schema);
    }

    /// @inheritdoc SchemaRegistryInterface
    function resolveSchema(bytes32 id) public view virtual _schemaExist(id) returns (SchemaRecord memory schemaRecord) {
        return _schemas[id];
    }

    function _createSchema(
        address identity,
        address actor,
        bytes32 id,
        string calldata issuerId,
        bytes calldata schema
    ) internal _senderIsTrusteeOrEndorserOrSteward _uniqueSchemaId(id) _validIssuer(issuerId, identity, actor) {
        _schemas[id].schema = schema;
        _schemas[id].metadata.created = block.timestamp;

        emit SchemaCreated(id, identity);
    }
}
