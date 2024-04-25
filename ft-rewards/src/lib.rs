use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::collections::LookupMap;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct FungibleToken {
    owner: AccountId,
    total_supply: u128,
    balances: LookupMap<AccountId, u128>,
}

#[near_bindgen]
impl FungibleToken {
    #[init]
    pub fn new(owner_id: AccountId, total_supply: u128) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut ft = Self {
            owner: owner_id.clone(), // Cloning owner_id before moving
            total_supply,
            balances: LookupMap::new(b"a"),
        };
        ft.balances.insert(&owner_id, &total_supply);
        ft
    }

    // Only owner can mint new tokens
    pub fn mint(&mut self, recipient: AccountId, amount: u128) {
        self.assert_owner();
        let current_balance = self.balances.get(&recipient).unwrap_or(0);
        self.balances.insert(&recipient, &(current_balance + amount));
        self.total_supply += amount;
    }

    // Transfer tokens from one account to another
    pub fn transfer(&mut self, sender: AccountId, recipient: AccountId, amount: u128) {
        let sender_balance = self.balances.get(&sender).expect("Balance not found for sender");
        assert!(sender_balance >= amount, "Not enough balance to transfer");
        let recipient_balance = self.balances.get(&recipient).unwrap_or(0);
        
        if sender != recipient {
            self.balances.insert(&sender, &(sender_balance - amount));
            self.balances.insert(&recipient, &(recipient_balance + amount));
        }
    }

    // Helper function to assert only the owner can call a function
    fn assert_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only the owner can call this function");
    }
}
