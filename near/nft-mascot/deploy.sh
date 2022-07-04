export NEAR_ENV=mainnet
ID=$1
near login
./build.sh && near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_simple.wasm --accountId $ID