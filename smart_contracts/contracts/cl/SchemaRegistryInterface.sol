// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

interface SchemaRegistryInterface {
    /**
     * @dev Event that is sent when a Schema is created.
     *
     * @param id        KECCAK256 hash of created schema id.
     * @param identity  Account address of schema issuer .
     * @param schema    AnonCreds schema object as bytes.
     */
    event SchemaCreated(bytes32 indexed id, address identity, bytes schema);

    /**
     * @dev Creates a new Schema.
     *
     * Once the Schema is created, this function emits a `SchemaCreated` event
     * with the new Schema ID.
     *
     * This function can revert with following errors:
     * - `SchemaAlreadyExist`: Raised if Schema with provided ID already exist.
     * - `UnauthorizedIssuer`: Raised when an issuer DID specified in Schema is not owned by sender
     *
     * @param identity  Account address of schema issuer.
     * @param id        KECCAK256 hash of schema id to be created.
     * @param schema    AnonCreds schema object as bytes.
     */
    function createSchema(address identity, bytes32 id, bytes calldata schema) external;

    /**
     * @dev Endorse a new Schema.
     *
     * Once the Schema is created, this function emits a `SchemaCreated` event
     * with the new Schema ID.
     *
     * This function can revert with following errors:
     * - `SchemaAlreadyExist`: Raised if Schema with provided ID already exist.
     * - `UnauthorizedIssuer`: Raised when an issuer DID specified in Schema is not owned by sender
     *
     * @param identity  Account address of schema issuer.
     * @param sigV      Part of EcDSA signature.
     * @param sigR      Part of EcDSA signature.
     * @param sigS      Part of EcDSA signature.
     * @param id        KECCAK256 hash of schema id to be created.
     * @param schema    AnonCreds schema object as bytes.
     */
    function createSchemaSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        bytes calldata schema
    ) external;

    /**
     * @dev Get the block number when a schema was created.
     *
     * @param id  KECCAK256 hash of schema id.
     */
    function created(bytes32 id) external returns (uint256 block);
}
