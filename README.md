# constant-pawn-program

This program create a peer-to-peer nft lending on Sol ecosystem.
Functions :

### Init loan instruction

> Accounts expected: 
> 
> 0. `[signer]` The account of the person initializing the lending
> 1. `[writable]` Temporary nft account that should be created prior to this instruction and owned by the initializer
> 2. `[]` The initializer's token account for the token they will receive should the trade go through
> 3. `[writable]` The loan info account, it will hold all necessary info about the trade.
> 4. `[]` The rent sysvar
> 5. `[]` The token program
>
> InitLoan {
>    
>    // The original sum of money transferred from lender to borrower at the beginning of the loan, measured in loanERC20Denomination's smallest units.
>    
>    loan_principal_amount: u64,
>    
>    // The amount of time (measured in seconds) that can elapse before the lender can liquidate the loan and seize the underlying collateral.
>    
>    loan_duration: u64,
>    
>    // The interestRate of money that the borrower would be required to repay retrieve their collateral, measured in loanERC20Denomination's smallest units.
>    
>    interest_rate: u64,
>    
>    // The token address for the NFT being used as collateral for this loan. The NFT is stored within this contract during the duration of the loan.
>    
>    nft_collateral_contract: Pubkey,
>    
>	// The contract of the currency being used as principal/interest for this loan.
>    
>    loan_currency: Pubkey,
>
>},

### Make offer instruction

> Accounts expected:
> 
> 0. `[signer]` lender
> 1. `[]` borrower
> 2. `[writable]` loan info account
> 3. `[writable]` Temporary denomination(USDT,..) account that should be created prior to this instruction and owned by the initializer
> 4. `[writable]` The offer info account, it will hold all necessary info about the offer.
> 5. `[]` The rent sysvar
> 6. `[]` The token program

>MakeOffer {
>
>    // The loan_id
>
>    loan_id: Pubkey,
>
>    // The original sum of money transferred from lender to borrower at the beginning of the loan, measured in loanERC20Denomination's smallest  units.
>
>    loan_principal_amount: u64,
>
>    // The amount of time (measured in seconds) that can elapse before the lender can liquidate the loan and seize the underlying collateral.
>
>    loan_duration: u64,
>
>    // The interestRate of money that the borrower would be required to repay retrieve their collateral, measured in loanERC20Denomination's
>
>    // smallest units.
>
>    interest_rate: u64,
>
>    // The contract of the currency being used as principal/interest for this loan.
>
>    loan_currency: Pubkey,
>
>    //expired time
>
>    expired: u64,
>},

### Accept offer instruction

> Accounts expected:
> 
> 0. `[signer]` borrower
> 1. `[]` lender
> 2. `[writable]` loan info account
> 3. `[writable]` borrower token account
> 4. `[writable]` offer info account
> 5. `[writable]` PDA's token account to transfer token for borrower
> 6. `[]` The token program
> 7. `[]` The PDA account
> 8. `[]` The clock account
>
>AcceptOffer {
>
>    // The loan_id
>
>    loan_id: Pubkey,
>
>    // The offer_id
>
>    offer_id: Pubkey,
>
>    // The original sum of money transferred from lender to borrower at the beginning of the loan, measured in loanERC20Denomination's smallest units.
>
>    loan_principal_amount: u64,
>
>    // The amount of time (measured in seconds) that can elapse before the lender can liquidate the loan and seize the underlying collateral.
>    
>    loan_duration: u64,
>    
>    // The interestRate of money that the borrower would be required to repay retrieve their collateral, measured in loanERC20Denomination's smallest units.
>    
>    interest_rate: u64,
>    
>    // The contract of the currency being used as principal/interest for this loan.
>    
>    loan_currency: Pubkey,
>},

### Cancel loan instruction

> Accounts expected:
> 
> 0. `[signer]` borrower
> 1. `[writable]` loan info account
> 2. `[writable]` PDA's collateral account to re-transfer nft to borrower
> 3. `[writable]` borrower nft token account
> 4. `[]` The token program
> 5. `[]` The PDA account
>
>CancelLoan { loan_id: Pubkey },

