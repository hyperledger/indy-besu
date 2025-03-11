// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title RevocationRegistryDefinitionRecord
 * @dev This struct holds the details of a revocation registry definition
 * and its associated metadata.
 *
 * @param revRegDef - Revocation Registry Definition JSON as bytes.
 * @param metadata  - Additional metadata associated with the revocation registry definition.
 */
struct RevocationRegistryDefinitionRecord {
    bytes revRegDef;
    RevocationRegistryDefinitionMetadata metadata;
}

/**
 * @title RevocationRegistryDefinitionMetadata
 * @dev This struct holds additional metadata for a revocation registry definition.
 *
 * @param created - Timestamp indicating when the revocation registry definition was created.
 * @param issuerId - DID of revocation registry issuer.
 * @param currentAccumulator - current RevocationRegistryDefinition accumulator.
 */
struct RevocationRegistryDefinitionMetadata {
    uint256 created;
    string issuerId;
    bytes currentAccumulator;
}

/**
 * @title RevocationRegistryEntry
 * @dev This struct holds the data of a new revocation registry entry (delta).
 *
 * @param currentAccumulator - New accumulator to be saved on-chain.
 * @param issued - list of newly issued credential indexes.
 * @param revoked - list of newly revoked credential indexes.
 */
struct RevocationRegistryEntry {
    bytes currentAccumulator;
    uint32[] issued;
    uint32[] revoked;
}
