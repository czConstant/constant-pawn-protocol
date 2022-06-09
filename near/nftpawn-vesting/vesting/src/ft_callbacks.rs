use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;

/// callbacks from FT Contracts

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        require!(
            env::predecessor_account_id() == self.token_id,
            "fungible token contract is not registerd"
        );
        require!(
            self.vesting_schedules.len() == 0,
            "vesting_schedules len is not zero"
        );
        let vesting_schedules: Vec<VestingSchedule> =
            near_sdk::serde_json::from_str(&msg).expect("Invalid VestingSchedule");
        require!(
            vesting_schedules.len() > 0,
            "vesting_schedules param len is zero",
        );
        let mut total = 0 as u128;
        let mut last_t = U64(0);
        for s in vesting_schedules.iter() {
            s.assert_valid();
            if last_t > U64(0) {
                require!(s.timestamp > last_t, "timestamp is larger than last_t");
            }
            last_t = s.timestamp;
            total = total + s.amount.0 as u128;
        }
        require!(total == amount.0 as u128, "amount is not equal total");
        self.vesting_schedules = vesting_schedules;
        PromiseOrValue::Value(U128(0))
    }
}