### Cancel offer instruction

> Accounts expected:
> 
> 0. `[signer]` lender
> 1. `[writable]` offer info account
> 2. `[writable]` PDA's token(usdt,..) account to re-transfer token to borrower
> 3. `[writable]` lender token account
> 4. `[]` The token program
> 5. `[]` The PDA account
>
>CancelOffer { offer_id: Pubkey },

### Pay loan instruction

> Accounts expected:
> 
> 0. `[signer]` borrower
> 1. `[]` lender
> 2. `[writable]` loan info account
> 3. `[writable]` offer info account
> 4. `[writable]` borrower nft account
> 5. `[writable]` borrower token account
> 6. `[writable]` PDA's nft account to transfer nft for borrower
> 7. `[writable]` PDA's token account for transfer token to lender
> 8. `[writable]` Admin token account to receive interest
> 9. `[]` The token program
> 10. `[]` The PDA account
> 11. `[]` The clock account
>
>PayLoan {
>
>    loan_id: Pubkey,
>
>    offer_id: Pubkey,
>
>    pay_amount: u64,
>},

### Liquidate loan instruction

> Accounts expected:
> 
> 0. `[signer]` lender
> 1. `[]` borrower
> 2. `[writable]` loan info account
> 3. `[writable]` offer info account
> 4. `[writable]` lender nft account
> 5. `[writable]` PDA's nft account to transfer nft for lender
> 6. `[writable]` PDA's token account for closing
> 7. `[]` The token program
> 8. `[]` The PDA account
> 
>LiquidateLoan { loan_id: Pubkey, offer_id: Pubkey },

### Close offer instruction

> Accounts expected:
> 
> 0. `[signer]` lender
> 1. `[writable]` offer info account
> 2. `[writable]` lender token account
> 3. `[writable]` PDA's token account for closing
> 4. `[]` The token program
> 5. `[]` The PDA account
>
>CloseOffer { offer_id: Pubkey },

### Offer now instruction

> Accounts expected:
> 
> 0. `[signer]` lender
> 1. `[]` borrower
> 2. `[writable]` loan info account
> 3. `[writable]` borrower token account
> 4. `[writable]` Temporary denomination(USDT,..) account that should be created prior to this instruction and owned by the initializer
> 5. `[writable]` The offer info account, it will hold all necessary info about the offer.
> 6. `[]` The rent sysvar
> 7. `[]` The token program
> 8. `[]` The PDA account
> 9. `[]` The clock account
>
>Order {
>    // The loan_id
>    
>    loan_id: Pubkey,
>},

# Deploy

Go https://doc.rust-lang.org/cargo/getting-started/installation.html to install Rust and Cargo

Go https://docs.solana.com/cli/install-solana-cli-tools to install the Solana dev tools. 

Use the `cargo build-bpf` command to compile your program to a file with the so file extension.

Run `solana-keygen` new to create and save a solana keypair locally

Update the config file to devnet environment. Ex : json_rpc_url: "https://api.devnet.solana.com"

Run `solana airdrop ...` to get some free SOL 

Use the solana deploy command to deploy the program to devnet
```
solana deploy PATH_TO_YOUR_PROGRAM 
```

# scripts

scripts that can be used to interact with an pawn-program

## Commands

All available commands can be found in the `package.json` file.
There are 9 scripts: loan, offer, pay ...... Please check keys folder and initialize some necessary token accounts . Start by installing the necessary dependencies.
```
npm install
```

You can then run the following command to test transactions.
```
npm run loan|offer|pay....
```

The main flow:
```
npm run loan
npm run offer
npm run accept_offer
npm run pay
npm run close_offer
```

Some information on devnet :

Program address: `8HMWC37seYPvhyEmXjCiEx6uhNt7mqVFmvvpSR9SWpGw` 

Token Y address: `CuJAxXZWkL2p2A7fvjyQHurE2ttstuzd4XVPmvvrxPCv` this token is used as USDC on devnet

Feel free to use Alice|Bob account(pub & private key in keys folder, it has some NFT and token Y) to create loan, offer, pay ....

See the `package.json` file for more.
