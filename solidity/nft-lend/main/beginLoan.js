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

    var _borrower = "0xaC9357969b310614DbfD7bdF513727896c86ED33"
    let borrowerPrk = "d4ed67dc0abe25085fede6e34b49abcef7f7a14523fa093bc267942f4a08abd9"

    var _lender = "0x15d9B2BFc48Fe9881afa0d5343b5cF8ba6CFD4e7"
    let lenderPrk = "408bc7531f431a7c61cf8536f8fd1daf0e1d89043ee65442808c28fd327f9a84"

    let chainId = '80001'

    // let usdToken = await TESTToken.at('0x0bB8Fe1750FF276d20c8A7D03E012034dB218941')
    // let nft = await TESTNft.at('0x4D9cc948E54E1C6C26Fd014D15b4bE994896f595');
    // let nftPawn = await NFTPawn.at('0xbce662a8b91e307445b2edc40741b0ca177f2784');

    let usdTokenAddres = '0x0bB8Fe1750FF276d20c8A7D03E012034dB218941'
    let usdToken = new web3.eth.Contract(require('../build/contracts/TESTToken.json').abi, usdTokenAddres);
    let nftAddress = '0x4D9cc948E54E1C6C26Fd014D15b4bE994896f595'
    let nft = new web3.eth.Contract(require('../build/contracts/TESTNft.json').abi, nftAddress);
    let nftPawnAddress = '0xFE3865908CDB81D8906C431Dc8451f7ECE7b95B0'
    let nftPawn = new web3.eth.Contract(require('../build/contracts/NFTPawn.json').abi, nftPawnAddress);

    var _nftCollateralId = '1'

    // {
    //   const tx = nft.methods.setApprovalForAll(nftPawn.options.address, true);
    //   const gas = (await tx.estimateGas({ from: _borrower })) * 2;
    //   const gasPrice = await web3.eth.getGasPrice();
    //   const data = tx.encodeABI();
    //   const nonce = await web3.eth.getTransactionCount(_borrower, 'pending');
    //   const signedTx = await web3.eth.accounts.signTransaction(
    //     {
    //       to: nft.options.address,
    //       data,
    //       gas,
    //       gasPrice,
    //       nonce,
    //       chainId: chainId
    //     },
    //     borrowerPrk
    //   );
    //   let r = await web3.eth.sendSignedTransaction(signedTx.rawTransaction);
    //   console.log(r)
    // }
    // return

    // {
    //   const tx = usdToken.methods.approve(nftPawn.options.address, web3.utils.toWei('1000000', 'ether'));
    //   const gas = (await tx.estimateGas({ from: _lender })) * 2;
    //   const gasPrice = await web3.eth.getGasPrice();
    //   const data = tx.encodeABI();
    //   const nonce = await web3.eth.getTransactionCount(_lender, 'pending');
    //   const signedTx = await web3.eth.accounts.signTransaction(
    //     {
    //       to: usdToken.options.address,
    //       data,
    //       gas,
    //       gasPrice,
    //       nonce,
    //       chainId: chainId
    //     },
    //     lenderPrk
    //   );
    //   let r = await web3.eth.sendSignedTransaction(signedTx.rawTransaction);
    //   console.log(r)
    // }

    var _loanPrincipalAmount = web3.utils.toWei('1000', 'picoether')
    var _loanDuration = '3600'
    var _loanInterestRate = '1250'//12.50%
    var _adminFee = '100' //1%
    var _lenderNonce = '1'
    var _nftCollateralContract = nft.options.address
    var _loanCurrency = usdToken.options.address

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

    const lenderWallet = web3.eth.accounts.privateKeyToAccount(lenderPrk);
    let lenderSig = lenderWallet.sign(lenderMsg)

    var _borrowerNonce = '1'

    borrowerMg = web3.utils.soliditySha3(
      _nftCollateralId,
      _borrowerNonce,
      _nftCollateralContract,
      _borrower,
      chainId,
    );
    const walletBorrower = web3.eth.accounts.privateKeyToAccount(borrowerPrk);
    let borrowerSig = walletBorrower.sign(borrowerMg)

    {
      const tx = nftPawn.methods.beginLoan(
        _loanPrincipalAmount,
        _nftCollateralId,
        _loanDuration,
        _loanInterestRate,
        _adminFee,
        [_borrowerNonce, _lenderNonce],
        _nftCollateralContract,
        _loanCurrency,
        _lender,
        [borrowerSig.signature, lenderSig.signature],
      );
      const gas = (await tx.estimateGas({ from: _borrower })) * 2;
      // const gas = '2000000';
      const gasPrice = await web3.eth.getGasPrice();
      const data = tx.encodeABI();
      const nonce = await web3.eth.getTransactionCount(_borrower, 'pending');
      const signedTx = await web3.eth.accounts.signTransaction(
        {
          to: nftPawn.options.address,
          data,
          gas,
          gasPrice,
          nonce,
          chainId: chainId
        },
        borrowerPrk
      );
      let r = await web3.eth.sendSignedTransaction(signedTx.rawTransaction);
      console.log(r)
    }

  } catch (error) {
    console.log(error);
    process.exit();
  }
}

main();