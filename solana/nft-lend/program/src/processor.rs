use crate::{
    enums::LoanStatus, error::LendingError, instruction::LendingInstruction, state::Loan,
    state::Offer, utils,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};
use spl_token::state::Account as TokenAccount;

static ADMIN_WALLET: &str = "2cBAH57hq9RVfA1DeScHRv1WHKL3f9LTxrTzYupDBocq";

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = LendingInstruction::unpack(instruction_data)?;
        match instruction {
            LendingInstruction::InitLoan {
                loan_principal_amount,
                loan_duration,
                interest_rate,
                nft_collateral_contract,
                loan_currency,
            } => {
                msg!("Instruction: InitLoan");
                Self::process_init_loan(
                    accounts,
                    loan_principal_amount,
                    loan_duration,
                    interest_rate,
                    nft_collateral_contract,
                    loan_currency,
                    program_id,
                )
            }
            LendingInstruction::MakeOffer {
                loan_id,
                loan_principal_amount,
                loan_duration,
                interest_rate,
                loan_currency,
                expired,
            } => {
                msg!("Instruction: MakeOffer");
                Self::process_make_offer(
                    accounts,
                    loan_id,
                    loan_principal_amount,
                    loan_duration,
                    interest_rate,
                    loan_currency,
                    expired,
                    program_id,
                )
            }
            LendingInstruction::AcceptOffer {
                loan_id,
                offer_id,
                loan_principal_amount,
                loan_duration,
                interest_rate,
                loan_currency,
            } => {
                msg!("Instruction: accept offer");
                Self::process_accept_offer(
                    accounts,
                    loan_id,
                    offer_id,
                    loan_principal_amount,
                    loan_duration,
                    interest_rate,
                    loan_currency,
                    program_id,
                )
            }
            LendingInstruction::CancelLoan { loan_id } => {
                msg!("Instruction: cancel loan");
                Self::process_cancel_loan(accounts, loan_id, program_id)
            }
            LendingInstruction::CancelOffer { offer_id } => {
                msg!("Instruction: cancel offer");
                Self::process_cancel_offer(accounts, offer_id, program_id)
            }
            LendingInstruction::PayLoan {
                loan_id,
                offer_id,
                pay_amount,
            } => {
                msg!("Instruction: repay loan");
                Self::process_pay_loan(accounts, loan_id, offer_id, pay_amount, program_id)
            }
            LendingInstruction::LiquidateLoan { loan_id, offer_id } => {
                msg!("Instruction: liquidate loan");
                Self::process_liquidate_loan(accounts, loan_id, offer_id, program_id)
            }
            LendingInstruction::CloseOffer { offer_id } => {
                msg!("Instruction: close offer");
                Self::process_close_offer(accounts, offer_id, program_id)
            }
            LendingInstruction::Order { loan_id } => {
                msg!("Instruction: Order");
                Self::process_order(accounts, loan_id, program_id)
            }
        }
    }

    fn process_init_loan(
        accounts: &[AccountInfo],
        loan_principal_amount: u64,
        loan_duration: u64,
        interest_rate: u64,
        nft_collateral_contract: Pubkey,
        loan_currency: Pubkey,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let borrower = next_account_info(account_info_iter)?;

        if !borrower.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let temp_nft_account = next_account_info(account_info_iter)?;

        let token_to_receive_account = next_account_info(account_info_iter)?;
        if *token_to_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let loan_info_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        if !rent.is_exempt(loan_info_account.lamports(), loan_info_account.data_len()) {
            return Err(LendingError::NotRentExempt.into());
        }
        let mut loan_info = Loan::unpack_unchecked(&loan_info_account.try_borrow_data()?)?;
        if loan_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        msg!("setup loan_info for");
        loan_info.is_initialized = true;
        loan_info.borrower_pubkey = *borrower.key;
        loan_info.collateral_account_pubkey = *temp_nft_account.key;
        loan_info.borrower_token_account_pubkey = *token_to_receive_account.key;
        loan_info.loan_principal_amount = loan_principal_amount;

        loan_info.loan_duration = loan_duration;
        loan_info.interest_rate = interest_rate;
        loan_info.nft_collateral_contract = nft_collateral_contract;
        loan_info.loan_currency = loan_currency;

        loan_info.status = LoanStatus::Open as u8;
        loan_info.loan_start_at = 0;
        loan_info.pay_amount =
            utils::estimate_pay_amount(loan_principal_amount, loan_duration, interest_rate);
        msg!("done setup loan_info");
        Loan::pack(loan_info, &mut loan_info_account.try_borrow_mut_data()?)?;
        let (pda, _nonce) = Pubkey::find_program_address(&[b"lending"], program_id);

        let token_program = next_account_info(account_info_iter)?;
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_nft_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            borrower.key,
            &[&borrower.key],
        )?;

        msg!("Calling the token program to transfer token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                temp_nft_account.clone(),
                borrower.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }

    fn process_make_offer(
        accounts: &[AccountInfo],
        loan_id: Pubkey,
        loan_principal_amount: u64,
        loan_duration: u64,
        interest_rate: u64,
        loan_currency: Pubkey,
        expired: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let lender = next_account_info(account_info_iter)?;

        if !lender.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let borrower = next_account_info(account_info_iter)?;

        let loan_info_account = next_account_info(account_info_iter)?;

        let loan_info = Loan::unpack(&loan_info_account.try_borrow_data()?)?;
        if loan_info.borrower_pubkey != *borrower.key {
            return Err(ProgramError::InvalidAccountData);
        }

        if loan_info.status != LoanStatus::Open as u8 {
            return Err(LendingError::LoanInvalidStatus.into());
        }

        let temp_token_account = next_account_info(account_info_iter)?;

        let offer_info_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(offer_info_account.lamports(), offer_info_account.data_len()) {
            return Err(LendingError::NotRentExempt.into());
        }

        let mut offer_info = Offer::unpack_unchecked(&offer_info_account.try_borrow_data()?)?;
        if offer_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        offer_info.is_initialized = true;
        offer_info.lender_pubkey = *lender.key;
        offer_info.loan_pubkey = loan_id;
        offer_info.loan_principal_amount = loan_principal_amount;

        offer_info.loan_duration = loan_duration;
        offer_info.interest_rate = interest_rate;
        offer_info.loan_currency = loan_currency;
        offer_info.tmp_token_account_pubkey = *temp_token_account.key;
        offer_info.status = LoanStatus::Open as u8;
        offer_info.paid_at = 0;
        offer_info.paid_amount = 0;
        offer_info.expired = expired;
        Offer::pack(offer_info, &mut offer_info_account.try_borrow_mut_data()?)?;
        let (pda, _nonce) = Pubkey::find_program_address(&[b"lending"], program_id);

        let token_program = next_account_info(account_info_iter)?;
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            lender.key,
            &[&lender.key],
        )?;

        msg!("Calling the token program to transfer token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                lender.clone(),
                token_program.clone(),
            ],
        )?;
        Ok(())
    }
    /**
    /// 0. `[signer]` borrower
    /// 1. `[]` lender
    /// 2. `[writable]` loan info account
    /// 3. `[writable]` borrower token account
    /// 4. `[writable]` offer info account
    /// 5. `[writable]` PDA's token account to transfer token for borrower
    /// 6. `[]` The token program
    /// 7. `[]` The PDA account
    /// 8. `[]` The clock account
     */
    fn process_accept_offer(
        accounts: &[AccountInfo],
        loan_id: Pubkey,
        offer_id: Pubkey,
        loan_principal_amount: u64,
        loan_duration: u64,
        interest_rate: u64,
        loan_currency: Pubkey,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let borrower = next_account_info(account_info_iter)?;
        if !borrower.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let lender = next_account_info(account_info_iter)?;

        let loan_info_account = next_account_info(account_info_iter)?;
        let mut loan_info = Loan::unpack(&loan_info_account.try_borrow_data()?)?;
        if loan_info.borrower_pubkey != *borrower.key || loan_id != *loan_info_account.key {
            return Err(ProgramError::InvalidAccountData);
        }
        if loan_info.status != LoanStatus::Open as u8 {
            return Err(LendingError::LoanInvalidStatus.into());
        }
        let borrower_token_account = next_account_info(account_info_iter)?;

        let offer_info_account = next_account_info(account_info_iter)?;
        let mut offer_info = Offer::unpack(&offer_info_account.try_borrow_data()?)?;
        if offer_info.lender_pubkey != *lender.key
            || offer_info.loan_pubkey != *loan_info_account.key
            || offer_id != *offer_info_account.key
        {
            return Err(ProgramError::InvalidAccountData);
        }

        if offer_info.status != LoanStatus::Open as u8 {
            return Err(LendingError::OfferInvalidStatus.into());
        }

        let pda_token_account = next_account_info(account_info_iter)?;
        let pda_token_account_info = TokenAccount::unpack(&pda_token_account.try_borrow_data()?)?;
        let (pda, nonce) = Pubkey::find_program_address(&[b"lending"], program_id);

        let token_program = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let clock = Clock::from_account_info(next_account_info(account_info_iter)?)?;
        // if offer_info.expired > 0 && offer_info.expired < clock.unix_timestamp as u64 {
        //     return Err(LendingError::OfferHasExpired.into());
        // }
        let transfer_token_to_borrower_ix = spl_token::instruction::transfer(
            token_program.key,
            pda_token_account.key,
            borrower_token_account.key,
            &pda,
            &[&pda],
            pda_token_account_info.amount,
        )?;
        msg!("Calling the token program to transfer tokens to the borrower...");
        invoke_signed(
            &transfer_token_to_borrower_ix,
            &[
                pda_token_account.clone(),
                borrower_token_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"lending"[..], &[nonce]]],
        )?;

        msg!("update loan info");
        loan_info.loan_principal_amount = loan_principal_amount;

        loan_info.loan_duration = loan_duration;
        loan_info.interest_rate = interest_rate;
        loan_info.loan_currency = loan_currency;

        loan_info.loan_start_at = clock.unix_timestamp as u64;
        loan_info.pay_amount =
            utils::estimate_pay_amount(loan_principal_amount, loan_duration, interest_rate);
        loan_info.status = LoanStatus::Processing as u8; //processing
        loan_info.lender_pubkey = *lender.key;
        loan_info.offer_id = *offer_info_account.key;
        msg!("update loan info done");
        Loan::pack(loan_info, &mut loan_info_account.try_borrow_mut_data()?)?;

        msg!("update offer info");
        offer_info.status = LoanStatus::Processing as u8; //processing
        msg!("update offer info done");
        Offer::pack(offer_info, &mut offer_info_account.try_borrow_mut_data()?)?;

        Ok(())
    }

    /// Accounts expected:
    /// 0. `[signer]` borrower
    /// 1. `[writable]` loan info account
    /// 2. `[writable]` PDA's collateral account to re-transfer nft to borrower
    /// 3. `[writable]` borrower nft token account
    /// 4. `[]` The token program
    /// 5. `[]` The PDA account
    fn process_cancel_loan(
        accounts: &[AccountInfo],
        loan_id: Pubkey,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let borrower = next_account_info(account_info_iter)?;

        if !borrower.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let loan_info_account = next_account_info(account_info_iter)?;

        let loan_info = Loan::unpack(&loan_info_account.try_borrow_data()?)?;

        if loan_info.borrower_pubkey != *borrower.key {
            return Err(ProgramError::InvalidAccountData);
        }
        if loan_info.status != LoanStatus::Open as u8 {
            return Err(LendingError::LoanInvalidStatus.into());
        }

        let pda_collateral_account = next_account_info(account_info_iter)?;

        let pda_collateral_account_info =
            TokenAccount::unpack(&pda_collateral_account.try_borrow_data()?)?;
        let (pda, nonce) = Pubkey::find_program_address(&[b"lending"], program_id);

        let borrower_nft_token_account = next_account_info(account_info_iter)?;

        let token_program = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let transfer_nft_to_borrower_ix = spl_token::instruction::transfer(
            token_program.key,
            pda_collateral_account.key,
            borrower_nft_token_account.key,
            &pda,
            &[&pda],
            pda_collateral_account_info.amount,
        )?;
        msg!("Calling the token program to transfer tokens to the borrower...");
        invoke_signed(
            &transfer_nft_to_borrower_ix,
            &[
                pda_collateral_account.clone(),
                borrower_nft_token_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"lending"[..], &[nonce]]],
        )?;

        let close_pdas_temp_acc_ix = spl_token::instruction::close_account(
            token_program.key,
            pda_collateral_account.key,
            borrower.key,
            &pda,
            &[&pda],
        )?;
        msg!("Calling the token program to close pda's temp account...");
        invoke_signed(
            &close_pdas_temp_acc_ix,
            &[
                pda_collateral_account.clone(),
                borrower.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"lending"[..], &[nonce]]],
        )?;

        msg!("Closing the loan account...");
        **borrower.try_borrow_mut_lamports()? = borrower
            .lamports()
            .checked_add(loan_info_account.lamports())
            .ok_or(LendingError::AmountOverflow)?;
        **loan_info_account.try_borrow_mut_lamports()? = 0;
        *loan_info_account.try_borrow_mut_data()? = &mut [];

        Ok(())
    }

    fn process_cancel_offer(
        accounts: &[AccountInfo],
        offer_id: Pubkey,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let lender = next_account_info(account_info_iter)?;

        if !lender.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let offer_info_account = next_account_info(account_info_iter)?;

        let offer_info = Offer::unpack(&offer_info_account.try_borrow_data()?)?;

        if offer_info.lender_pubkey != *lender.key {
            return Err(ProgramError::InvalidAccountData);
        }
        if offer_info.status != LoanStatus::Open as u8 {
            return Err(LendingError::OfferInvalidStatus.into());
        }

        let pda_token_account = next_account_info(account_info_iter)?;

        let pda_token_account_info = TokenAccount::unpack(&pda_token_account.try_borrow_data()?)?;
        let (pda, nonce) = Pubkey::find_program_address(&[b"lending"], program_id);

        let lender_token_account = next_account_info(account_info_iter)?;

        let token_program = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let transfer_token_to_lender_ix = spl_token::instruction::transfer(
            token_program.key,
            pda_token_account.key,
            lender_token_account.key,
            &pda,
            &[&pda],
            pda_token_account_info.amount,
        )?;
        msg!("Calling the token program to transfer tokens to the lender...");
        invoke_signed(
            &transfer_token_to_lender_ix,
            &[
                pda_token_account.clone(),
                lender_token_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"lending"[..], &[nonce]]],
        )?;

        let close_pdas_temp_acc_ix = spl_token::instruction::close_account(
            token_program.key,
            pda_token_account.key,
            lender.key,
            &pda,
            &[&pda],
        )?;
        msg!("Calling the token program to close pda's temp account...");
        invoke_signed(
            &close_pdas_temp_acc_ix,
            &[
                pda_token_account.clone(),
                lender.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"lending"[..], &[nonce]]],
        )?;

        msg!("Closing the offer account...");
        **lender.try_borrow_mut_lamports()? = lender
            .lamports()
            .checked_add(offer_info_account.lamports())
            .ok_or(LendingError::AmountOverflow)?;
        **offer_info_account.try_borrow_mut_lamports()? = 0;
        *offer_info_account.try_borrow_mut_data()? = &mut [];

        Ok(())
    }

    /**
    /// 0. `[signer]` lender
    /// 1. `[]` borrower
    /// 2. `[writable]` loan info account
    /// 3. `[writable]` offer info account
    /// 4. `[writable]` lender nft account
    /// 5. `[writable]` PDA's nft account to transfer nft for lender
    /// 6. `[writable]` PDA's token account for closing
    /// 7. `[]` The token program
    /// 8. `[]` The PDA account
    /// 9. `[]` The clock account
     */
    fn process_liquidate_loan(
        accounts: &[AccountInfo],
        loan_id: Pubkey,
        offer_id: Pubkey,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let lender = next_account_info(account_info_iter)?;
        if !lender.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let borrower = next_account_info(account_info_iter)?;
        let loan_info_account = next_account_info(account_info_iter)?;
        let loan_info = Loan::unpack(&loan_info_account.try_borrow_data()?)?;
        let offer_info_account = next_account_info(account_info_iter)?;
        let offer_info = Offer::unpack(&offer_info_account.try_borrow_data()?)?;
        let lender_nft_account = next_account_info(account_info_iter)?;
        let pda_nft_account = next_account_info(account_info_iter)?;
        let pda_nft_account_info = TokenAccount::unpack(&pda_nft_account.try_borrow_data()?)?;

        let pda_token_account = next_account_info(account_info_iter)?;

        let (pda, nonce) = Pubkey::find_program_address(&[b"lending"], program_id);

        let token_program = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let clock = Clock::from_account_info(next_account_info(account_info_iter)?)?;

        let now = clock.unix_timestamp as u64;
        let expired = loan_info.loan_start_at + loan_info.loan_duration;

        //validate
        if loan_info.borrower_pubkey != *borrower.key {
            return Err(ProgramError::InvalidAccountData);
        }
        if loan_info.status != LoanStatus::Processing as u8 {
            return Err(LendingError::LoanInvalidStatus.into());
        }
        if loan_info.lender_pubkey != *lender.key
            || loan_info.offer_id != *offer_info_account.key
            || loan_id != *loan_info_account.key
        {
            return Err(ProgramError::InvalidAccountData);
        }
        if offer_info.lender_pubkey != *lender.key
            || offer_id != *offer_info_account.key
            || offer_info.status != LoanStatus::Processing as u8
        {
            return Err(ProgramError::InvalidAccountData);
        }
        if now < expired {
            return Err(LendingError::LoanInvalidStatus.into());
        }
        let transfer_nft_to_lender_ix = spl_token::instruction::transfer(
            token_program.key,
            pda_nft_account.key,
            lender_nft_account.key,
            &pda,
            &[&pda],
            pda_nft_account_info.amount,
        )?;
        msg!("Calling the token program to transfer nft to the lender...");
        invoke_signed(
            &transfer_nft_to_lender_ix,
            &[
                pda_nft_account.clone(),
                lender_nft_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"lending"[..], &[nonce]]],
        )?;

        let close_pdas_temp_acc_ix = spl_token::instruction::close_account(
            token_program.key,
            pda_token_account.key,
            lender.key,
            &pda,
            &[&pda],
        )?;
        msg!("Calling the token program to close pda's temp account...");
        invoke_signed(
            &close_pdas_temp_acc_ix,
            &[
                pda_token_account.clone(),
                lender.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"lending"[..], &[nonce]]],
        )?;

        msg!("Closing the offer account...");
        **lender.try_borrow_mut_lamports()? = lender
            .lamports()
            .checked_add(offer_info_account.lamports())
            .ok_or(LendingError::AmountOverflow)?;
        **offer_info_account.try_borrow_mut_lamports()? = 0;
        *offer_info_account.try_borrow_mut_data()? = &mut [];

        Ok(())
    }

    /// 0. `[signer]` borrower
    /// 1. `[]` lender
    /// 2. `[writable]` loan info account
    /// 3. `[writable]` offer info account
    /// 4. `[writable]` borrower nft account
    /// 5. `[writable]` borrower token account
    /// 6. `[writable]` PDA's nft account to transfer nft for borrower
    /// 7. `[writable]` PDA's token account for transfer token to lender
    /// 8. `[writable]` Admin token account to receive interest
    /// 9. `[]` The token program
    /// 10. `[]` The PDA account
    /// 11. `[]` The clock account
    fn process_pay_loan(
        accounts: &[AccountInfo],
        loan_id: Pubkey,
        offer_id: Pubkey,
        pay_amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let borrower = next_account_info(account_info_iter)?;
        if !borrower.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let lender = next_account_info(account_info_iter)?;

        let loan_info_account = next_account_info(account_info_iter)?;
        let loan_info = Loan::unpack(&loan_info_account.try_borrow_data()?)?;

        let offer_info_account = next_account_info(account_info_iter)?;
        let mut offer_info = Offer::unpack(&offer_info_account.try_borrow_data()?)?;

        let borrower_nft_account = next_account_info(account_info_iter)?;
        let borrower_token_account = next_account_info(account_info_iter)?;

        let pda_nft_account = next_account_info(account_info_iter)?;
        let pda_nft_account_info = TokenAccount::unpack(&pda_nft_account.try_borrow_data()?)?;

        let pda_token_account = next_account_info(account_info_iter)?;
        let (pda, nonce) = Pubkey::find_program_address(&[b"lending"], program_id);

        let admin_token_account = next_account_info(account_info_iter)?;

        let token_program = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;
        let clock = Clock::from_account_info(next_account_info(account_info_iter)?)?;

        //validate
        if loan_info.borrower_pubkey != *borrower.key {
            return Err(ProgramError::InvalidAccountData);
        }
        if loan_info.status != LoanStatus::Processing as u8 {
            return Err(LendingError::LoanInvalidStatus.into());
        }
        if loan_info.lender_pubkey != *lender.key
            || loan_info.offer_id != *offer_info_account.key
            || loan_id != *loan_info_account.key
        {
            return Err(ProgramError::InvalidAccountData);
        }
        if offer_info.lender_pubkey != *lender.key
            || offer_id != *offer_info_account.key
            || offer_info.status != LoanStatus::Processing as u8
        {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let now = clock.unix_timestamp as u64;
        let expired = loan_info.loan_start_at + loan_info.loan_duration;
        if now > expired {
            return Err(LendingError::LoanInvalidStatus.into());
        }
        msg!("input pay_amount: {}", pay_amount);
        //calculate pay_amount
        let real_pay_amount = utils::calculate_pay_amount(
            loan_info.loan_principal_amount,
            loan_info.loan_duration,
            loan_info.interest_rate,
            loan_info.loan_start_at,
            clock.unix_timestamp as u64,
        );
        msg!("input real_pay_amount: {}", real_pay_amount);
        // if pay_amount != real_pay_amount {
        //     return Err(LendingError::ExpectedAmountMismatch.into());
        // }
        msg!(
            "input admin wallet: {}",
            admin_token_account.key.to_string()
        );
        if admin_token_account.key.to_string() != ADMIN_WALLET {
            return Err(LendingError::InvalidAdminWallet.into());
        }
        //repay
        //1% fee
        let fee: u64 = utils::calculate_fee(loan_info.loan_principal_amount);

        //transfer token to lender
        let transfer_token_to_lender_ix = spl_token::instruction::transfer(
            token_program.key,
            borrower_token_account.key,
            pda_token_account.key,
            borrower.key,
            &[&borrower.key],
            real_pay_amount - fee,
        )?;
        msg!("Calling the token program to repay tokens to the lender");
        invoke(
            &transfer_token_to_lender_ix,
            &[
                borrower_token_account.clone(),
                pda_token_account.clone(),
                borrower.clone(),
                token_program.clone(),
            ],
        )?;

        //transfer fee to admin
        // let transfer_fee_to_admin_ix = spl_token::instruction::transfer(
        //     token_program.key,
        //     borrower_token_account.key,
        //     admin_token_account.key,
        //     borrower.key,
        //     &[&borrower.key],
        //     fee,
        // )?;
        // msg!("Calling the token program to transfer fee to the admin");
        // invoke(
        //     &transfer_fee_to_admin_ix,
        //     &[
        //         borrower_token_account.clone(),
        //         admin_token_account.clone(),
        //         borrower.clone(),
        //         token_program.clone(),
        //     ],
        // )?;

        //withdraw nft
        let transfer_nft_to_borrower_ix = spl_token::instruction::transfer(
            token_program.key,
            pda_nft_account.key,
            borrower_nft_account.key,
            &pda,
            &[&pda],
            pda_nft_account_info.amount,
        )?;
        msg!("Calling the token program to return nft to the borrower");
        invoke_signed(
            &transfer_nft_to_borrower_ix,
            &[
                pda_nft_account.clone(),
                borrower_nft_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"lending"[..], &[nonce]]],
        )?;

        msg!("update offer info");
        offer_info.status = LoanStatus::Done as u8; //done
        offer_info.paid_at = clock.unix_timestamp as u64;
        offer_info.paid_amount = real_pay_amount - fee;
        msg!("update offer info done");
        Offer::pack(offer_info, &mut offer_info_account.try_borrow_mut_data()?)?;

        //close account
        let close_pdas_temp_acc_ix = spl_token::instruction::close_account(
            token_program.key,
            pda_nft_account.key,
            borrower.key,
            &pda,
            &[&pda],
        )?;
        msg!("Calling the token program to close pda's temp account...");
        invoke_signed(
            &close_pdas_temp_acc_ix,
            &[
                pda_nft_account.clone(),
                borrower.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"lending"[..], &[nonce]]],
        )?;

        msg!("Closing the loan account...");
        **borrower.try_borrow_mut_lamports()? = borrower
            .lamports()
            .checked_add(loan_info_account.lamports())
            .ok_or(LendingError::AmountOverflow)?;
        **loan_info_account.try_borrow_mut_lamports()? = 0;
        *loan_info_account.try_borrow_mut_data()? = &mut [];
        Ok(())
    }

    /**
    /// 0. `[signer]` lender
    /// 1. `[writable]` offer info account
    /// 2. `[writable]` lender token account
    /// 3. `[writable]` PDA's token account for closing
    /// 4. `[]` The token program
    /// 5. `[]` The PDA account
     */
    fn process_close_offer(
        accounts: &[AccountInfo],
        offer_id: Pubkey,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let lender = next_account_info(account_info_iter)?;
        if !lender.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let offer_info_account = next_account_info(account_info_iter)?;
        let offer_info = Offer::unpack(&offer_info_account.try_borrow_data()?)?;
        if offer_info.lender_pubkey != *lender.key
            || offer_id != *offer_info_account.key
            || offer_info.status != LoanStatus::Done as u8
        {
            return Err(ProgramError::InvalidAccountData);
        }

        let lender_token_account = next_account_info(account_info_iter)?;

        let pda_token_account = next_account_info(account_info_iter)?;
        let pda_token_account_info = TokenAccount::unpack(&pda_token_account.try_borrow_data()?)?;

        let (pda, nonce) = Pubkey::find_program_address(&[b"lending"], program_id);

        let token_program = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let transfer_token_to_lender_ix = spl_token::instruction::transfer(
            token_program.key,
            pda_token_account.key,
            lender_token_account.key,
            &pda,
            &[&pda],
            pda_token_account_info.amount,
        )?;
        msg!("Calling the token program to transfer nft to the lender...");
        invoke_signed(
            &transfer_token_to_lender_ix,
            &[
                pda_token_account.clone(),
                lender_token_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"lending"[..], &[nonce]]],
        )?;

        let close_pdas_temp_acc_ix = spl_token::instruction::close_account(
            token_program.key,
            pda_token_account.key,
            lender.key,
            &pda,
            &[&pda],
        )?;
        msg!("Calling the token program to close pda's temp account...");
        invoke_signed(
            &close_pdas_temp_acc_ix,
            &[
                pda_token_account.clone(),
                lender.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"lending"[..], &[nonce]]],
        )?;

        msg!("Closing the offer account...");
        **lender.try_borrow_mut_lamports()? = lender
            .lamports()
            .checked_add(offer_info_account.lamports())
            .ok_or(LendingError::AmountOverflow)?;
        **offer_info_account.try_borrow_mut_lamports()? = 0;
        *offer_info_account.try_borrow_mut_data()? = &mut [];
        Ok(())
    }

    /// Accounts expected:
    ///
    /// 0. `[signer]` lender
    /// 1. `[]` borrower
    /// 2. `[writable]` loan info account
    /// 3. `[writable]` borrower token account
    /// 4. `[writable]` Temporary denomination(USDT,..) account that should be created prior to this instruction and owned by the initializer
    /// 5. `[writable]` The offer info account, it will hold all necessary info about the offer.
    /// 6. `[]` The rent sysvar
    /// 7. `[]` The token program
    /// 8. `[]` The PDA account
    /// 9. `[]` The clock account
    fn process_order(
        accounts: &[AccountInfo],
        loan_id: Pubkey,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let lender = next_account_info(account_info_iter)?;

        if !lender.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let borrower = next_account_info(account_info_iter)?;

        let loan_info_account = next_account_info(account_info_iter)?;
        let mut loan_info = Loan::unpack(&loan_info_account.try_borrow_data()?)?;
        if loan_info.borrower_pubkey != *borrower.key {
            return Err(ProgramError::InvalidAccountData);
        }
        if loan_info.status != LoanStatus::Open as u8 {
            return Err(LendingError::LoanInvalidStatus.into());
        }
        let borrower_token_account = next_account_info(account_info_iter)?;

        let temp_token_account = next_account_info(account_info_iter)?;

        let offer_info_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(offer_info_account.lamports(), offer_info_account.data_len()) {
            return Err(LendingError::NotRentExempt.into());
        }

        let token_program = next_account_info(account_info_iter)?;

        let pda_account = next_account_info(account_info_iter)?;

        let clock = Clock::from_account_info(next_account_info(account_info_iter)?)?;

        let mut offer_info = Offer::unpack_unchecked(&offer_info_account.try_borrow_data()?)?;
        if offer_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        offer_info.is_initialized = true;
        offer_info.lender_pubkey = *lender.key;
        offer_info.loan_pubkey = loan_id;
        offer_info.loan_principal_amount = loan_info.loan_principal_amount;

        offer_info.loan_duration = loan_info.loan_duration;
        offer_info.interest_rate = loan_info.interest_rate;
        offer_info.loan_currency = loan_info.loan_currency;
        offer_info.tmp_token_account_pubkey = *temp_token_account.key;
        offer_info.status = LoanStatus::Processing as u8; //processing
        Offer::pack(offer_info, &mut offer_info_account.try_borrow_mut_data()?)?;
        let (pda, _nonce) = Pubkey::find_program_address(&[b"lending"], program_id);

        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            lender.key,
            &[&lender.key],
        )?;

        msg!("Calling the token program to transfer token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                lender.clone(),
                token_program.clone(),
            ],
        )?;
        //accept

        let transfer_token_to_borrower_ix = spl_token::instruction::transfer(
            token_program.key,
            temp_token_account.key,
            borrower_token_account.key,
            &pda,
            &[&pda],
            loan_info.loan_principal_amount,
        )?;
        msg!("Calling the token program to transfer tokens to the borrower...");
        invoke_signed(
            &transfer_token_to_borrower_ix,
            &[
                temp_token_account.clone(),
                borrower_token_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"lending"[..], &[_nonce]]],
        )?;

        msg!("update loan info");
        loan_info.loan_principal_amount = loan_info.loan_principal_amount;

        loan_info.loan_duration = loan_info.loan_duration;
        loan_info.interest_rate = loan_info.interest_rate;
        loan_info.loan_currency = loan_info.loan_currency;

        loan_info.loan_start_at = clock.unix_timestamp as u64;
        loan_info.pay_amount = utils::estimate_pay_amount(
            loan_info.loan_principal_amount,
            loan_info.loan_duration,
            loan_info.interest_rate,
        );
        loan_info.status = LoanStatus::Processing as u8; //processing
        loan_info.lender_pubkey = *lender.key;
        loan_info.offer_id = *offer_info_account.key;
        msg!("update loan info done");
        Loan::pack(loan_info, &mut loan_info_account.try_borrow_mut_data()?)?;

        //
        Ok(())
    }
}
