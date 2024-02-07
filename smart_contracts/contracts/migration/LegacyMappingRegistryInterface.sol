// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

interface LegacyMappingRegistryInterface {
    /**
     * @dev Event that is sent when a DID mapping is created.
     *
     * @param identifier    legacy DID identifier.
     * @param identity      Corresponding account address of DID owner.
     */
    event DidMappingCreated(string identifier, address identity);

    /**
     * @dev Event that is sent when a new Resource (SchemaId/CredentialDefinitionId) mapping is created.
     *
     * @param legacyIdentifier   Legacy ID of resource.
     * @param newIdentifier      New ID of resource.
     */
    event ResourceMappingCreated(string legacyIdentifier, string newIdentifier);

    /**
     * @dev Creates a new mapping of legacy indy/sov DID identifier to account address.
     *
     * Once the mapping is created, this function emits a `DidMappingCreated` event
     * with the legacy identifier and identity address.
     *
     * This function can revert with following errors:
     * - `MappingAlreadyExist`: Raised if DID mapping with provided identifier already exist.
     * - `InvalidEd25519Key`: Raised if provided ED25519 verification key does not match to the DID identifier.
     * - `NotIdentityOwner`: Raised if sender account is not owner of the provided identity
     *
     * @param identity          Account address of the DID's owner.
     * @param identifier        legacy DID identifier.
     * @param ed25519Key        Ed25519 verification key of the legacy DID identifier.
     * @param ed25519Signature  ED25519 signature to prove key possession.
     */
    function createDidMapping(
        address identity,
        string calldata identifier,
        bytes32 ed25519Key,
        bytes calldata ed25519Signature
    ) external;

    /**
     * @dev Endorse a new mapping of legacy indy/sov DID identifier to account address.
     *
     * Once the mapping is created, this function emits a `DidMappingCreated` event
     * with the legacy identifier and identity address.
     *
     * This function can revert with following errors:
     * - `MappingAlreadyExist`: Raised if DID mapping with provided identifier already exist.
     * - `InvalidEd25519Key`: Raised if provided ED25519 verification key does not match to the DID identifier.
     * - `NotIdentityOwner`: Raised if signer account is not owner of the provided identity
     *
     * @param identity          Account address of the DID's owner.
     * @param sigV              Part of EcDSA signature.
     * @param sigR              Part of EcDSA signature.
     * @param sigS              Part of EcDSA signature.
     * @param identifier        legacy DID identifier.
     * @param ed25519Key        Ed25519 verification key of the legacy DID identifier.
     * @param ed25519Signature  ED25519 signature to prove key possession.
     */
    function createDidMappingSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        string calldata identifier,
        bytes32 ed25519Key,
        bytes calldata ed25519Signature
    ) external;

    /**
     * @dev Creates a new mapping of legacy schema/credential definition identifier to new one.
     *
     * Once the mapping is created, this function emits a `ResourceMappingCreated` event
     * with the legacy identifier and new one.
     *
     * This function can revert with following errors:
     * - `MappingAlreadyExist`: Raised if resource mapping with provided identifier already exist.
     * - `NotIdentityOwner`: Raised if identity account is not owner of the legacy Issuer DID
     * - `NotIdentityOwner`: Raised if sender account is not owner of provided identity
     *
     * @param identity                  Account address of the issuer.
     * @param legacyIssuerIdentifier    Legacy issuer identifier.
     * @param legacyIdentifier          Legacy identifier.
     * @param newIdentifier             New identifier.
     */
    function createResourceMapping(
        address identity,
        string calldata legacyIssuerIdentifier,
        string calldata legacyIdentifier,
        string calldata newIdentifier
    ) external;

    /**
     * @dev Endorse a new mapping of legacy schema/credential definition identifier to new one.
     *
     * Once the mapping is created, this function emits a `ResourceMappingCreated` event
     * with the legacy identifier and new one.
     *
     * This function can revert with following errors:
     * - `MappingAlreadyExist`: Raised if resource mapping with provided identifier already exist.
     * - `NotIdentityOwner`: Raised if identity account is not owner of the legacy Issuer DID
     * - `NotIdentityOwner`: Raised if signer account is not owner of the provided identity
     *
     * @param identity                  Account address of the issuer.
     * @param sigV                      Part of EcDSA signature.
     * @param sigR                      Part of EcDSA signature.
     * @param sigS                      Part of EcDSA signature.
     * @param legacyIssuerIdentifier    Legacy issuer identifier.
     * @param legacyIdentifier          Legacy identifier.
     * @param newIdentifier             New identifier.
     */
    function createResourceMappingSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        string calldata legacyIssuerIdentifier,
        string calldata legacyIdentifier,
        string calldata newIdentifier
    ) external;
}
