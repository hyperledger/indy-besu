// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";

import { CredentialDefinitionRegistryInterface } from "./CredentialDefinitionRegistryInterface.sol";
import { CredentialDefinitionAlreadyExist, SchemaNotFound } from "./ClErrors.sol";
import { CLRegistry } from "./CLRegistry.sol";
import { SchemaRegistryInterface } from "./SchemaRegistryInterface.sol";
import { EthereumExtDidRegistry } from "../did/EthereumExtDidRegistry.sol";

contract CredentialDefinitionRegistry is CredentialDefinitionRegistryInterface, ControlledUpgradeable, CLRegistry {
    /**
     * @dev Reference to the contract that manages anoncreds schemas
     */
    SchemaRegistryInterface private _schemaRegistry;

    /**
     * Mapping to track created credential definitions by their id to block number.
     */
    mapping(bytes32 id => uint block) public created;

    /**
     * Checks the uniqueness of the credential definition ID
     */
    modifier _uniqueCredDefId(bytes32 id) {
        if (created[id] != 0) revert CredentialDefinitionAlreadyExist(id);
        _;
    }

    /**
     * Checks that the schema exist
     */
    modifier _schemaExist(bytes32 id) {
        if (_schemaRegistry.created(id) == 0) revert SchemaNotFound(id);
        _;
    }

    function initialize(
        address upgradeControlAddress,
        address ethereumExtDidRegistry,
        address schemaRegistryAddress
    ) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _didRegistry = EthereumExtDidRegistry(ethereumExtDidRegistry);
        _schemaRegistry = SchemaRegistryInterface(schemaRegistryAddress);
    }

    /// @inheritdoc CredentialDefinitionRegistryInterface
    function createCredentialDefinition(
        address identity,
        bytes32 id,
        bytes32 schemaId,
        bytes calldata credDef
    ) public virtual {
        _createCredentialDefinition(identity, msg.sender, id, schemaId, credDef);
    }

    /// @inheritdoc CredentialDefinitionRegistryInterface
    function createCredentialDefinitionSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        bytes32 schemaId,
        bytes calldata credDef
    ) public virtual {
        bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0x19),
                bytes1(0),
                address(this),
                identity,
                "createCredentialDefinition",
                id,
                schemaId,
                credDef
            )
        );
        _createCredentialDefinition(identity, _checkSignature(identity, hash, sigV, sigR, sigS), id, schemaId, credDef);
    }

    function _createCredentialDefinition(
        address identity,
        address actor,
        bytes32 id,
        bytes32 schemaId,
        bytes calldata credDef
    ) internal _uniqueCredDefId(id) _validIssuer(identity, actor) _schemaExist(schemaId) {
        created[id] = block.number;
        emit CredentialDefinitionCreated(id, actor, credDef);
    }
}
