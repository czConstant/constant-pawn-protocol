pragma solidity >=0.4.22 <0.9.0;

import "./openzeppelin/ERC20.sol";
import "./openzeppelin/Context.sol";

contract TESTToken is ERC20 {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) {}

    function decimals() public view virtual override returns (uint8) {
        return 6;
    }

    function faucet(address user, uint256 amount) public virtual {
        _mint(user, amount);
    }
}
