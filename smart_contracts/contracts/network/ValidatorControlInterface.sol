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
     * @dev Error that occurs when initial validators are required but not provided.
     */
    error InitialValidatorsRequired();

    /**
     * @dev Error that occurs when an invalid validator address is provided.
     */
    error InvalidValidatorAddress();

    /**
     * @dev Error that occurs when an invalid account address is provided.
     */
    error InvalidValidatorAccountAddress();

    /**
     * @dev Error that occurs when attempting to add more validators than the allowed limit.
     * @param limit The maximum number of validators allowed.
     */
    error ExceedsValidatorLimit(uint16 limit);

    /**
     * @dev Error that occurs when trying to add a validator that already exists.
     * @param validator The address of the validator.
     */
    error ValidatorAlreadyExists(address validator);

    /**
     * @dev Error that occurs when the sender already has an active validator.
     * @param sender The address of the sender.
     */
    error SenderHasActiveValidator(address sender);

    /**
     * @dev Error that occurs when trying to deactivate the last remaining validator.
     */
    error CannotDeactivateLastValidator();

    /**
     * @dev Error that occurs when the specified validator is not found.
     * @param validator The address of the validator.
     */
    error ValidatorNotFound(address validator);

    /**
     * @dev Gets the list of active validators.
     * @return A array of the active validators.
     */
    function getValidators() external view returns (address[] memory);

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
}
