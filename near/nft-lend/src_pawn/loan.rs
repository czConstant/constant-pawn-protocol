use crate::*;
use near_sdk::promise_result_as_success;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Loan {
    pub loan_id: u128,
    // The address of the borrower.
    pub borrower: AccountId,
    // The address of the lender.
    pub lender: AccountId,
    // The ERC20 contract of the currency being used as principal/interest for this loan.
    pub loan_currency: AccountId,
    // The ERC721 contract of the NFT collateral
    pub nft_collateral_contract: AccountId,
    // The ID within the NFTCollateralContract for the NFT being used as
    // collateral for this loan. The NFT is stored within this contract
    // during the duration of the loan.
    pub nft_collateral_id: String,

    pub approval_id: u64,
    // // The original sum of money transferred from lender to borrower at the
    // // beginning of the loan, measured in loanERC20Denomination's smallest units.
    // pub loan_principal_amount: u128,
    // // The amount of time (measured in seconds) that can elapse before the
    // // lender can liquidate the loan and seize the underlying collateral.
    // pub loan_duration: u32,
    // // The interest rate
    // pub loan_interest_rate: u32,
    // // The block.timestamp when the loan first began (measured in seconds).
    // pub loan_start_time: U64,
    // // admin fee  (ex: input 1 ~1%)
    // pub loan_admin_fee: u32,
}
#[near_bindgen]
impl NftPawn {
    /// for add sale see: nft_callbacks.rs

    /// TODO remove without redirect to wallet? panic reverts
    #[payable]
    pub fn begin_loan(
        &mut self,
        _loan_principal_amount: u128,
        _nft_collateral_id: String,
        _loan_duration: u32,
        _loan_interest_rate: u32,
        _admin_fee: u32,
        _nft_collateral_contract: AccountId,
        _loan_currency: AccountId,
        _lender: AccountId,
    ) {
        // a

        /* TODO */
        //validate

        // //
        // self.loan_id_to_loan.insert(&self.total_num_loans, &loan);
        // self.total_num_loans = u128::wrapping_add(self.total_num_loans, 1);

        // self.process_purchase(loan);
        // // Transfer collateral from borrower to this contract to be held until
        // // loan completion.
        // IERC721(loan.nftCollateralContract).transferFrom(
        //     msg.sender,
        //     address(this),
        //     loan.nftCollateralId,
        // );

        // // Transfer principal from lender to borrower.
        // IERC20(loan.loanCurrency).transferFrom(_lender, msg.sender, loan.loanPrincipalAmount);

        // Issue an ERC721 promissory note to the lender that gives them the
        // right to either the principal-plus-interest or the collateral.
        // _mint(_lender, loan.loanId);
    }

    #[payable]
    pub fn liquidate_overdue_loan(&mut self, _loan_id: u128) {}

    #[payable]
    pub fn pay_back_loan(&mut self, _loan_id: u128) {}

    #[payable]
    pub fn offer_now(
        &mut self,
        _loan_principal_amount: u128,
        _nft_collateral_id: u128,
        _loan_duration: u128,
        _loan_interest_rate: u128,
        _admin_fee: u128,
        _nft_collateral_contract: AccountId,
        _loan_currency: AccountId,
        _lender: AccountId,
    ) {
    }

    // #[payable]
    // pub fn transfer_nft(
    //     &mut self,
    //     _nft_collateral_id: String,
    //     _nft_collateral_contract: AccountId,
    // ) {
    //     ext_contract::nft_transfer(
    //         loan.lender.clone(),
    //         _nft_collateral_id,
    //         loan.approval_id,
    //         "payout from market".to_string(),
    //         &loan.nft_collateral_contract,
    //         1,
    //         GAS_FOR_NFT_TRANSFER,
    //     );
    // }

    #[private]
    pub fn process_purchase(&mut self, loan: Loan) -> Promise {
        ext_contract::nft_transfer(
            loan.lender.clone(),
            loan.nft_collateral_id,
            loan.approval_id,
            "payout from market".to_string(),
            &loan.nft_collateral_contract,
            1,
            GAS_FOR_NFT_TRANSFER,
        )
        .then(ext_self::resolve_purchase(
            loan.loan_currency,
            loan.lender,
            U128(0),
            &env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_ROYALTIES,
        ))
    }
}
#[ext_contract(ext_self)]
trait ExtSelf {
    fn resolve_purchase(
        &mut self,
        ft_token_id: AccountId,
        buyer_id: AccountId,
        price: U128,
    ) -> Promise;
}
