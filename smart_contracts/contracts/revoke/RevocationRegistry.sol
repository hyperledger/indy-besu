// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { UniversalDidResolverInterface } from "../did/UniversalDidResolverInterface.sol";
import { IndyDidRegistryInterface } from "../did/IndyDidRegistryInterface.sol";
import { ControlledUpgradeable } from "../upgrade/ControlledUpgradeable.sol";
import { RevocationRegistryInterface } from "./RevocationRegistryInterface.sol";
import { CredentialDefinitionRecord } from "../anoncreds/CredentialDefinitionTypes.sol";
import { CredentialDefinitionRegistryInterface } from "../anoncreds/CredentialDefinitionRegistryInterface.sol";
import { RevocationRecord,Status  } from "./RevocationRegistryTypes.sol";
import { AnoncredsRegistry } from "../anoncreds/AnoncredsRegistry.sol";
import { NotIdentityOwner,DidNotFound } from "../did/DidErrors.sol";
import { RevocationNotFound, RevocationAlreadyExist, CredentialDefinitionNotFound,RevocationIsNotActived,RevocationIsNotsuspended,RevocationIsNotRevoked,CredentialIsAlreadyRevoked,InvalidIssuer } from "../anoncreds/AnoncredsErrors.sol";
import { RoleControlInterface } from "../auth/RoleControl.sol";



