use near_sdk::{
    AccountId,
    BorshStorageKey,
    borsh::{self, BorshSerialize},
};


#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Players,
    Characters(AccountId),
    CharacterNames(AccountId),
    Ranking,
}