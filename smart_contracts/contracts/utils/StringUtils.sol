// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { toSlice } from "@dk1a/solidity-stringutils/src/StrSlice.sol";

using { toSlice } for string;

library StringUtils {
    bytes1 private constant _ASCII_0 = 0x30;
    bytes1 private constant _ASCII_9 = 0x39;
    bytes1 private constant _ASCII_CAPITAL_A = 0x41;
    bytes1 private constant _ASCII_CAPITAL_F = 0x46;
    bytes1 private constant _ASCII_SMALL_A = 0x61;
    bytes1 private constant _ASCII_SMALL_F = 0x66;
    string private constant _HEX_PREFIX = "0x";
    bytes private constant _ZERO_BYTES = "";

    /**
     * @dev Checks if two strings are equal.
     * @param str First string to compare.
     * @param other Second string to compare.
     * @return True if strings are equal, false otherwise.
     */
    function equals(string memory str, string memory other) internal pure returns (bool) {
        return str.toSlice().eq(other.toSlice());
    }

    /**
     * @dev Checks if a string is empty.
     * @param str String to check.
     * @return Returns `true` if the string is empty, `false` otherwise.
     */
    function isEmpty(string memory str) internal pure returns (bool) {
        return length(str) == 0;
    }

    /**
     * @dev Returns the length of a string.
     * @param str String to check.
     * @return Length of the string.
     */
    function length(string memory str) internal pure returns (uint256) {
        return bytes(str).length;
    }

    /**
     * @dev Check if a given string has a hex prefix
     * @param str String to check.
     * @return Returns `true` if strings has a hex prefix, `false` otherwise.
     */
    function hasHexPrefix(string memory str) internal pure returns (bool) {
        return str.toSlice().startsWith(_HEX_PREFIX.toSlice());
    }
}
