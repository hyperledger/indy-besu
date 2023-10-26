// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";

import { DidAlreadyExist, DidHasBeenDeactivated, DidNotFound } from "./DidErrors.sol";
import { DidRegistryInterface } from "./DidRegistryInterface.sol";
import { DidDocument, DidDocumentStorage } from "./DidTypes.sol";
import { DidValidator } from "./DidValidator.sol";

contract DidRegistry is DidRegistryInterface, ControlledUpgradeable {
    /**
     * @dev Mapping DID to its corresponding DID Document.
     */
    mapping(string => DidDocumentStorage) private dids;

    /**
     * Checks that DID already exists
     */
    modifier didExist(string memory did) {
        if (dids[did].metadata.created == 0) revert DidNotFound(did);
        _;
    }

    /**
     * Checks that the DID has not yet been added
     */
    modifier didNotExist(string memory did) {
        if (dids[did].metadata.created != 0) revert DidAlreadyExist(did);
        _;
    }

    /**
     * Сhecks that the DID has not been deactivated
     */
    modifier didIsActive(string memory did) {
        if (dids[did].metadata.deactivated) revert DidHasBeenDeactivated(did);
        _;
    }

    function initialize(address upgradeControlAddress) public reinitializer(1) {
      _initializeUpgradeControl(upgradeControlAddress);
    }

    /// @inheritdoc DidRegistryInterface
    function createDid(
        DidDocument calldata document
    ) public _didNotExist(document.id) {
        DidValidator.validateDid(document.id);
        DidValidator.validateVerificationKey(document);

        dids[document.id].document = document;
        dids[document.id].metadata.created = block.timestamp;
        dids[document.id].metadata.updated = block.timestamp;

        emit DIDCreated(document.id);
    }

    /// @inheritdoc DidRegistryInterface
    function updateDid(
        DidDocument calldata document
    ) public _didExist(document.id) _didIsActive(document.id) {
        DidValidator.validateVerificationKey(document);

        dids[document.id].document = document;
        dids[document.id].metadata.updated = block.timestamp;

        emit DIDUpdated(document.id);
    }

    /// @inheritdoc DidRegistryInterface
    function deactivateDid(
        string calldata id
    ) public _didExist(id) _didIsActive(id) {
        dids[id].metadata.deactivated = true;

        emit DIDDeactivated(id);
    }

    /// @inheritdoc DidRegistryInterface
    function resolveDid(
        string calldata id
    ) public _didExist(id) view virtual returns (DidDocumentStorage memory didDocumentStorage) {
        return dids[id];
    }
}
