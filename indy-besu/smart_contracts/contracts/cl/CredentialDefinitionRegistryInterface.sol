// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { CredentialDefinitionRecord } from "./CredentialDefinitionTypes.sol";

interface CredentialDefinitionRegistryInterface {
    /**
     * @dev Event that is sent when a Credential Definition is created
     *
     * @param credentialDefinitionId ID of created credential definition
     */
    event CredentialDefinitionCreated(string credentialDefinitionId);

    /**
     * @dev Creates a new Credential Definition.
     *
     * Once the Credential Definition is created, this function emits a `CredentialDefinitionCreated` event
     * with the new Credential Definition's ID.
     *
     * This function can revert with following errors:
     * - `CredentialDefinitionAlreadyExist`: Raised if Credential Definition with provided ID already exist.
     * - `SchemaNotFound`: Raised if the associated schema doesn't exist.
     * - `IssuerNotFound`: Raised if the associated issuer doesn't exist.
     * - `IssuerHasBeenDeactivated`: Raised if the associated issuer is not active.
     * - `InvalidCredentialDefinitionId`: Raised if the Credential Definition ID syntax is invalid.
     * - `UnauthorizedSender`: Raised when an issuer DID specified in CredentialDefinition is not owned by sender
     *
     * @param id        Id of credential definition to be created.
     * @param issuerId  Id of credential definition issuer.
     * @param schemaId  Id of credential definition schema.
     * @param credDef   AnonCreds credential definition as JSON string.
     */
    function createCredentialDefinition(
        string calldata id,
        string calldata issuerId,
        string calldata schemaId,
        string calldata credDef
    ) external;

    /**
     * @dev Resolve the Credential Definition associated with the given ID.
     *
     * If no matching Credential Definition is found, the function revert with `CredentialDefinitionNotFound` error
     *
     * @param id The ID of the Credential Definition to be resolved.
     *
     * @return credentialDefinitionRecord Returns the credential definition with metadata.
     */
    function resolveCredentialDefinition(
        string calldata id
    ) external returns (CredentialDefinitionRecord memory credentialDefinitionRecord);
}
