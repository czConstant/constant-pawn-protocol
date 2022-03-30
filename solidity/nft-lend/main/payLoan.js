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
    let chainId = '80001'
    let nftPawnAddress = '0xAb06fc8919176CB61a042fd7785aF64F33571330'
    let nftPawn = new web3.eth.Contract(require('../build/contracts/NFTPawn.json').abi, nftPawnAddress);
    var _borrower = "0xaC9357969b310614DbfD7bdF513727896c86ED33"
    let borrowerPrk = "d4ed67dc0abe25085fede6e34b49abcef7f7a14523fa093bc267942f4a08abd9"
    let usdTokenAddres = '0x0bB8Fe1750FF276d20c8A7D03E012034dB218941'
    let usdToken = new web3.eth.Contract(require('../build/contracts/TESTToken.json').abi, usdTokenAddres);
    // {
    //   const tx = usdToken.methods.approve(nftPawn.options.address, web3.utils.toWei('1000000', 'ether'));
    //   const gas = (await tx.estimateGas({ from: _borrower })) * 2;
    //   const gasPrice = await web3.eth.getGasPrice();
    //   const data = tx.encodeABI();
    //   const nonce = await web3.eth.getTransactionCount(_borrower, 'pending');
    //   const signedTx = await web3.eth.accounts.signTransaction(
    //     {
    //       to: usdToken.options.address,
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

    {
      const tx = nftPawn.methods.payBackLoan(
        0,
      );
      const gas = (await tx.estimateGas({ from: _borrower })) * 2;
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