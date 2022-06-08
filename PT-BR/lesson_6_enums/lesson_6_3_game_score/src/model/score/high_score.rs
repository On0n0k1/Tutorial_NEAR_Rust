use near_sdk::{
    AccountId,
    borsh::{ BorshDeserialize, BorshSerialize, self },
};

use crate::model::{
    character::Character,
    Errors,
    score::Score,
};



#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct HighScore{
    character: Character,
    score: Score,
    player: AccountId,
}

impl Ord for HighScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for HighScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HighScore {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for HighScore {}

impl HighScore{
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

    /// If a high score has been achieved, return a new one. Else, return None.
    pub fn score_comparison(&self, score: Score, character: &Character, player: AccountId) -> Option<HighScore> {
        if score > self.score {
            return Some(HighScore::new(
                score, 
                character,
                player,
            ));
        }

        None
    }

    pub fn update_highscore(current_highscore: &mut Option<HighScore>, character: &Character, score: Score, player: AccountId) -> Result<Option<HighScore>, Errors> {
        match &current_highscore {
            None => {
                let high_score: HighScore = HighScore::new(
                    score, 
                    &character,
                    player,
                );

                Ok(Some(high_score))
            },
            Some(value) => {
                let high_score = value.score_comparison(score, &character, player);

                match high_score{
                    None => {
                        return Ok(None);
                    }
                    Some(value) => {
                        *current_highscore = Some(value.clone());
                        Ok(Some(value))
                    }
                }
            }
        }
    }


    pub fn get_score(&self) -> Score {
        self.score
    }

    pub fn get_character(&self) -> Character {
        self.character.clone()
    }

}
