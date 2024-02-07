// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";

import { DidMappingAlreadyExist, ClMappingAlreadyExist, NotIdentityOwner, InvalidEd25519Key } from "../utils/Errors.sol";
import { EthereumExtDidRegistry } from "../did/EthereumExtDidRegistry.sol";
import { LegacyIdentifiersRegistryInterface } from "./LegacyIdentifiersRegistryInterface.sol";
import { DidValidator } from "../did/DidValidator.sol";

contract LegacyIdentifiersRegistry is LegacyIdentifiersRegistryInterface, ControlledUpgradeable, DidValidator {
    /*
     * Mapping storing indy/sov DID identifiers to the corresponding account address
     */
    mapping(bytes16 => address) public didMapping;

    /*
     * Mapping storing indy/sov formatted identifiers of schema/credential definition to the corresponding new form
     */
    mapping(string => string) public clMapping;

    /**
     * Checks the uniqueness of the DID mapping
     */
    modifier _uniqueDidMapping(bytes16 identifier) {
        if (didMapping[identifier] != address(0x00)) revert DidMappingAlreadyExist(identifier);
        _;
    }

    /**
     * Checks the uniqueness of the CL mapping
     */
    modifier _uniqueClMapping(string calldata identifier) {
        if (bytes(clMapping[identifier]).length != 0) revert ClMappingAlreadyExist(identifier);
        _;
    }

    /**
     * Checks that Ed25519 matches to the legacy DID identifier
     */
    modifier _validEd25518Key(bytes16 identifier, bytes32 ed25519Key) {
        if (identifier != bytes16(ed25519Key)) revert InvalidEd25519Key(ed25519Key, identifier);
        _;
    }

    /**
     * Checks that owner of legacy DID controlled by actor
     */
    modifier _legacyDidOwner(bytes16 legacyIssuerIdentifier, address actor) {
        if (actor != didMapping[legacyIssuerIdentifier])
            revert NotIdentityOwner(actor, didMapping[legacyIssuerIdentifier]);
        _;
    }

    function initialize(address upgradeControlAddress, address ethereumExtDidRegistry) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _didRegistry = EthereumExtDidRegistry(ethereumExtDidRegistry);
    }

    /// @inheritdoc LegacyIdentifiersRegistryInterface
    function createDidMapping(
        address identity,
        bytes16 identifier,
        bytes32 ed25519Key,
        bytes calldata ed25519Signature
    ) public virtual {
        _createDidMapping(identity, msg.sender, identifier, ed25519Key, ed25519Signature);
    }

    /// @inheritdoc LegacyIdentifiersRegistryInterface
    function createDidMappingSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes16 identifier,
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

    function _createDidMapping(
        address identity,
        address actor,
        bytes16 identifier,
        bytes32 ed25519Key,
        bytes calldata ed25518Signature
    ) internal _uniqueDidMapping(identifier) _validEd25518Key(identifier, ed25519Key) identityOwner(identity, actor) {
        // check ed25519 signature validity
        didMapping[identifier] = identity;
        emit DidMappingCreated(identifier, identity);
    }

    /// @inheritdoc LegacyIdentifiersRegistryInterface
    function createClMapping(
        address identity,
        bytes16 legacyIssuerIdentifier,
        string calldata legacyIdentifier,
        string calldata newIdentifier
    ) public virtual {
        _createClMapping(identity, msg.sender, legacyIssuerIdentifier, legacyIdentifier, newIdentifier);
    }

    /// @inheritdoc LegacyIdentifiersRegistryInterface
    function createClMappingSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes16 legacyIssuerIdentifier,
        string calldata legacyIdentifier,
        string calldata newIdentifier
    ) public virtual {
        bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0x19),
                bytes1(0),
                address(this),
                identity,
                "createClMapping",
                legacyIssuerIdentifier,
                legacyIdentifier,
                newIdentifier
            )
        );
        _createClMapping(
            identity,
            _checkSignature(identity, hash, sigV, sigR, sigS),
            legacyIssuerIdentifier,
            legacyIdentifier,
            newIdentifier
        );
    }

    function _createClMapping(
        address identity,
        address actor,
        bytes16 legacyIssuerIdentifier,
        string calldata legacyIdentifier,
        string calldata newIdentifier
    )
        internal
        _uniqueClMapping(legacyIdentifier)
        _legacyDidOwner(legacyIssuerIdentifier, actor)
        identityOwner(identity, actor)
    {
        clMapping[legacyIdentifier] = newIdentifier;
        emit ClMappingCreated(legacyIdentifier, newIdentifier);
    }
}
