// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title RevocationRecord
 * @dev This struct holds the details of a Revocation
 * and its associated metadata.
 *
 * @param document The revocation document in bytes.
 * @param metadata Additional metadata associated with the revocation.
 */
struct RevocationRecord {
    bytes document;
    RevocationMetadata metadata;
}

/**
 * @title RevocationMetadata
 * @dev This struct holds additional metadata for a Revocation.
 *
 * @param created Timestamp indicating when the revocation was created.
 * @param creator The address of the creator of the revocation.
 * @param updated Timestamp indicating when the revocation was last updated.
 * @param status The current status of the revocation (active, suspended, revoked).
 */
struct RevocationMetadata {
    uint256 created;
    address creator;
    uint256 updated;
    Status status;
}

/**
 * @title Status
 * @dev Enum representing the possible statuses of a revocation.
 *
 * @param active Indicates that the revocation is currently active.
 * @param suspended Indicates that the revocation is currently suspended.
 * @param revoked Indicates that the revocation has been revoked.
 */
enum Status {
    active,
    suspended,
    revoked
}
