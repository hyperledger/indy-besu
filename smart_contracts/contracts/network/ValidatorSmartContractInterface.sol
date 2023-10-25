// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title ValidatorSmartContractInterface
 * @dev The contract interface that is used to getting an allowlist of validators.
 */
interface ValidatorSmartContractInterface {

    /**
     * @dev Error that thrown when initial validators are required but not provided.
     */
    error InitialValidatorsRequired();

    /**
     * @dev Error that thrown when an invalid validator address is provided.
     */
    error InvalidValidatorAddress();

    /**
     * @dev Error that thrown when an invalid account address is provided.
     */
    error InvalidValidatorAccountAddress();

    /**
     * @dev Error that thrown when attempting to add more validators than the allowed limit.
     * @param limit The maximum number of validators allowed.
     */
    error ExceedsValidatorLimit(uint limit);

    /**
     * @dev Error that thrown when attempting to add a validator that already exists.
     * @param validator The address of the validator.
     */
    error ValidatorAlreadyExists(address validator);

    /**
     * @dev Error that thrown when the sender already has an active validator.
     * @param sender The address of the sender.
     */
    error SenderHasActiveValidator(address sender);

    /**
     * @dev Error that thrown when attempting to deactivate the last remaining validator.
     */
    error CannotDeactivateLastValidator();

    /**
     * @dev Error that thrown when the specified validator is not found.
     * @param validator The address of the validator.
     */
    error ValidatorNotFound(address validator);

    /**
     * @dev Event emitting when new validator node added.
     * @param validator The address of the validator.
     * @param byAccount The address of the account that added the validator.
     * @param numValidators The total number of active validators after adding.
     */
    event ValidatorAdded (
        address indexed validator,
        address indexed byAccount,
        uint numValidators
    );

    /**
     * @dev Event emitting when removed validator.
     * @param validator The address of the validator.
     * @param byAccount The address of the account that removed the validator.
     * @param numValidators The total number of active validators after removal.
     */
    event ValidatorRemoved (
        address indexed validator,
        address indexed byAccount,
        uint numValidators
    );

    /**
     * @dev Gets the list of active validators.
     * @return A array of the active validators.
     */
    function getValidators() external view returns (address[] memory);
}
