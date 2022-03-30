const NFTPawn = artifacts.require("NFTPawn");
const TESTToken = artifacts.require("TESTToken");
const TESTNft = artifacts.require("TESTNft");
const TestChain = artifacts.require("TestChain");

/*
 * uncomment accounts to access the test accounts made available by the
 * Ethereum client
 * See docs: https://www.trufflesuite.com/docs/truffle/testing/writing-tests-in-javascript
 */
function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}
contract("NFTPawn", function (accounts) {
  it("should assert true", async function () {

    let testChain = await TestChain.new()
    let chainId = await testChain.getTestChainID()

    let usdToken = await TESTToken.new('USDCToken', 'USDC');
    let nft = await TESTNft.new('World', 'World');
    let nftPawn = await NFTPawn.new();
    // let nftfi = await NFTPawn.at('0x1684d2eD9c28a25EB2a75fd8fE529d895D87972f');

    // let usdToken = await TESTToken.at('0x0bB8Fe1750FF276d20c8A7D03E012034dB218941')

    await nftPawn.whitelistERC20Currency(usdToken.address, true)
    await nftPawn.whitelistNFTContract(nft.address, true)

    let addr = await web3.eth.accounts.create();
    console.log(addr)
    await web3.eth.personal.importRawKey(addr.privateKey, '')
    await web3.eth.personal.unlockAccount(addr.address, '', 10000)
    var _borrower = addr.address
    let borrowerPrk = addr.privateKey

    addr = await web3.eth.accounts.create();
    await web3.eth.personal.importRawKey(addr.privateKey, '')
    await web3.eth.personal.unlockAccount(addr.address, '', 10000)
    var _lender = addr.address
    let lenderPrk = addr.privateKey

    await web3.eth.sendTransaction({ from: accounts[0], to: _borrower, value: web3.utils.toWei('0.1', 'ether') });
    await web3.eth.sendTransaction({ from: accounts[0], to: _lender, value: web3.utils.toWei('0.1', 'ether') });

    var _nftCollateralId = '1'

    await nft.faucet(_borrower, _nftCollateralId)
    await usdToken.faucet(_lender, web3.utils.toWei('10000', 'ether'))

    await nft.setApprovalForAll(nftPawn.address, true, { from: _borrower })
    await usdToken.approve(nftPawn.address, web3.utils.toWei('1000000', 'ether'), { from: _lender })

    await usdToken.approve(nftPawn.address, web3.utils.toWei('1000000', 'ether'), { from: _borrower })

    var _loanPrincipalAmount = web3.utils.toWei('1000', 'ether')
    var _loanDuration = '1'
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
      _loanPrincipalAmount,
      _nftCollateralId,
      _loanDuration,
      _loanInterestRate,
      _adminFee,
      _borrowerNonce,
      _nftCollateralContract,
      _loanCurrency,
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
    // await nftPawn.payBackLoan(0, { from: _borrower });
    //liquidate
    // await nftPawn.liquidateOverdueLoan(0, { from: _lender });


    return assert.isTrue(true);
  });
});
