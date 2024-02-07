// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

// Schema errors

/**
 * @notice Error that occurs when trying to create an already existing schema.
 * @param id Schema ID.
 */
error SchemaAlreadyExist(bytes32 id);

/**
 * @notice Error that occurs when the specified schema is not found.
 * @param id Schema ID.
 */
error SchemaNotFound(bytes32 id);

// CredDef errors

/**
 * @notice Error that occurs when trying to create an existing credential definition.
 * @param id Credential definition ID.
 */
error CredentialDefinitionAlreadyExist(bytes32 id);

/**
 * @notice Error that occurs when the specified credential definition is not found.
 * @param id Credential definition ID.
 */
error CredentialDefinitionNotFound(bytes32 id);
