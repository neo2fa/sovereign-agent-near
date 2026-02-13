use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Agent {
    pub owner: AccountId,
    pub codehash: String,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ExecutionLog {
    pub action: String,
    pub timestamp: u64,
    pub proof: String,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Agents,
    Logs { account_hash: Vec<u8> },
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AgentRegistry {
    agents: UnorderedMap<AccountId, Agent>,
}

#[near_bindgen]
impl AgentRegistry {
    #[init]
    pub fn new() -> Self {
        Self {
            agents: UnorderedMap::new(StorageKey::Agents),
        }
    }

    pub fn register_agent(&mut self, codehash: String) {
        let caller = env::predecessor_account_id();
        let agent = Agent { owner: caller.clone(), codehash };
        self.agents.insert(&caller, &agent);
        env::log_str("Agent registered successfully");
    }

    pub fn update_agent_hash(&mut self, new_hash: String) {
        let caller = env::predecessor_account_id();
        let mut agent = self.agents.get(&caller).expect("Agent not found");
        assert_eq!(agent.owner, caller, "Not owner");
        agent.codehash = new_hash;
        self.agents.insert(&caller, &agent);
        env::log_str("Agent hash updated");
    }

    pub fn verify_hash(&self, account: AccountId, provided_hash: String) -> bool {
        if let Some(agent) = self.agents.get(&account) {
            return agent.codehash == provided_hash;
        }
        false
    }

    pub fn log_execution(&mut self, action: String, proof: String) {
        let caller = env::predecessor_account_id();
        let key = StorageKey::Logs { account_hash: env::sha256(caller.as_bytes()) };
        let mut logs: Vector<ExecutionLog> = Vector::new(key);
        logs.push(&ExecutionLog { action, timestamp: env::block_timestamp(), proof });
        env::log_str("Execution logged");
    }

    pub fn get_agent(&self, account: AccountId) -> Option<Agent> {
        self.agents.get(&account)
    }
}
