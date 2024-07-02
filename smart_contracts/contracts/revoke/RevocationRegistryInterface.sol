// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { RevocationRecord } from "./RevocationRegistryTypes.sol";

interface RevocationRegistryInterface {
    /**
     * @dev Event that is sent when a Revocation is created
     *
     * @param sender Sender's address
     * @param revocationId Created Revocation ID
     */
    event RevocationCreated(address sender, bytes32 revocationId);
    event CredentialRevoked(address sender, bytes32 revocationId);
    event CredentialSuspended(address sender, bytes32 revocationId);
    event CredentialUnrevoked(address sender, bytes32 revocationId);

    /**
     * @notice Function to revoke a credential.
     * @param identity The address of the identity.
     * @param id The ID of the credential to be revoked.
     */
    function revokeCredential(address identity, bytes32 id) external;

    /**
     * @notice Function to revoke a credential with a signed message.
     * @param identity The address of the identity.
     * @param hash The hash of the message.
     * @param sigV The V part of the signature.
     * @param sigR The R part of the signature.
     * @param sigS The S part of the signature.
     * @param id The ID of the credential to be revoked.
     */
    function revokeCredentialSigned(
        address identity,
        bytes32 hash,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id
    ) external;

    /**
     * @notice Function to suspend a credential.
     * @param identity The address of the identity.
     * @param id The ID of the credential to be suspended.
     */
    function suspendCredential(address identity, bytes32 id) external;

    /**
     * @notice Function to suspend a credential with a signed message.
     * @param identity The address of the identity.
     * @param hash The hash of the message.
     * @param sigV The V part of the signature.
     * @param sigR The R part of the signature.
     * @param sigS The S part of the signature.
     * @param id The ID of the credential to be suspended.
     */
    function suspendCredentialSigned(
        address identity,
        bytes32 hash,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id
    ) external;

    /**
     * @notice Function to unrevoke a credential.
     * @param identity The address of the identity.
     * @param id The ID of the credential to be unrevoked.
     */
    function unrevokeCredential(address identity, bytes32 id) external;

    /**
     * @notice Function to unrevoke a credential with a signed message.
     * @param identity The address of the identity.
     * @param hash The hash of the message.
     * @param sigV The V part of the signature.
     * @param sigR The R part of the signature.
     * @param sigS The S part of the signature.
     * @param id The ID of the credential to be unrevoked.
     */
    function unrevokeCredentialSigned(
        address identity,
        bytes32 hash,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id
    ) external;

    /**
     * @notice Function to create a revocation registry.
     * @param identity The address of the identity.
     * @param id The ID of the revocation registry to be created.
     * @param revokeDocument The document of the revocation.
     */
    function createRevocationRegistry(address identity, bytes32 id, bytes calldata revokeDocument) external;

    /**
     * @notice Function to create a revocation registry with a signed message.
     * @param identity The address of the identity.
     * @param hash The hash of the message.
     * @param sigV The V part of the signature.
     * @param sigR The R part of the signature.
     * @param sigS The S part of the signature.
     * @param id The ID of the revocation registry to be created.
     * @param revokeDocument The document of the revocation.
     */
    function createRevocationRegistrySigned(
        address identity,
        bytes32 hash,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        bytes calldata revokeDocument
    ) external;

    /**
     * @notice Function to resolve a revocation.
     * @param id The ID of the revocation to be resolved.
     * @return The RevocationRecord associated with the given ID.
     */
    function resolveRevocation(bytes32 id) external view returns (RevocationRecord memory);

    /**
     * @notice Function to resolve a  revocation signed.
     * @param id The ID of the revocation to be resolved.
     * @return The RevocationRecord associated with the given ID.
     */
    function resolveRevocationSig(bytes32 id) external view returns (RevocationRecord memory);
}
