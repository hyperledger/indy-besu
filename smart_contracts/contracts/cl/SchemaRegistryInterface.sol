// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { SchemaRecord } from "./SchemaTypes.sol";

interface SchemaRegistryInterface {
    /**
     * @dev Event that is sent when a Schema is created
     *
     * @param schemaId      Keccak hash of created schema id
     * @param identity      Issuer address
     */
    event SchemaCreated(bytes32 schemaId, address identity);

    /**
     * @dev Creates a new Schema.
     *
     * Once the Schema is created, this function emits a `SchemaCreated` event
     * with the new Schema ID and issuer address.
     *
     * This function can revert with following errors:
     * - `SchemaAlreadyExist`: Raised if Schema with provided ID already exist.
     * - `IssuerNotFound`: Raised if the associated issuer doesn't exist.
     * - `InvalidIssuerId`: Raised if the provided issuer DID is invalid.
     * - `IssuerHasBeenDeactivated`: Raised if the associated issuer is not active.
     * - `NotIdentityOwner`: Raised when an issuer DID specified in Schema is not owned by sender
     *
     * @param identity  Account address of schema issuer.
     * @param id        Keccak hash of Schema id to be created.
     * @param issuerId  DID of Schema issuer.
     * @param schema    AnonCreds schema JSON as bytes.
     */
    function createSchema(address identity, bytes32 id, string calldata issuerId, bytes calldata schema) external;

    /**
     * @dev Endorse a new Schema (off-chain author signature)..
     *
     * Once the Schema is created, this function emits a `SchemaCreated` event
     * with the new Schema ID and issuer address.
     *
     * Restrictions:
     * - Only senders with either TRUSTEE or ENDORSER or STEWARD role are permitted to create new object;
     *
     * This function can revert with following errors:
     * - `SchemaAlreadyExist`: Raised if Schema with provided ID already exist.
     * - `IssuerNotFound`: Raised if the associated issuer doesn't exist.
     * - `InvalidIssuerId`: Raised if the provided issuer DID is invalid.
     * - `IssuerHasBeenDeactivated`: Raised if the associated issuer is not active.
     * - `NotIdentityOwner`: Raised when an issuer DID specified in Schema is not owned by signer
     *
     * @param identity  Account address of schema issuer.
     * @param sigV      Part of EcDSA signature.
     * @param sigR      Part of EcDSA signature.
     * @param sigS      Part of EcDSA signature.
     * @param id        Keccak hash of Schema id to be created.
     * @param issuerId  DID of Schema issuer.
     * @param schema    AnonCreds schema JSON as bytes.
     */
    function createSchemaSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        string calldata issuerId,
        bytes calldata schema
    ) external;

    /**
     * @dev Resolve the Schema associated with the given ID.
     *
     * If no matching Schema is found, the function revert with `SchemaNotFound` error
     *
     * @param id    Keccak hash of Schema id to be resolved.
     *
     * @return schemaRecord Returns the Schema with Metadata.
     */
    function resolveSchema(bytes32 id) external returns (SchemaRecord memory schemaRecord);
}
