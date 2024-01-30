// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";

import { SchemaAlreadyExist } from "./ClErrors.sol";
import { SchemaRegistryInterface } from "./SchemaRegistryInterface.sol";
import { CLRegistry } from "./CLRegistry.sol";
import { EthereumExtDidRegistry } from "../did/EthereumExtDidRegistry.sol";

contract SchemaRegistry is SchemaRegistryInterface, ControlledUpgradeable, CLRegistry {
    /**
     * Mapping to track created schemas by their id to the block number when it was created.
     */
    mapping(bytes32 id => uint block) public created;

    /**
     * Checks the uniqueness of the Schema ID
     */
    modifier _uniqueSchemaId(bytes32 id) {
        if (created[id] != 0) revert SchemaAlreadyExist(id);
        _;
    }

    function initialize(address upgradeControlAddress, address ethereumExtDidRegistry) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _didRegistry = EthereumExtDidRegistry(ethereumExtDidRegistry);
    }

    /// @inheritdoc SchemaRegistryInterface
    function createSchema(address identity, bytes32 id, bytes calldata schema) public virtual {
        _createSchema(identity, msg.sender, id, schema);
    }

    /// @inheritdoc SchemaRegistryInterface
    function createSchemaSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        bytes calldata schema
    ) public virtual {
        bytes32 hash = keccak256(
            abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "createSchema", id, schema)
        );
        _createSchema(identity, _checkSignature(identity, hash, sigV, sigR, sigS), id, schema);
    }

    function _createSchema(
        address identity,
        address actor,
        bytes32 id,
        bytes calldata schema
    ) internal _uniqueSchemaId(id) _validIssuer(identity, actor) {
        created[id] = block.number;
        emit SchemaCreated(id, actor, schema);
    }
}
