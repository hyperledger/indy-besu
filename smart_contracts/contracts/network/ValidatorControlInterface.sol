// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title ValidatorControlInterface
 * @dev The interface that defines function for controlling the list of the network validator nodes
 */
interface ValidatorControlInterface {
    /**
     * @dev Event emitting when new validator node added.
     * @param validator The address of the validator.
     * @param byAccount The address of the account that added the validator.
     * @param numValidators The total number of active validators after adding.
     */
    event ValidatorAdded(address indexed validator, address indexed byAccount, uint8 numValidators);

    /**
     * @dev Event emitting when removed validator.
     * @param validator The address of the validator.
     * @param byAccount The address of the account that removed the validator.
     * @param numValidators The total number of active validators after removal.
     */
    event ValidatorRemoved(address indexed validator, address indexed byAccount, uint8 numValidators);

    /**
     * @dev Adds a new validator to the list.
     *
     * Restrictions:
     * - Only accounts with the steward role are permitted to call this method; otherwise, will revert with an `Unauthorized` error.
     * - The validator address must be non-zero; otherwise, will revert with an `InvalidValidatorAddress` error.
     * - The total number of validators must not exceed 256; otherwise, will revert with an `ExceedsValidatorLimit` error.
     * - The validator must not already exist in the list; otherwise, will revert with an `ValidatorAlreadyExists` error.
     * - The sender of the transaction must not have an active validator; otherwise, will revert with a `SenderHasActiveValidator` error.
     *
     * @param newValidator      The address of the validator node to add.
     *
     * Events:
     * - On successful validator creation, will emit a `ValidatorAdded` event.
     */
    function addValidator(address newValidator) external;

    /**
     * @dev Remove an existing validator from the list.
     *
     * Restrictions:
     * - Only accounts with the steward role are permitted to call this method; otherwise, will revert with an `Unauthorized` error.
     * - The validator address must be non-zero; otherwise, will revert with an `InvalidValidatorAddress` error.
     * - The validator must not be last one; otherwise, will revert with an `CannotDeactivateLastValidator` error.
     * - The validator must exist; otherwise, will revert with an `ValidatorNotFound` error.
     *
     * @param validator      The address of the validator node to remove.
     *
     * Events:
     * - On successful validator removal, will emit a `ValidatorRemoved` event.
     */
    function removeValidator(address validator) external;

    /**
     * @dev Gets the list of active validators.
     * @return A array of the active validators.
     */
    function getValidators() external view returns (address[] memory);
}
