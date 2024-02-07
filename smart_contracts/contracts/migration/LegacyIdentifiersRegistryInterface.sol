// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

interface LegacyIdentifiersRegistryInterface {
    /**
     * @dev Event that is sent when a DID mapping is created.
     *
     * @param identifier    KECCAK256 hash of legacy DID identifier.
     * @param identity      Corresponding account address of DID owner.
     */
    event DidMappingCreated(bytes16 identifier, address identity);
    event ClMappingCreated(string legacyIdentifier, string newIdentifier);

    /**
     * @dev Creates a new mapping of legacy indy/sov DID identifier to account address.
     *
     * Once the mapping is created, this function emits a `DidMappingCreated` event
     * with the legacy identifier and identity address.
     *
     * This function can revert with following errors:
     * - `MappingAlreadyExist`: Raised if DID mapping with provided identifier already exist.
     * - `InvalidEd25519Key`: Raised if provided ED25519 verification key does not match to the DID identifier.
     *
     * @param identity          Account address of the DID's owner.
     * @param identifier        legacy DID identifier.
     * @param ed25519Key        Ed25519 verification key of the legacy DID identifier.
     * @param ed25519Signature  ED25519 signature to prove key possession.
     */
    function createDidMapping(
        address identity,
        bytes16 identifier,
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
     * - `NotIdentityOwner`: Raised when an issuer DID specified in the mapping is not owned by sender
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
        bytes16 identifier,
        bytes32 ed25519Key,
        bytes calldata ed25519Signature
    ) external;

    /**
     * @dev Creates a new mapping of legacy schema/credential definition identifier to new one.
     *
     * Once the mapping is created, this function emits a `ClMappingCreated` event
     * with the legacy identifier and new one.
     *
     * This function can revert with following errors:
     * - `MappingAlreadyExist`: Raised if DID mapping with provided identifier already exist.
     *
     * @param identity                  Account address of the issuer.
     * @param legacyIssuerIdentifier    legacy issuer identifier.
     * @param legacyIdentifier          legacy identifier.
     * @param newIdentifier             new identifier.
     */
    function createClMapping(
        address identity,
        bytes16 legacyIssuerIdentifier,
        string calldata legacyIdentifier,
        string calldata newIdentifier
    ) external;

    /**
     * @dev Endorse a new mapping of legacy schema/credential definition identifier to new one.
     *
     * Once the mapping is created, this function emits a `ClMappingCreated` event
     * with the legacy identifier and new one.
     *
     * This function can revert with following errors:
     * - `MappingAlreadyExist`: Raised if DID mapping with provided identifier already exist.
     * - `NotIdentityOwner`: Raised when an issuer DID specified in the mapping is not owned by sender
     *
     * @param identity                  Account address of the issuer DID's owner.
     * @param sigV                      Part of EcDSA signature.
     * @param sigR                      Part of EcDSA signature.
     * @param sigS                      Part of EcDSA signature.
     * @param legacyIssuerIdentifier    legacy issuer identifier.
     * @param legacyIdentifier          legacy identifier.
     * @param newIdentifier             new identifier.
     */
    function createClMappingSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes16 legacyIssuerIdentifier,
        string calldata legacyIdentifier,
        string calldata newIdentifier
    ) external;
}
