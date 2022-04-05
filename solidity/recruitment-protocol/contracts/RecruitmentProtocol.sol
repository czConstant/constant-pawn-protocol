// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

import "./openzeppelin/utils/Counters.sol";
import "./openzeppelin/utils/Context.sol";

contract RecruitmentProtocol is Context {
    event UserWalletRegistered(uint256 userId, address userAddress);
    event UserWalletChanged(
        uint256 userId,
        address prevAddress,
        address newAddress
    );

    using Counters for Counters.Counter;
    Counters.Counter private _userIds;

    mapping(uint256 => address) users;

    constructor() public {}

    function register() external returns (uint256){
        _userIds.increment();
        uint256 userId = _userIds.current();
        users[userId] = _msgSender();
        emit UserWalletRegistered(userId, _msgSender());
    }

    function changeWallet(newAddress address) external {
        _userIds.increment();
        uint256 userId = _userIds.current();
        users[userId] = _msgSender();
        emit UserWalletRegistered(userId, _msgSender());
    }
}
}

