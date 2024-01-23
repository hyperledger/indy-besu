// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @dev Error that occurs when the specified DID is not found.
 */
error DidNotFound(string did);

/**
 * @dev Error that occurs when trying to create an already existing DID.
 */
error DidAlreadyExist(string did);

/**
 * @dev Error that occurs when trying to perform an operation with a deactivated DID.
 */
error DidHasBeenDeactivated(string did);

/**
 * @dev Error that occurs when message sender address is nether DID owner or original creator.
 */
error UnauthorizedSender(address sender);

/**
 * @dev Error that occurs when the specified DID is incorrect.
 */
error IncorrectDid(string did);
