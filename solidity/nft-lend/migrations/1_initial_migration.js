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

// const TESTNft = artifacts.require("TESTNft");
// module.exports = async (deployer, network) => {
//   // await deployer.deploy(TESTNft, 'omgkirby', 'OMG');
//   // const testNft = await TESTNft.deployed();
//   // await testNft.mintNFT('0x78785402f9b65Ca3e7B2207EE33Ae98b1d3D1Cd2', '1', 'ipfs://QmS445wqvjd3Tq2GoHWv1qVUEDyxt5UqHnu6nVDnCMLBdV/1')
//   // await testNft.mintNFT('0x78785402f9b65Ca3e7B2207EE33Ae98b1d3D1Cd2', '2', 'ipfs://QmS445wqvjd3Tq2GoHWv1qVUEDyxt5UqHnu6nVDnCMLBdV/2')
//   // await testNft.mintNFT('0x78785402f9b65Ca3e7B2207EE33Ae98b1d3D1Cd2', '3', 'ipfs://QmS445wqvjd3Tq2GoHWv1qVUEDyxt5UqHnu6nVDnCMLBdV/3')

//   // await deployer.deploy(TESTNft, 'Meta Bounty Hunters', 'MBH');
//   // const testNft = await TESTNft.deployed();
//   // await testNft.mintNFT('0x78785402f9b65Ca3e7B2207EE33Ae98b1d3D1Cd2', '1', 'https://mbh.mypinata.cloud/ipfs/QmTBDKUQGVyj8owvP7WSseaJQ77tD2ADidjdCJQjZXpA6H/1')
//   // await testNft.mintNFT('0x78785402f9b65Ca3e7B2207EE33Ae98b1d3D1Cd2', '2', 'https://mbh.mypinata.cloud/ipfs/QmTBDKUQGVyj8owvP7WSseaJQ77tD2ADidjdCJQjZXpA6H/2')
//   // await testNft.mintNFT('0x78785402f9b65Ca3e7B2207EE33Ae98b1d3D1Cd2', '3', 'https://mbh.mypinata.cloud/ipfs/QmTBDKUQGVyj8owvP7WSseaJQ77tD2ADidjdCJQjZXpA6H/3')

//   await deployer.deploy(TESTNft, 'TECHNOFISH by Calvin Harris x Emil Nava', 'TECHNOFISHBYCALVINHARRISXEMILNAVA');
//   const testNft = await TESTNft.deployed();
//   await testNft.mintNFT('0x78785402f9b65Ca3e7B2207EE33Ae98b1d3D1Cd2', '1', 'https://api.niftygateway.com/harrisnava/35500020007')
//   await testNft.mintNFT('0x78785402f9b65Ca3e7B2207EE33Ae98b1d3D1Cd2', '2', 'https://api.niftygateway.com/harrisnava/35500020009')
//   await testNft.mintNFT('0x78785402f9b65Ca3e7B2207EE33Ae98b1d3D1Cd2', '3', 'https://api.niftygateway.com/harrisnava/35500020009')

//   console.log(testNft.address)
// };
