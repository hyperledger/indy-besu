// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @dev Error that occurs when the specified DID is not found.
 */
error DidNotFound(address identity);

/**
 * @dev Error that occurs when trying to create an already existing DID.
 */
error DidAlreadyExist(address identity);

/**
 * @dev Error that occurs when trying to perform an operation with a deactivated DID.
 */
error DidHasBeenDeactivated(address identity);

/**
 * @dev Error that occurs when the specified DID is incorrect.
 */
error IncorrectDid(string did);

/**
 * @notice Error that occurs when performed identity operation by not owned account.
 */
error NotIdentityOwner(address actor, address identity);
