// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { NotIdentityOwner } from "./DidErrors.sol";
import { EthereumExtDidRegistry } from "./EthereumExtDidRegistry.sol";

contract DidValidator {
    /**
     * @dev Reference to the DID registry contract
     */
    EthereumExtDidRegistry internal _didRegistry;

    modifier identityOwner(address identity, address actor) {
        if (actor != _didRegistry.identityOwner(identity)) {
            revert NotIdentityOwner(identity, actor);
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
            revert NotIdentityOwner(identity, signer);
        }
        return signer;
    }
}
