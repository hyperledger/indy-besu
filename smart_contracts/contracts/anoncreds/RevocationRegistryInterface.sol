// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { RevocationRegistryDefinitionRecord, RevocationRegistryEntry } from "./RevocationRegistryTypes.sol";

interface RevocationRegistryInterface {
    /**
     * @dev Event that is sent when a Revocation Registry Definition is created
     *
     * @param revocationRegistryDefinitionId    Keccak hash of created revocation registry definition id
     * @param identity                          Issuer address
     */
    event RevocationRegistryDefinitionCreated(bytes32 revocationRegistryDefinitionId, address identity);

    event RevocationRegistryEntryCreated(
        bytes32 indexed revocationRegistryDefinitionId,
        uint64 indexed timestamp,
        RevocationRegistryEntry revRegEntry
    );

    function createRevocationRegistryDefinition(
        address identity,
        bytes32 id,
        bytes32 credDefId,
        string calldata issuerId,
        bytes calldata revRegDef
    ) external;

    function createRevocationRegistryDefinitionSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        bytes32 credDefId,
        string calldata issuerId,
        bytes calldata revRegDef
    ) external;

    function resolveRevocationRegistryDefinition(
        bytes32 id
    ) external returns (RevocationRegistryDefinitionRecord memory revocationRegistryDefinitionRecord);

    function createRevocationRegistryEntry(
        address identity,
        bytes32 revRegDefId,
        string calldata issuerId,
        RevocationRegistryEntry calldata revRegEntry
    ) external;

    //TODO:
    // /**
    //  * @dev Resolve the Revocation Registry Definition associated with the given ID.
    //  *
    //  * If no matching Revocation Registry Definition is found, the function revert with `RevocationRegistryDefinitionNotFound` error
    //  *
    //  * @param id Keccak hash of the Revocation Registry Definition to be resolved.
    //  *
    //  * @return revocationRegistryDefinitionRecord Returns the credential definition with metadata.
    //  */
    // function resolveRevocationRegistryDefinition(
    //     bytes32 id
    // ) external returns (RevocationRegistryDefinitionRecord memory revocationRegistryDefinitionRecord);
}
