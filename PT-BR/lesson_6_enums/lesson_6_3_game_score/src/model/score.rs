use near_sdk::borsh::{
    BorshDeserialize,
    BorshSerialize,
    self,
};

use crate::model::character::Character;

pub type Score = u32;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct HighScore{
    value: Score,
    character: Character,
}


impl HighScore{
    pub fn new(
        value: Score,
        character: &Character,
    ) -> Self {
        HighScore{
            value,
            character: character.clone(),
        }
    }

}

