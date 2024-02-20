// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { UniversalDidResolverInterface } from "../did/UniversalDidResolverInterface.sol";
import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";

import { CredentialDefinitionRecord } from "./CredentialDefinitionTypes.sol";
import { CredentialDefinitionRegistryInterface } from "./CredentialDefinitionRegistryInterface.sol";
import { CredentialDefinitionAlreadyExist, CredentialDefinitionNotFound } from "./AnoncredsErrors.sol";
import { SchemaRegistryInterface } from "./SchemaRegistryInterface.sol";
import { RoleControlInterface } from "../auth/RoleControl.sol";
import { AnoncredsRegistry } from "./AnoncredsRegistry.sol";

contract CredentialDefinitionRegistry is
    CredentialDefinitionRegistryInterface,
    ControlledUpgradeable,
    AnoncredsRegistry
{
    /**
     * @dev Reference to the contract that manages AnonCreds schemas
     */
    SchemaRegistryInterface private _schemaRegistry;

    /**
     * Mapping Credential Definition ID to its Credential Definition Details and Metadata.
     */
    mapping(bytes32 id => CredentialDefinitionRecord credentialDefinitionRecord) private _credDefs;

    /**
     * Checks the uniqueness of the credential definition ID
     */
    modifier _uniqueCredDefId(bytes32 id) {
        if (_credDefs[id].metadata.created != 0) revert CredentialDefinitionAlreadyExist(id);
        _;
    }

    /**
     * Checks that the credential definition exist
     */
    modifier _credDefExist(bytes32 id) {
        if (_credDefs[id].metadata.created == 0) revert CredentialDefinitionNotFound(id);
        _;
    }

    /**
     * Checks that the schema exist
     */
    modifier _schemaExist(bytes32 id) {
        _schemaRegistry.resolveSchema(id);
        _;
    }

    function initialize(
        address upgradeControlAddress,
        address didResolverAddress,
        address schemaRegistryAddress,
        address roleControlContractAddress
    ) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _didResolver = UniversalDidResolverInterface(didResolverAddress);
        _schemaRegistry = SchemaRegistryInterface(schemaRegistryAddress);
        _roleControl = RoleControlInterface(roleControlContractAddress);
    }

    /// @inheritdoc CredentialDefinitionRegistryInterface
    function createCredentialDefinition(
        address identity,
        bytes32 id,
        string calldata issuerId,
        bytes32 schemaId,
        bytes calldata credDef
    ) public virtual {
        _createCredentialDefinition(identity, msg.sender, id, issuerId, schemaId, credDef);
    }

    /// @inheritdoc CredentialDefinitionRegistryInterface
    function createCredentialDefinitionSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        string calldata issuerId,
        bytes32 schemaId,
        bytes calldata credDef
    ) public virtual {
        bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0x19),
                bytes1(0),
                address(this),
                identity,
                "createCredentialDefinition",
                id,
                issuerId,
                schemaId,
                credDef
            )
        );
        _createCredentialDefinition(identity, ecrecover(hash, sigV, sigR, sigS), id, issuerId, schemaId, credDef);
    }

    /// @inheritdoc CredentialDefinitionRegistryInterface
    function resolveCredentialDefinition(
        bytes32 id
    ) public view virtual _credDefExist(id) returns (CredentialDefinitionRecord memory credentialDefinitionRecord) {
        return _credDefs[id];
    }

    function _createCredentialDefinition(
        address identity,
        address actor,
        bytes32 id,
        string calldata issuerId,
        bytes32 schemaId,
        bytes calldata credDef
    )
        internal
        _senderIsTrusteeOrEndorserOrSteward
        _uniqueCredDefId(id)
        _validIssuer(issuerId, identity, actor)
        _schemaExist(schemaId)
    {
        _credDefs[id].credDef = credDef;
        _credDefs[id].metadata.created = block.timestamp;

        emit CredentialDefinitionCreated(id, identity);
    }
}
