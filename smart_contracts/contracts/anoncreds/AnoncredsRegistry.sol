// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { DidNotFound, IncorrectDid, NotIdentityOwner } from "../did/DidErrors.sol";
import { DidMetadata } from "../did/DidTypes.sol";
import { UniversalDidResolverInterface } from "../did/UniversalDidResolverInterface.sol";
import { Errors } from "../utils/Errors.sol";
import { InvalidIssuerId, IssuerHasBeenDeactivated, IssuerNotFound } from "./AnoncredsErrors.sol";
import { RoleControlInterface } from "../auth/RoleControl.sol";

contract AnoncredsRegistry {
    /**
     * @dev Reference to the contract that resolves DIDs
     */
    UniversalDidResolverInterface internal _didResolver;

    /**
     * @dev Reference to the contract that controls account roles
     */
    RoleControlInterface internal _roleControl;

    /**
     * Checks that method was called either by Trustee or Endorser or Steward
     */
    modifier _senderIsTrusteeOrEndorserOrSteward() {
        _roleControl.isTrusteeOrEndorserOrSteward(msg.sender);
        _;
    }

    /**
     * Checks that actor matches to the identity
     */
    modifier _identityOwner(address identity, address actor) {
        if (identity != actor) revert NotIdentityOwner(actor, identity);
        _;
    }

    /**
     * @dev Check that the Issuer DID exist, authorized for actor, and active.
     * @param did        The Issuer's Account.
     * @param identity   The Issuer's DID.
     * @param actor      Actor identity address.
     */
    modifier _validIssuer(
        string calldata did,
        address identity,
        address actor
    ) {
        if (identity != actor) revert NotIdentityOwner(actor, identity);

        try _didResolver.resolveMetadata(did) returns (DidMetadata memory metadata) {
            if (identity != metadata.owner) {
                revert NotIdentityOwner(actor, identity);
            }
            if (metadata.deactivated) revert IssuerHasBeenDeactivated(did);
        } catch (bytes memory reason) {
            if (Errors.equals(reason, DidNotFound.selector)) revert IssuerNotFound(did);
            if (Errors.equals(reason, IncorrectDid.selector)) revert InvalidIssuerId(did);

            Errors.rethrow(reason);
        }
        _;
    }
}
