// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

interface CredentialDefinitionRegistryInterface {
    /**
     * @dev Event that is sent when a Credential Definition is created.
     *
     * @param id        KECCAK256 hash of credential definition id.
     * @param identity  Account address of credential definition issuer.
     * @param credDef   AnonCreds credential definition object as bytes.
     */
    event CredentialDefinitionCreated(bytes32 indexed id, address identity, bytes credDef);

    /**
     * @dev Creates a new Credential Definition.
     *
     * Once the Credential Definition is created, this function emits a `CredentialDefinitionCreated` event
     * with the new Credential Definition's ID.
     *
     * This function can revert with following errors:
     * - `CredentialDefinitionAlreadyExist`: Raised if Credential Definition with provided ID already exist.
     * - `SchemaNotFound`: Raised if the associated schema doesn't exist.
     * - `UnauthorizedIssuer`: Raised when an issuer DID specified in CredentialDefinition is not owned by sender
     *
     * @param identity  Account address of credential definition issuer.
     * @param id        KECCAK256 hash of credential definition id to be created.
     * @param schemaId  KECCAK256 hash of schema id.
     * @param credDef   AnonCreds credential definition object as bytes.
     */
    function createCredentialDefinition(
        address identity,
        bytes32 id,
        bytes32 schemaId,
        bytes calldata credDef
    ) external;

    /**
     * @dev Endorse a new Credential Definition.
     *
     * Once the Credential Definition is created, this function emits a `CredentialDefinitionCreated` event
     * with the new Credential Definition's ID.
     *
     * This function can revert with following errors:
     * - `CredentialDefinitionAlreadyExist`: Raised if Credential Definition with provided ID already exist.
     * - `SchemaNotFound`: Raised if the associated schema doesn't exist.
     * - `UnauthorizedIssuer`: Raised when an issuer DID specified in CredentialDefinition is not owned by sender
     *
     * @param identity  Account address of credential definition issuer.
     * @param sigR      Part of EcDSA signature.
     * @param sigV      Part of EcDSA signature.
     * @param sigS      Part of EcDSA signature.
     * @param id        KECCAK256 hash of credential definition id to be created.
     * @param schemaId  KECCAK256 hash of schema id.
     * @param credDef   AnonCreds credential definition object as bytes.
     */
    function createCredentialDefinitionSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        bytes32 schemaId,
        bytes calldata credDef
    ) external;

    /**
     * @dev Get the block number when a schema was created.
     *
     * @param id  KECCAK256 hash of credential definition id.
     */
    function created(bytes32 id) external returns (uint256 block);
}
