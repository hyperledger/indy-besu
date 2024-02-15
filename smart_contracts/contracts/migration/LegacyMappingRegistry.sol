// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";
import { DidMappingAlreadyExist, ResourceMappingAlreadyExist, InvalidEd25519Key, InvalidResourceId } from "./LegacyMappingErrors.sol";
import { NotIdentityOwner } from "../did/DidErrors.sol";
import { EthereumExtDidRegistry } from "../did/EthereumExtDidRegistry.sol";
import { LegacyMappingRegistryInterface } from "./LegacyMappingRegistryInterface.sol";
import { DidValidator } from "../did/DidValidator.sol";

import { Base58 } from "../utils/Base58.sol";
import { toSlice } from "@dk1a/solidity-stringutils/src/StrSlice.sol";

using { toSlice } for string;

contract LegacyMappingRegistry is LegacyMappingRegistryInterface, ControlledUpgradeable, DidValidator {
    /*
     * Mapping storing indy/sov DID identifiers to the corresponding account address
     */
    mapping(string legacyDid => address account) public didMapping;

    /*
     * Mapping storing indy/sov formatted identifiers of schema/credential-definition to the corresponding new form
     */
    mapping(string legacyId => string newId) public resourceMapping;

    function initialize(address upgradeControlAddress, address ethereumExtDidRegistry) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _didRegistry = EthereumExtDidRegistry(ethereumExtDidRegistry);
    }

    /// @inheritdoc LegacyMappingRegistryInterface
    function createDidMapping(
        address identity,
        string calldata identifier,
        bytes32 ed25519Key,
        bytes calldata ed25519Signature
    ) public virtual {
        _createDidMapping(identity, msg.sender, identifier, ed25519Key, ed25519Signature);
    }

    /// @inheritdoc LegacyMappingRegistryInterface
    function createDidMappingSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        string calldata identifier,
        bytes32 ed25519Key,
        bytes calldata ed25518Signature
    ) public virtual {
        bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0x19),
                bytes1(0),
                address(this),
                identity,
                "createDidMapping",
                identifier,
                ed25519Key,
                ed25518Signature
            )
        );
        _createDidMapping(
            identity,
            _checkSignature(identity, hash, sigV, sigR, sigS),
            identifier,
            ed25519Key,
            ed25518Signature
        );
    }

    /// @inheritdoc LegacyMappingRegistryInterface
    function createResourceMapping(
        address identity,
        string calldata legacyIssuerIdentifier,
        string calldata legacyIdentifier,
        string calldata newIdentifier
    ) public virtual {
        _createResourceMapping(identity, msg.sender, legacyIssuerIdentifier, legacyIdentifier, newIdentifier);
    }

    /// @inheritdoc LegacyMappingRegistryInterface
    function createResourceMappingSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        string calldata legacyIssuerIdentifier,
        string calldata legacyIdentifier,
        string calldata newIdentifier
    ) public virtual {
        bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0x19),
                bytes1(0),
                address(this),
                identity,
                "createResourceMapping",
                legacyIssuerIdentifier,
                legacyIdentifier,
                newIdentifier
            )
        );
        _createResourceMapping(
            identity,
            _checkSignature(identity, hash, sigV, sigR, sigS),
            legacyIssuerIdentifier,
            legacyIdentifier,
            newIdentifier
        );
    }

    function _createDidMapping(
        address identity,
        address actor,
        string calldata identifier,
        bytes32 ed25519Key,
        bytes calldata ed25518Signature
    ) internal identityOwner(identity, actor) {
        // Checks the uniqueness of the DID mapping
        if (didMapping[identifier] != address(0x00)) revert DidMappingAlreadyExist(identifier);

        // Checks that Ed25519 key matches to the legacy DID identifier
        if (bytes16(Base58.decodeFromString(identifier)) != bytes16(ed25519Key))
            revert InvalidEd25519Key(ed25519Key, identifier);

        // TODO: check ed25519 signature validity
        didMapping[identifier] = identity;
        emit DidMappingCreated(identifier, identity);
    }

    function _createResourceMapping(
        address identity,
        address actor,
        string calldata legacyIssuerIdentifier,
        string calldata legacyIdentifier,
        string calldata newIdentifier
    ) internal identityOwner(identity, actor) {
        // Checks the uniqueness of the Resource mapping
        if (bytes(resourceMapping[legacyIdentifier]).length != 0) revert ResourceMappingAlreadyExist(legacyIdentifier);

        // Checks that owner of legacy DID controlled by identity account
        if (identity != didMapping[legacyIssuerIdentifier])
            revert NotIdentityOwner(identity, didMapping[legacyIssuerIdentifier]);

        // Checks that legacy issuer identifier is included into resource identifier
        if (!legacyIdentifier.toSlice().contains(legacyIssuerIdentifier.toSlice()))
            revert InvalidResourceId(legacyIdentifier, legacyIssuerIdentifier);

        resourceMapping[legacyIdentifier] = newIdentifier;
        emit ResourceMappingCreated(legacyIdentifier, newIdentifier);
    }
}
