use crate::*;
use near_sdk::promise_result_as_success;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Offer {
    pub offer_id: u32,
    pub lender_id: AccountId,
    pub loan_principal_amount: u128,
    pub loan_duration: u128,
    pub loan_interest_rate: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Sale {
    pub owner_id: AccountId,
    pub approval_id: u64,
    pub nft_contract_id: String,
    pub token_id: String,
    pub loan_principal_amount: u128,
    pub loan_duration: u128,
    pub loan_currency: TokenId,
    pub loan_interest_rate: u32,
    pub status: u32,
    pub lender: AccountId,
    pub created_at: U64,
    pub offers: Offers,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PurchaseArgs {
    pub nft_contract_id: ValidAccountId,
    pub token_id: TokenId,
    pub action: String,
}

#[near_bindgen]
impl Contract {
    /// for add sale see: nft_callbacks.rs

    #[payable]
    pub fn order_now(&mut self, nft_contract_id: ValidAccountId, token_id: String) {
        let contract_id: AccountId = nft_contract_id.into();
        let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id);
        let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");
        let lender_id = env::predecessor_account_id();
        assert_ne!(sale.owner_id, lender_id, "Cannot bid on your own sale.");
        let ft_token_id = "near".to_string();
        // let price = sale
        //     .sale_conditions
        //     .get(&ft_token_id)
        //     .expect("Not for sale in NEAR")
        //     .0;

        let deposit = env::attached_deposit();
        assert!(deposit > 0, "Attached deposit must be greater than 0");

        self.process_purchase(
            contract_id,
            token_id,
            ft_token_id,
            U128(deposit),
            sale.approval_id,
            sale.owner_id,
            lender_id,
        );
    }

    #[payable]
    pub fn liquidate_overdue_loan(&mut self, nft_contract_id: ValidAccountId, token_id: String) {
        let contract_id: AccountId = nft_contract_id.into();
        let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id);
        let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");
        let lender_id = env::predecessor_account_id();

        self.process_liquidate_loan(contract_id, token_id, sale.lender);
    }

    pub fn accept_offer(
        &mut self,
        nft_contract_id: ValidAccountId,
        token_id: String,
        ft_token_id: ValidAccountId,
    ) {
        // let contract_id: AccountId = nft_contract_id.into();
        // let contract_and_token_id =
        //     format!("{}{}{}", contract_id.clone(), DELIMETER, token_id.clone());
        // // remove bid before proceeding to process purchase
        // let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");
        // let bids_for_token_id = sale.bids.remove(ft_token_id.as_ref()).expect("No bids");
        // let bid = &bids_for_token_id[bids_for_token_id.len() - 1];
        // self.sales.insert(&contract_and_token_id, &sale);
        // // panics at `self.internal_remove_sale` and reverts above if predecessor is not sale.owner_id
        // self.process_purchase(
        //     contract_id,
        //     token_id,
        //     ft_token_id.into(),
        //     bid.price,
        //     bid.owner_id.clone(),
        // );
    }

    #[private]
    pub fn process_purchase(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        ft_token_id: AccountId,
        amount: U128,
        approval_id: u64,
        borrower_id: AccountId,
        lender_id: AccountId,
    ) -> Promise {
        // let sale = self.internal_remove_sale(nft_contract_id.clone(), token_id.clone());

        ext_contract::nft_transfer(
            env::current_account_id(),
            token_id,
            approval_id,
            "payout from market".to_string(),
            &nft_contract_id,
            1,
            GAS_FOR_NFT_TRANSFER,
        )
        .then(ext_self::resolve_purchase(
            ft_token_id,
            borrower_id,
            amount,
            &env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_ROYALTIES,
        ))
    }

    pub fn process_payback_loan(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        ft_token_id: AccountId,
        amount: U128,
        approval_id: u64,
        borrower_id: AccountId,
        lender_id: AccountId,
    ) -> Promise {
        // let sale = self.internal_remove_sale(nft_contract_id.clone(), token_id.clone());

        ext_contract::ft_transfer(
            lender_id,
            amount,
            Some("pay_back_loan".to_string()),
            &ft_token_id,
            1,
            GAS_FOR_FT_TRANSFER,
        )
        .then(ext_contract::nft_transfer(
            borrower_id,
            token_id,
            0,
            "payout from market".to_string(),
            &nft_contract_id,
            1,
            GAS_FOR_NFT_TRANSFER,
        ))
    }

    pub fn process_liquidate_loan(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        lender_id: AccountId,
    ) -> Promise {
        ext_contract::nft_transfer(
            lender_id,
            token_id,
            0,
            "liquidate loan".to_string(),
            &nft_contract_id,
            1,
            GAS_FOR_NFT_TRANSFER,
        )
    }

    /// self callback

    #[private]
    pub fn resolve_purchase(
        &mut self,
        ft_token_id: AccountId,
        borrower_id: AccountId,
        price: U128,
    ) -> U128 {
        ext_contract::ft_transfer(
            borrower_id,
            price,
            None,
            &ft_token_id,
            1,
            GAS_FOR_FT_TRANSFER,
        );
        U128(0)
    }
}

/// self call

#[ext_contract(ext_self)]
trait ExtSelf {
    fn resolve_purchase(
        &mut self,
        ft_token_id: AccountId,
        borrower_id: AccountId,
        price: U128,
    ) -> Promise;
}
