pragma solidity >=0.4.22 <0.9.0;

import "./openzeppelin/ERC20.sol";
import "./openzeppelin/Context.sol";

contract TESTToken is ERC20 {

    uint8 _decimals;

    constructor(string memory name, string memory symbol, uint8 decimals) ERC20(name, symbol) {
        _decimals = decimals;
    }

    function decimals() public view virtual override returns (uint8) {
        return _decimals;
    }

    function faucet(address user, uint256 amount) public virtual {
        _mint(user, amount);
    }
}
