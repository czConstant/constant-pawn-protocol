const NFTPawn = artifacts.require("NFTPawn");
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
// (0) 0xeC320dcCdcd01DE733aC9Ea47d75fa06a5Cad284 (100 ETH)
// (1) 0xCa0BC1c6b19901b1759d57F1896CC7DBed6de863 (100 ETH)
// (2) 0xA1173853f0A6B6DE2D49b51dB26Dfe84894f83D6 (100 ETH)
// (3) 0xA01380e639E68aa590C156A5037f54878D37b848 (100 ETH)
// (4) 0x155ABA347391E8DfFE26F726201907a2a6498bE6 (100 ETH)
// (5) 0x0e1f08f54dC6336A788dFCaf211AaDB1Ad546979 (100 ETH)
// (6) 0x27676C03d927F793EceE2E3591c0921b0C830134 (100 ETH)
// (7) 0x678E982F6A3bD5A93d6D5EA3e4F45b68BFbB3C33 (100 ETH)
// (8) 0x8091E892455E082c9038382B1b3D235F32F7E78d (100 ETH)
// (9) 0xA1b11B6f6A14D59567659785B3768E7794594D84 (100 ETH)

// Private Keys
// ==================
// (0) 0x3cb9116d63cc22814d903b4843d4e1728a5b68a0318c8c5a7dde2a6a92bca304
// (1) 0xabfd861bdeaf1d7caacc67750be847e82b88af74cbdaf5d6f23e6d5e35fe2087
// (2) 0xa9873772b5e4d7f9366b26d1e56b7a97e67826ecdfed5e74fe6853114a3d6dd3
// (3) 0x3f414a3247b2f5ed6d7002b4511ae0bcf4ef110a53d39b3d7329be94cccc15ae
// (4) 0x6dbb4dd5184a4985ef25d22fa9d33fe051cf9ea735537c1a0668c762fd52b1b8
// (5) 0x43dbc71f5658d81bf529c7e34c9c0f0f6eaaaa3fc2be4cc7d530c003ce12ebbb
// (6) 0xb8d271ddd59da08c4418897326c96c8354be3f18907bb02f23a02cc61b88877c
// (7) 0xf260acfe5adeda505a4519eff7c2b6c209761afff57f49bfd851c3d0129193b3
// (8) 0xc21245cff289413bb13d6b386c5de1f6d38501e9cf0008e7a453923977af5da1
// (9) 0x072a99164c4cb154b7f315d3ff14e50ad689ffd06819bd30f1a1d58617f9b23b

contract("NFTPawn", function (accounts) {
  it("should assert true", async function () {
    let testChain = await TestChain.new()
    let chainId = await testChain.getTestChainID()

    let usdToken = await TESTToken.new('USDCToken', 'USDC');
    let nft = await TESTNft.new('World', 'World');
    let nftPawn = await NFTPawn.new();
    // let nftfi = await NFTfi.at('0xC618ED0213b7370D02dF331474Bd727B5fB02dAc');

    // let usdToken = await TESTToken.at('0x0bB8Fe1750FF276d20c8A7D03E012034dB218941')

    await nftPawn.whitelistERC20Currency(usdToken.address, true)
    await nftPawn.whitelistNFTContract(nft.address, true)

    var _borrower = accounts[8]
    var _lender = accounts[9]

    let borrowerPrk = '0xc21245cff289413bb13d6b386c5de1f6d38501e9cf0008e7a453923977af5da1'
    let lenderPrk = '0x072a99164c4cb154b7f315d3ff14e50ad689ffd06819bd30f1a1d58617f9b23b'

    var _nftCollateralId = '1'

    await nft.faucet(_borrower, _nftCollateralId)
    await usdToken.faucet(_lender, web3.utils.toWei('10000', 'ether'))

    await nft.setApprovalForAll(nftPawn.address, true, { from: _borrower })
    await usdToken.approve(nftPawn.address, web3.utils.toWei('1000000', 'ether'), { from: _lender })

    var _loanPrincipalAmount = web3.utils.toWei('1000', 'ether')
    var _loanDuration = '3600'
    var _loanInterestRate = '2'
    var _adminFee = '1'
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

    return assert.isTrue(true);
  });
});
