// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { StringUtils } from "../utils/StringUtils.sol";
import { DidUtils, ParsedDid } from "../utils/DidUtils.sol";
import { IncorrectDid } from "./DidErrors.sol";

using StringUtils for string;

library IndyDidValidator {
    function validateDidSyntax(string memory did) public view {
        ParsedDid memory parsedDid = DidUtils.parseDid(did);

        if (!DidUtils.isIndyMethod(parsedDid.method)) revert IncorrectDid(did);

        uint256 identifierLength = parsedDid.identifier.length();
        if (identifierLength != 21 && identifierLength != 22) revert IncorrectDid(did);
    }
}
