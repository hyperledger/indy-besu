// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { DidRecord } from "./DidTypes.sol";

/**
 * @dev The interface that defines functions for managing DID documents.
 */
interface IndyDidRegistryInterface {
    /**
     * @dev Event that is sent when a DID Document is created.
     *
     * @param did Created DID.
     */
    event DIDCreated(string did);

    /**
     * @dev Event that is sent when a DID Document is updated.
     *
     * @param did Updated DID.
     */
    event DIDUpdated(string did);

    /**
     * @dev Event that is sent when a DID Document is deactivated.
     *
     * @param did Deactivated DID.
     */
    event DIDDeactivated(string did);

    /**
     * @dev Creates a new DID.
     *
     * @param identity  Address of DID owner.
     * @param did       The new DID.
     * @param document  The new DID Document as JSON string.
     */
    function createDid(address identity, string calldata did, string calldata document) external;

    /**
     * @dev Updates an existing DID.
     *
     * Restrictions:
     *
     * - DID must not already exist; otherwise, will revert with a `DidAlreadyExist` error.
     * - DID must be active; otherwise, will revert with a `DidHasBeenDeactivated` error.
     * - Sender address must be equal either to DID owner or creator; otherwise, will revert with a `UnauthorizedSender` error.
     *
     * Events:
     * - On successful DID update, will emit a `DIDUpdated` event.
     *
     * @param did      The DID to update.
     * @param document The updated DID Document as JSON string.
     */
    function updateDid(string calldata did, string calldata document) external;

    /**
     * @dev Deactivates a DID.
     *
     * Restrictions:
     * - DID must be active; otherwise, will revert with a `DidHasBeenDeactivated` error.
     * - DID must exist; otherwise, will revert with a `DidNotFound` error.
     * - Sender address must be equal either to DID owner or creator; otherwise, will revert with a `UnauthorizedSender` error.
     *
     * Events:
     * - On successful DID deactivation, will emit a `DIDDeactivated` event.
     *
     * @param did The DID to be deactivated.
     */
    function deactivateDid(string calldata did) external;

    /**
     * @dev Function to resolve DID Document for the given DID.
     *
     * Restrictions:
     * - DID must exist; otherwise, will revert with a `DidNotFound` error.
     *
     * @param did The DID to be resolved.
     *
     * @return didRecord The resolved DID document associated with provided DID.
     */
    function resolveDid(string calldata did) external view returns (DidRecord memory didRecord);
}
