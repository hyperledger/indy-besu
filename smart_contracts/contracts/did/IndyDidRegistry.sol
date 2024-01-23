// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";

import { DidAlreadyExist, DidHasBeenDeactivated, DidNotFound, UnauthorizedSender } from "./DidErrors.sol";
import { IndyDidRegistryInterface } from "./IndyDidRegistryInterface.sol";
import { DidRecord } from "./DidTypes.sol";
import { IndyDidValidator } from "./IndyDidValidator.sol";

contract IndyDidRegistry is IndyDidRegistryInterface, ControlledUpgradeable {
    /**
     * @dev Mapping DID to its corresponding DID Document/Metadata.
     */
    mapping(string did => DidRecord didRecord) private _dids;

    /**
     * Checks that DID already exists
     */
    modifier _didExist(string memory did) {
        if (_dids[did].metadata.created == 0) revert DidNotFound(did);
        _;
    }

    /**
     * Checks that the DID has not yet been added
     */
    modifier _didNotExist(string memory did) {
        if (_dids[did].metadata.created != 0) revert DidAlreadyExist(did);
        _;
    }

    /**
     * Checks that the DID has not been deactivated
     */
    modifier _didIsActive(string memory did) {
        if (_dids[did].metadata.deactivated) revert DidHasBeenDeactivated(did);
        _;
    }

    /**
     * Checks that method was called either by did owner or sender
     */
    modifier _senderIsAuthorized(string memory did) {
        // FIXME: once we add strict role and endorsement, the check here should be either owner or Trustee
        if (msg.sender != _dids[did].metadata.owner && msg.sender != _dids[did].metadata.sender)
            revert UnauthorizedSender(msg.sender);
        _;
    }

    function initialize(address upgradeControlAddress) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
    }

    /// @inheritdoc IndyDidRegistryInterface
    function createDid(address identity, string calldata did, string calldata document) public _didNotExist(did) {
        IndyDidValidator.validateDidSyntax(did);

        _dids[did].document = document;
        _dids[did].metadata.owner = identity;
        _dids[did].metadata.sender = msg.sender;
        _dids[did].metadata.created = block.timestamp;
        _dids[did].metadata.updated = block.timestamp;

        emit DIDCreated(did);
    }

    /// @inheritdoc IndyDidRegistryInterface
    function updateDid(
        string calldata did,
        string calldata document
    ) public _didExist(did) _didIsActive(did) _senderIsAuthorized(did) {
        _dids[did].document = document;
        _dids[did].metadata.updated = block.timestamp;

        emit DIDUpdated(did);
    }

    /// @inheritdoc IndyDidRegistryInterface
    function deactivateDid(string calldata did) public _didExist(did) _didIsActive(did) _senderIsAuthorized(did) {
        _dids[did].metadata.deactivated = true;

        emit DIDDeactivated(did);
    }

    /// @inheritdoc IndyDidRegistryInterface
    function resolveDid(string calldata did) public view virtual _didExist(did) returns (DidRecord memory didRecord) {
        return _dids[did];
    }
}
