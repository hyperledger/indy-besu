// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";

import { DidAlreadyExist, DidHasBeenDeactivated, DidNotFound, NotIdentityOwner } from "./DidErrors.sol";
import { DidRecord } from "./DidTypes.sol";
import { IndyDidRegistryInterface } from "./IndyDidRegistryInterface.sol";
import { RoleControlInterface } from "../auth/RoleControl.sol";

contract IndyDidRegistry is IndyDidRegistryInterface, ControlledUpgradeable {
    // TODO: add nonce for endorsing transactions

    RoleControlInterface internal _roleControl;

    /**
     * @dev Mapping DID to its corresponding DidRecord (Document/Metadata).
     */
    mapping(address identity => DidRecord didRecord) private _dids;

    /**
     * Checks that DID already exists
     */
    modifier _didExist(address identity) {
        if (_dids[identity].metadata.created == 0) revert DidNotFound(identity);
        _;
    }

    /**
     * Checks that the DID has not yet been added
     */
    modifier _didNotExist(address identity) {
        if (_dids[identity].metadata.created != 0) revert DidAlreadyExist(identity);
        _;
    }

    /**
     * Checks that the DID has not been deactivated
     */
    modifier _didIsActive(address identity) {
        if (_dids[identity].metadata.deactivated) revert DidHasBeenDeactivated(identity);
        _;
    }

    /**
     * Checks that method was called either by Trustee or Endorser or Steward
     */
    modifier _senderIsTrusteeOrEndorserOrSteward() {
        _roleControl.isTrusteeOrEndorserOrSteward(msg.sender);
        _;
    }

    /**
     * Checks that method was called either by Identity owner or Trustee or Endorser or Steward
     */
    modifier _senderIsIdentityOwnerOrTrustee(address identity) {
        if (msg.sender == identity) {
            _;
        } else {
            _roleControl.isTrustee(msg.sender);
            _;
        }
    }

    /**
     * Checks that actor matches to the identity
     */
    modifier _identityOwner(address identity, address actor) {
        if (identity != actor) revert NotIdentityOwner(actor, identity);
        _;
    }

    function initialize(address upgradeControlAddress, address roleControlContractAddress) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _roleControl = RoleControlInterface(roleControlContractAddress);
    }

    /// @inheritdoc IndyDidRegistryInterface
    function createDid(address identity, bytes calldata document) public {
        _createDid(identity, msg.sender, document);
    }

    /// @inheritdoc IndyDidRegistryInterface
    function createDidSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes calldata document) public {
        bytes32 hash = keccak256(
            abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "createDid", document)
        );
        _createDid(identity, ecrecover(hash, sigV, sigR, sigS), document);
    }

    /// @inheritdoc IndyDidRegistryInterface
    function updateDid(address identity, bytes calldata document) public {
        _updateDid(identity, msg.sender, document);
    }

    /// @inheritdoc IndyDidRegistryInterface
    function updateDidSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes calldata document) public {
        bytes32 hash = keccak256(
            abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "updateDid", document)
        );
        _updateDid(identity, ecrecover(hash, sigV, sigR, sigS), document);
    }

    /// @inheritdoc IndyDidRegistryInterface
    function deactivateDid(address identity) public {
        _deactivateDid(identity, msg.sender);
    }

    /// @inheritdoc IndyDidRegistryInterface
    function deactivateDidSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS) public {
        bytes32 hash = keccak256(abi.encodePacked(bytes1(0x19), bytes1(0), address(this), identity, "deactivateDid"));
        _deactivateDid(identity, ecrecover(hash, sigV, sigR, sigS));
    }

    /// @inheritdoc IndyDidRegistryInterface
    function resolveDid(address identity) public view virtual _didExist(identity) returns (DidRecord memory didRecord) {
        return _dids[identity];
    }

    function _createDid(
        address identity,
        address actor, // actor is either message sender in case of `createDid` or signer in case of `createDidSigner`
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
        address actor, // actor is either message sender in case of `updateDid` or signer in case of `updateDidSigner`
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
        address actor // actor is either message sender in case of `deactivateDid` or signer in case of `deactivateDidSigner`
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
