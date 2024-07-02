// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";
import { DidAlreadyExist, DidHasBeenDeactivated, DidNotFound, NotIdentityOwner } from "./DidErrors.sol";
import { DidRecord } from "./DidTypes.sol";
import { IndyDidRegistryInterface } from "./IndyDidRegistryInterface.sol";
import { RoleControlInterface } from "../auth/RoleControl.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

contract IndyDidRegistry is IndyDidRegistryInterface, ControlledUpgradeable {
    RoleControlInterface internal _roleControl;

    mapping(address => DidRecord) private _dids;

    modifier _didExist(address identity) {
        if (_dids[identity].metadata.created == 0) revert DidNotFound(identity);
        _;
    }

    modifier _didNotExist(address identity) {
        if (_dids[identity].metadata.created != 0) revert DidAlreadyExist(identity);
        _;
    }

    modifier _didIsActive(address identity) {
        if (_dids[identity].metadata.deactivated) revert DidHasBeenDeactivated(identity);
        _;
    }

    modifier _senderIsTrusteeOrEndorserOrSteward() {
        _roleControl.isTrusteeOrEndorserOrSteward(msg.sender);
        _;
    }

    modifier _senderIsIdentityOwnerOrTrustee(address identity) {
        if (msg.sender == identity) {
            _;
        } else {
            _roleControl.isTrustee(msg.sender);
            _;
        }
    }

    modifier _identityOwner(address identity, address actor) {
        if (identity != actor) revert NotIdentityOwner(actor, identity);
        _;
    }

    function initialize(address upgradeControlAddress, address roleControlContractAddress) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _roleControl = RoleControlInterface(roleControlContractAddress);
    }

    function createDid(address identity, bytes calldata document) public {
        _createDid(identity, msg.sender, document);
    }

    function createDidSigned(
        address identity,
        bytes32 hash,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes calldata document
    ) public returns (address) {
        address recoveredAddress = ECDSA.recover(hash, sigV, sigR, sigS);

        _createDid(identity, recoveredAddress, document);

        return recoveredAddress;
    }

    function updateDid(address identity, bytes calldata document) public {
        _updateDid(identity, msg.sender, document);
    }

    function updateDidSigned(
        address identity,
        bytes32 hash,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes calldata document
    ) public {
        _updateDid(identity, ECDSA.recover(hash, sigV, sigR, sigS), document);
    }

    function deactivateDid(address identity) public {
        _deactivateDid(identity, msg.sender);
    }

    function deactivateDidSigned(address identity, bytes32 hash, uint8 sigV, bytes32 sigR, bytes32 sigS) public {
        _deactivateDid(identity, ECDSA.recover(hash, sigV, sigR, sigS));
    }

    function resolveDid(address identity) public view _didExist(identity) returns (DidRecord memory didRecord) {
        return _dids[identity];
    }

    function _createDid(
        address identity,
        address actor,
        bytes calldata document
    ) internal _didNotExist(identity) _identityOwner(identity, actor) _senderIsTrusteeOrEndorserOrSteward {
        _dids[identity].document = document;
        _dids[identity].metadata.owner = identity;
        _dids[identity].metadata.created = block.timestamp;
        _dids[identity].metadata.updated = block.timestamp;
        _dids[identity].metadata.versionId = block.number;

        emit DIDCreated(identity);
    }

    function _updateDid(
        address identity,
        address actor,
        bytes calldata document
    )
        internal
        _didExist(identity)
        _didIsActive(identity)
        _identityOwner(identity, actor)
        _senderIsIdentityOwnerOrTrustee(identity)
    {
        _dids[identity].document = document;
        _dids[identity].metadata.updated = block.timestamp;
        _dids[identity].metadata.versionId = block.number;

        emit DIDUpdated(identity);
    }

    function _deactivateDid(
        address identity,
        address actor
    )
        internal
        _didExist(identity)
        _didIsActive(identity)
        _identityOwner(identity, actor)
        _senderIsIdentityOwnerOrTrustee(identity)
    {
        _dids[identity].metadata.deactivated = true;
        _dids[identity].metadata.versionId = block.number;

        emit DIDDeactivated(identity);
    }
}
