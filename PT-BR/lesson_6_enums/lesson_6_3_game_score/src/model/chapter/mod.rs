mod reward;

pub use reward::ChapterReward;
use crate::model::{
    character::Character,
    score::Score,
    Errors,
};


// Calculates score/rewards for each match (chapter)

pub enum Chapter{
    Chapter1,
    Chapter2,
    Chapter3,
}

impl Chapter {

    pub fn check_reward(&self) -> ChapterReward {
        match self {
            Chapter::Chapter1 => {
                ChapterReward::new(
                    10, 
                    0.9, 
                    1, 
                    0.9,
                )
            },
            Chapter::Chapter2 => {
                ChapterReward::new(
                    100, 
                    0.9, 
                    5, 
                    0.9,
                )
            },
            Chapter::Chapter3 => {
                ChapterReward::new(
                    1000, 
                    0.9, 
                    10, 
                    0.9,
                )
            },
        }
    }

    
    /// Doesn't do anything in this tutorial.
    /// 
    /// The idea is that the user will send a report that includes everything that happened during the match.
    /// 
    /// The contract guarantees that the user didn't attempt to cheat at the game.
    /// 
    fn validate_chapter_report(
        &self,
        _character: &Character,
        _score: &Score,
        // Validation_report: EncryptedValidationType,
    ) -> Result<ChapterReward, Errors> {

        // Do something with the validation_report and given character and score.
        // validation_report should be a block of bytes signed with a public key owned by the smart contract.
        // The report should have information about everything that happened in the match.
        // Since the gameplay happens in the browser
        
        Result::Ok(
            Self::check_reward(&self)
        )
    }
}
