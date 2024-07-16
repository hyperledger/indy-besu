// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { RevocationRecord } from "./RevocationRegistryTypes.sol";

interface RevocationRegistryInterface {
    /**
     * @dev Event emitted when a revocation is created
     * @param sender Address of the sender
     * @param revocationId ID of the created revocation
     */
    event RevocationCreated(address sender, bytes32 revocationId);

    /**
     * @dev Event emitted when a credential is revoked
     * @param sender Address of the sender
     * @param revocationId ID of the revoked credential
     */
    event CredentialRevoked(address sender, bytes32 revocationId);

    /**
     * @dev Event emitted when a credential is suspended
     * @param sender Address of the sender
     * @param revocationId ID of the suspended credential
     */
    event CredentialSuspended(address sender, bytes32 revocationId);

    /**
     * @dev Event emitted when a credential is unrevoked
     * @param sender Address of the sender
     * @param revocationId ID of the unrevoked credential
     */
    event CredentialUnrevoked(address sender, bytes32 revocationId);

    /**
     * @notice Function to revoke a credential
     * @param identity The address of the identity
     * @param id The id of revogation
     *      */
    function revokeCredential(address identity, bytes32 id) external;

    /**
     * @notice Function to revoke a credential with a signed message
     * @param identity The address of the identity
     * @param sigV The V part of the signature
     * @param sigR The R part of the signature
     * @param sigS The S part of the signature
     * @param id The id of revogation
     */
    function revokeCredentialSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes32 id) external;

    /**
     * @notice Function to suspend a credential
     * @param identity The address of the identity
     * @param id The id of revogation
     * d
     */
    function suspendCredential(address identity, bytes32 id) external;

    /**
     * @notice Function to suspend a credential with a signed message
     * @param identity The address of the identity
     * @param sigV The V part of the signature
     * @param sigR The R part of the signature
     * @param sigS The S part of the signature
     * @param id The id of revogation
     * d
     */
    function suspendCredentialSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes32 id) external;

    /**
     * @notice Function to unrevoke a credential
     * @param identity The address of the identity
     * @param id The id of revogation
     * d
     */
    function unrevokeCredential(address identity, bytes32 id) external;

    /**
     * @notice Function to unrevoke a credential with a signed message
     * @param identity The address of the identity
     * @param sigV The V part of the signature
     * @param sigR The R part of the signature
     * @param sigS The S part of the signature
     * @param id The id of revogation
     * d
     */
    function unrevokeCredentialSigned(address identity, uint8 sigV, bytes32 sigR, bytes32 sigS, bytes32 id) external;

    /**
     * @notice Function to create a revocation registry
     * @param identity The address of the identity
     * @param CredDefId The ID of the credential registry
     * @param RevicationId The ID of the revocation registry to be created
     * @param revokeDocument The document of the revocation
     */
    function createRevocationRegistry(
        address identity,
        bytes32 CredDefId,
        bytes32 RevicationId,
        bytes calldata revokeDocument
    ) external;

    /**
     * @notice Function to create a revocation registry with a signed message
     * @param identity The address of the identity
     * @param sigV The V part of the signature
     * @param sigR The R part of the signature
     * @param sigS The S part of the signature
     * @param CredDefId The ID of the credential registry
     * @param RevicationId The ID of the revocation registry to be created
     * @param revokeDocument The document of the revocation
     */
    function createRevocationRegistrySigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 CredDefId,
        bytes32 RevicationId,
        bytes calldata revokeDocument
    ) external;

    /**
     * @notice Function to resolve a revocation
     * @param id The ID of the revocation to be resolved
     * @return The RevocationRecord associated with the given ID
     */
    function resolveRevocation(bytes32 id) external view returns (RevocationRecord memory);

    
}
