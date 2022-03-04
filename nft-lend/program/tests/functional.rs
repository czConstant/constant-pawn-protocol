#![cfg(feature = "test-bpf")]
use std::str::FromStr;

use solana_program::{hash::Hash, msg, pubkey::Pubkey, rent::Rent, system_program, sysvar};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{
    account::Account, instruction::AccountMeta, instruction::Instruction, keyed_account,
    signature::Keypair, signature::Signer, system_instruction, transaction::Transaction,
};
use spl_token::{
    self,
    instruction::{initialize_account, initialize_mint, mint_to, transfer},
};

use nft_lending::entrypoint::process_instruction;
use nft_lending::instruction::{init_loan_instruction, init_offer_instruction};
#[tokio::test]
async fn test_nft_lending() {
    // Create program and test environment
    let program_id = Pubkey::from_str("LendingbGKPFXCWuBvfkegQfZyiNwAJb9Ss623VQ5DA").unwrap();

    let payer_account = Keypair::new();

    let nft_mint_auth = Keypair::new();
    let nft_mint = Keypair::new();

    let usd_mint_auth = Keypair::new();
    let usd_mint = Keypair::new(); //usd

    let borrower_account = Keypair::new();
    let borrower_nft_account = Keypair::new(); //nft
    let borrower_usd_account = Keypair::new();

    let lender_account = Keypair::new();
    let lender_usd_account = Keypair::new();

    let mut program_test =
        ProgramTest::new("nft_lending", program_id, processor!(process_instruction));

    // Add accounts
    program_test.add_account(
        payer_account.pubkey(),
        Account {
            lamports: 5000000,
            ..Account::default()
        },
    );

    // Start and process transactions on the test network
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Initialize the token accounts
    banks_client
        .process_transaction(mint_init_transaction(
            &payer,
            &nft_mint,
            &nft_mint_auth,
            recent_blockhash,
        ))
        .await
        .unwrap();

    banks_client
        .process_transaction(mint_init_transaction(
            &payer,
            &usd_mint,
            &usd_mint_auth,
            recent_blockhash,
        ))
        .await
        .unwrap();

    banks_client
        .process_transaction(create_token_account(
            &payer,
            &nft_mint,
            recent_blockhash,
            &borrower_nft_account,
            &borrower_account.pubkey(),
        ))
        .await
        .unwrap();

    banks_client
        .process_transaction(create_token_account(
            &payer,
            &usd_mint,
            recent_blockhash,
            &borrower_usd_account,
            &borrower_account.pubkey(),
        ))
        .await
        .unwrap();

    banks_client
        .process_transaction(create_token_account(
            &payer,
            &usd_mint,
            recent_blockhash,
            &lender_usd_account,
            &lender_account.pubkey(),
        ))
        .await
        .unwrap();

    // create nft token
    let nft_mint_instructions = [mint_to(
        &spl_token::id(),
        &nft_mint.pubkey(),
        &borrower_nft_account.pubkey(),
        &nft_mint_auth.pubkey(),
        &[],
        1,
    )
    .unwrap()];

    let mut nft_mint_transaction =
        Transaction::new_with_payer(&nft_mint_instructions, Some(&payer.pubkey()));
    nft_mint_transaction.partial_sign(&[&payer, &nft_mint_auth], recent_blockhash);
    banks_client
        .process_transaction(nft_mint_transaction)
        .await
        .unwrap();

    // create usd token
    let token_mint_instructions = [mint_to(
        &spl_token::id(),
        &usd_mint.pubkey(),
        &lender_usd_account.pubkey(),
        &usd_mint_auth.pubkey(),
        &[],
        1000000,
    )
    .unwrap()];

    let mut token_mint_transaction =
        Transaction::new_with_payer(&token_mint_instructions, Some(&payer.pubkey()));
    token_mint_transaction.partial_sign(&[&payer, &usd_mint_auth], recent_blockhash);
    banks_client
        .process_transaction(token_mint_transaction)
        .await
        .unwrap();
    //start test
    let temp_nft_account = Keypair::new();
    let loan_info_account = Keypair::new();
    let loan_instructions = [
        create_temp_token_account_ix(
            &spl_token::id(),
            &payer.pubkey(),
            &temp_nft_account.pubkey(),
        ),
        init_temp_account_ix(
            &spl_token::id(),
            &nft_mint.pubkey(),
            &temp_nft_account.pubkey(),
            &borrower_account.pubkey(), // pda account
        ),
        transfer_x_tokens_to_temp_acc_ix(
            &spl_token::id(),
            &borrower_nft_account.pubkey(),
            &temp_nft_account.pubkey(),
            &borrower_account.pubkey(),
            1,
        ),
        create_state_account_ix(
            &nft_lending::id(),
            &payer.pubkey(),
            &loan_info_account.pubkey(),
            227,
        ),
        init_loan_ix(
            &nft_lending::id(),
            &borrower_account.pubkey(),
            &temp_nft_account.pubkey(),
            &borrower_usd_account.pubkey(),
            &loan_info_account.pubkey(),
            &sysvar::rent::id(),
            &spl_token::id(),
            1000,
            100,
            1,
            &nft_mint.pubkey(),
            &usd_mint.pubkey(),
        ),
    ];
    let mut loan_transaction =
        Transaction::new_with_payer(&loan_instructions, Some(&payer.pubkey()));
    loan_transaction.partial_sign(
        &[
            &payer,
            &temp_nft_account,
            &borrower_account,
            &loan_info_account,
        ],
        recent_blockhash,
    );
    banks_client
        .process_transaction(loan_transaction)
        .await
        .unwrap();

    // make offer
    let temp_usd_account = Keypair::new(); //usdt
    let offer_info_account = Keypair::new();
    let offer_instructions = [
        create_temp_token_account_ix(
            &spl_token::id(),
            &payer.pubkey(),
            &temp_usd_account.pubkey(),
        ),
        init_temp_account_ix(
            &spl_token::id(),
            &usd_mint.pubkey(),
            &temp_usd_account.pubkey(),
            &lender_account.pubkey(),
        ),
        transfer_x_tokens_to_temp_acc_ix(
            &spl_token::id(),
            &lender_usd_account.pubkey(),
            &temp_usd_account.pubkey(),
            &lender_account.pubkey(),
            1,
        ),
        create_state_account_ix(
            &nft_lending::id(),
            &payer.pubkey(),
            &offer_info_account.pubkey(),
            146,
        ),
        init_offer_ix(
            &nft_lending::id(),
            &lender_account.pubkey(),
            &temp_usd_account.pubkey(),
            &offer_info_account.pubkey(),
            &sysvar::rent::id(),
            &spl_token::id(),
            &loan_info_account.pubkey(),
            1000,
            100,
            1,
            &usd_mint.pubkey(),
        ),
    ];

    let mut offer_transaction =
        Transaction::new_with_payer(&offer_instructions, Some(&payer.pubkey()));
    offer_transaction.partial_sign(
        &[
            &payer,
            &temp_usd_account,
            &lender_account,
            &offer_info_account,
        ],
        recent_blockhash,
    );
    banks_client
        .process_transaction(offer_transaction)
        .await
        .unwrap();
}

