#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, Symbol, String, Vec, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct JournalEntry {
    pub timestamp: u64,
    pub title: String,
    pub content: String,
}

#[contracttype]
pub enum JournalBook {
    Entry(String), // maps user identifier to journal entry list
}

#[contract]
pub struct JournalContract;

#[contractimpl]
impl JournalContract {
    // Function to write a new journal entry
    pub fn write_entry(env: Env, user: String, title: String, content: String) {
        let mut entries: Vec<JournalEntry> = env.storage().instance()
            .get(&JournalBook::Entry(user.clone()))
            .unwrap_or(Vec::new(&env));

        let timestamp = env.ledger().timestamp();

        entries.push_back(JournalEntry {
            timestamp,
            title,
            content,
        });

        env.storage().instance().set(&JournalBook::Entry(user), &entries);
    }

    // Function to retrieve all journal entries of a user
    pub fn get_entries(env: Env, user: String) -> Vec<JournalEntry> {
        env.storage().instance()
            .get(&JournalBook::Entry(user))
            .unwrap_or(Vec::new(&env))
    }

    // Function to count entries for a user
    pub fn count_entries(env: Env, user: String) -> u32 {
        let entries: Vec<JournalEntry> = env.storage().instance()
            .get(&JournalBook::Entry(user))
            .unwrap_or(Vec::new(&env));
        entries.len()
    }
}
