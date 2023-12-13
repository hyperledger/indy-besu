// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { StringUtils } from "../utils/StringUtils.sol";
import { FieldRequired, ConflictingFields } from "../utils/Errors.sol";
import { DidUtils, ParsedDid } from "../utils/DidUtils.sol";
import { AuthenticationKeyNotFound, AuthenticationKeyRequired, IncorrectDid } from "./DidErrors.sol";
import { IncorrectDid } from "./DidErrors.sol";
import { DidDocument, VerificationMethod, VerificationRelationship } from "./DidTypes.sol";

using StringUtils for string;

library IndyDidValidator {
    /**
     * @dev Validates the format of a DID document
     * @param didDocument The DID document to be validated
     */
    function validateDidDocumentFormat(DidDocument memory didDocument) public view {
        _validateDidSyntax(didDocument.id);
        _validateVerificationMethodsFormat(didDocument.verificationMethod);
        _validateVerificationRelationshipsFormat(didDocument.authentication);
        _validateVerificationRelationshipsFormat(didDocument.assertionMethod);
        _validateVerificationRelationshipsFormat(didDocument.capabilityInvocation);
        _validateVerificationRelationshipsFormat(didDocument.capabilityDelegation);
        _validateVerificationRelationshipsFormat(didDocument.keyAgreement);
    }

    /**
     * @dev Validates the authentication keys
     * @param didDocument The DID document to be validated
     */
    function validateAuthenticationKeys(DidDocument memory didDocument) public pure {
        if (didDocument.authentication.length == 0) revert AuthenticationKeyRequired(didDocument.id);

        for (uint256 i = 0; i < didDocument.authentication.length; i++) {
            if (!didDocument.authentication[i].verificationMethod.id.isEmpty()) {
                continue;
            }

            if (!_contains(didDocument.verificationMethod, didDocument.authentication[i].id)) {
                revert AuthenticationKeyNotFound(didDocument.authentication[i].id);
            }
        }
    }

    function _validateDidSyntax(string memory did) private view {
        ParsedDid memory parsedDid = DidUtils.parseDid(did);

        if (!DidUtils.isIndyMethod(parsedDid.method)) revert IncorrectDid(did);

        uint256 identifierLength = parsedDid.identifier.length();
        if (identifierLength != 21 && identifierLength != 22) revert IncorrectDid(did);
    }

    function _validateVerificationRelationshipsFormat(VerificationRelationship[] memory relationships) private pure {
        for (uint256 i = 0; i < relationships.length; i++) {
            _validateVerificationRelationshipFormat(relationships[i]);
        }
    }

    function _validateVerificationRelationshipFormat(VerificationRelationship memory relationship) private pure {
        bool isIdEmpty = relationship.id.isEmpty();
        bool isVerificationMethodEmpty = _isEmpty(relationship.verificationMethod);

        if (isIdEmpty && isVerificationMethodEmpty) {
            revert FieldRequired("VerificationRelationship.id or VerificationRelationship.method");
        }

        if (!isIdEmpty && !isVerificationMethodEmpty) {
            revert ConflictingFields("VerificationRelationship.id, VerificationRelationship.method");
        }

        if (isIdEmpty && !isVerificationMethodEmpty) {
            _validateVerificationMethodFormat(relationship.verificationMethod);
        }
    }

    function _validateVerificationMethodsFormat(VerificationMethod[] memory verificationMethods) private pure {
        for (uint256 i = 0; i < verificationMethods.length; i++) {
            _validateVerificationMethodFormat(verificationMethods[i]);
        }
    }

    function _validateVerificationMethodFormat(VerificationMethod memory verificationMethod) private pure {
        if (verificationMethod.id.isEmpty()) {
            revert FieldRequired("VerificationMethod.id");
        }

        if (verificationMethod.verificationMethodType.isEmpty()) {
            revert FieldRequired("VerificationMethod.verificationMethodType");
        }

        if (verificationMethod.controller.isEmpty()) {
            revert FieldRequired("VerificationMethod.controller");
        }

        bool isPublicKeyJwkEmpty = verificationMethod.publicKeyJwk.isEmpty();
        bool isPublicKeyMultibaseEmpty = verificationMethod.publicKeyMultibase.isEmpty();

        if (isPublicKeyJwkEmpty && isPublicKeyMultibaseEmpty) {
            revert FieldRequired("VerificationMethod.publicKeyJwk or VerificationMethod.publicKeyMultibase");
        }

        if (!isPublicKeyJwkEmpty && !isPublicKeyMultibaseEmpty) {
            revert ConflictingFields("VerificationMethod.publicKeyJwk, VerificationMethod.publicKeyMultibase");
        }
    }

    function _contains(VerificationMethod[] memory methods, string memory methodId) private pure returns (bool) {
        for (uint256 i; i < methods.length; i++) {
            if (methods[i].id.equals(methodId)) {
                return true;
            }
        }

        return false;
    }

    function _isEmpty(VerificationMethod memory method) private pure returns (bool) {
        return
            method.id.isEmpty() &&
            method.verificationMethodType.isEmpty() &&
            method.controller.isEmpty() &&
            method.publicKeyJwk.isEmpty() &&
            method.publicKeyMultibase.isEmpty();
    }
}
