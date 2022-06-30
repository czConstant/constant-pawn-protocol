export NEAR_ENV=testnet
dev=pawn-dev1.testnet
PAWN_ID=duynguyen-pawn9.testnet
./build.sh && near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_pawn.wasm --accountId $PAWN_ID
./build.sh && near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_pawn.wasm --accountId $dev