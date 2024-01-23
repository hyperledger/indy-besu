// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { InvalidCredentialDefinitionId } from "./ClErrors.sol";

import { toSlice } from "@dk1a/solidity-stringutils/src/StrSlice.sol";

using { toSlice } for string;

library CredentialDefinitionValidator {
    string private constant _DELIMITER = "/";
    string private constant _CRED_DEF_ID_MIDDLE_PART = "/anoncreds/v0/CLAIM_DEF/";
    string private constant _ANONCREDS_TYPE = "CL";

    /**
     * @dev Validates the Credential Definition ID syntax
     */
    function validateIdSyntax(string memory self, string calldata issuerId, string calldata schemaId) internal pure {
        string memory credDefId = string.concat(issuerId, _CRED_DEF_ID_MIDDLE_PART, schemaId, _DELIMITER);

        if (!self.toSlice().startsWith(credDefId.toSlice())) revert InvalidCredentialDefinitionId(self);
    }
}
