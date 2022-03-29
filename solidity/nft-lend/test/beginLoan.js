const NFTPawn = artifacts.require("NFTPawn");
const TESTToken = artifacts.require("TESTToken");
const TESTNft = artifacts.require("TESTNft");
const TestChain = artifacts.require("TestChain");

/*
 * uncomment accounts to access the test accounts made available by the
 * Ethereum client
 * See docs: https://www.trufflesuite.com/docs/truffle/testing/writing-tests-in-javascript
 */

contract("NFTPawn", function (accounts) {
  it("should assert true", async function () {

    // let testChain = await TestChain.new()
    // let chainId = await testChain.getTestChainID()

    var _borrower = "0xaC9357969b310614DbfD7bdF513727896c86ED33"
    let borrowerPrk = "0xd4ed67dc0abe25085fede6e34b49abcef7f7a14523fa093bc267942f4a08abd9"
    await web3.eth.personal.importRawKey(borrowerPrk, '')
    await web3.eth.personal.unlockAccount(_borrower, '', 10000)

    var _lender = "0x15d9B2BFc48Fe9881afa0d5343b5cF8ba6CFD4e7"
    let lenderPrk = "0x408bc7531f431a7c61cf8536f8fd1daf0e1d89043ee65442808c28fd327f9a84"
    await web3.eth.personal.importRawKey(lenderPrk, '')
    await web3.eth.personal.unlockAccount(_lender, '', 10000)

    let chainId = '80001'

    let usdToken = await TESTToken.at('0x0bB8Fe1750FF276d20c8A7D03E012034dB218941')
    let nft = await TESTNft.at('0x4D9cc948E54E1C6C26Fd014D15b4bE994896f595');
    let nftPawn = await NFTPawn.at('0xbce662a8b91e307445b2edc40741b0ca177f2784');

    var _nftCollateralId = '1'

    await nft.setApprovalForAll(nftPawn.address, true, { from: _borrower })
    await usdToken.approve(nftPawn.address, web3.utils.toWei('1000000', 'ether'), { from: _lender })

    var _loanPrincipalAmount = web3.utils.toWei('1000', 'ether')
    var _loanDuration = '3600'
    var _loanInterestRate = '1250'//12.50%
    var _adminFee = '100' //1%
    var _lenderNonce = '1'
    var _nftCollateralContract = nft.address
    var _loanCurrency = usdToken.address

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

    await nftPawn.beginLoan(
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
      { from: _borrower }
    );

    //pay back
    // await nftPawn.payBackLoan(1, { from: _borrower });

    return assert.isTrue(true);
  });
});
