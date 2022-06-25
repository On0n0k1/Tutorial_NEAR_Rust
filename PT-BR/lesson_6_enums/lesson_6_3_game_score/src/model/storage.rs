use near_sdk::{
    AccountId,
    BorshStorageKey,
    borsh::{self, BorshSerialize},
};


/// Keys used for storing values in trie. Required for collections like UnorderedSet and LookupMap.
/// 
/// We have to make sure that each collection has a unique prefix.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    // Holds no argument because only one LookupMap for Players in the entire smart contract.
    Players,
    
    // Holds an account Id because Each player has one UnorderedSet for the characters. The AccountId is used to tell these types apart.
    Characters(AccountId),

    // Same for character names. It's an UnorderedSet that is different between each player. 
    // So we use the AccountId (unique for each player) within the tuple.
    CharacterNames(AccountId),
}