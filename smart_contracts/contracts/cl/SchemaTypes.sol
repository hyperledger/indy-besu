// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title SchemaRecord
 * @dev This struct holds the details of a schema
 * and its associated metadata.
 *
 * @param schema - Schema as a JSON string.
 * @param metadata - Additional metadata associated with the schema.
 */
struct SchemaRecord {
    string schema;
    SchemaMetadata metadata;
}

/**
 * @title SchemaMetadata
 * @dev This struct holds additional metadata for a schema.
 *
 * @param created - Timestamp indicating when the schema was created.
 */
struct SchemaMetadata {
    uint256 created;
}
