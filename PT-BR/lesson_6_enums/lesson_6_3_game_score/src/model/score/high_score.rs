use near_sdk::{
    AccountId,
    borsh::{ BorshDeserialize, BorshSerialize, self },
    serde::{ Deserialize, Serialize },
};

use crate::model::{
    character::Character,
    score::Score,
};


/// Represents a highscore for a player or character.
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HighScore{
    character: Character,
    score: Score,
    player: AccountId,
}

// Used for ordering HighScores within a Vec
impl Ord for HighScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

// Required by Ord.
impl PartialOrd for HighScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Used for partial comparisons between HighScores. Required by Ord and Eq.
//
// A = B sometimes doesn't mean B = A.
impl PartialEq for HighScore {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

// Used for comparisons between HighScores. Required by Ord.
//
// This implies that A = B means B = A.
impl Eq for HighScore {}


impl HighScore{
    /// Returns a new instance of Highscore.
    pub fn new(
        score: Score,
        character: &Character,
        player: AccountId,
    ) -> Self {

        HighScore{
            score,
            character: character.clone(),
            player,
        }
    }

    /// Makes a comparison between the new and old high scores. If a new high_score for the player is achieved, update current and return a copy.
    pub fn update_highscore(
        current_highscore: &mut Option<HighScore>,
        new_high_score: Option<HighScore>,
    ) -> Option<HighScore> {

        // This match will stop assigning the new highscore if one has not been achieved.
        match (&current_highscore, &new_high_score) {
            (_, None) => { 
                // No highscore was achieved by the character.
                return None; 
            },
            (None, Some(_)) => {},
            (Some(old_high_score), Some(new_high_score)) => {
                // A character achieved a highscore
                // there is a highscore recorded.
                // makes a comparison and maintain the highest.
                if old_high_score > new_high_score {
                    return None;
                }
            },
        }

        // assign the new highscore
        *current_highscore = new_high_score.clone();

        return new_high_score;
    }

    /// Returns score achieved.
    pub fn get_score(&self) -> Score {
        self.score
    }

    /// Returns character for this highscore.
    pub fn get_character(&self) -> Character {
        self.character.clone()
    }

}
