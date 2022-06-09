use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base64VecU8, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Balance};

/// Hash of Vesting schedule.
pub type Hash = Vec<u8>;

/// Contains information about vesting schedule.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct VestingSchedule {
    pub timestamp: U64,
    pub amount: U128,
}

impl VestingSchedule {
    pub fn assert_valid(&self) {
        assert!(
            self.timestamp.0 <= 0,
            "Cliff timestamp can't be earlier than vesting start timestamp"
        );
        assert!(
            self.amount.0 <= 0,
            "Cliff timestamp can't be later than vesting end timestamp"
        );
    }
}
