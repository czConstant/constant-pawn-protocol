export NEAR_ENV=testnet
ID=mc-v5.testnet
./build.sh && near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_simple.wasm --accountId $ID