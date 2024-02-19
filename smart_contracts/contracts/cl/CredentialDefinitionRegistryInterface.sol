// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { CredentialDefinitionRecord } from "./CredentialDefinitionTypes.sol";

interface CredentialDefinitionRegistryInterface {
    /**
     * @dev Event that is sent when a Credential Definition is created
     *
     * @param credentialDefinitionId    Keccak hash of created credential definition id
     * @param identity                  Issuer address
     */
    event CredentialDefinitionCreated(bytes32 credentialDefinitionId, address identity);

    /**
     * @dev Creates a new Credential Definition.
     *
     * Once the Credential Definition is created, this function emits a `CredentialDefinitionCreated` event
     * with the new Credential Definition's ID and issuer address.
     *
     * Restrictions:
     * - Only senders with either TRUSTEE or ENDORSER or STEWARD role are permitted to create new object;
     *
     * This function can revert with following errors:
     * - `CredentialDefinitionAlreadyExist`: Raised if Credential Definition with provided ID already exist.
     * - `SchemaNotFound`: Raised if the associated schema doesn't exist.
     * - `IssuerNotFound`: Raised if the associated issuer doesn't exist.
     * - `InvalidIssuerId`: Raised if the provided issuer DID is invalid.
     * - `IssuerHasBeenDeactivated`: Raised if the associated issuer is not active.
     * - `NotIdentityOwner`: Raised when specified issuer DID is not owned by sender.
     *
     * @param identity  Account address of credential definition issuer.
     * @param id        Keccak hash of Credential Definition id to be created.
     * @param issuerId  DID of Credential Definition issuer.
     * @param schemaId  Keccak hash of Schema id.
     * @param credDef   AnonCreds Credential Definition JSON as bytes.
     */
    function createCredentialDefinition(
        address identity,
        bytes32 id,
        string calldata issuerId,
        bytes32 schemaId,
        bytes calldata credDef
    ) external;

    /**
     * @dev Endorse a new Credential Definition (off-chain author signature).
     *
     * Once the Credential Definition is created, this function emits a `CredentialDefinitionCreated` event
     * with the new Credential Definition's ID and issuer address.
     *
     * Restrictions:
     * - Only senders with either TRUSTEE or ENDORSER or STEWARD role are permitted to create new object;
     *
     * This function can revert with following errors:
     * - `CredentialDefinitionAlreadyExist`: Raised if Credential Definition with provided ID already exist.
     * - `SchemaNotFound`: Raised if the associated schema doesn't exist.
     * - `IssuerNotFound`: Raised if the associated issuer doesn't exist.
     * - `InvalidIssuerId`: Raised if the provided issuer DID is invalid.
     * - `IssuerHasBeenDeactivated`: Raised if the associated issuer is not active.
     * - `NotIdentityOwner`: Raised when specified issuer DID is not owned by signer
     *
     * @param identity  Account address of credential definition issuer.
     * @param sigR      Part of EcDSA signature.
     * @param sigV      Part of EcDSA signature.
     * @param sigS      Part of EcDSA signature.
     * @param id        Keccak hash of Credential Definition id to be created.
     * @param issuerId  DID of Credential Definition issuer.
     * @param schemaId  Keccak hash of Schema id.
     * @param credDef   AnonCreds Credential Definition JSON as bytes.
     */
    function createCredentialDefinitionSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        string calldata issuerId,
        bytes32 schemaId,
        bytes calldata credDef
    ) external;

    /**
     * @dev Resolve the Credential Definition associated with the given ID.
     *
     * If no matching Credential Definition is found, the function revert with `CredentialDefinitionNotFound` error
     *
     * @param id Keccak hash of the Credential Definition to be resolved.
     *
     * @return credentialDefinitionRecord Returns the credential definition with metadata.
     */
    function resolveCredentialDefinition(
        bytes32 id
    ) external returns (CredentialDefinitionRecord memory credentialDefinitionRecord);
}
