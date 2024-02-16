// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title CredentialDefinitionRecord
 * @dev This struct holds the details of a credential definition
 * and its associated metadata.
 *
 * @param credDef - Credential Definition JSON as bytes.
 * @param metadata - Additional metadata associated with the credential definition.
 */
struct CredentialDefinitionRecord {
    bytes credDef;
    CredentialDefinitionMetadata metadata;
}

/**
 * @title CredentialDefinitionMetadata
 * @dev This struct holds additional metadata for a credential definition.
 *
 * @param created - Timestamp indicating when the credential definition was created.
 */
struct CredentialDefinitionMetadata {
    uint256 created;
}
