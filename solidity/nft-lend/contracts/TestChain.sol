// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

import "./NFTfiSigningUtils.sol";

contract TestChain is NFTfiSigningUtils {
    constructor() public {}

    function getTestChainID() public view returns (uint256) {
        return getChainID();
    }
}
