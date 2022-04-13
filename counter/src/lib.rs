use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, log};
use serde::Serialize;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize,Clone,Serialize)]
pub struct Counter {
    value: u64,
}

impl Default for Counter {
    fn default() -> Self {
        // Check incase the contract is not initialized
        env::panic(b"The contract is not initialized.")
    }
}

#[near_bindgen]
impl Counter {

    #[init]
    pub fn new(value: u64) -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        log!("Custom counter initialization!");
        Self { value }
    }
}

#[near_bindgen]
impl Counter {
    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn decrement(&mut self) {
        self.value -= 1;
    }

    pub fn get_count(&self) -> Self {
        self.clone()
    }

}