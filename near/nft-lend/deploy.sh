git reset --hard
git checkout master
git pull
PAWN_ID=$1
./build.sh && near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_pawn.wasm --accountId $PAWN_ID