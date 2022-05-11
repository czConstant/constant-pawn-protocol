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
    pub loan_duration: u32,
    pub loan_interest_rate: u32,
    pub available_at: u64,
    pub status: u32,
    pub created_at: U64,
    pub updated_at: U64,
    pub started_at: U64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Sale {
    pub owner_id: AccountId,
    pub approval_id: u64,
    pub nft_contract_id: String,
    pub token_id: String,
    pub loan_principal_amount: u128,
    pub loan_duration: u32,
    pub loan_currency: TokenId,
    pub loan_interest_rate: u32,
    pub loan_config: u32,
    pub available_at: u64,
    pub status: u32,
    pub lender: AccountId,
    pub created_at: U64,
    pub updated_at: U64,
    pub offers: Offers,
    pub started_at: U64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PurchaseArgs {
    pub nft_contract_id: ValidAccountId,
    pub token_id: TokenId,
    pub action: String,
    pub loan_principal_amount: U128,
    pub loan_duration: u32,
    pub loan_interest_rate: u32,
    pub available_at: u64,
}

#[near_bindgen]
impl Contract {
    /// for add sale see: nft_callbacks.rs
    #[payable]
    pub fn cancel_loan(&mut self, nft_contract_id: ValidAccountId, token_id: String) {
        let contract_id: AccountId = nft_contract_id.into();
        let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id);
        let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");
        assert!(
            sale.owner_id == env::predecessor_account_id(),
            "invalid owner owner's loan:{}, signer:{}",
            sale.owner_id,
            env::predecessor_account_id(),
        );
        assert!(
            sale.status == LoanStatus::Open as u32,
            "invalid loan status",
        );
        if sale.status == LoanStatus::Open as u32 {
            sale.status = LoanStatus::Canceled as u32;
            let mut offers: Vec<Offer> = Vec::new();
            for v in sale.offers {
                if v.status == LoanStatus::Open as u32 {
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
                    clone.updated_at = U64(env::block_timestamp() / 1000000000);
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
                    loan_config: sale.loan_config,
                    available_at: sale.available_at,
                    created_at: sale.created_at,
                    updated_at: U64(env::block_timestamp() / 1000000000),
                    started_at: sale.started_at,
                    status: LoanStatus::Canceled as u32,
                    lender: sale.lender.to_string(),
                    offers: offers,
                },
            );
        }
    }
    #[payable]
    pub fn cancel_offer(&mut self, nft_contract_id: AccountId, token_id: String, offer_id: u32) {
        let contract_id: AccountId = nft_contract_id.into();
        let contract_and_token_id =
            format!("{}{}{}", contract_id.clone(), DELIMETER, token_id.clone());
        let sale = self.sales.get(&contract_and_token_id).expect("No sale");
        let mut offers: Vec<Offer> = Vec::new();
        for v in sale.offers {
            if v.offer_id == offer_id {
                assert!(
                    v.status == LoanStatus::Open as u32,
                    "Unable cancel 'not open' offer "
                );
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
                clone.updated_at = U64(env::block_timestamp() / 1000000000);
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
                loan_config: sale.loan_config,
                available_at: sale.available_at,
                created_at: sale.created_at,
                updated_at: U64(env::block_timestamp() / 1000000000),
                started_at: sale.started_at,
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
        let now: u128 = (env::block_timestamp() / 1000000000) as u128;
        let expired = sale.started_at.0 as u128 + sale.loan_duration as u128;
        assert!(expired < now, "invalid time to liquidate loan");
        if sale.status == LoanStatus::Processing as u32 {
            sale.status = LoanStatus::Liquidated as u32;
            sale.updated_at = U64(env::block_timestamp() / 1000000000);
            self.process_liquidate_loan(contract_id, token_id, sale.lender.clone());
            let mut offers: Vec<Offer> = Vec::new();
            for v in sale.offers {
                if v.status == LoanStatus::Processing as u32 {
                    let mut clone = self.clone_offer(v);
                    clone.status = LoanStatus::Liquidated as u32;
                    clone.updated_at = U64(env::block_timestamp() / 1000000000);
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
                    token_id: sale.token_id.clone(),
                    loan_principal_amount: sale.loan_principal_amount,
                    loan_duration: sale.loan_duration,
                    loan_currency: sale.loan_currency,
                    loan_interest_rate: sale.loan_interest_rate,
                    loan_config: sale.loan_config,
                    available_at: sale.available_at,
                    created_at: sale.created_at,
                    updated_at: U64(env::block_timestamp() / 1000000000),
                    started_at: sale.started_at,
                    status: sale.status,
                    lender: sale.lender.clone(),
                    offers: offers,
                },
            );
        }
    }

    #[payable]
    pub fn accept_offer(&mut self, nft_contract_id: AccountId, token_id: String, offer_id: u32) {
        let contract_id: AccountId = nft_contract_id.into();
        let contract_and_token_id =
            format!("{}{}{}", contract_id.clone(), DELIMETER, token_id.clone());
        let sale = self.sales.get(&contract_and_token_id).expect("No sale");
        assert!(
            sale.status == LoanStatus::Open as u32,
            "Loan is in proccessing "
        );
        let mut offers: Vec<Offer> = Vec::new();
        let mut updated_sale = Sale {
            owner_id: sale.owner_id.clone().into(),
            approval_id: sale.approval_id,
            nft_contract_id: sale.nft_contract_id.clone(),
            token_id: token_id.clone(),
            loan_principal_amount: sale.loan_principal_amount,
            loan_duration: sale.loan_duration,
            loan_currency: sale.loan_currency,
            loan_interest_rate: sale.loan_interest_rate,
            loan_config: sale.loan_config,
            available_at: sale.available_at,
            created_at: sale.created_at,
            started_at: U64(env::block_timestamp() / 1000000000),
            updated_at: U64(env::block_timestamp() / 1000000000),
            status: LoanStatus::Processing as u32,
            lender: sale.lender.to_string(),
            offers: Vec::new(),
        };
        for v in sale.offers {
            let mut clone = self.clone_offer(v);
            clone.updated_at = U64(env::block_timestamp() / 1000000000);
            if clone.offer_id == offer_id && clone.status == LoanStatus::Open as u32 {
                self.process_purchase(
                    contract_id.clone(),
                    token_id.clone(),
                    updated_sale.loan_currency.clone(),
                    U128(clone.loan_principal_amount),
                    sale.approval_id,
                    sale.owner_id.clone(),
                    sale.lender.clone(),
                );
                clone.status = LoanStatus::Processing as u32;
                clone.started_at = U64(env::block_timestamp() / 1000000000);
                updated_sale.loan_duration = clone.loan_duration;
                updated_sale.loan_principal_amount = clone.loan_principal_amount;
                updated_sale.loan_interest_rate = clone.loan_interest_rate;
            } else {
                if clone.status == LoanStatus::Open as u32 {
                    ext_contract::ft_transfer(
                        clone.lender_id.clone(),
                        U128(clone.loan_principal_amount.clone()),
                        Some(String::from("refund from market")),
                        &updated_sale.loan_currency,
                        1,
                        GAS_FOR_FT_TRANSFER,
                    );
                    clone.status = LoanStatus::Refunded as u32;
                }
            }
            offers.push(clone);
        }
        updated_sale.offers = offers;
        self.sales.insert(&contract_and_token_id, &updated_sale);
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
            updated_at: v.updated_at,
            started_at: v.started_at,
            status: v.status,
            available_at: v.available_at,
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
        let contract_id: AccountId = nft_contract_id.clone();
        let contract_and_token_id =
            format!("{}{}{}", contract_id.clone(), DELIMETER, token_id.clone());
        let sale = self.sales.get(&contract_and_token_id).expect("No sale");
        let now: u128 = (env::block_timestamp() / 1000000000) as u128;
        let expired = sale.started_at.0 as u128 + sale.loan_duration as u128;
        assert!(expired > now, "invalid time to pay back loan ");
        let real_pay_amount = self.calculate_pay_amount(
            sale.loan_principal_amount,
            sale.loan_duration,
            sale.loan_interest_rate,
            sale.started_at.0 as u128,
            (env::block_timestamp() / 1000000000) as u128,
        );
        assert!(
            real_pay_amount == amount.0,
            "invalid payment amount, pay_amount:{}, input_amount{}",
            real_pay_amount,
            amount.0
        );
        let mut offers: Vec<Offer> = Vec::new();
        for v in sale.offers {
            if v.status == LoanStatus::Processing as u32 {
                let mut clone = self.clone_offer(v);
                clone.status = LoanStatus::Done as u32;
                clone.updated_at = U64(env::block_timestamp() / 1000000000);
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
                token_id: sale.token_id.clone(),
                loan_principal_amount: sale.loan_principal_amount,
                loan_duration: sale.loan_duration,
                loan_currency: sale.loan_currency,
                loan_interest_rate: sale.loan_interest_rate,
                loan_config: sale.loan_config,
                available_at: sale.available_at,
                created_at: sale.created_at,
                updated_at: U64(env::block_timestamp() / 1000000000),
                started_at: sale.started_at,
                status: sale.status,
                lender: sale.lender.clone(),
                offers: offers,
            },
        );
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

    pub fn calculate_fee(&mut self, loan_principal_amount: u128) -> u128 {
        let fee = loan_principal_amount * 1 / 100;
        fee
    }
    pub fn calculate_pay_amount(
        &mut self,
        loan_principal_amount: u128,
        loan_duration: u32,
        interest_rate: u32,
        loan_started_at: u128,
        pay_at: u128,
    ) -> u128 {
        const DAY_SECS: u128 = 86400;
        //1%(principla) + 100% interest to pay_at + 50% interest for the rest
        let mut max_loan_day: u128 = loan_duration as u128 / DAY_SECS;
        if max_loan_day == 0 {
            max_loan_day = 1;
        }
        let mut loan_day: u128 = max_loan_day;
        if pay_at < loan_started_at + loan_duration as u128 && pay_at > loan_started_at {
            loan_day = ((pay_at - loan_started_at) / DAY_SECS) + 1;
        }
        if loan_day >= max_loan_day {
            loan_day = max_loan_day
        }
        //100% interest loan day
        let mut full_interst =
            ((loan_principal_amount * (interest_rate as u128) / 10000) * loan_day) / 365;
        if max_loan_day > loan_day {
            //50% interest remain day
            full_interst = full_interst
                + (((loan_principal_amount * (interest_rate as u128) / 10000)
                    * (max_loan_day - loan_day))
                    / 365)
                    / 2;
        }
        //1% fee (base on principal amount)
        let fee = self.calculate_fee(loan_principal_amount);
        fee + full_interst + loan_principal_amount
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