fn mint_init_transaction(
    payer: &Keypair,
    mint: &Keypair,
    mint_authority: &Keypair,
    recent_blockhash: Hash,
) -> Transaction {
    let instructions = [
        system_instruction::create_account(
            &payer.pubkey(),
            &mint.pubkey(),
            Rent::default().minimum_balance(82),
            82,
            &spl_token::id(),
        ),
        initialize_mint(
            &spl_token::id(),
            &mint.pubkey(),
            &mint_authority.pubkey(),
            None,
            0,
        )
        .unwrap(),
    ];
    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
    transaction.partial_sign(&[payer, mint], recent_blockhash);
    transaction
}

fn create_token_account(
    payer: &Keypair,
    mint: &Keypair,
    recent_blockhash: Hash,
    token_account: &Keypair,
    token_account_owner: &Pubkey,
) -> Transaction {
    let instructions = [
        system_instruction::create_account(
            &payer.pubkey(),
            &token_account.pubkey(),
            Rent::default().minimum_balance(165),
            165,
            &spl_token::id(),
        ),
        initialize_account(
            &spl_token::id(),
            &token_account.pubkey(),
            &mint.pubkey(),
            token_account_owner,
        )
        .unwrap(),
    ];
    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
    transaction.partial_sign(&[payer, token_account], recent_blockhash);
    transaction
}

fn create_temp_token_account_ix(
    program_id: &Pubkey,
    initializer_account: &Pubkey,
    temp_token_account: &Pubkey,
) -> Instruction {
    let ix = system_instruction::create_account(
        initializer_account,
        temp_token_account,
        Rent::default().minimum_balance(165),
        165,
        program_id,
    );
    ix
}

fn init_temp_account_ix(
    program_id: &Pubkey,
    mint: &Pubkey,
    account: &Pubkey,
    owner: &Pubkey,
) -> Instruction {
    let ix = initialize_account(program_id, account, mint, owner).unwrap();
    ix
}

fn transfer_x_tokens_to_temp_acc_ix(
    program_id: &Pubkey,
    source: &Pubkey,
    destination: &Pubkey,
    owner: &Pubkey,
    amount: u64,
) -> Instruction {
    let ix = transfer(program_id, source, destination, owner, &[], amount).unwrap();
    ix
}

fn create_state_account_ix(
    program_id: &Pubkey,
    initializer_account: &Pubkey,
    state_account: &Pubkey,
    space: u64,
) -> Instruction {
    let b = space as usize;
    let b = usize::try_from(space);
    let ix = system_instruction::create_account(
        initializer_account,
        state_account,
        Rent::default().minimum_balance(b.unwrap()),
        space,
        program_id,
    );
    ix
}

fn init_loan_ix(
    lending_program_id: &Pubkey,
    borrower_account: &Pubkey,
    borrower_temp_token_account: &Pubkey,
    borrower_denomination_account: &Pubkey,
    loan_info_account: &Pubkey,
    sys_var_rent: &Pubkey,
    token_program: &Pubkey,

    loan_principal_amount: u64,
    loan_duration: u64,
    interest_rate: u8,
    nft_collateral_contract: &Pubkey,
    loan_currency: &Pubkey,
) -> Instruction {
    let ix = init_loan_instruction(
        lending_program_id,
        borrower_account,
        borrower_temp_token_account,
        borrower_denomination_account,
        loan_info_account,
        sys_var_rent,
        token_program,
        loan_principal_amount,
        loan_duration,
        interest_rate,
        nft_collateral_contract,
        loan_currency,
    )
    .unwrap();
    ix
}

fn init_offer_ix(
    lending_program_id: &Pubkey,
    lender_account: &Pubkey,
    lender_temp_token_account: &Pubkey,
    offer_info_account: &Pubkey,
    sys_var_rent: &Pubkey,
    token_program: &Pubkey,

    loan_id: &Pubkey,
    loan_principal_amount: u64,
    loan_duration: u64,
    interest_rate: u8,
    loan_currency: &Pubkey,
) -> Instruction {
    let ix = init_offer_instruction(
        lending_program_id,
        lender_account,
        lender_temp_token_account,
        offer_info_account,
        sys_var_rent,
        token_program,
        loan_id,
        loan_principal_amount,
        loan_duration,
        interest_rate,
        loan_currency,
    )
    .unwrap();
    ix
}
