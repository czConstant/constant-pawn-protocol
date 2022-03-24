const NFTfi = artifacts.require("NFTfi");

/*
 * uncomment accounts to access the test accounts made available by the
 * Ethereum client
 * See docs: https://www.trufflesuite.com/docs/truffle/testing/writing-tests-in-javascript
 */
contract("Basic", function (/* accounts */) {
  it("should assert true", async function () {
    await NFTfi.new()
    // let contract = await NFTfi.at('0xa452D5B8788d947adc490Df54429Dc6bb9840496');
    // console.log(contract);
    // const theValueYouNeed = web3.utils.soliditySha3(
    //   { type: 'bytes32', value: 'theBytes32Value' },
    //   { type: 'address', value: '0xa452D5B8788d947adc490Df54429Dc6bb9840496' },
    //   { type: 'bytes32', value: 'IfYouAreDirectlyPuttingValueUseQuotes' },
    // );
    // console.log(theValueYouNeed);

    // var _loanPrincipalAmount = '1e18'
    // var _maximumRepaymentAmount = '2e18'
    // var _nftCollateralId = '2'
    // var _loanDuration = '3600'
    // var _loanInterestRateForDurationInBasisPoints = '2'
    // var _adminFeeInBasisPoints = '25'
    // var _lenderNonce = '1'
    // var _nftCollateralContract = '0x8fd57fe99ae9e8e5447e35ab6b50b28bb5687539'
    // var _loanERC20Denomination = '0xd27e714b8810ade3eb5559827036ffc69bbaa6b6'
    // var _lender = '0xDD14F32A7550CD51F0F405D5199f7Ec93e08E7d7'
    // var _interestIsProRated = false
    // var chainId = '97'
    // var nftFiContract = '0x7e9FFb416b947A34030A203d022500e7B8141021'
    // let erc20Contract = await IERC20.at('0xd27e714b8810ade3eb5559827036ffc69bbaa6b6');
    // await erc20Contract.approve('0x7e9FFb416b947A34030A203d022500e7B8141021', '100000000000000000000')
    // let allowance = await erc20Contract.allowance('0xDD14F32A7550CD51F0F405D5199f7Ec93e08E7d7', '0x7e9FFb416b947A34030A203d022500e7B8141021')

    // let collateralContract = await IERC721.at('0x8fd57fe99ae9e8e5447e35ab6b50b28bb5687539');
    // await collateralContract.approve('0x7e9FFb416b947A34030A203d022500e7B8141021', '2')

    // let contract = await NFTfi.at('0x7e9FFb416b947A34030A203d022500e7B8141021');
    // await contract.beginLoan('1000000000000000000', '2000000000000000000', '2', '3600', '2', '25', ['1', '1'], '0x8fd57fE99aE9E8e5447E35ab6b50B28bB5687539', '0xd27e714B8810ade3eb5559827036fFc69bbAA6B6', '0xDD14F32A7550CD51F0F405D5199f7Ec93e08E7d7', '0xfbd7ef16491ee8f90de8c74032383372e732ad1aebca6aad912cdf806bcdc1332f8c3ae5d72dbf75a526d30f5f10e3137aa23ca05b102ff5afc514e69f88250e1c', '0xa46c977e8b041065c6ef485feef32bd2f6e843d3da639283cd632bc12925ce8267b13a0022041e43f218e73f649592164f69a7dd15bdc7d1871fcdc693f76e5f1c')

    return assert.isTrue(true);
  });
});
