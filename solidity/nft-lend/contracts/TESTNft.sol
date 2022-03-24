pragma solidity >=0.4.22 <0.9.0;

import "./openzeppelin/ERC721.sol";
import "./openzeppelin/Context.sol";

contract TESTNft is ERC721 {
    constructor(string memory name, string memory symbol)
        ERC721(name, symbol)
    {}

    function faucet(address user, uint256 tokenId) public virtual {
        _mint(user, tokenId);
    }
}
