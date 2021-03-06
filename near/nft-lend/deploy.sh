git reset --hard
git checkout master
git pull
export NEAR_ENV=mainnet
near login

PAWN_ID=nftpawn-protocol.near
./build.sh && near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_pawn.wasm --accountId $PAWN_ID