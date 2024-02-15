// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { InitialValidatorsRequired, InvalidValidatorAddress, InvalidValidatorAccountAddress, ExceedsValidatorLimit, ValidatorAlreadyExists, SenderHasActiveValidator, CannotDeactivateLastValidator, ValidatorNotFound } from "./ValidatorErrors.sol";
import { RoleControlInterface } from "../auth/RoleControl.sol";
import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";

import { ValidatorControlInterface } from "./ValidatorControlInterface.sol";

contract ValidatorControl is ValidatorControlInterface, ControlledUpgradeable {
    /**
     * @dev Type describing initial validator details.
     */
    struct InitialValidatorInfo {
        address validator;
        address account;
    }

    /**
     * @dev Type describing validator details.
     */
    struct ValidatorInfo {
        address account;
        uint8 validatorIndex;
    }

    /**
     * @dev Max allowed number of validators.
     */
    uint16 private constant _MAX_VALIDATORS = 256;

    /**
     * @dev Reference to the contract managing auth permissions.
     */
    RoleControlInterface private _roleControl;

    /**
     * @dev List of active validators.
     */
    address[] private _validators;

    /**
     * @dev Mapping of validator address to validator info (owner, index, active).
     */
    mapping(address validatorAddress => ValidatorInfo validatorInfo) private _validatorInfos;

    /**
     * @dev Modifier that checks that the sender account has Steward role assigned.
     */
    modifier _senderIsSteward() {
        _roleControl.isSteward(msg.sender);
        _;
    }

    /**
     * @dev Modifier that checks that the validator address is non-zero.
     */
    modifier _nonZeroValidatorAddress(address validator) {
        if (validator == address(0)) revert InvalidValidatorAddress();
        _;
    }

    function initialize(
        address roleControlContractAddress,
        address upgradeControlAddress,
        InitialValidatorInfo[] memory initialValidators
    ) public reinitializer(1) {
        if (initialValidators.length == 0) revert InitialValidatorsRequired();
        if (initialValidators.length >= _MAX_VALIDATORS) revert ExceedsValidatorLimit(_MAX_VALIDATORS);

        for (uint256 i = 0; i < initialValidators.length; i++) {
            if (initialValidators[i].account == address(0)) revert InvalidValidatorAccountAddress();
            if (initialValidators[i].validator == address(0)) revert InvalidValidatorAddress();

            InitialValidatorInfo memory validator = initialValidators[i];

            _validators.push(validator.validator);
            _validatorInfos[validator.validator] = ValidatorInfo(validator.account, uint8(i));
        }

        _roleControl = RoleControlInterface(roleControlContractAddress);
        _initializeUpgradeControl(upgradeControlAddress);
    }

    /// @inheritdoc ValidatorControlInterface
    function addValidator(address newValidator) public _senderIsSteward _nonZeroValidatorAddress(newValidator) {
        if (_validators.length >= _MAX_VALIDATORS) revert ExceedsValidatorLimit(_MAX_VALIDATORS);

        uint8 validatorsCount = uint8(_validators.length);
        for (uint8 i = 0; i < validatorsCount; i++) {
            ValidatorInfo memory validatorInfo = _validatorInfos[_validators[i]];
            if (newValidator == _validators[i]) revert ValidatorAlreadyExists(_validators[i]);
            if (msg.sender == validatorInfo.account) revert SenderHasActiveValidator(msg.sender);
        }

        _validatorInfos[newValidator] = ValidatorInfo(msg.sender, validatorsCount);
        _validators.push(newValidator);

        // emit success event
        emit ValidatorAdded(newValidator, msg.sender, uint8(_validators.length));
    }

    /// @inheritdoc ValidatorControlInterface
    function removeValidator(address validator) public _senderIsSteward _nonZeroValidatorAddress(validator) {
        if (_validators.length == 1) revert CannotDeactivateLastValidator();

        ValidatorInfo memory removedValidatorInfo = _validatorInfos[validator];
        if (removedValidatorInfo.account == address(0)) revert ValidatorNotFound(validator);

        uint8 removedValidatorIndex = removedValidatorInfo.validatorIndex;

        // put last validator in the list on place of removed validator
        address validatorRemoved = _validators[removedValidatorIndex];
        address validatorToBeMoved = _validators[_validators.length - 1];
        _validators[removedValidatorIndex] = validatorToBeMoved;

        // update indexes
        _validatorInfos[validatorToBeMoved].validatorIndex = removedValidatorIndex;

        // remove last validator which was copied to new place
        _validators.pop();
        delete (_validatorInfos[validatorRemoved]);

        // emit success event
        emit ValidatorRemoved(validatorRemoved, msg.sender, uint8(_validators.length));
    }

    /// @inheritdoc ValidatorControlInterface
    function getValidators() public view override returns (address[] memory) {
        return _validators;
    }
}
