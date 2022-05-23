## Install lastest node or > 16
```
https://nodejs.org/en/download/package-manager/
```

## Install rust 
```
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
```

## Installing the near-cli
```
npm install -g near-cli
```

## Clone nft-pawn project
```
git clone https://github.com/czConstant/constant-pawn-protocol/
cd ./constant-pawn-protocol/near/nft-lend
```
## Setup enviroment
```
export NEAR_ENV=mainnet
PAWN_ID=xxx.near #fill your account here
```

## Login near wallet
```
near login
```

## Deploy contract
```
./build.sh && near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_pawn.wasm --accountId $PAWN_ID
```

## Init contract
```
near call $PAWN_ID new '{"owner_id": "'$PAWN_ID'"}' --accountId $PAWN_ID
```
