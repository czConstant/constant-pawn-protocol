// const HTTP_PROVIDER_LINK = 'https://rpc-mumbai.maticvigil.com/v1/a815d432597b6045371ea97a2a258a0a88354cb0'
const HTTP_PROVIDER_LINK = 'http://127.0.0.1:8545'
const PRIVATE_PHRASE = 'end cloud any speed come federal alert tunnel grape sauce stadium good';
const PRIVATE_KEY = '2e51913cdf274302f1135ca1174b057bb296715b9a4350dc5d00d444460ba796';
const NFTFI_ADDRESS = '0x702B3a65De0d1FB89F1E0761E41439abe161e353';
const NFTFI_ABI = require('../build/contracts/NFTfi.json').abi;
const NFTFI_BYTECODE = require('../build/contracts/NFTfi.json').bytecode;

module.exports = {
    HTTP_PROVIDER_LINK,
    PRIVATE_PHRASE,
    PRIVATE_KEY,
    NFTFI_ADDRESS,
    NFTFI_ABI,
    NFTFI_BYTECODE,
}