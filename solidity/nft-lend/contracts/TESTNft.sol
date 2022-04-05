pragma solidity >=0.4.22 <0.9.0;

import "./openzeppelin/ERC721URIStorage.sol";
import "./openzeppelin/Context.sol";

contract TESTNft is ERC721URIStorage {
    constructor(string memory name, string memory symbol)
        ERC721(name, symbol)
    {}

    function faucet(address user, uint256 tokenId) public virtual {
        _mint(user, tokenId);
    }

    function mintNFT(address recipient, uint256 tokenId, string memory tokenURI)
        public
        returns (uint256)
    {
        _mint(recipient, tokenId);
        _setTokenURI(tokenId, tokenURI);
        return tokenId;
    }
}
