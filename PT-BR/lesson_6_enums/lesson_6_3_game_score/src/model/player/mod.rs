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

mod view;

use crate::model::{
    chapter::Chapter,
    character,
    character::Character,
    score::HighScore,
    score::Score,
    Errors,
};

use crate::StorageKey;


pub type Name = AccountId;

pub use view::View;


/// Holds information pertaining to a single user.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Player{
    name: Name,
    high_score: Option<HighScore>,
    // For storing and checking characters by name, can't iterate.
    characters: LookupMap<character::Name, Character>,
    // For storing character names, can iterate.
    character_names: UnorderedSet<character::Name>,
    // With both those above, we can check characters O(1) and iterate through the characters at the same time.

    latest_chapter: Chapter,
}

// Initial state of the player.
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

        let latest_chapter: Chapter = Chapter::default();
        
        Player{
            name,
            high_score,
            characters,
            character_names,
            latest_chapter,
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

    fn assert_character_exists(&self, character_name: &character::Name) -> Result<(), Errors> {
        if !self.characters.contains_key(character_name) {
            return Err(Errors::CharacterNotFound(character_name.clone()));
        }

        Ok(())
    }

    /// Returns the character state. Should be replaced by pointer access later.
    pub fn load_character(&self, name: character::Name) -> Result<Character, Errors> {
        match self.characters.get(&name){
            None => Err(Errors::CharacterNotFound(name.to_string())),
            Some(character) => Ok(character),
        }
    }

    /// This should be replaced by pointer access later.
    fn save_character(&mut self, character: &Character) -> Result<(), Errors> {
        let character_name = character.get_name();

        self.assert_character_exists(&character_name)?;

        self.characters.insert(&character_name, character).unwrap();

        Ok(())
    }

    pub fn assign_character(&mut self, character: Character) -> Result<(), Errors> {
        let character_name = character.get_name();
        
        self.assert_character_doesnt_exist(&character_name)?;

        // If this error happens. It's an unexpected server error. That means that something else is going wrong.
        // It should be reported.
        assert!(self.characters.insert(&character_name, &character).is_none(), "Server error: Character doesn't exist. Please Report.");
        self.character_names.insert(&character_name);

        Ok(())
    }

    /// Move to the next chapter.
    pub fn next_match(&mut self){
        self.latest_chapter.next_match();
    }

    /// Start timer and return the current chapter.
    pub fn start_match(&mut self) -> Chapter {
        self.latest_chapter.start_match()
    }

    /// End the timer and reward the character. Then update highscores.
    /// 
    /// Returns a HighScore if it was achieved,
    pub fn report_match(
        &mut self,
        character: String,
        score: Score,
        // validation_report: ValidationReport,
    ) -> Result<Option<HighScore>, Errors> {
        let mut character: Character = self.load_character(character)?;

        let exp: character::EXP = self.latest_chapter.validate_match(
            &character, 
            &score,
        )?;

        let new_character_highscore: Option<HighScore> = character.check_highscore(score);

        let high_score: Option<HighScore> = HighScore::update_highscore(
            &mut self.high_score, 
            new_character_highscore,
        );

        character.reward_exp(exp);
        self.save_character(&character)?;

        Ok(high_score)
    }

    /// Get characters' name.
    pub fn get_name(&self) -> Name {
        self.name.clone()
    }

    /// Returns information about this player as json.
    /// 
    /// LookupMap and UnorderedSet can't be serialized into json. We create a View with serializable data types and return it instead.
    pub fn get_view(&self) -> Result<View, Errors> {
        let name: Name = self.name.clone();
        let high_score: Option<HighScore> = self.high_score.clone();
        let mut characters: Vec<Character> = Vec::with_capacity(self.character_names.len() as usize);

        for character_name in self.character_names.iter() {
            let character: Character = self.load_character(character_name)?;
            characters.push(character);
        };

        Ok(View { name, high_score, characters })
    }
}
