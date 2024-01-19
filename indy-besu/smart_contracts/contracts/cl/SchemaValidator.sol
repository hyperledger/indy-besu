// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { InvalidSchemaId } from "./ClErrors.sol";

import { toSlice } from "@dk1a/solidity-stringutils/src/StrSlice.sol";

using { toSlice } for string;

library SchemaValidator {
    string private constant _DELIMITER = "/";
    string private constant _SCHEMA_ID_MIDDLE_PART = "/anoncreds/v0/SCHEMA/";

    /**
     * @dev Validates the Schema ID syntax
     */
    function validateIdSyntax(string memory self, string calldata issuerId) internal pure {
        string memory schemaId = string.concat(issuerId, _SCHEMA_ID_MIDDLE_PART);

        if (!self.toSlice().startsWith(schemaId.toSlice())) revert InvalidSchemaId(self);
    }
}
