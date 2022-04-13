// use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::{env, log, metadata, near_bindgen, AccountId};

// use std::collections::HashMap;

// metadata! {
// #[near_bindgen]
// #[derive(Default, BorshDeserialize, BorshSerialize)]
// pub struct StatusMessage {
//     records: HashMap<AccountId, String>,
// }

// #[near_bindgen]
// impl StatusMessage {
//     #[payable]
//     pub fn set_status(&mut self, message: String) {
//         let account_id = env::signer_account_id();
//         log!("{} set_status with message {}", account_id, message);
//         self.records.insert(account_id, message);
//     }

//     pub fn get_status(&self, account_id: AccountId) -> Option::<String> {
//         log!("get_status for account_id {}", account_id);
//         self.records.get(&account_id).cloned()
//     }
// }
 
// }
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::{env, near_bindgen, BorshStorageKey, AccountId};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Records,
    UniqueValues,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StatusMessage {
    pub records: LookupMap<AccountId, String>,
    pub unique_values: LookupSet<String>,
}

impl Default for StatusMessage {
    fn default() -> Self {
        Self {
            records: LookupMap::new(StorageKey::Records),
            unique_values: LookupSet::new(StorageKey::UniqueValues),
        }
    }
}

#[near_bindgen]
impl StatusMessage {
    /// Returns true if the message is unique
    pub fn set_status(&mut self, message: String) -> bool {
        let account_id = env::signer_account_id();
        if( self.unique_values.insert(&message))
        {
        self.records.insert(&account_id, &message);
        true
        }
        else{
        false
        }
      
    }

    pub fn get_status(&self, account_id: AccountId) -> Option<String> {
        self.records.get(&account_id)
    }
}