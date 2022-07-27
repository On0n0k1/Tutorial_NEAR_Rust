mod reward;

use near_sdk::{
    borsh,
    borsh::{ BorshDeserialize, BorshSerialize },
    env,
    serde::{ Deserialize, Serialize },
};

pub use reward::ChapterReward;
use crate::model::{
    character::{
        Character,
        EXP,
    },
    score::Score,
    Errors,
};


// Calculates score/rewards for each match (chapter)
/// This is meant to represent a game chapter. Each has it's own rewards. Each has it's own validation method.
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Chapter{
    Chapter1(Option<u64>),
    Chapter2(Option<u64>),
    Chapter3(Option<u64>),
}

impl Default for Chapter{
    fn default() -> Self {
        Chapter::Chapter1(None)
    }
}

impl Chapter {
    fn get_time(&self) -> Option<u64> {
        match self{
            Chapter::Chapter1(value) => value.clone(),
            Chapter::Chapter2(value) => value.clone(),
            Chapter::Chapter3(value) => value.clone(),
        }
    }

    /// Used by Self::clear_time and Self::start_time. Change the value of the last time a match was started.
    fn set_time(&mut self, value: Option<u64>) {
        *self = match self{
            Chapter::Chapter1(_) => Self::Chapter1(value),
            Chapter::Chapter2(_) => Self::Chapter2(value),
            Chapter::Chapter3(_) => Self::Chapter3(value),
        };
    }

    /// Used after reporting. Means that no match is going at the moment.
    fn stop_time(&mut self) {
        self.set_time(None);
    }

    /// Used when loading a chapter. Get the latest time for starting the match. Report can't have a longer time than this.
    fn start_time(&mut self) {
        let current_time_ms: u64 = env::block_timestamp_ms();

        self.set_time(Some(current_time_ms));
    }

    /// Returns the multipliers for each chapter reward.
    pub fn check_reward(&self) -> ChapterReward {
        match self {
            Chapter::Chapter1(_) => {
                ChapterReward::new(
                    10, 
                    0.9, 
                    1, 
                    0.9,
                )
            },
            Chapter::Chapter2(_) => {
                ChapterReward::new(
                    100, 
                    0.9, 
                    5, 
                    0.9,
                )
            },
            Chapter::Chapter3(_) => {
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
    fn validate_get_reward(
        &self,
        _character: &Character,
        _score: &Score,
        // Validation_report: EncryptedValidationType,
    ) -> Result<ChapterReward, Errors> {

        if self.get_time().is_none(){
            return Err(Errors::ChapterNotStarted);
        }

        // Do something with the validation_report and given character and score.
        // validation_report should be a block of bytes signed with a public key owned by the smart contract.
        // The report should have information about everything that happened in the match.
        // Since the gameplay happens in the browser. It's very easy for the user to cheat and send a fake report.
        // So maybe the report should be an entire replay of the match.
        //
        // One useful check that could be done is making sure that the match between start and report can't

        
        Result::Ok(
            Self::check_reward(&self)
        )
    }

    /// Can only be called after the timer has started (with Self::start_match). Receives a chapter report and validates it before returning rewards.
    pub fn validate_match(
        &mut self, 
        character: &Character, 
        score: &Score,
        // Validation_report: ValidationReport,
    ) -> Result<EXP, Errors> {
        let reward = self.validate_get_reward(character, score)?;

        self.stop_time();
        Ok(reward.compute_reward(character.get_level(), score))
    }

    /// Go to the next chapter and stop the timer.
    pub fn next_match(&mut self) {
        *self = match self {
            Chapter::Chapter1(_) => Chapter::Chapter2(None),
            Chapter::Chapter2(_) => Chapter::Chapter3(None),
            Chapter::Chapter3(_) => Chapter::Chapter1(None),
        };
    }

    /// Start match timer. Needed for validation.
    pub fn start_match(&mut self) -> Self {
        self.start_time();

        self.clone()
    }

}

#[cfg(test)]
mod tests{
    use super::Chapter;

    fn new_chapter_1() -> Chapter{
        Chapter::default()
    }

    fn new_chapter_2() -> Chapter {
        let mut chapter = Chapter::default();
        chapter.next_match();
        chapter
    }

    fn new_chapter_3() -> Chapter {
        let mut chapter = Chapter::default();
        chapter.next_match();
        chapter.next_match();
        chapter
    }

    fn assert_chapter_number(chapter: &Chapter, number: u8) {
        if number < 1 && number > 3 {
            panic!("Invalid argument for test. Chapter can only be 1, 2 or 3.")
        }

        match chapter {
            Chapter::Chapter1(_) => assert!(number == 1),
            Chapter::Chapter2(_) => assert!(number == 2),
            Chapter::Chapter3(_) => assert!(number == 3),
        }
    }

    #[test]
    fn chapter_next_chapter() {
        let (chapter1, chapter2, chapter3) = (
            new_chapter_1(),
            new_chapter_2(),
            new_chapter_3(),
        );

        assert_chapter_number(&chapter1, 1);
        assert_chapter_number(&chapter2, 2);
        assert_chapter_number(&chapter3, 3);
    }

    #[test]
    fn chapter_clear_timer() {
        
        let (mut chapter1, mut chapter2, mut chapter3) = (
            new_chapter_1(),
            new_chapter_2(),
            new_chapter_3(),
        );

        chapter1.start_match();
        chapter2.start_match();
        chapter3.start_match();

        assert!(chapter1.get_time().is_some());
        assert!(chapter2.get_time().is_some());
        assert!(chapter3.get_time().is_some());

        chapter1.stop_time();
        chapter2.stop_time();
        chapter3.stop_time();

        assert!(chapter1.get_time().is_none());
        assert!(chapter2.get_time().is_none());
        assert!(chapter3.get_time().is_none());
    }


}