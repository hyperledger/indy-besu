// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @notice Error that occurs when performed identity operation by not owned account.
 * @param sender Sender account address.
 * @param owner Owner account address.
 */
error NotIdentityOwner(address sender, address owner);
