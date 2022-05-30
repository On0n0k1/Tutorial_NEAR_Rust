use near_sdk::{
    AccountId,
    borsh::{
        BorshDeserialize,
        BorshSerialize,
        self,
    },
    collections::{
        LookupMap,
        UnorderedSet,
    },
    env,
};

use crate::model::{
    character,
    character::Character,
    score::HighScore,
    Errors,
};

use crate::StorageKey;


pub type Name = AccountId;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Player{
    name: Name,
    high_score: Option<HighScore>,
    // For storing and checking characters by name, can't iterate.
    characters: LookupMap<character::Name, Character>,
    // For storing character names, can iterate.
    character_names: UnorderedSet<character::Name>,
    // With both those above, we can check characters O(1) and iterate through the characters at the same time.
}

impl Default for Player{
    fn default() -> Self {
        let name = env::predecessor_account_id();

        let high_score = None;
        let characters = 
            LookupMap::new(
                StorageKey::Characters(name.clone())
            );

        let character_names: UnorderedSet<character::Name> = UnorderedSet::new(
            StorageKey::CharacterNames(name.clone())
        );
        
        Player{
            name,
            high_score,
            characters,
            character_names,
        }
    }
}

impl Player{
    /// Will panic if a character with that name already exists.
    fn assert_character_doesnt_exist(&self, character_name: &character::Name) -> Result<(), Errors> {
        if self.characters.contains_key(character_name) {
            return Err(Errors::CharacterAlreadyExists(character_name.clone()));
        }

        Ok(())
    }

    pub fn get_character(&self, name: character::Name) -> Result<Character, Errors> {
        match self.characters.get(&name){
            None => Err(Errors::CharacterNotFound(name)),
            Some(character) => Ok(character),
        }
    }

    pub fn assign_character(&mut self, character: Character) -> Result<(), Errors> {
        let character_name = character.get_name();
        
        self.assert_character_doesnt_exist(&character_name)?;
        self.characters.insert(&character_name, &character).unwrap();

        Ok(())
    }
}

