// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { UniversalDidResolverInterface } from "../did/UniversalDidResolverInterface.sol";
import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";

import { RevocationRegistryDefinitionRecord, RevocationRegistryEntry } from "./RevocationRegistryTypes.sol";
import { CredentialDefinitionRecord } from "./CredentialDefinitionTypes.sol";
import { RevocationRegistryInterface } from "./RevocationRegistryInterface.sol";
import { NotRevocationRegistryDefinitionIssuer, RevocationRegistryDefinitionAlreadyExist, RevocationRegistryDefinitionNotFound, AccumulatorMismatch } from "./AnoncredsErrors.sol";
import { CredentialDefinitionRegistryInterface } from "./CredentialDefinitionRegistryInterface.sol";
import { RoleControlInterface } from "../auth/RoleControl.sol";
import { AnoncredsRegistry } from "./AnoncredsRegistry.sol";
import { StringUtils } from "../utils/StringUtils.sol";

import { Errors } from "../utils/Errors.sol";

contract RevocationRegistry is RevocationRegistryInterface, ControlledUpgradeable, AnoncredsRegistry {
    /**
     * @dev Reference to the contract that manages AnonCreds Credential Definitions
     */
    CredentialDefinitionRegistryInterface private _credentialDefinitionRegistry;

    /**
     * Mapping Revocation Registry  Definition ID to its Revocation Registry Definition Details and Metadata.
     */
    mapping(bytes32 id => RevocationRegistryDefinitionRecord revocationRegistryDefinitionRecord) private _revRegDefs;

    /**
     * Checks that the Credential Definition exist
     */
    modifier _credentialDefinitionExists(bytes32 id) {
        _credentialDefinitionRegistry.resolveCredentialDefinition(id);
        _;
    }

    /**
     * Checks the uniqueness of the revocation registry definition ID
     */
    modifier _uniqueRevRegDefId(bytes32 id) {
        if (_revRegDefs[id].metadata.created != 0) revert RevocationRegistryDefinitionAlreadyExist(id);
        _;
    }

    /**
     * Checks that the revocation registry definition exist
     */
    modifier _revRecDefExist(bytes32 id) {
        if (_revRegDefs[id].metadata.created == 0) revert RevocationRegistryDefinitionNotFound(id);
        _;
    }

    /**
     * Checks that the previous accumulator specified by the new Revocation Registry Entry matches
     * the current accumulator on chain (only if not the first entry)
     */
    modifier _accumulatorsMatch(bytes32 _revRegDefId, RevocationRegistryEntry calldata _revRegEntry) {
        if (_revRegDefs[_revRegDefId].metadata.currentAccumulator.length != 0) {
            if (_revRegDefs[_revRegDefId].metadata.currentAccumulator.length != _revRegEntry.prevAccumulator.length) {
                revert AccumulatorMismatch(_revRegEntry.prevAccumulator);
            }
            for (uint256 i = 0; i < _revRegEntry.prevAccumulator.length; i++) {
                if (_revRegDefs[_revRegDefId].metadata.currentAccumulator[i] != _revRegEntry.prevAccumulator[i]) {
                    revert AccumulatorMismatch(_revRegEntry.prevAccumulator);
                }
            }
        }
        _;
    }

    function initialize(
        address upgradeControlAddress,
        address didResolverAddress,
        address credentialDefinitionRegistry,
        address roleControlContractAddress
    ) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _didResolver = UniversalDidResolverInterface(didResolverAddress);
        _credentialDefinitionRegistry = CredentialDefinitionRegistryInterface(credentialDefinitionRegistry);
        _roleControl = RoleControlInterface(roleControlContractAddress);
    }

    /// @inheritdoc RevocationRegistryInterface
    function createRevocationRegistryDefinition(
        address identity,
        bytes32 id,
        bytes32 credDefId,
        string calldata issuerId,
        bytes calldata revRegDef
    ) external override {
        _createRevocationRegistryDefinition(identity, msg.sender, id, credDefId, issuerId, revRegDef);
    }

    /// @inheritdoc RevocationRegistryInterface
    function createRevocationRegistryDefinitionSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        bytes32 credDefId,
        string calldata issuerId,
        bytes calldata revRegDef
    ) public virtual {
        bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0x19),
                bytes1(0),
                address(this),
                identity,
                "createRevocationRegistryDefinition",
                id,
                credDefId,
                issuerId,
                revRegDef
            )
        );
        _createRevocationRegistryDefinition(
            identity,
            ecrecover(hash, sigV, sigR, sigS),
            id,
            credDefId,
            issuerId,
            revRegDef
        );
    }

    /// @inheritdoc RevocationRegistryInterface
    function resolveRevocationRegistryDefinition(
        bytes32 id
    )
        public
        view
        override
        _revRecDefExist(id)
        returns (RevocationRegistryDefinitionRecord memory revocationRegistryDefinitionRecord)
    {
        return _revRegDefs[id];
    }

    /// @inheritdoc RevocationRegistryInterface
    function createRevocationRegistryEntry(
        address identity,
        bytes32 revRegDefId,
        string calldata issuerId,
        RevocationRegistryEntry calldata revRegEntry
    ) external override {
        _createRevocationRegistryEntry(identity, msg.sender, revRegDefId, issuerId, revRegEntry);
    }

    /// @inheritdoc RevocationRegistryInterface
    function createRevocationRegistryEntrySigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 revRegDefId,
        string calldata issuerId,
        RevocationRegistryEntry calldata revRegEntry
    ) public virtual {
        bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0x19),
                bytes1(0),
                address(this),
                identity,
                "createRevocationRegistryEntry",
                revRegDefId,
                issuerId,
                abi.encode(revRegEntry)
            )
        );
        _createRevocationRegistryEntry(identity, ecrecover(hash, sigV, sigR, sigS), revRegDefId, issuerId, revRegEntry);
    }

    function _createRevocationRegistryDefinition(
        address identity,
        address actor,
        bytes32 id,
        bytes32 credDefId,
        string calldata issuerId,
        bytes calldata revRegDef
    )
        internal
        _senderIsTrusteeOrEndorserOrSteward
        _uniqueRevRegDefId(id)
        _validIssuer(issuerId, identity, actor)
        _credentialDefinitionExists(credDefId)
    {
        _revRegDefs[id].revRegDef = revRegDef;
        _revRegDefs[id].metadata.created = block.timestamp;
        _revRegDefs[id].metadata.issuerId = issuerId;

        emit RevocationRegistryDefinitionCreated(id, identity);
    }

    function _createRevocationRegistryEntry(
        address identity,
        address actor,
        bytes32 revRegDefId,
        string calldata issuerId,
        RevocationRegistryEntry calldata revRegEntry
    )
        internal
        _senderIsTrusteeOrEndorserOrSteward
        _revRecDefExist(revRegDefId)
        _validIssuer(issuerId, identity, actor)
        _accumulatorsMatch(revRegDefId, revRegEntry)
    {
        if (!StringUtils.equals(_revRegDefs[revRegDefId].metadata.issuerId, issuerId)) {
            revert NotRevocationRegistryDefinitionIssuer(issuerId);
        }

        _revRegDefs[revRegDefId].metadata.currentAccumulator = revRegEntry.currentAccumulator;
        emit RevocationRegistryEntryCreated(revRegDefId, revRegEntry.timestamp, revRegEntry);
    }
}
