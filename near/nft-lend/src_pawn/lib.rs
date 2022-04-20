use crate::external::*;
use crate::loan::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::env::STORAGE_PRICE_PER_BYTE;
use near_sdk::json_types::{ValidAccountId, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey,
    CryptoHash, Gas, PanicOnDefault, Promise, PromiseOrValue,
};
use std::cmp::min;
use std::collections::HashMap;
near_sdk::setup_alloc!();
mod external;
mod loan;
// TODO check seller supports storage_deposit at ft_token_id they want to post sale in

const GAS_FOR_FT_TRANSFER: Gas = 5_000_000_000_000;
/// greedy max Tgas for resolve_purchase
const GAS_FOR_ROYALTIES: Gas = 115_000_000_000_000;
const GAS_FOR_NFT_TRANSFER: Gas = 15_000_000_000_000;
const NO_DEPOSIT: Balance = 0;
const STORAGE_PER_SALE: u128 = 1000 * STORAGE_PRICE_PER_BYTE;
static DELIMETER: &str = "||";
pub type TokenId = String;
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NftPawn {
    // @notice A continuously increasing counter that simultaneously allows
    //         every loan to have a unique ID and provides a running count of
    //         how many loans have been started by this contract.
    pub total_num_loans: u128,

    // // @notice A mapping from a loan's identifier to the loan's details,
    // //         represted by the loan struct. To fetch the lender, call
    // //         NFTfi.ownerOf(loanId).
    pub loan_id_to_loan: LookupMap<u128, Loan>,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Loan,
}

#[near_bindgen]
impl NftPawn {
    #[init]
    pub fn new() -> Self {
        let this = Self {
            total_num_loans: 0,
            loan_id_to_loan: LookupMap::new(StorageKey::Loan),
        };

        this
    }

    pub fn get_total_loan(&self) -> u128 {
        return self.total_num_loans;
    }

    pub fn increment(&mut self) {
        // note: adding one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        // e.g. self.val = i8::wrapping_add(self.val, 1);
        // https://doc.rust-lang.org/std/primitive.i8.html#method.wrapping_add
        self.total_num_loans += 1;
        let log_message = format!("Increased number to {}", self.total_num_loans);
        env::log(log_message.as_bytes());
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::serde::export::TryFrom;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    // simple helper function to take a string literal and return a ValidAccountId
    fn to_valid_account(account: &str) -> ValidAccountId {
        ValidAccountId::try_from(account.to_string()).expect("Invalid account")
    }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn increment() {
        // set up the mock context into the testing environment
        let context = get_context(to_valid_account("foo.near"));
        testing_env!(context.build());
        // instantiate a contract variable with the counter at zero
        let mut contract = NftPawn {
            total_num_loans: 0,
            loan_id_to_loan: LookupMap::new(StorageKey::Loan),
        };
        contract.increment();
        println!("Value after increment: {}", contract.get_total_loan());
        // confirm that we received 1 when calling get_num
        assert_eq!(1, contract.get_total_loan());
    }
}
