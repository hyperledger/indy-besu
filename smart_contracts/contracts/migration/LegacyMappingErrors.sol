// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @notice Error that occurs when trying to add a duplicating mapping for a legacy identifier.
 * @param identifier Legacy DID identifier.
 */
error DidMappingAlreadyExist(string identifier);

/**
 * @notice Error that occurs when DID mapping does not exist.
 * @param identifier Legacy DID identifier.
 */
error DidMappingDoesNotExist(string identifier);

/**
 * @notice Error that occurs when trying to add a duplicating mapping for a legacy resource  identifier.
 * @param identifier Legacy Schema/CredDef identifier.
 */
error ResourceMappingAlreadyExist(string identifier);

/**
 * @notice Error that occurs when provided ed25519Key key does not match to the provided legacy DID identifier.
 * @param ed25519Key verification key matching to the DID.
 * @param identifier legacy DID identifier.
 */
error InvalidEd25519Key(bytes32 ed25519Key, string identifier);

/**
 * @notice Error that occurs when provided resource id does not contain controller legacy DID identifier.
 * @param identifier legacy resource identifier.
 * @param legacyDid legacy did.
 */
error InvalidResourceId(string identifier, string legacyDid);
