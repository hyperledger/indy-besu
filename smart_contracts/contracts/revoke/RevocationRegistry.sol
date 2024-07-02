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
import { NotIdentityOwner, DidNotFound } from "../did/DidErrors.sol";
import { RevocationNotFound, RevocationAlreadyExist, CredentialDefinitionNotFound, RevocationIsNotActived, RevocationIsNotsuspended, RevocationIsNotRevoked, CredentialIsAlreadyRevoked, InvalidIssuer } from "../anoncreds/AnoncredsErrors.sol";
import { RoleControlInterface } from "../auth/RoleControl.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

contract RevocationRegistry is RevocationRegistryInterface, ControlledUpgradeable, AnoncredsRegistry {
    /**
     * @dev Reference to the contract that manages anoncreds credDefs
     */
    CredentialDefinitionRegistryInterface private _credDefRegistry;

    /**
     * Mapping Revocation ID to its Revocation Details and Metadata.
     */
    mapping(bytes32 id => RevocationRecord revocationRecord) private _revReg;
    mapping(bytes32 id => RevocationRecord revocationSignedRecord) private _revSig;

    /**
     * Сhecks that the revocation exist
     */
    modifier _revocationNotExist(bytes32 id) {
        if (_revReg[id].metadata.created != 0) revert RevocationAlreadyExist(id);
        _;
    }
    modifier _revocationNotExistSig(bytes32 id) {
        if (_revSig[id].metadata.created != 0) revert RevocationAlreadyExist(id);
        _;
    }
    /**
     * Сhecks  the Status of revocation
     */
    modifier _CredentialActive(bytes32 id) {
        if (_revReg[id].metadata.status == Status.revoked) revert CredentialIsAlreadyRevoked(id);
        _;
    }
    modifier _CredentialActiveSig(bytes32 id) {
        if (_revSig[id].metadata.status == Status.revoked) revert CredentialIsAlreadyRevoked(id);
        _;
    }
    /**
     * Сhecks  the Status of revocation
     */
    modifier _CredentialSuspend(bytes32 id) {
        if (_revReg[id].metadata.status != Status.active) revert RevocationIsNotActived(id);
        _;
    }
    modifier _CredentialSuspendSig(bytes32 id) {
        if (_revSig[id].metadata.status != Status.active) revert RevocationIsNotActived(id);
        _;
    }

    /**
     * Сhecks  the Status of revocation
     */
    modifier _CredentialRevoked(bytes32 id) {
        if (_revReg[id].metadata.status == Status.active) revert RevocationIsNotRevoked(id);
        _;
    }

    modifier _CredentialRevokedSig(bytes32 id) {
        if (_revSig[id].metadata.status == Status.active) revert RevocationIsNotRevoked(id);
        _;
    }

    /**
     * Сhecks  the Issuer of revocation
     */

    modifier _checkIssuer(bytes32 id) {
        if (_revReg[id].metadata.creator != msg.sender) revert InvalidIssuer(id);
        _;
    }

    modifier _checkIssuerSig(bytes32 id) {
        if (_revSig[id].metadata.creator != msg.sender) revert InvalidIssuer(id);
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
        bytes32 id
    ) public _CredentialActive(id) _senderIsTrusteeOrEndorserOrSteward {
        _revokeCredential(identity, msg.sender, id);
    }

    /// @inheritdoc RevocationRegistryInterface
    function revokeCredentialSigned(
        address identity,
        bytes32 hash,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id
    ) public _CredentialActiveSig(id) _credDefExist(id) _senderIsTrusteeOrEndorserOrSteward {
        _revokeCredentialSig(identity, ECDSA.recover(hash, sigV, sigR, sigS), id);
    }

    /**
     * Suspend functions:
     */
    function suspendCredential(
        address identity,
        bytes32 id
    ) public _CredentialSuspend(id) _credDefExist(id) _senderIsTrusteeOrEndorserOrSteward {
        _suspendCredential(identity, msg.sender, id);
    }

    /// @inheritdoc RevocationRegistryInterface
    function suspendCredentialSigned(
        address identity,
        bytes32 hash,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id
    ) public _CredentialSuspendSig(id) _credDefExist(id) _senderIsTrusteeOrEndorserOrSteward {
        _suspendCredentialSig(identity, ECDSA.recover(hash, sigV, sigR, sigS), id);
    }

    /**
     * Unrevok functions:
     */
    function unrevokeCredential(
        address identity,
        bytes32 id
    ) public _CredentialRevoked(id) _senderIsTrusteeOrEndorserOrSteward {
        _UnrevokedCredential(identity, msg.sender, id);
    }

    /// @inheritdoc RevocationRegistryInterface
    function unrevokeCredentialSigned(
        address identity,
        bytes32 hash,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id
    ) public _CredentialRevokedSig(id) _credDefExist(id) _senderIsTrusteeOrEndorserOrSteward {
        _UnrevokedCredentialSig(identity, ECDSA.recover(hash, sigV, sigR, sigS), id);
    }

    /**
     * create Revocation functions:
     */
    function createRevocationRegistry(
        address identity,
        bytes32 id,
        bytes calldata revokeDocument
    ) public _revocationNotExist(id) _senderIsTrusteeOrEndorserOrSteward _credDefExist(id) {
        _createRevocation(identity, msg.sender, id, revokeDocument);
    }

    /// @inheritdoc RevocationRegistryInterface
    function createRevocationRegistrySigned(
        address identity,
        bytes32 hash,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        bytes calldata revokeDocument
    ) public _revocationNotExistSig(id) _senderIsTrusteeOrEndorserOrSteward _credDefExist(id) {
        _createRevocationSig(identity, ECDSA.recover(hash, sigV, sigR, sigS), id, revokeDocument);
    }

    /**
     * Create Revocation functions:
     */

    function _createRevocation(
        address identity,
        address actor,
        bytes32 id,
        bytes calldata document
    )
        private
        _identityOwner(identity, actor) // the sender must be equal to the identity
    {
        _revReg[id].document = document;
        _revReg[id].metadata.created = block.timestamp;
        _revReg[id].metadata.creator = msg.sender;
        _revReg[id].metadata.updated = block.timestamp;
        _revReg[id].metadata.status = Status.active;

        emit RevocationCreated(msg.sender, id);
    }

    function _createRevocationSig(
        address identity,
        address actor,
        bytes32 id,
        bytes calldata document
    )
        private
        _identityOwner(identity, actor) // the signer must be equal to the identity
    {
        _revSig[id].document = document;
        _revSig[id].metadata.created = block.timestamp;
        _revSig[id].metadata.creator = msg.sender;
        _revSig[id].metadata.updated = block.timestamp;
        _revSig[id].metadata.status = Status.active;

        emit RevocationCreated(msg.sender, id);
    }

    /**
     * Revok Credential functions:
     */

    function _revokeCredential(
        address identity,
        address actor,
        bytes32 id
    )
        private
        _identityOwner(identity, actor) // the sender must be equal to the identity
        _checkIssuer(id)
    {
        _revReg[id].metadata.status = Status.revoked;
        _revReg[id].metadata.updated = block.timestamp;

        ///credential revocation event
        emit CredentialRevoked(msg.sender, id);
    }

    function _revokeCredentialSig(
        address identity,
        address actor,
        bytes32 id
    )
        private
        _identityOwner(identity, actor) // the signer must be equal to the identity
        _checkIssuerSig(id)
    {
        _revSig[id].metadata.status = Status.revoked;
        _revSig[id].metadata.updated = block.timestamp;

        ///credential revocation event
        emit CredentialRevoked(msg.sender, id);
    }

    /**
     * suspend Credential functions:
     */

    function _suspendCredential(
        address identity,
        address actor,
        bytes32 id
    )
        private
        _identityOwner(identity, actor) // the sender must be equal to the identity
        _checkIssuer(id)
    {
        _revReg[id].metadata.status = Status.suspended;
        _revReg[id].metadata.updated = block.timestamp;

        ///suspended credential event
        emit CredentialSuspended(msg.sender, id);
    }

    function _suspendCredentialSig(
        address identity,
        address actor,
        bytes32 id
    )
        private
        _identityOwner(identity, actor) // the signer must be equal to the identity
        _checkIssuerSig(id)
    {
        _revSig[id].metadata.status = Status.suspended;
        _revSig[id].metadata.updated = block.timestamp;

        ///suspended credential event
        emit CredentialSuspended(msg.sender, id);
    }

    /**
     * Unrevoke Credential functions:
     */

    function _UnrevokedCredential(
        address identity,
        address actor,
        bytes32 id
    )
        private
        _identityOwner(identity, actor) // the sender must be equal to the identity
        _checkIssuer(id)
    {
        _revReg[id].metadata.status = Status.active;
        _revReg[id].metadata.updated = block.timestamp;

        ///credential Unrevoked event
        emit CredentialUnrevoked(msg.sender, id);
    }

    function _UnrevokedCredentialSig(
        address identity,
        address actor,
        bytes32 id
    )
        private
        _identityOwner(identity, actor) // the signer must be equal to the identity
        _checkIssuerSig(id)
    {
        _revSig[id].metadata.status = Status.active;
        _revSig[id].metadata.updated = block.timestamp;

        ///credential Unrevoked event
        emit CredentialUnrevoked(msg.sender, id);
    }

    /**
     * Resolve Revocation functions:
     */

    /// @inheritdoc RevocationRegistryInterface
    function resolveRevocation(bytes32 id) public view returns (RevocationRecord memory revocationRecord) {
        return _revReg[id];
    }

    function resolveRevocationSig(bytes32 id) public view returns (RevocationRecord memory revocationRecord) {
        return _revSig[id];
    }
}
