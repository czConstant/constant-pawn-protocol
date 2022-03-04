use crate::error::LendingError::InvalidInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::convert::TryInto;
use std::mem::size_of;

pub enum LendingInstruction {
    /// Starts the trade by creating and populating an lending account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the lending
    /// 1. `[writable]` Temporary nft account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The loan info account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    InitLoan {
        // The original sum of money transferred from lender to borrower at the
        // beginning of the loan, measured in loanERC20Denomination's smallest
        // units.
        loan_principal_amount: u64,
        // The amount of time (measured in seconds) that can elapse before the
        // lender can liquidate the loan and seize the underlying collateral.
        loan_duration: u64,
        // The interestRate of money that the borrower would be required to
        // repay retrieve their collateral, measured in loanERC20Denomination's
        // smallest units.
        interest_rate: u64,
        // The token address for the NFT being used as
        // collateral for this loan. The NFT is stored within this contract
        // during the duration of the loan.
        nft_collateral_contract: Pubkey,
        // The contract of the currency being used as principal/interest
        // for this loan.
        loan_currency: Pubkey,
    },
    /// Init offer
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` lender
    /// 1. `[]` borrower
    /// 2. `[writable]` loan info account
    /// 3. `[writable]` Temporary denomination(USDT,..) account that should be created prior to this instruction and owned by the initializer
    /// 4. `[writable]` The offer info account, it will hold all necessary info about the offer.
    /// 5. `[]` The rent sysvar
    /// 6. `[]` The token program
    MakeOffer {
        // The loan_id
        loan_id: Pubkey,
        // The original sum of money transferred from lender to borrower at the
        // beginning of the loan, measured in loanERC20Denomination's smallest
        // units.
        loan_principal_amount: u64,
        // The amount of time (measured in seconds) that can elapse before the
        // lender can liquidate the loan and seize the underlying collateral.
        loan_duration: u64,
        // The interestRate of money that the borrower would be required to
        // repay retrieve their collateral, measured in loanERC20Denomination's
        // smallest units.
        interest_rate: u64,
        // The contract of the currency being used as principal/interest
        // for this loan.
        loan_currency: Pubkey,
    },

    /// Accepts a trade
    /// Accounts expected:
    /// 0. `[signer]` borrower
    /// 1. `[]` lender
    /// 2. `[writable]` loan info account
    /// 3. `[writable]` borrower token account
    /// 4. `[writable]` offer info account
    /// 5. `[writable]` PDA's token account to transfer token for borrower
    /// 6. `[]` The token program
    /// 7. `[]` The PDA account
    /// 8. `[]` The clock account
    AcceptOffer {
        // The loan_id
        loan_id: Pubkey,
        // The offer_id
        offer_id: Pubkey,
        // The original sum of money transferred from lender to borrower at the
        // beginning of the loan, measured in loanERC20Denomination's smallest
        // units.
        loan_principal_amount: u64,
        // The amount of time (measured in seconds) that can elapse before the
        // lender can liquidate the loan and seize the underlying collateral.
        loan_duration: u64,
        // The interestRate of money that the borrower would be required to
        // repay retrieve their collateral, measured in loanERC20Denomination's
        // smallest units.
        interest_rate: u64,
        // The contract of the currency being used as principal/interest
        // for this loan.
        loan_currency: Pubkey,
    },
    /// Cancel loan
    /// Accounts expected:
    /// 0. `[signer]` borrower
    /// 1. `[writable]` loan info account
    /// 2. `[writable]` PDA's collateral account to re-transfer nft to borrower
    /// 3. `[writable]` borrower nft token account
    /// 4. `[]` The token program
    /// 5. `[]` The PDA account
    CancelLoan { loan_id: Pubkey },
    /// Cancel offer
    /// Accounts expected:
    /// 0. `[signer]` lender
    /// 1. `[writable]` offer info account
    /// 2. `[writable]` PDA's token(usdt,..) account to re-transfer token to borrower
    /// 3. `[writable]` lender token account
    /// 4. `[]` The token program
    /// 5. `[]` The PDA account
    CancelOffer { offer_id: Pubkey },
    /// Pay loan
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
    PayLoan {
        loan_id: Pubkey,
        offer_id: Pubkey,
        pay_amount: u64,
    },
    /// Liquidate loan
    /// 0. `[signer]` lender
    /// 1. `[]` borrower
    /// 2. `[writable]` loan info account
    /// 3. `[writable]` offer info account
    /// 4. `[writable]` lender nft account
    /// 5. `[writable]` PDA's nft account to transfer nft for lender
    /// 6. `[writable]` PDA's token account for closing
    /// 7. `[]` The token program
    /// 8. `[]` The PDA account
    LiquidateLoan { loan_id: Pubkey, offer_id: Pubkey },
    /**
    /// 0. `[signer]` lender
    /// 1. `[writable]` offer info account
    /// 2. `[writable]` lender token account
    /// 3. `[writable]` PDA's token account for closing
    /// 4. `[]` The token program
    /// 5. `[]` The PDA account
     */
    CloseOffer { offer_id: Pubkey },

    /// order now
    ///
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
    Order {
        // The loan_id
        loan_id: Pubkey,
    },
}

impl LendingInstruction {
    /// Unpacks a byte buffer into a [LendingInstruction](enum.LendingInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitLoan {
                loan_principal_amount: rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?,
                loan_duration: rest
                    .get(8..16)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?,
                interest_rate: rest
                    .get(16..24)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?,
                nft_collateral_contract: rest
                    .get(24..56)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
                loan_currency: rest
                    .get(56..88)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
            },
            1 => Self::MakeOffer {
                loan_id: rest
                    .get(..32)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
                loan_principal_amount: rest
                    .get(32..40)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?,
                loan_duration: rest
                    .get(40..48)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?,
                interest_rate: rest
                    .get(48..56)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?,
                loan_currency: rest
                    .get(56..88)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
            },
            2 => Self::AcceptOffer {
                loan_id: rest
                    .get(..32)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
                offer_id: rest
                    .get(32..64)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
                loan_principal_amount: rest
                    .get(64..72)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?,
                loan_duration: rest
                    .get(72..80)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?,
                interest_rate: rest
                    .get(80..88)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?,
                loan_currency: rest
                    .get(88..120)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
            },
            3 => Self::CancelLoan {
                loan_id: rest
                    .get(..32)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
            },
            4 => Self::CancelOffer {
                offer_id: rest
                    .get(..32)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
            },
            5 => Self::PayLoan {
                loan_id: rest
                    .get(..32)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
                offer_id: rest
                    .get(32..64)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
                pay_amount: rest
                    .get(64..72)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?,
            },
            6 => Self::LiquidateLoan {
                loan_id: rest
                    .get(..32)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
                offer_id: rest
                    .get(32..64)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
            },
            7 => Self::CloseOffer {
                offer_id: rest
                    .get(..32)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
            },
            8 => Self::Order {
                loan_id: rest
                    .get(..32)
                    .and_then(|slice| slice.try_into().ok())
                    .map(Pubkey::new)
                    .ok_or(InvalidInstruction)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            &Self::InitLoan {
                loan_principal_amount,
                loan_duration,
                interest_rate,
                nft_collateral_contract,
                loan_currency,
            } => {
                buf.push(0);
                buf.extend_from_slice(&loan_principal_amount.to_le_bytes());
                buf.extend_from_slice(&loan_duration.to_le_bytes());
                buf.extend_from_slice(&interest_rate.to_le_bytes());
                buf.extend_from_slice(&nft_collateral_contract.to_bytes());
                buf.extend_from_slice(&loan_currency.to_bytes());
            }
            &Self::MakeOffer {
                loan_id,
                loan_principal_amount,
                loan_duration,
                interest_rate,
                loan_currency,
            } => {
                buf.push(1);
                buf.extend_from_slice(&loan_id.to_bytes());
                buf.extend_from_slice(&loan_principal_amount.to_le_bytes());
                buf.extend_from_slice(&loan_duration.to_le_bytes());
                buf.extend_from_slice(&interest_rate.to_le_bytes());
                buf.extend_from_slice(&loan_currency.to_bytes());
            }
            &Self::AcceptOffer {
                loan_id,
                offer_id,
                loan_principal_amount,
                loan_duration,
                interest_rate,
                loan_currency,
            } => {
                buf.push(2);
                buf.extend_from_slice(&loan_id.to_bytes());
                buf.extend_from_slice(&offer_id.to_bytes());
                buf.extend_from_slice(&loan_principal_amount.to_le_bytes());
                buf.extend_from_slice(&loan_duration.to_le_bytes());
                buf.extend_from_slice(&interest_rate.to_le_bytes());
                buf.extend_from_slice(&loan_currency.to_bytes());
            }
            &Self::CancelLoan { loan_id } => {
                buf.push(3);
                buf.extend_from_slice(&loan_id.to_bytes());
            }
            &Self::CancelOffer { offer_id } => {
                buf.push(4);
                buf.extend_from_slice(&offer_id.to_bytes());
            }
            &Self::PayLoan {
                loan_id,
                offer_id,
                pay_amount,
            } => {
                buf.push(5);
                buf.extend_from_slice(&loan_id.to_bytes());
                buf.extend_from_slice(&offer_id.to_bytes());
                buf.extend_from_slice(&pay_amount.to_le_bytes());
            }
            &Self::LiquidateLoan { loan_id, offer_id } => {
                buf.push(6);
                buf.extend_from_slice(&loan_id.to_bytes());
                buf.extend_from_slice(&offer_id.to_bytes());
            }
            &Self::CloseOffer { offer_id } => {
                buf.push(7);
                buf.extend_from_slice(&offer_id.to_bytes());
            }
            &Self::Order { loan_id } => {
                buf.push(8);
                buf.extend_from_slice(&loan_id.to_bytes());
            }
        };
        buf
    }
}

pub fn init_loan_instruction(
    lending_program_id: &Pubkey,
    borrower_account: &Pubkey,
    temp_token_account: &Pubkey,
    borrower_denomination_account: &Pubkey,
    loan_info_account: &Pubkey,
    sys_var_rent: &Pubkey,
    token_program: &Pubkey,

    loan_principal_amount: u64,
    loan_duration: u64,
    interest_rate: u64,
    nft_collateral_contract: &Pubkey,
    loan_currency: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = LendingInstruction::InitLoan {
        loan_principal_amount,
        loan_duration,
        interest_rate,
        nft_collateral_contract: *nft_collateral_contract,
        loan_currency: *loan_currency,
    }
    .pack();
    let accounts = vec![
        AccountMeta::new_readonly(*borrower_account, true),
        AccountMeta::new(*temp_token_account, false),
        AccountMeta::new_readonly(*borrower_denomination_account, false),
        AccountMeta::new(*loan_info_account, false),
        AccountMeta::new_readonly(*sys_var_rent, false),
        AccountMeta::new_readonly(*token_program, false),
    ];
    Ok(Instruction {
        program_id: *lending_program_id,
        accounts,
        data,
    })
}
