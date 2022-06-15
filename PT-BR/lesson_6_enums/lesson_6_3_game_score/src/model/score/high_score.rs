use near_sdk::{
    AccountId,
    borsh::{ BorshDeserialize, BorshSerialize, self },
    serde::{ Deserialize, Serialize },
};

use crate::model::{
    character::Character,
    Errors,
    score::Score,
};



#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
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

    /// Makes a comparison between the new and old high scores. If a new high_score for the player is achieved, return it.
    pub fn update_highscore(
        current_highscore: &mut Option<HighScore>,
        new_high_score: Option<HighScore>,
    ) -> Result<Option<HighScore>, Errors> {

        // This match will stop assigning the new highscore if one has not been achieved.
        match (&current_highscore, &new_high_score) {
            (_, None) => { 
                // No highscore was achieved by the character.
                return Ok(None); 
            },
            (None, Some(_)) => {},
            (Some(old_high_score), Some(new_high_score)) => {
                // A character achieved a highscore
                // there is a highscore recorded.
                // makes a comparison and maintain the highest.
                if old_high_score > new_high_score {
                    return Ok(None);
                }
            },
        }

        // assign the new highscore
        *current_highscore = new_high_score.clone();

        return Ok(new_high_score);
    }

    pub fn get_score(&self) -> Score {
        self.score
    }

    pub fn get_character(&self) -> Character {
        self.character.clone()
    }

}


// impl std::fmt::Display for HighScore {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // let mut description: String = format!("Ranking: \n");

//         // for entry in self.values.iter(){
//         //     // description = format!("   \n{}{}\n", description, String::from_utf8(entry.try_to_vec().unwrap()).unwrap());
//         //     description = format!("    {}{}\n", description, entry);
//         // }

//         // description = format!("{}\n", description);

//         write!(f, "{\n    {}\n}\n", description)
//     }
// }
