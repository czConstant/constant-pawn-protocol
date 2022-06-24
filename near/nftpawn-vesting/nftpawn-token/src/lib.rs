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
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{
    env, log, near_bindgen, require, AccountId, Balance, BorshStorageKey, PanicOnDefault,
    PromiseOrValue,
};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U64;
pub use crate::types::*;
mod types;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner : AccountId,
    token: FungibleToken,
    vesting_schedules: LookupMap<AccountId, Vec<VestingSchedule>>,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    FungibleToken,
    Metadata,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    // #[init]
    // pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
    //     Self::new(
    //         owner_id,
    //         total_supply,
    //         FungibleTokenMetadata {
    //             spec: FT_METADATA_SPEC.to_string(),
    //             name: "Example NEAR fungible token".to_string(),
    //             symbol: "EXAMPLE".to_string(),
    //             icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
    //             reference: None,
    //             reference_hash: None,
    //             decimals: 24,
    //         },
    //     )
    // }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(owner_id: AccountId, pub_sale_id: AccountId, private_sale_id: AccountId, seed_id: AccountId, 
        advisor_id: AccountId, community_id: AccountId, core_id: AccountId, staking_id: AccountId,
        total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
        require!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            owner : owner_id.clone(),
            token: FungibleToken::new(StorageKey::FungibleToken),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            vesting_schedules : LookupMap::new(b"v"),
        };
        //public sale vesting
        this.token.internal_register_account(&pub_sale_id);
        let pub_sale_vesting = this.init_vesting_pub_sale();
        this.vesting_schedules.insert(&pub_sale_id, &pub_sale_vesting);
        
        //private sale vesting
        this.token.internal_register_account(&private_sale_id);
        let private_sale_vesting = this.init_vesting_pri_sale();
        this.vesting_schedules.insert(&private_sale_id, &private_sale_vesting);
        
        //seed vesting
        this.token.internal_register_account(&seed_id);
        let seed_vesting = this.init_vesting_seed();
        this.vesting_schedules.insert(&seed_id, &seed_vesting);
        
        //advisor_vesting
        this.token.internal_register_account(&advisor_id);
        let advisor_vesting = this.init_vesting_advisor();
        this.vesting_schedules.insert(&advisor_id, &advisor_vesting);
        
        //community vesting
        this.token.internal_register_account(&community_id);
        let community_vesting = this.init_vesting_community();
        this.vesting_schedules.insert(&community_id, &community_vesting);
        
        //core vesting
        this.token.internal_register_account(&core_id);
        let core_vesting = this.init_vesting_core();
        this.vesting_schedules.insert(&core_id, &core_vesting);
        
        //staking vesting
        this.token.internal_register_account(&staking_id);
        let staking_vesting = this.init_vesting_staking();
        this.vesting_schedules.insert(&staking_id, &staking_vesting);
        
        //mint token owner        
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial tokens supply is minted"),
        }
        .emit();
        this
    }
   
    pub fn get_vesting_schedules(&self, account_id: AccountId) -> Vec<VestingSchedule> {
        match self.vesting_schedules.get(&account_id) {
            Some(value) => {
                value
            },
            None => {
                Vec::new()
            }
        }
    }

    pub fn get_release_available(&self, account_id: AccountId) -> U128 {
        match self.vesting_schedules.get(&account_id) {
            Some(value) => {
                let mut amount = 0 as u128;
                for s in value.iter() {
                    if s.timestamp <= U64(env::block_timestamp() / 1000000000) {
                        amount = amount + s.amount.0 as u128
                    }
                }
                U128(amount)
            },
            None => {
                U128(0)
            }
        }
    }

    pub fn release(&mut self, account_id: AccountId) {
        match self.vesting_schedules.get(&account_id) {
            Some(value) => {
                let mut amount = 0 as u128;
                let mut rets: Vec<VestingSchedule> = Vec::new();
                for s in value.iter() {
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
                self.vesting_schedules.insert(&account_id, &rets);
                if amount > 0{
                    //internal transfer
                    let log_message = format!("Release {:?} token to account {:?}", amount.clone(), account_id);
                    self.token.internal_transfer(&self.owner, &account_id, amount, Some(log_message));
                }
               
            },
            None => {}
        }
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
    #[private]
    pub fn init_vesting_pub_sale(&mut self) -> Vec<VestingSchedule>{
        let mut vesting = Vec::new();
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 6*30*86400),
            amount: U128(22500000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 12*30*86400),
            amount: U128(22500000000000000),
        });
        vesting
    }

    pub fn init_vesting_pri_sale(&mut self) -> Vec<VestingSchedule>{
        let mut vesting = Vec::new();
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 6*30*86400),
            amount: U128(11250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 9*30*86400),
            amount: U128(11250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 12*30*86400),
            amount: U128(11250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 15*30*86400),
            amount: U128(11250000000000000),
        });
        vesting
    }

    pub fn init_vesting_seed(&mut self) -> Vec<VestingSchedule>{
        let mut vesting = Vec::new();
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 12*30*86400),
            amount: U128(11250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 15*30*86400),
            amount: U128(11250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 18*30*86400),
            amount: U128(11250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 21*30*86400),
            amount: U128(11250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 24*30*86400),
            amount: U128(11250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 27*30*86400),
            amount: U128(11250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 30*30*86400),
            amount: U128(11250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 33*30*86400),
            amount: U128(11250000000000000),
        });

        vesting
    }
    pub fn init_vesting_advisor(&mut self) -> Vec<VestingSchedule>{
        let mut vesting = Vec::new();
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 12*30*86400),
            amount: U128(2250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 18*30*86400),
            amount: U128(2250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 24*30*86400),
            amount: U128(2250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 30*30*86400),
            amount: U128(2250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 36*30*86400),
            amount: U128(2250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 42*30*86400),
            amount: U128(2250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 48*30*86400),
            amount: U128(2250000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 54*30*86400),
            amount: U128(2250000000000000),
        });
        vesting
    }
    pub fn init_vesting_core(&mut self) -> Vec<VestingSchedule>{
        let mut vesting = Vec::new();
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 18*30*86400),
            amount: U128(15000000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 21*30*86400),
            amount: U128(15000000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 24*30*86400),
            amount: U128(15000000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 27*30*86400),
            amount: U128(15000000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 30*30*86400),
            amount: U128(15000000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 33*30*86400),
            amount: U128(15000000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 36*30*86400),
            amount: U128(15000000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 39*30*86400),
            amount: U128(15000000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 42*30*86400),
            amount: U128(15000000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 45*30*86400),
            amount: U128(15000000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 48*30*86400),
            amount: U128(15000000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 51*30*86400),
            amount: U128(15000000000000000),
        });
        vesting
    }

    pub fn init_vesting_community(&mut self) -> Vec<VestingSchedule>{
        let mut vesting = Vec::new();
        vesting.push(VestingSchedule {
            timestamp: U64(0),
            amount: U128(43200000000000000),
        });

        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 3*30*86400),
            amount: U128(43200000000000000),
        });

        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 6*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 6*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 12*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 15*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 18*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 21*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 24*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 27*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 30*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 33*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 36*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 39*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 42*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 45*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 48*30*86400),
            amount: U128(21600000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 51*30*86400),
            amount: U128(21600000000000000),
        });

        vesting
    }

    pub fn init_vesting_staking(&mut self) -> Vec<VestingSchedule>{
        let mut vesting = Vec::new();
        vesting.push(VestingSchedule {
            timestamp: U64(0),
            amount: U128(36000000000000000),
        });

        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 12*30*86400),
            amount: U128(27000000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 24*30*86400),
            amount: U128(13500000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 36*30*86400),
            amount: U128(6750000000000000),
        });
        vesting.push(VestingSchedule {
            timestamp: U64((env::block_timestamp() / 1000000000) + 48*30*86400),
            amount: U128(6750000000000000),
        });
        vesting
    }

    
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}