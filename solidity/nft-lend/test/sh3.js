const NFTfi = artifacts.require("NFTfi");
const TESTToken = artifacts.require("TESTToken");
const TESTNft = artifacts.require("TESTNft");
const TestChain = artifacts.require("TestChain");

/*
 * uncomment accounts to access the test accounts made available by the
 * Ethereum client
 * See docs: https://www.trufflesuite.com/docs/truffle/testing/writing-tests-in-javascript
 */

contract("SH3", function (accounts) {
  it("should assert true", async function () {

    let msg = web3.utils.soliditySha3(
      '1234',
    );

    console.log(msg)

    let borrowerPrk = '0xc00d228bb268d8de087d14697b0a91e3283cc8a9bee49014a42460be007c1d8b'
    const walletBorrower = web3.eth.accounts.privateKeyToAccount(borrowerPrk);
    let borrowerSig = walletBorrower.sign(msg)

    console.log(borrowerSig)

    return assert.isTrue(true);
  });
});
