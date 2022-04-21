use crate::*;
use near_sdk::promise_result_as_success;

//status : //0 : pending, 1: processing , 2 : done, 3: liquidated, 4: refunded
pub enum LoanStatus {
    // An `enum` may either be `unit-like`,
    Open = 0,
    Processing = 1,
    Done = 2,
    Liquidated = 3,
    Refunded = 4,
    Canceled = 5,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Offer {
    pub offer_id: u32,
    pub lender_id: AccountId,
    pub loan_principal_amount: u128,
    pub loan_duration: u128,
    pub loan_interest_rate: u32,
    pub status: u32,
    pub created_at: U64,
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
    pub loan_principal_amount: U128,
    pub loan_duration: u128,
    pub loan_interest_rate: u32,
}

#[near_bindgen]
impl Contract {
    /// for add sale see: nft_callbacks.rs

    #[payable]
    pub fn cancel_loan(&mut self, nft_contract_id: ValidAccountId, token_id: String) {
        let contract_id: AccountId = nft_contract_id.into();
        let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id);
        let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");
        sale.status = LoanStatus::Canceled as u32;
        self.sales.insert(&contract_and_token_id, &sale);
        //TODO : refund open offer
    }
    #[payable]
    pub fn cancel_offer(&mut self, nft_contract_id: AccountId, token_id: String, offer_id: u32) {
        let contract_id: AccountId = nft_contract_id.into();
        let contract_and_token_id =
            format!("{}{}{}", contract_id.clone(), DELIMETER, token_id.clone());
        // remove bid before proceeding to process purchase
        let sale = self.sales.get(&contract_and_token_id).expect("No sale");
        // self.refund_unmatch_offer(sale, offer_id);
        let mut offers: Vec<Offer> = Vec::new();
        for v in sale.offers {
            if v.offer_id == offer_id {
                ext_contract::ft_transfer(
                    v.lender_id.clone(),
                    U128(v.loan_principal_amount),
                    Some(String::from("refund from market")),
                    &sale.loan_currency,
                    1,
                    GAS_FOR_FT_TRANSFER,
                );
                let mut clone = self.clone_offer(v);
                clone.status = LoanStatus::Refunded as u32;
                offers.push(clone);
            } else {
                let clone = self.clone_offer(v);
                offers.push(clone);
            }
        }
        self.sales.insert(
            &contract_and_token_id,
            &Sale {
                owner_id: sale.owner_id.clone().into(),
                approval_id: sale.approval_id,
                nft_contract_id: sale.nft_contract_id.clone(),
                token_id: token_id.clone(),
                loan_principal_amount: sale.loan_principal_amount,
                loan_duration: sale.loan_duration,
                loan_currency: sale.loan_currency,
                loan_interest_rate: sale.loan_interest_rate,
                created_at: U64(env::block_timestamp() / 1000000),
                status: sale.status,
                lender: sale.lender.to_string(),
                offers: offers,
            },
        );
    }

    #[payable]
    pub fn liquidate_overdue_loan(&mut self, nft_contract_id: ValidAccountId, token_id: String) {
        let contract_id: AccountId = nft_contract_id.into();
        let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id);
        let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");
        sale.status = LoanStatus::Liquidated as u32;
        self.sales.insert(&contract_and_token_id, &sale);
        self.process_liquidate_loan(contract_id, token_id, sale.lender);
    }

    #[payable]
    pub fn accept_offer(&mut self, nft_contract_id: AccountId, token_id: String, offer_id: u32) {
        let contract_id: AccountId = nft_contract_id.into();
        let contract_and_token_id =
            format!("{}{}{}", contract_id.clone(), DELIMETER, token_id.clone());
        // remove bid before proceeding to process purchase
        let sale = self.sales.get(&contract_and_token_id).expect("No sale");
        // let mut sale_clone = self.clone_sale(sale);
        // self.refund_unmatch_offer(sale, offer_id);
        let mut offers: Vec<Offer> = Vec::new();
        for v in sale.offers {
            if v.offer_id == offer_id {
                self.process_purchase(
                    contract_id.clone(),
                    token_id.clone(),
                    sale.loan_currency.clone(),
                    U128(v.loan_principal_amount),
                    sale.approval_id,
                    sale.owner_id.clone(),
                    sale.lender.clone(),
                );
                let mut clone = self.clone_offer(v);
                clone.status = LoanStatus::Processing as u32;
                offers.push(clone);
            } else {
                ext_contract::ft_transfer(
                    v.lender_id.clone(),
                    U128(v.loan_principal_amount),
                    Some(String::from("refund from market")),
                    &sale.loan_currency,
                    1,
                    GAS_FOR_FT_TRANSFER,
                );
                let mut clone = self.clone_offer(v);
                clone.status = LoanStatus::Refunded as u32;
                offers.push(clone);
            }
        }
        // sale_clone.status = LoanStatus::Processing as u32;
        // sale_clone.offers = offers;
        self.sales.insert(
            &contract_and_token_id,
            &Sale {
                owner_id: sale.owner_id.clone().into(),
                approval_id: sale.approval_id,
                nft_contract_id: sale.nft_contract_id.clone(),
                token_id: token_id.clone(),
                loan_principal_amount: sale.loan_principal_amount,
                loan_duration: sale.loan_duration,
                loan_currency: sale.loan_currency,
                loan_interest_rate: sale.loan_interest_rate,
                created_at: sale.created_at,
                status: LoanStatus::Processing as u32,
                lender: sale.lender.to_string(),
                offers: offers,
            },
        );
    }

    #[private]
    pub fn clone_offer(&mut self, v: Offer) -> Offer {
        let clone = Offer {
            offer_id: v.offer_id,
            lender_id: v.lender_id,
            loan_principal_amount: v.loan_principal_amount,
            loan_duration: v.loan_duration,
            loan_interest_rate: v.loan_interest_rate,
            created_at: v.created_at,
            status: v.status,
        };
        clone
    }

    pub fn clone_sale(&mut self, sale: Sale) -> Sale {
        let clone = Sale {
            owner_id: sale.owner_id,
            approval_id: sale.approval_id,
            nft_contract_id: sale.nft_contract_id,
            token_id: sale.token_id,
            loan_principal_amount: sale.loan_principal_amount,
            loan_duration: sale.loan_duration,
            loan_currency: sale.loan_currency,
            loan_interest_rate: sale.loan_interest_rate,
            created_at: sale.created_at,
            status: sale.status,
            lender: sale.lender,
            offers: sale.offers,
        };
        clone
    }

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

    #[private]
    pub fn resolve_offer(&mut self) -> U128 {
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
