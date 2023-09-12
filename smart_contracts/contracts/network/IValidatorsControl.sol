pragma solidity ^0.8.20;

interface IValidatorsControl {
    /**
     * @dev Get the list of active validators
     */
    function getValidators() external view returns (address[] memory);
}
