use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};

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
        require!(self.timestamp.0 > 0, "timestamp can't be zero");
        require!(self.amount.0 > 0, "amount can't be zero");
    }
}
