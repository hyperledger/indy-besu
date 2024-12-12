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
     * Checks that the schema exist
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

    modifier _accumulatorsMatch(bytes32 _revRegDefId, RevocationRegistryEntry calldata initialRevRegEntry) {
        if (
            _revRegDefs[_revRegDefId].metadata.currentAccumulator.length != 0 &&
            keccak256(abi.encodePacked(_revRegDefs[_revRegDefId].metadata.currentAccumulator)) !=
            keccak256(abi.encodePacked(initialRevRegEntry.prevAccumulator))
        ) revert AccumulatorMismatch(initialRevRegEntry.prevAccumulator);
        _;
    }

    //TODO: using as modifier caused 'Stack Too Deep' Compile Error
    // modifier _isRevRegDefIssuer(bytes32 revRegDefId, string calldata issuerId) {
    //     if (
    //         keccak256(abi.encodePacked(_revRegDefs[revRegDefId].metadata.issuerId)) !=
    //         keccak256(abi.encodePacked(issuerId))
    //     ) revert NotRevocationRegistryDefinitionIssuer(issuerId);
    //     _;
    // }

    //TODO:
    // Seems odd but keep in mind...
    //https://github.com/hyperledger/indy-node/blob/main/design/anoncreds.md#revoc_reg_entry
    // Creation of Revocation Registry (Def and Enteries):
    // RevocReg Issuer may not be the same as Schema Author and CredDef issuer.
    /**
     * Checks wether Credential Definition Exists
     */
    modifier _credDefExists(bytes32 credDefId) {
        _credentialDefinitionRegistry.resolveCredentialDefinition(credDefId);
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

    function createRevocationRegistryEntry(
        address identity,
        bytes32 revRegDefId,
        string calldata issuerId,
        RevocationRegistryEntry calldata revRegEntry
    ) external override {
        _createRevocationRegistryEntry(identity, msg.sender, revRegDefId, issuerId, revRegEntry);
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
        _credDefExists(credDefId)
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
        //TODO: using as modifier caused 'Stack too deep' compile error
        if (
            keccak256(abi.encodePacked(_revRegDefs[revRegDefId].metadata.issuerId)) !=
            keccak256(abi.encodePacked(issuerId))
        ) revert NotRevocationRegistryDefinitionIssuer(issuerId);

        _revRegDefs[revRegDefId].metadata.currentAccumulator = revRegEntry.currentAccumulator;
        emit RevocationRegistryEntryCreated(revRegDefId, revRegEntry.timestamp, revRegEntry);
    }
}
