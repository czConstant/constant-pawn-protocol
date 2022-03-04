use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum LendingError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    /// Expected Amount Mismatch
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,
    /// Amount Overflow
    #[error("Amount Overflow")]
    AmountOverflow,
    #[error("Invalid offer status")]
    OfferInvalidStatus,
    #[error("Invalid loan status")]
    LoanInvalidStatus,
    #[error("Invalid admin wallet")]
    InvalidAdminWallet,
}

impl From<LendingError> for ProgramError {
    fn from(e: LendingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
