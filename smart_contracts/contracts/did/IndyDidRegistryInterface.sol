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
     * @param identity Address of created DID Record.
     */
    event DIDCreated(address identity);

    /**
     * @dev Event that is sent when a DID Document is updated.
     *
     * @param identity Address of updated DID Record.
     */
    event DIDUpdated(address identity);

    /**
     * @dev Event that is sent when a DID Document is deactivated.
     *
     * @param identity Address of deactivated DID Record
     */
    event DIDDeactivated(address identity);

    /**
     * @dev Creates a new DID record.
     *
     * Restrictions:
     * - DID must not already exist; otherwise, will revert with a `DidAlreadyExist` error.
     * - Sender address must has either TRUSTEE or ENDORSER or STEWARD role; otherwise, will revert with a `Unauthorized` error.
     * - Sender address must be equal to passed identity address; otherwise, will revert with a `NotIdentityOwner` error.
     *
     * @param identity  Address of DID identity owner.
     * @param document  DID Document JSON as bytes.
     */
    function createDid(address identity, bytes calldata document) external;

    /**
     * @dev Endorses a new DID record (off-chain author signature).
     *
     * Restrictions:
     * - DID must not already exist; otherwise, will revert with a `DidAlreadyExist` error.
     * - Sender address must has either TRUSTEE or ENDORSER or STEWARD role; otherwise, will revert with a `Unauthorized` error.
     * - Signer address must be equal to passed identity address; otherwise, will revert with a `NotIdentityOwner` error.
     *
     * @param identity  Address of DID identity owner.
     * @param sigV      Part of EcDSA signature.
     * @param sigR      Part of EcDSA signature.
     * @param sigS      Part of EcDSA signature.
     * @param document  DID Document JSON as bytes.
     */
    function createDidSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes calldata document
    ) external;

    /**
     * @dev Updates an existing DID record.
     *
     * Restrictions:
     *
     * - DID must already exist; otherwise, will revert with a `DidNotFound` error.
     * - DID must be active; otherwise, will revert with a `DidHasBeenDeactivated` error.
     * - Sender address must be equal either to DID owner or has TRUSTEE role; otherwise, will revert with a `Unauthorized` error.
     * - Sender address must be equal to passed identity address; otherwise, will revert with a `NotIdentityOwner` error.
     *
     * Events:
     * - On successful DID update, will emit a `DIDUpdated` event.
     *
     * @param identity Address of the DID to update.
     * @param document Updated DID Document JSON as bytes.
     */
    function updateDid(address identity, bytes calldata document) external;

    /**
     * @dev Endorses an updated DID document for an existing DID record (off-chain author signature).
     *
     * Restrictions:
     * - DID must already exist; otherwise, will revert with a `DidNotFound` error.
     * - DID must be active; otherwise, will revert with a `DidHasBeenDeactivated` error.
     * - Sender address must be equal either to DID owner or has TRUSTEE role; otherwise, will revert with a `Unauthorized` error.
     * - Signer address must be equal to passed identity address; otherwise, will revert with a `NotIdentityOwner` error.
     *
     * Events:
     * - On successful DID update, will emit a `DIDUpdated` event.
     *
     * @param identity  Address of the DID to update.
     * @param sigV      Part of EcDSA signature.
     * @param sigR      Part of EcDSA signature.
     * @param sigS      Part of EcDSA signature.
     * @param document  The updated DID Document as JSON string.
     */
    function updateDidSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes calldata document
    ) external;

    /**
     * @dev Deactivates an existing DID.
     *
     * Restrictions:
     * - DID must be active; otherwise, will revert with a `DidHasBeenDeactivated` error.
     * - DID must exist; otherwise, will revert with a `DidNotFound` error.
     * - Sender address must be equal either to DID owner or has TRUSTEE role; otherwise, will revert with a `Unauthorized` error.
     * - Sender address must be equal to passed identity address; otherwise, will revert with a `NotIdentityOwner` error.
     *
     * Events:
     * - On successful DID deactivation, will emit a `DIDDeactivated` event.
     *
     * @param identity Address of the DID to be deactivated.
     */
    function deactivateDid(address identity) external;

    /**
     * @dev Endorses deactivation of an existing DID (off-chain author signature).
     *
     * Restrictions:
     * - DID must be active; otherwise, will revert with a `DidHasBeenDeactivated` error.
     * - DID must exist; otherwise, will revert with a `DidNotFound` error.
     * - Sender address must be equal either to DID owner or has TRUSTEE role; otherwise, will revert with a `Unauthorized` error.
     * - Signer address must be equal to passed identity address; otherwise, will revert with a `NotIdentityOwner` error.
     *
     * Events:
     * - On successful DID deactivation, will emit a `DIDDeactivated` event.
     *
     * @param identity  Address of the DID to be deactivated.
     * @param sigV      Part of EcDSA signature.
     * @param sigR      Part of EcDSA signature.
     * @param sigS      Part of EcDSA signature.
     */
    function deactivateDidSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS) external;

    /**
     * @dev Function to resolve DID Document for the given DID.
     *
     * Restrictions:
     * - DID must exist; otherwise, will revert with a `DidNotFound` error.
     *
     * @param identity   Address of the DID  be resolved.
     *
     * @return didRecord The resolved DID record associated with provided DID identity address.
     */
    function resolveDid(address identity) external view returns (DidRecord memory didRecord);
}
