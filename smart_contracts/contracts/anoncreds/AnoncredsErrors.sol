// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

// Issuer errors

/**
 * @notice Error that occurs when the provided issuer is not found.
 * @param did   Issuer DID.
 */
error IssuerNotFound(string did);

/**
 * @notice Error that occurs when the provided issuer ID is invalid.
 * @param did   Issuer DID.
 */
error InvalidIssuerId(string did);

/**
 * @notice Error that occurs when attempting to perform an operation on behalf of a deactivated issuer.
 * @param did   Issuer DID.
 */
error IssuerHasBeenDeactivated(string did);

// Schema errors

/**
 * @notice Error that occurs when trying to create a schema with already existing schema identifier.
 * @param id    Keccak hash of Schema ID.
 */
error SchemaAlreadyExist(bytes32 id);

/**
 * @notice Error that occurs when the specified schema is not found.
 * @param id    Keccak hash of Schema ID.
 */
error SchemaNotFound(bytes32 id);

// CredDef errors

/**
 * @notice Error that occurs when trying to create a credential definition with already existing identifier.
 * @param id     Keccak hash of Credential definition ID.
 */
error CredentialDefinitionAlreadyExist(bytes32 id);

/**
 * @notice Error that occurs when the specified credential definition is not found.
 * @param id    Keccak hash of Credential definition ID.
 */
error CredentialDefinitionNotFound(bytes32 id);

// Revocation errors

/**
 * @notice Error that occurs when the specified revocation is not found.
 * @param id    Keccak hash of Revocation ID.
 */
error RevocationNotFound(bytes32 id);

/**
 * @notice Error that occurs when trying to create a revocation with an already existing identifier.
 * @param id    Keccak hash of Revocation ID.
 */
error RevocationAlreadyExist(bytes32 id);

/**
 * @notice Error that occurs when trying to revoke a credential without being listed in the Revocation registry.
 * @param id    Keccak hash of Revocation ID.
 */
error RevocationDoesntExist(bytes32 id);

/**
 * @notice Error that occurs when attempting to perform an operation on a revocation that is not active.
 * @param id    Keccak hash of Revocation ID.
 */
error RevocationIsNotActived(bytes32 id);

/**
 * @notice Error that occurs when attempting to perform an operation on a revocation that is not suspended.
 * @param id    Keccak hash of Revocation ID.
 */
error RevocationIsNotsuspended(bytes32 id);

/**
 * @notice Error that occurs when attempting to perform an operation on a revocation that is not revoked.
 * @param id    Keccak hash of Revocation ID.
 */
error RevocationIsNotRevoked(bytes32 id);

/**
 * @notice Error that occurs when attempting to revoke a credential that is already revoked.
 * @param id    Keccak hash of Credential ID.
 */
error CredentialIsAlreadyRevoked(bytes32 id);

/**
 * @notice Error that occurs when the specified issuer is invalid.
 * @param id    Keccak hash of Issuer ID.
 */
error InvalidIssuer(bytes32 id);
