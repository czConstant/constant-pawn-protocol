var Web3 = require('web3');
var abiDecoder = require('abi-decoder');
var Tx = require('ethereumjs-tx').Transaction;
var BN = require('big-number');
// const { HTTP_PROVIDER_LINK, PRIVATE_KEY, NFTFI_ADDRESS, NFTFI_ABI, NFTFI_BYTECODE } = require('./const.js');
const { soliditySha3 } = require("web3-utils");
const EthCrypto = require("eth-crypto");
var web3;
var nftfiContract;

async function createWeb3() {
  try {
    web3 = new Web3(new Web3.providers.HttpProvider('https://rpc-mumbai.maticvigil.com/v1/a815d432597b6045371ea97a2a258a0a88354cb0'));
    return true;
  } catch (error) {
    console.log(error);
    return false;
  }
}

async function main() {
  try {
    if (await createWeb3() == false) {
      console.log('Web3 Create Error'.yellow);
      process.exit();
    }

    var _borrower = "0x7A63FD46d5eDB9bA7b09CAb488Eb7950e1D8cE78"
    var borrowerPrk = 'aea144296cc26d448733fe165eec2597b1ef108bfb19d4edd4f0a5d2fb5ccb9c'

    var _lender = "0x15d9B2BFc48Fe9881afa0d5343b5cF8ba6CFD4e7"
    let lenderPrk = "408bc7531f431a7c61cf8536f8fd1daf0e1d89043ee65442808c28fd327f9a84"

    let chainId = '80001'

    let usdTokenAddres = '0x0bB8Fe1750FF276d20c8A7D03E012034dB218941'
    let usdToken = new web3.eth.Contract(require('../build/contracts/TESTToken.json').abi, usdTokenAddres);
    let nftAddress = '0x66bcbd2a4008e814d878b692eb9d4a3e2c016bb2'
    let nft = new web3.eth.Contract(require('../build/contracts/TESTNft.json').abi, nftAddress);
    let nftPawnAddress = '0xFE3865908CDB81D8906C431Dc8451f7ECE7b95B0'
    let nftPawn = new web3.eth.Contract(require('../build/contracts/NFTPawn.json').abi, nftPawnAddress);

    var _nftCollateralId = '6'

    var _loanPrincipalAmount = web3.utils.toWei('1', 'picoether')
    var _loanDuration = '8640'
    var _loanInterestRate = '100'//12.50%
    var _adminFee = '100' //1%
    var _lenderNonce = '1'
    var _nftCollateralContract = nft.options.address
    var _loanCurrency = usdToken.options.address

    var _borrowerNonce = '0x4d29c250578b9babb4e7b23b00ff2837a709973e959e5cbfb55f93178c937d58'

    borrowerMg = web3.utils.soliditySha3(
      _nftCollateralId,
      _borrowerNonce,
      _nftCollateralContract,
      _borrower,
      chainId,
    );
    
    const walletBorrower = web3.eth.accounts.privateKeyToAccount(borrowerPrk);
    let borrowerSig = walletBorrower.sign(borrowerMg)

    console.log(walletBorrower)
    
    console.log(borrowerMg)
    console.log(borrowerSig)

    let lenderMsg = web3.utils.soliditySha3(
      _loanPrincipalAmount,
      _nftCollateralId,
      _loanDuration,
      _loanInterestRate,
      _adminFee,
      _lenderNonce,
      _nftCollateralContract,
      _loanCurrency,
      _lender,
      chainId,
    );

    // console.log(lenderMsg)

  } catch (error) {
    console.log(error);
    process.exit();
  }
}

main();