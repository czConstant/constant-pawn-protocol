## Installing the near-cli
```
npm install -g near-cli
```

## Clone nft-pawn project
```
git clone https://github.com/czConstant/constant-pawn-protocol/
cd ./near/nft-lend
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
