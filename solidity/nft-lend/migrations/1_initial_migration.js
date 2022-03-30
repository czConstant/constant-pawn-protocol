const NFTPawn = artifacts.require("NFTPawn");

module.exports = async (deployer, network) => {
  await deployer.deploy(NFTPawn);
  const nftPawn = await NFTPawn.deployed();
  if (network == 'matic_testnet') {
    await nftPawn.whitelistERC20Currency('0x0bB8Fe1750FF276d20c8A7D03E012034dB218941', true)
    await nftPawn.whitelistERC20Currency('0x82B71b3E1090502DEDE449d766FA5a700b7D8880', true)
  }
  if (network == 'avax_testnet') {
    await nftPawn.whitelistERC20Currency('0xB639D653019Ffd7fC04001Eabd44EBad5fafC56C', true)
  }
  if (network == 'development') {
    console.log('nothing')
  }
  console.log(nftPawn.address)
};
