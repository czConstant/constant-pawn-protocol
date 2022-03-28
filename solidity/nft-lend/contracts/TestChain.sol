// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

import "./NFTPawnSigningUtils.sol";

contract TestChain is NFTPawnSigningUtils {
    constructor() public {}

    function getTestChainID() public view returns (uint256) {
        return getChainID();
    }
}
