use crate::*;

/// callbacks from FT Contracts

trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // PromiseOrValue::Value(U128(0))

        let PurchaseArgs {
            nft_contract_id,
            token_id,
            action,
        } = near_sdk::serde_json::from_str(&msg).expect("Invalid PurchaseArgs");

        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        let mut sale = self
            .sales
            .get(&contract_and_token_id)
            .expect("No sale in ft_on_transfer");

        assert_ne!(sale.owner_id, sender_id, "Cannot buy your own sale.");

        let ft_token_id = env::predecessor_account_id();

        assert!(amount.0 > 0, "Amount must be greater than 0");
        if action == "offer_now" {
            let log_message = format!(
                "Principle amount {}, real amount {}",
                sale.loan_principal_amount, amount.0
            );
            env::log(log_message.as_bytes());
            assert!(
                amount.0 == sale.loan_principal_amount,
                "Amount must equals loan principal amount ",
            );
            sale.lender = sender_id;
            self.sales.insert(&contract_and_token_id, &sale);
            self.process_purchase(
                nft_contract_id.into(),
                token_id,
                ft_token_id,
                amount,
                sale.approval_id,
                sale.owner_id,
                sale.lender,
            )
            .into()
            //
        } else if action == "pay_back_loan" {
            assert!(
                amount.0 > sale.loan_principal_amount,
                "Amount must greater than loan principal amount ",
            );
            self.process_payback_loan(
                nft_contract_id.into(),
                token_id,
                ft_token_id,
                amount,
                sale.approval_id,
                sale.owner_id,
                sale.lender,
            )
            .into()
        } else {
            PromiseOrValue::Value(U128(0))
        }
    }
}
