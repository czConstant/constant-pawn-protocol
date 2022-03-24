var Web3 = require('web3');
var abiDecoder = require('abi-decoder');
var Tx = require('ethereumjs-tx').Transaction;
var BN = require('big-number');
const { HTTP_PROVIDER_LINK, PRIVATE_KEY, NFTFI_ADDRESS, NFTFI_ABI, NFTFI_BYTECODE } = require('./const.js');
const { soliditySha3 } = require("web3-utils");
const EthCrypto = require("eth-crypto");
var web3;
var nftfiContract;

async function createWeb3() {
    try {
        web3 = new Web3(new Web3.providers.HttpProvider(HTTP_PROVIDER_LINK));
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

        var _loanPrincipalAmount = '1000000000000000000'
        var _maximumRepaymentAmount = '2000000000000000000'
        var _nftCollateralId = '2'
        var _loanDuration = '3600'
        var _loanInterestRateForDurationInBasisPoints = '2'
        var _adminFeeInBasisPoints = '25'
        var _lenderNonce = '1'
        var _nftCollateralContract = '0x8fd57fE99aE9E8e5447E35ab6b50B28bB5687539'
        var _loanERC20Denomination = '0xd27e714b8810ade3eb5559827036ffc69bbaa6b6'
        var _lender = '0xDD14F32A7550CD51F0F405D5199f7Ec93e08E7d7'
        var _interestIsProRated = true
        var chainId = '1337'

        // uint256 _loanPrincipalAmount,
        // uint256 _maximumRepaymentAmount,
        // uint256 _nftCollateralId,
        // uint256 _loanDuration,
        // uint256 _loanInterestRateForDurationInBasisPoints,
        // uint256 _adminFeeInBasisPoints,
        // uint256[2] memory _borrowerAndLenderNonces,
        // address _nftCollateralContract,
        // address _loanERC20Denomination,
        // address _lender,
        // bytes memory _borrowerSignature,
        // bytes memory _lenderSignature

        let message = soliditySha3(
            _loanPrincipalAmount,
            _maximumRepaymentAmount,
            _nftCollateralId,
            _loanDuration,
            _loanInterestRateForDurationInBasisPoints,
            _adminFeeInBasisPoints,
            _lenderNonce,
            _nftCollateralContract,
            _loanERC20Denomination,
            _lender,
            _interestIsProRated,
            chainId,
        );
        // let result = Web3.utils.soliditySha3(
        //     '\x19Ethereum Signed Message:\n32',
        //     message,
        // );
        console.log("lender");
        console.log(message);

        const walletLender = web3.eth.accounts.privateKeyToAccount('ff67a58ea6996f5f1895462929031bd8b0ca64cfe0439f6bc6530c0225179e33');
        console.log(walletLender.sign(message));

        //borrow
        var _borrowerNonce = '1'
        var _borrower = '0xaC9357969b310614DbfD7bdF513727896c86ED33'

        message = Web3.utils.soliditySha3(
            _nftCollateralId,
            _borrowerNonce,
            _nftCollateralContract,
            _borrower,
            chainId,
        );

        // message = web3.eth.abi.encodeParameters(
        //     ['uint256', 'uint256', 'address', 'address', 'uint256'],
        //     [_nftCollateralId,
        //         _borrowerNonce,
        //         _nftCollateralContract,
        //         _borrower,
        //         chainId]
        // );

        // result = soliditySha3(
        //     '\x19Ethereum Signed Message:\n32',
        //     message,
        // );
        console.log("borrower");
        console.log(message);




        const walletBorrower = web3.eth.accounts.privateKeyToAccount('d4ed67dc0abe25085fede6e34b49abcef7f7a14523fa093bc267942f4a08abd9');
        console.log(walletBorrower)
        console.log(walletBorrower.sign(message));
        console.log("borrower===============end");
        // console.log(web3.eth.accounts.sign(result, 'd4ed67dc0abe25085fede6e34b49abcef7f7a14523fa093bc267942f4a08abd9'));
        // console.log(web3.eth.accounts.sign('0xeead7bbb720265c863244e113796b170429a379fc75273b83cc13e488f3dfe0f', 'd4ed67dc0abe25085fede6e34b49abcef7f7a14523fa093bc267942f4a08abd9'));
        // console.log(web3.eth.accounts.recover('0x7aeb8c5ed71ca165fb8f34257fa658905a791dbd91967a5d2a3ac77817942eee', '0xfbd7ef16491ee8f90de8c74032383372e732ad1aebca6aad912cdf806bcdc1332f8c3ae5d72dbf75a526d30f5f10e3137aa23ca05b102ff5afc514e69f88250e1c'));
        // 0x7aeb8c5ed71ca165fb8f34257fa658905a791dbd91967a5d2a3ac77817942eee

        // console.log((new Web3(new Web3.providers.HttpProvider('https://data-seed-prebsc-2-s2.binance.org:8545/'))).eth.accounts.sign(result, 'd4ed67dc0abe25085fede6e34b49abcef7f7a14523fa093bc267942f4a08abd9'));

        // const HDWalletProvider = require('@truffle/hdwallet-provider');
        // var provider = new HDWalletProvider(['d4ed67dc0abe25085fede6e34b49abcef7f7a14523fa093bc267942f4a08abd9'], "https://data-seed-prebsc-2-s2.binance.org:8545/");
        // console.log(provider)

        // var v = await testChain.getValidBorrower('0xeead7bbb720265c863244e113796b170429a379fc75273b83cc13e488f3dfe0f', '0xc62bce7a458cdc074ef7b8a03636efa653a0d032a6306d5310121cdaab1cb51f43bd49fbfec70a6425955954638ed0ea4275fc1e58c7840cecdc22ec15d9b8f11c')


        // const wallet = web3.eth.accounts.privateKeyToAccount(PRIVATE_KEY);

        // let nftfiContract = new web3.eth.Contract(NFTFI_ABI, NFTFI_ADDRESS);
        // console.log(await nftfiContract.methods.adminFeeInBasisPoints().call())

        // encodedABI = nftfiContract.methods.addPauser('0x088D8A4a03266870EDcbbbADdA3F475f404dB9B2').encodeABI()
        // var signedTx = await wallet.signTransaction({
        //     from: wallet.address,
        //     to: NFTFI_ADDRESS,
        //     gas: 500000,
        //     gasPrice: 3 * (10 ** 9),
        //     data: encodedABI
        // });
        // console.log(await web3.eth.sendSignedTransaction(signedTx.rawTransaction))


        // 0xb7973b42bcd29b45e09a72abe4888ce486f8850019f57761c8cb4997a72da50460bce15bd62a77b85e5d91b7132fb8d6c8163900c9c37fb97ee090a27660a1561c

        const signature = EthCrypto.sign("d4ed67dc0abe25085fede6e34b49abcef7f7a14523fa093bc267942f4a08abd9", "0x7aeb8c5ed71ca165fb8f34257fa658905a791dbd91967a5d2a3ac77817942eee");
        console.log(signature)
    } catch (error) {
        console.log(error);
        process.exit();
    }
}

main();