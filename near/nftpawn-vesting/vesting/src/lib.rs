/*!
Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::json_types::U64;
use near_sdk::serde_json;
use near_sdk::{
    env, near_bindgen, require, AccountId, Gas, PanicOnDefault, PromiseOrValue, PromiseResult,
};

pub use crate::types::*;

mod ft_callbacks;
mod types;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub recipient_id: AccountId,
    pub token_id: AccountId,
    pub vesting_schedules: Vec<VestingSchedule>,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(token_id: AccountId, recipient_id: AccountId) -> Self {
        require!(!env::state_exists(), "Already initialized");
        let this = Self {
            owner_id: env::predecessor_account_id(),
            recipient_id: recipient_id.clone(),
            token_id: token_id,
            vesting_schedules: Vec::new(),
        };
        this
    }

    pub fn get_vesting_schedules(&self) -> Vec<VestingSchedule> {
        let mut rets: Vec<VestingSchedule> = Vec::new();
        for s in self.vesting_schedules.iter() {
            let item = VestingSchedule {
                timestamp: s.timestamp,
                amount: s.amount,
            };
            rets.push(item);
        }
        rets
    }

    pub fn get_release_available(&self) -> U128 {
        let mut amount = 0 as u128;
        for s in self.vesting_schedules.iter() {
            if s.timestamp <= U64(env::block_timestamp() / 1000000000) {
                amount = amount + s.amount.0 as u128
            }
        }
        U128(amount)
    }

    pub fn release(&mut self) {
        let mut amount = 0 as u128;
        let mut rets: Vec<VestingSchedule> = Vec::new();
        for s in self.vesting_schedules.iter() {
            let mut new_amount = 0 as u128;
            if s.timestamp <= U64(env::block_timestamp() / 1000000000) {
                amount = amount + s.amount.0 as u128
            } else {
                new_amount = s.amount.0 as u128
            }
            let item = VestingSchedule {
                timestamp: s.timestamp,
                amount: U128(new_amount),
            };
            rets.push(item);
        }
        self.vesting_schedules = rets;
        env::promise_create(
            self.token_id.clone(),
            "ft_transfer",
            &serde_json::to_vec(&(
                self.recipient_id.clone(),
                amount.to_string(),
                Some("release"),
            ))
            .unwrap(),
            1,
            Gas(5_000_000_000_000),
        );
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, Balance};

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }
}
