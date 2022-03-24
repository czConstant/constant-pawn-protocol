const NFTfi = artifacts.require("NFTfi");
const TESTToken = artifacts.require("TESTToken");
const TESTNft = artifacts.require("TESTNft");
const TestChain = artifacts.require("TestChain");

/*
 * uncomment accounts to access the test accounts made available by the
 * Ethereum client
 * See docs: https://www.trufflesuite.com/docs/truffle/testing/writing-tests-in-javascript
 */

// Available Accounts
// ==================
// (0) 0xEdf581DE0E756b7C2A5E87323f692c7baCC57226 (100 ETH)
// (1) 0x252fD9335693E183e8532bD9db07aa292d80eD0f (100 ETH)
// (2) 0x9377fdb4bc3688507ac99B5b5968B0c5BE0475e7 (100 ETH)
// (3) 0xa0DD99f55b95883268bAec3d9A9e7ad2F30febe3 (100 ETH)
// (4) 0xBe1b09F8C075cAac90E17dCE6c03B0c24d8E9A1E (100 ETH)
// (5) 0x25bEd093B4a18421FA68a1770F5D7690C30dDD73 (100 ETH)
// (6) 0x30F9CB84c26c46CfC66944cd0b6F7863e6B0E027 (100 ETH)
// (7) 0x6919665D1bB50693ccd96c27Ae6df52f54871A84 (100 ETH)
// (8) 0x7a9C8e116Fd13292CFDdcF817b8Ba83678bAb458 (100 ETH)
// (9) 0x523eF7DeFEc96dBFff3045562063232119e1e6D8 (100 ETH)

// Private Keys
// ==================
// (0) 0x8f7bf34349e7ac5b290a021d651001419d443902386be0aa0270e7dfa05b6ea8
// (1) 0x0169c2c426c4921e77effa279d7f7fdfac237d4d5c29b206d98d5fbee527c72a
// (2) 0x1a38e1c04a982a4778b1422789a1bb78f8ac85bc77e86c1fd94a099e066afbb6
// (3) 0xcc0f7ac775422aa7c8c4bf8106c8b38d6b037e3bd3d7af031b656c28f7bbf2f0
// (4) 0x2fe95ec6861d39633057edd94caab030193be4fa30d2d003a12783126f7c1029
// (5) 0x348cdac809265091fd1fa51a03026fc90a624334c33d03c83d6283b3474804ff
// (6) 0xdc36f9eefd5b5233f1d916e49b6d395a1d7ab407369a24bb9f832c355773958f
// (7) 0x0bf4aec9bd8115dbb523b47e121fbd26ec4cad9bf5ba3cb7f59aa712526e9393
// (8) 0xc00d228bb268d8de087d14697b0a91e3283cc8a9bee49014a42460be007c1d8b
// (9) 0x786b24f78f8bea5c7f25fde0000f85f6359b6f6d8688b67d595a2520855b1032

contract("NFTfi", function (accounts) {
  it("should assert true", async function () {
    let testChain = await TestChain.new()
    let chainId = await testChain.getTestChainID()

    let usdToken = await TESTToken.new('USDCToken', 'USDC');
    let nft = await TESTNft.new('World', 'World');
    let nftfi = await NFTfi.new();

    await nftfi.whitelistERC20Currency(usdToken.address, true)
    await nftfi.whitelistNFTContract(nft.address, true)

    var _borrower = accounts[8]
    var _lender = accounts[9]

    let borrowerPrk = '0xc00d228bb268d8de087d14697b0a91e3283cc8a9bee49014a42460be007c1d8b'
    let lenderPrk = '0x786b24f78f8bea5c7f25fde0000f85f6359b6f6d8688b67d595a2520855b1032'

    var _nftCollateralId = '1'

    await nft.faucet(_borrower, _nftCollateralId)
    await usdToken.faucet(_lender, web3.utils.toWei('10000', 'ether'))

    await nft.setApprovalForAll(nftfi.address, true, { from: _borrower })
    await usdToken.approve(nftfi.address, web3.utils.toWei('1000000', 'ether'), { from: _lender })

    var _loanPrincipalAmount = web3.utils.toWei('1000', 'ether')
    var _maximumRepaymentAmount = web3.utils.toWei('1200', 'ether')
    var _loanDuration = '3600'
    var _loanInterestRateForDurationInBasisPoints = '2'
    var _adminFeeInBasisPoints = '25'
    var _lenderNonce = '1'
    var _nftCollateralContract = nft.address
    var _loanERC20Denomination = usdToken.address

    let lenderMsg = web3.utils.soliditySha3(
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

    await nftfi.beginLoan(
      _loanPrincipalAmount,
      _maximumRepaymentAmount,
      _nftCollateralId,
      _loanDuration,
      _loanInterestRateForDurationInBasisPoints,
      _adminFeeInBasisPoints,
      [_borrowerNonce, _lenderNonce],
      _nftCollateralContract,
      _loanERC20Denomination,
      _lender,
      [borrowerSig.signature, lenderSig.signature],
      { from: _borrower }
    );


    // await nftfi.beginLoan1(
    //   _loanPrincipalAmount,
    //   _maximumRepaymentAmount,
    //   _nftCollateralId,
    //   _loanDuration,
    //   _loanInterestRateForDurationInBasisPoints,
    //   _adminFeeInBasisPoints,
    //   [_borrowerNonce, _lenderNonce],
    //   _nftCollateralContract,
    //   _loanERC20Denomination,
    //   _lender,
    //   [borrowerSig.signature, lenderSig.signature],
    // );

    return assert.isTrue(true);
  });
});