contract RevocationRegistry is RevocationRegistryInterface, ControlledUpgradeable,AnoncredsRegistry {
    /**
     * @dev Reference to the contract that manages anoncreds credDefs
     */
    CredentialDefinitionRegistryInterface private _credDefRegistry;
   
   
    /**
     * Mapping Revocation ID to its Revocation Details and Metadata.
     */
    mapping(bytes32 id => RevocationRecord revocationRecord) private _revReg;
    

    /**
     * Check that the revocation does not exist
     */
    modifier _revocationNotExist(bytes32 id) {
        if (_revReg[id].metadata.created != 0) revert RevocationAlreadyExist(id);
        _;
    }
   
    /**
     * 
     * Check that the status is not revoked
     */
    modifier _CredentialNotRevoked(bytes32 id) {
        if (_revReg[id].metadata.status == Status.revoked) revert CredentialIsAlreadyRevoked(id);
        _;
    }
   
    /**
     * Check that the status is not actived
     */
    modifier _CredentialNotActived(bytes32 id) {
        if (_revReg[id].metadata.status != Status.active) revert RevocationIsNotActived(id);
        _;
    }
    
    /**
     * Check that the status is actived
     */
    modifier _CredentialIsActived(bytes32 id) {
        if (_revReg[id].metadata.status == Status.active ) revert RevocationIsNotRevoked(id);
        _;
    }

   
    /**
     * Сhecks  the Issuer of revocation 
     */

    modifier _checkIssuer(bytes32 id) { 
    if (_revReg[id].metadata.creator != msg.sender) revert InvalidIssuer(id);
        _;
    }

 
    /**
    * Checks that the credDef exists
    */
    modifier _credDefExist(bytes32 id) { 
           _credDefRegistry.resolveCredentialDefinition(id);
        _;
    }
    
 
    function initialize(
        address upgradeControlAddress,
        address credDefRegistryAddress,
        address roleControlContractAddress
    ) public reinitializer(1) {
        _initializeUpgradeControl(upgradeControlAddress);
        _credDefRegistry = CredentialDefinitionRegistryInterface(credDefRegistryAddress);
        _roleControl = RoleControlInterface(roleControlContractAddress);
    }

    

     /**
     * Revoke functions:
     */

    function revokeCredential(
        address identity,
        bytes32 id
    ) public  
        _CredentialNotRevoked(id)
        _senderIsTrusteeOrEndorserOrSteward 
    {
        _revokeCredential(identity,msg.sender, id);

    }


    /// @inheritdoc RevocationRegistryInterface
    function revokeCredentialSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id
    ) public virtual 
        _CredentialNotRevoked(id)
        _credDefExist(id)
        _senderIsTrusteeOrEndorserOrSteward 
    {

         bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0x19),
                bytes1(0),
                address(this),
                identity,
                "revokeCredential",
                id
               
            )
        );
        
        _revokeCredential(identity,ecrecover(hash, sigV, sigR, sigS), id);

    }

    /**
     * Suspend functions:
     */
    function suspendCredential(
        address identity,
        bytes32 id
    ) public
        _CredentialNotActived(id)
        _credDefExist(id)
        _senderIsTrusteeOrEndorserOrSteward 

    {
        _suspendCredential(identity,msg.sender, id);

    }
    /// @inheritdoc RevocationRegistryInterface
    function suspendCredentialSigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
         bytes32 id
    ) public virtual 
        _CredentialNotActived(id)
        _credDefExist(id)
        _senderIsTrusteeOrEndorserOrSteward 
    {

         bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0x19),
                bytes1(0),
                address(this),
                identity,
                "suspendCredential",
                id
               
            )
        );
       
        _suspendCredential (identity,ecrecover(hash, sigV, sigR, sigS), id);

    }

    /**
     * Unrevok functions:
     */
    function unrevokeCredential(
        address identity,
        bytes32 id
    ) public  
        _CredentialIsActived(id)
        _senderIsTrusteeOrEndorserOrSteward 
    {
        _UnrevokedCredential(identity,msg.sender, id);

    }
    /// @inheritdoc RevocationRegistryInterface
    function unrevokeCredentialSigned(
        address identity,
       uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id
    ) public virtual 
       _CredentialIsActived(id)
       _credDefExist(id)
       _senderIsTrusteeOrEndorserOrSteward
    {

         bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0x19),
                bytes1(0),
                address(this),
                identity,
                "unrevokeCredential",
                id
               
            )
        );
   
    
        _UnrevokedCredential(identity,ecrecover(hash, sigV, sigR, sigS), id);

    }


     /**
     * create Revocation functions:
     */
    function createRevocationRegistry(
        address identity,
        bytes32 id,
        bytes calldata revokeDocument
    )
        public 
        _revocationNotExist(id)
        _senderIsTrusteeOrEndorserOrSteward 
        _credDefExist(id)
 
    {
        _createRevocation(identity, msg.sender, id, revokeDocument);
    }

        /// @inheritdoc RevocationRegistryInterface
      function createRevocationRegistrySigned(
        address identity,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS,
        bytes32 id,
        bytes calldata revokeDocument
    )
        public virtual 
        _revocationNotExist(id)
        _senderIsTrusteeOrEndorserOrSteward
        _credDefExist(id)

        {

         bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0x19),
                bytes1(0),
                address(this),
                identity,
                "createRevocationRegistry",
                id
               
            )
        );
     
    
        _createRevocation(identity, ecrecover(hash, sigV, sigR, sigS), id, revokeDocument);
    }

     /**
     * Create Revocation functions:
     */

    function _createRevocation(
        address identity,
        address actor,
        bytes32 id,
        bytes calldata document
    ) private 
        
        _identityOwner( identity, actor) // the sender must be equal to the identity
    {
       
        _revReg[id].document = document;
        _revReg[id].metadata.created = block.timestamp;
        _revReg[id].metadata.creator = msg.sender;
        _revReg[id].metadata.updated = block.timestamp;
        _revReg[id].metadata.status = Status.active;

        
        emit RevocationCreated(msg.sender, id);
    }

   
     /**
     * Revok Credential functions:
     */

    function _revokeCredential(
        address identity,
        address actor,
        bytes32 id
    ) private 
        
      _identityOwner( identity, actor) // the sender must be equal to the identity
      _checkIssuer(id)

    {
        _revReg[id].metadata.status = Status.revoked;
        _revReg[id].metadata.updated = block.timestamp;

        ///credential revocation event
        emit CredentialRevoked(msg.sender, id);
    }

    

     /**
     * suspend Credential functions:
     */

    function _suspendCredential(
        address identity,
         address actor,
        bytes32 id
    ) private 
        
      _identityOwner( identity, actor) // the sender must be equal to the identity
      _checkIssuer(id)

    {
        _revReg[id].metadata.status = Status.suspended;
        _revReg[id].metadata.updated = block.timestamp;

       ///suspended credential event
        emit CredentialSuspended(msg.sender, id);
    }

   

     /**
     * Unrevoke Credential functions:
     */

    function _UnrevokedCredential(
        address identity,
        address actor,
        bytes32 id
    ) private 
        
      _identityOwner( identity, actor) // the sender must be equal to the identity
      _checkIssuer(id)

    {
        _revReg[id].metadata.status = Status.active;
        _revReg[id].metadata.updated = block.timestamp;

       ///credential Unrevoked event
        emit CredentialUnrevoked(msg.sender, id);
    }


    
     /**
     * Resolve Revocation functions:
     */
     
    /// @inheritdoc RevocationRegistryInterface
    function resolveRevocation(
        bytes32 id
    ) public view returns (RevocationRecord memory revocationRecord) {
        return _revReg[id];
    }

      string private name;

    function changeName(string memory _name) public returns (string memory) {
        name = _name;
        return name;
    }

    function seeName() public view returns (string memory) {
        return name;
    }

    
}

