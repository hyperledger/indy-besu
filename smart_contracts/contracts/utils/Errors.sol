// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

// General

error Unauthorized(address sender);

/**
 * @notice Error that occurs when performed identity operation by not owned account.
 * @param sender Sender account address.
 * @param owner Owner account address.
 */
error NotIdentityOwner(address sender, address owner);

// Schema related errors

/**
 * @notice Error that occurs when trying to create an already existing schema.
 * @param id Schema ID.
 */
error SchemaAlreadyExist(bytes32 id);

/**
 * @notice Error that occurs when the specified schema is not found.
 * @param id Schema ID.
 */
error SchemaNotFound(bytes32 id);

// Credential Definition related errors

/**
 * @notice Error that occurs when trying to create an existing credential definition.
 * @param id Credential definition ID.
 */
error CredentialDefinitionAlreadyExist(bytes32 id);

/**
 * @notice Error that occurs when the specified credential definition is not found.
 * @param id Credential definition ID.
 */
error CredentialDefinitionNotFound(bytes32 id);

// Legacy identifiers mapping related errors

/**
 * @notice Error that occurs when trying to add a duplicating mapping for a legacy identifier.
 * @param identifier Legacy DID identifier.
 */
error DidMappingAlreadyExist(bytes16 identifier);

/**
 * @notice Error that occurs when trying to add a duplicating mapping for a legacy CL entity identifier.
 * @param identifier Legacy Schema/CredDef identifier.
 */
error ClMappingAlreadyExist(string identifier);

/**
 * @notice Error that occurs when provided ed25519Key key does not match to the provided legacy DID identifier.
 * @param ed25519Key verification key matching to the DID.
 * @param identifier KECCAK256 hash of legacy DID identifier.
 */
error InvalidEd25519Key(bytes32 ed25519Key, bytes16 identifier);

/**
 * @title Errors
 * @dev A library that provides utility functions for error handling.
 */
library Errors {
    /**
     * @dev Compares the selector of the provided error reason with a custom error selector.
     * @param reason The error reason returned by a failed contract call, encoded in bytes.
     * @param errorSelector The selector of the custom error to compare against.
     * @return bool Returns true if the selectors match, indicating the errors are the same; otherwise, returns false.
     */
    function equals(bytes memory reason, bytes4 errorSelector) internal pure returns (bool) {
        bytes4 reasonSelector = abi.decode(reason, (bytes4));
        return reasonSelector == errorSelector;
    }

    /**
     * @dev Rethrows an error using its encoded reason.
     * @param reason The error reason returned by a failed contract call, encoded in bytes.
     */
    function rethrow(bytes memory reason) internal pure {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            let start := add(reason, 0x20)
            let end := add(reason, mload(reason))
            revert(start, end)
        }
    }
}
