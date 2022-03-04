use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Offer {
    pub is_initialized: bool,
    pub lender_pubkey: Pubkey,
    pub loan_pubkey: Pubkey,
    pub loan_principal_amount: u64,
    pub loan_duration: u64,
    pub interest_rate: u64,
    pub loan_currency: Pubkey,
    pub tmp_token_account_pubkey: Pubkey,
    pub status: u8,
    pub paid_at: u64,
    pub paid_amount: u64,
}

impl Sealed for Offer {}

impl IsInitialized for Offer {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Offer {
    const LEN: usize = 170;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Offer::LEN];
        let (
            is_initialized,
            lender_pubkey,
            loan_pubkey,
            loan_principal_amount,
            loan_duration,
            interest_rate,
            loan_currency,
            tmp_token_account_pubkey,
            status,
            paid_at,
            paid_amount,
        ) = array_refs![src, 1, 32, 32, 8, 8, 8, 32, 32, 1, 8, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Offer {
            is_initialized,
            lender_pubkey: Pubkey::new_from_array(*lender_pubkey),
            loan_pubkey: Pubkey::new_from_array(*loan_pubkey),
            loan_principal_amount: u64::from_le_bytes(*loan_principal_amount),
            loan_duration: u64::from_le_bytes(*loan_duration),
            interest_rate: u64::from_le_bytes(*interest_rate),
            loan_currency: Pubkey::new_from_array(*loan_currency),
            tmp_token_account_pubkey: Pubkey::new_from_array(*tmp_token_account_pubkey),
            status: u8::from_le_bytes(*status),
            paid_at: u64::from_le_bytes(*paid_at),
            paid_amount: u64::from_le_bytes(*paid_amount),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Offer::LEN];
        let (
            is_initialized_dst,
            lender_pubkey_dst,
            loan_pubkey_dst,
            loan_principal_amount_dst,
            loan_duration_dst,
            interest_rate_dst,
            loan_currency_dst,
            tmp_token_account_pubkey_dst,
            status_dst,
            paid_at_dst,
            paid_amount_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 8, 32, 32, 1, 8, 8];

        let Offer {
            is_initialized,
            lender_pubkey,
            loan_pubkey,
            loan_principal_amount,
            loan_duration,
            interest_rate,
            loan_currency,
            tmp_token_account_pubkey,
            status,
            paid_at,
            paid_amount,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        lender_pubkey_dst.copy_from_slice(lender_pubkey.as_ref());
        loan_pubkey_dst.copy_from_slice(loan_pubkey.as_ref());
        *loan_principal_amount_dst = loan_principal_amount.to_le_bytes();
        *loan_duration_dst = loan_duration.to_le_bytes();
        *interest_rate_dst = interest_rate.to_le_bytes();
        loan_currency_dst.copy_from_slice(loan_currency.as_ref());
        tmp_token_account_pubkey_dst.copy_from_slice(tmp_token_account_pubkey.as_ref());
        *status_dst = status.to_le_bytes();
        *paid_at_dst = paid_at.to_le_bytes();
        *paid_amount_dst = paid_amount.to_le_bytes();
    }
}

pub struct Loan {
    pub is_initialized: bool,
    pub borrower_pubkey: Pubkey,
    pub collateral_account_pubkey: Pubkey,
    pub borrower_token_account_pubkey: Pubkey,
    pub loan_principal_amount: u64,
    pub loan_duration: u64,
    pub interest_rate: u64,
    pub nft_collateral_contract: Pubkey,
    pub loan_currency: Pubkey,
    pub status: u8,
    pub loan_start_at: u64,
    pub pay_amount: u64,
    pub lender_pubkey: Pubkey,
    pub offer_id: Pubkey,
}

impl Sealed for Loan {}

impl IsInitialized for Loan {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Loan {
    const LEN: usize = 266;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Loan::LEN];
        let (
            is_initialized,
            borrower_pubkey,
            collateral_account_pubkey,
            borrower_token_account_pubkey,
            loan_principal_amount,
            loan_duration,
            interest_rate,
            nft_collateral_contract,
            loan_currency,
            status,
            loan_start_at,
            pay_amount,
            lender_pubkey,
            offer_id,
        ) = array_refs![src, 1, 32, 32, 32, 8, 8, 8, 32, 32, 1, 8, 8, 32, 32];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Loan {
            is_initialized,
            borrower_pubkey: Pubkey::new_from_array(*borrower_pubkey),
            collateral_account_pubkey: Pubkey::new_from_array(*collateral_account_pubkey),
            borrower_token_account_pubkey: Pubkey::new_from_array(*borrower_token_account_pubkey),
            loan_principal_amount: u64::from_le_bytes(*loan_principal_amount),
            loan_duration: u64::from_le_bytes(*loan_duration),
            interest_rate: u64::from_le_bytes(*interest_rate),
            nft_collateral_contract: Pubkey::new_from_array(*nft_collateral_contract),
            loan_currency: Pubkey::new_from_array(*loan_currency),
            status: u8::from_le_bytes(*status),
            loan_start_at: u64::from_le_bytes(*loan_start_at),
            pay_amount: u64::from_le_bytes(*pay_amount),
            lender_pubkey: Pubkey::new_from_array(*lender_pubkey),
            offer_id: Pubkey::new_from_array(*offer_id),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Loan::LEN];
        let (
            is_initialized_dst,
            borrower_pubkey_dst,
            collateral_account_pubkey_dst,
            borrower_token_account_pubkey_dst,
            loan_principal_amount_dst,
            loan_duration_dst,
            interest_rate_dst,
            nft_collateral_contract_dst,
            loan_currency_dst,
            status_dst,
            loan_start_at_dst,
            pay_amount_dst,
            lender_pubkey_dst,
            offer_id_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8, 8, 8, 32, 32, 1, 8, 8, 32, 32];

        let Loan {
            is_initialized,
            borrower_pubkey,
            collateral_account_pubkey,
            borrower_token_account_pubkey,
            loan_principal_amount,
            loan_duration,
            interest_rate,
            nft_collateral_contract,
            loan_currency,
            status,
            loan_start_at,
            pay_amount,
            lender_pubkey,
            offer_id,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        borrower_pubkey_dst.copy_from_slice(borrower_pubkey.as_ref());
        collateral_account_pubkey_dst.copy_from_slice(collateral_account_pubkey.as_ref());
        borrower_token_account_pubkey_dst.copy_from_slice(borrower_token_account_pubkey.as_ref());
        *loan_principal_amount_dst = loan_principal_amount.to_le_bytes();
        *loan_duration_dst = loan_duration.to_le_bytes();
        *interest_rate_dst = interest_rate.to_le_bytes();
        nft_collateral_contract_dst.copy_from_slice(nft_collateral_contract.as_ref());
        loan_currency_dst.copy_from_slice(loan_currency.as_ref());
        *status_dst = status.to_le_bytes();
        *loan_start_at_dst = loan_start_at.to_le_bytes();
        *pay_amount_dst = pay_amount.to_le_bytes();
        lender_pubkey_dst.copy_from_slice(lender_pubkey.as_ref());
        offer_id_dst.copy_from_slice(offer_id.as_ref());
    }
}
