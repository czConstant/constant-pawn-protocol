git reset --hard
git checkout master
git pull
export NEAR_ENV=$1
near login

PAWN_ID=$2
./build.sh && near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_pawn.wasm --accountId $PAWN_ID