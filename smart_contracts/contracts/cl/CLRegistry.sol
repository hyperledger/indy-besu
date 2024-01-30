// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { UnauthorizedIssuer } from "./ClErrors.sol";
import { EthereumExtDidRegistry } from "../did/EthereumExtDidRegistry.sol";

contract CLRegistry {
    /**
     * @dev Reference to the DID registry contract
     */
    EthereumExtDidRegistry internal _didRegistry;

    modifier _validIssuer(address identity, address actor) {
        if (actor != _didRegistry.identityOwner(identity)) {
            revert UnauthorizedIssuer(identity, actor);
        }
        _;
    }

    function _checkSignature(
        address identity,
        bytes32 hash,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS
    ) internal pure returns (address) {
        address signer = ecrecover(hash, sigV, sigR, sigS);
        if (identity != signer) {
            revert UnauthorizedIssuer(identity, signer);
        }
        return signer;
    }
}
