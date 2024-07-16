// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { UniversalDidResolverInterface } from "../did/UniversalDidResolverInterface.sol";
import { IndyDidRegistryInterface } from "../did/IndyDidRegistryInterface.sol";
import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";
import { RevocationRegistryInterface } from "./RevocationRegistryInterface.sol";
import { CredentialDefinitionRecord } from "../anoncreds/CredentialDefinitionTypes.sol";
import { CredentialDefinitionRegistryInterface } from "../anoncreds/CredentialDefinitionRegistryInterface.sol";
import { RevocationRecord, Status } from "./RevocationRegistryTypes.sol";
import { AnoncredsRegistry } from "../anoncreds/AnoncredsRegistry.sol";
import { RevocationNotFound, RevocationAlreadyExist, CredentialDefinitionNotFound, RevocationIsNotActived, RevocationIsNotsuspended, RevocationIsNotRevoked, CredentialIsAlreadyRevoked, InvalidIssuer, RevocationDoesntExist } from "../anoncreds/AnoncredsErrors.sol";
import { RoleControlInterface } from "../auth/RoleControl.sol";

contract RevocationRegistry is RevocationRegistryInterface, ControlledUpgradeable, AnoncredsRegistry {
    /**
     * @dev Reference to the contract that manages anoncreds credDefs
     */
    CredentialDefinitionRegistryInterface private _credDefRegistry;

    /**
     * Mapping Revocation ID to its Revocation Details and Metadata.
     */
    mapping(bytes32 id => RevocationRecord revocationRecord) private _revReg;

    /**
     * Check that the revocation exist
     */
    modifier _revocationExist(bytes32 id) {
        if (_revReg[id].metadata.created != 0) revert RevocationAlreadyExist(id);
        _;
    }

    /**
     * Check that the revocation does not exist
     */
    modifier _revocationNotExist(bytes32 id) {
        if (_revReg[id].metadata.created == 0) revert RevocationDoesntExist(id);
        _;
    }

    /**
     *
     * Check that the status is not revoked
     */
    modifier _CredentialNotRevoked(bytes32 id) {
        if (_revReg[id].metadata.status == Status.revoked) revert CredentialIsAlreadyRevoked(id);
        _;
    }

    /**
     * Check that the status is not actived
     */
    modifier _CredentialNotActived(bytes32 id) {
        if (_revReg[id].metadata.status != Status.active) revert RevocationIsNotActived(id);
        _;
    }

    /**
     * Check that the status is actived
     */
    modifier _CredentialIsActived(bytes32 id) {
        if (_revReg[id].metadata.status == Status.active) revert RevocationIsNotRevoked(id);
        _;
    }

    /**
     * Сhecks  the Issuer of revocation
     */

    modifier _checkIssuer(bytes32 id) {
        if (_revReg[id].metadata.creator != msg.sender) revert InvalidIssuer(id);
        _;
    }

    /**
     * Checks that the credDef exists
     */
    modifier _credDefExist(bytes32 id) {
        _credDefRegistry.resolveCredentialDefinition(id);
        _;
    }

    function initialize(
        address upgradeControlAddress,
        address credDefRegistryAddress,
        address roleControlContractAddress
    ) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _credDefRegistry = CredentialDefinitionRegistryInterface(credDefRegistryAddress);
        _roleControl = RoleControlInterface(roleControlContractAddress);
    }

    /**
     * Revoke functions:
     */

    function revokeCredential(
        address identity,
        bytes32 RevokId
    ) public _CredentialNotRevoked(RevokId) _senderIsTrusteeOrEndorserOrSteward _revocationNotExist(RevokId) {
        _revokeCredential(identity, msg.sender, RevokId);
    }

    /// @inheritdoc RevocationRegistryInterface
    function revokeCredentialSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 RevokId
    ) public virtual _CredentialNotRevoked(RevokId) _senderIsTrusteeOrEndorserOrSteward _revocationNotExist(RevokId) {
        bytes32 hash = keccak256(
            abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "revokeCredential", RevokId)
        );

        _revokeCredential(identity, ecrecover(hash, sigV, sigR, sigS), RevokId);
    }

    /**
     * Suspend functions:
     */
    function suspendCredential(
        address identity,
        bytes32 RevokId
    ) public _CredentialNotActived(RevokId) _senderIsTrusteeOrEndorserOrSteward _revocationNotExist(RevokId) {
        _suspendCredential(identity, msg.sender, RevokId);
    }

    /// @inheritdoc RevocationRegistryInterface
    function suspendCredentialSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 RevokId
    ) public virtual _CredentialNotActived(RevokId) _senderIsTrusteeOrEndorserOrSteward _revocationNotExist(RevokId) {
        bytes32 hash = keccak256(
            abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "suspendCredential", RevokId)
        );

        _suspendCredential(identity, ecrecover(hash, sigV, sigR, sigS), RevokId);
    }

    /**
     * Unrevok functions:
     */
    function unrevokeCredential(
        address identity,
        bytes32 RevokId
    ) public _CredentialIsActived(RevokId) _senderIsTrusteeOrEndorserOrSteward _revocationNotExist(RevokId) {
        _UnrevokedCredential(identity, msg.sender, RevokId);
    }

    /// @inheritdoc RevocationRegistryInterface
    function unrevokeCredentialSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 RevokId
    ) public virtual _CredentialIsActived(RevokId) _senderIsTrusteeOrEndorserOrSteward _revocationNotExist(RevokId) {
        bytes32 hash = keccak256(
            abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "unrevokeCredential", RevokId)
        );

        _UnrevokedCredential(identity, ecrecover(hash, sigV, sigR, sigS), RevokId);
    }

    /**
     * create Revocation functions:
     */
    function createRevocationRegistry(
        address identity,
        bytes32 CreDefid,
        bytes32 RevokId,
        bytes calldata revokeDocument
    ) public
     _revocationExist(RevokId)
     _senderIsTrusteeOrEndorserOrSteward
     _credDefExist(CreDefid) 
       {
        _createRevocation(identity, msg.sender, RevokId, revokeDocument);
    }

    /// @inheritdoc RevocationRegistryInterface
    function createRevocationRegistrySigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 CreDefid,
        bytes32 RevokId,
        bytes calldata revokeDocument
    ) public virtual _revocationExist(RevokId) _senderIsTrusteeOrEndorserOrSteward _credDefExist(CreDefid) {
        bytes32 hash = keccak256(
            abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "createRevocationRegistry", RevokId)
        );

        _createRevocation(identity, ecrecover(hash, sigV, sigR, sigS), RevokId, revokeDocument);
    }

    /**
     * Create Revocation functions:
     */

    function _createRevocation(
        address identity,
        address actor,
        bytes32 RevokId,
        bytes calldata document
    )
        private
        _identityOwner(identity, actor) // the sender must be equal to the identity
    {
        _revReg[RevokId].document = document;
        _revReg[RevokId].metadata.created = block.timestamp;
        _revReg[RevokId].metadata.creator = msg.sender;
        _revReg[RevokId].metadata.updated = block.timestamp;
        _revReg[RevokId].metadata.status = Status.active;

        emit RevocationCreated(msg.sender, RevokId);
    }

    /**
     * Revok Credential functions:
     */

    function _revokeCredential(
        address identity,
        address actor,
        bytes32 RevokId
    )
        private
        _identityOwner(identity, actor) // the sender must be equal to the identity
        _checkIssuer(RevokId)
    {
        _revReg[RevokId].metadata.status = Status.revoked;
        _revReg[RevokId].metadata.updated = block.timestamp;

        ///credential revocation event
        emit CredentialRevoked(msg.sender, RevokId);
    }

    /**
     * suspend Credential functions:
     */

    function _suspendCredential(
        address identity,
        address actor,
        bytes32 RevokId
    )
        private
        _identityOwner(identity, actor) // the sender must be equal to the identity
        _checkIssuer(RevokId)
    {
        _revReg[RevokId].metadata.status = Status.suspended;
        _revReg[RevokId].metadata.updated = block.timestamp;

        ///suspended credential event
        emit CredentialSuspended(msg.sender, RevokId);
    }

    /**
     * Unrevoke Credential functions:
     */

    function _UnrevokedCredential(
        address identity,
        address actor,
        bytes32 RevokId
    )
        private
        _identityOwner(identity, actor) // the sender must be equal to the identity
        _checkIssuer(RevokId)
    {
        _revReg[RevokId].metadata.status = Status.active;
        _revReg[RevokId].metadata.updated = block.timestamp;

        ///credential Unrevoked event
        emit CredentialUnrevoked(msg.sender, RevokId);
    }

    /**
     * Resolve Revocation functions:
     */

    /// @inheritdoc RevocationRegistryInterface
    function resolveRevocation(bytes32 RevokId) public view returns (RevocationRecord memory revocationRecord) {
        return _revReg[RevokId];
    }

    

}
