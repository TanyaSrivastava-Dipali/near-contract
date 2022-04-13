use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupSet;
use near_sdk::{env, near_bindgen, AccountId};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct WhitelistContract {
    /// The account ID of the NEAR Foundation. It allows to whitelist new staking pool accounts.
    /// It also allows to whitelist new Staking Pool Factories, which can whitelist staking pools.
    pub foundation_account_id: AccountId,

    /// The whitelisted account IDs of approved staking pool contracts.
    pub whitelist: LookupSet<AccountId>,

    /// The whitelist of staking pool factories. Any account from this list can whitelist staking
    /// pools.
    pub factory_whitelist: LookupSet<AccountId>,
}

impl Default for WhitelistContract {
    fn default() -> Self {
        env::panic(b"The contract should be initialized before usage")
    }
}

#[near_bindgen]
impl WhitelistContract {
    /// Initializes the contract with the given NEAR foundation account ID.
    #[init]
    pub fn new(foundation_account_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert!(
            env::is_valid_account_id(foundation_account_id.as_bytes()),
            "The NEAR Foundation account ID is invalid"
        );
        Self {
            foundation_account_id,
            whitelist: LookupSet::new(b"w".to_vec()),
            factory_whitelist: LookupSet::new(b"f".to_vec()),
        }
    }

    /***********/
    /* Getters */
    /***********/

    /// Returns `true` if the given staking pool account ID is whitelisted.
    pub fn is_whitelisted(&self, staking_pool_account_id: AccountId) -> bool {
        assert!(
            env::is_valid_account_id(staking_pool_account_id.as_bytes()),
            "The given account ID is invalid"
        );
        self.whitelist.contains(&staking_pool_account_id)
    }

    /// Returns `true` if the given factory contract account ID is whitelisted.
    pub fn is_factory_whitelisted(&self, factory_account_id: AccountId) -> bool {
        assert!(
            env::is_valid_account_id(factory_account_id.as_bytes()),
            "The given account ID is invalid"
        );
        self.factory_whitelist.contains(&factory_account_id)
    }

    /************************/
    /* Factory + Foundation */
    /************************/

    /// Adds the given staking pool account ID to the whitelist.
    /// Returns `true` if the staking pool was not in the whitelist before, `false` otherwise.
    /// This method can be called either by the NEAR foundation or by a whitelisted factory.
    pub fn add_staking_pool(&mut self, staking_pool_account_id: AccountId) -> bool {
        assert!(
            env::is_valid_account_id(staking_pool_account_id.as_bytes()),
            "The given account ID is invalid"
        );
        // Can only be called by a whitelisted factory or by the foundation.
        if !self
            .factory_whitelist
            .contains(&env::predecessor_account_id())
        {
            self.assert_called_by_foundation();
        }
        self.whitelist.insert(&staking_pool_account_id)
    }

    /**************/
    /* Foundation */
    /**************/

    /// Removes the given staking pool account ID from the whitelist.
    /// Returns `true` if the staking pool was present in the whitelist before, `false` otherwise.
    /// This method can only be called by the NEAR foundation.
    pub fn remove_staking_pool(&mut self, staking_pool_account_id: AccountId) -> bool {
        self.assert_called_by_foundation();
        assert!(
            env::is_valid_account_id(staking_pool_account_id.as_bytes()),
            "The given account ID is invalid"
        );
        self.whitelist.remove(&staking_pool_account_id)
    }

    /// Adds the given staking pool factory contract account ID to the factory whitelist.
    /// Returns `true` if the factory was not in the whitelist before, `false` otherwise.
    /// This method can only be called by the NEAR foundation.
    pub fn add_factory(&mut self, factory_account_id: AccountId) -> bool {
        assert!(
            env::is_valid_account_id(factory_account_id.as_bytes()),
            "The given account ID is invalid"
        );
        self.assert_called_by_foundation();
        self.factory_whitelist.insert(&factory_account_id)
    }

    /// Removes the given staking pool factory account ID from the factory whitelist.
    /// Returns `true` if the factory was present in the whitelist before, `false` otherwise.
    /// This method can only be called by the NEAR foundation.
    pub fn remove_factory(&mut self, factory_account_id: AccountId) -> bool {
        self.assert_called_by_foundation();
        assert!(
            env::is_valid_account_id(factory_account_id.as_bytes()),
            "The given account ID is invalid"
        );
        self.factory_whitelist.remove(&factory_account_id)
    }

    /************/
    /* Internal */
    /************/

    /// Internal method to verify the predecessor was the NEAR Foundation account ID.
    fn assert_called_by_foundation(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.foundation_account_id,
            "Can only be called by NEAR Foundation"
        );
    }
}



  
