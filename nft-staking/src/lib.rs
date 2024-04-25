use near_sdk::collections::{UnorderedMap};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::json_types::{U128, U64};
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct StakingContract {
    owner: AccountId,
    nft_contract_id: AccountId,
    rewards_contract_id: AccountId,
    staked_nfts: UnorderedMap<AccountId, HashMap<String, U64>>, // Corrected type for staking timestamp
    reward_rates: f64, // Rewards per NFT per day
}

#[near_bindgen]
impl StakingContract {
    #[init]
    pub fn new(owner_id: AccountId, nft_contract_id: AccountId, rewards_contract_id: AccountId, reward_rate: f64) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner: owner_id,
            nft_contract_id,
            rewards_contract_id,
            staked_nfts: UnorderedMap::new(b"s".to_vec()),
            reward_rates: reward_rate,
        }
    }

    pub fn stake_nft(&mut self, account_id: AccountId, nft_id: String) {
        let now = env::block_timestamp();
        let mut user_nfts = self.staked_nfts.get(&account_id).unwrap_or_else(HashMap::new);

        // Check if NFT is already staked
        assert!(!user_nfts.contains_key(&nft_id), "NFT is already staked");

        // Simulate NFT ownership check (replace with actual check using NFT contract)
        // assert!(check_nft_ownership(&account_id, &nft_id), "Not the owner of the NFT");

        // Staking the NFT
        user_nfts.insert(nft_id, U64(now));
        self.staked_nfts.insert(&account_id, &user_nfts);
    }

    pub fn unstake_nft(&mut self, account_id: AccountId, nft_id: String) {
        let mut user_nfts = self.staked_nfts.get(&account_id).expect("No NFTs staked for this account");
        
        // Check if NFT is actually staked
        assert!(user_nfts.contains_key(&nft_id), "NFT not staked");
        
        // Unstake the NFT
        user_nfts.remove(&nft_id);
        if user_nfts.is_empty() {
            self.staked_nfts.remove(&account_id);
        } else {
            self.staked_nfts.insert(&account_id, &user_nfts);
        }
    }

    fn calculate_rewards(&self, account_id: &AccountId) -> U128 {
        let user_nfts = self.staked_nfts.get(account_id).unwrap_or_else(HashMap::new);
        let now = env::block_timestamp();
        let total_rewards: u128 = user_nfts.iter().map(|(_, &start_time)| {
            let duration = (now - start_time.0) / 86_400_000_000_000; // Convert nanoseconds to days
            (duration as f64 * self.reward_rates) as u128
        }).sum();

        U128(total_rewards)
    }

    pub fn get_rewards(&self, account_id: AccountId) -> U128 {
        self.calculate_rewards(&account_id)
    }
}

pub enum StorageKey {
    StakedNfts,
}

impl BorshSerialize for StorageKey {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        match *self {
            StorageKey::StakedNfts => writer.write_all(b"staked_nfts"),
        }
    }
}