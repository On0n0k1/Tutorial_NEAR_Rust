use near_sdk::{
    borsh::{ BorshDeserialize, BorshSerialize, self},
    // collections::Vector,
    log, 
    serde::{Deserialize, Serialize},
};

use crate::model::{
    score::HighScore,
    Errors,
    // StorageKey,
};

// Just for this exampĺe, we just want max 10 values in the ranking.
// const RANKSIZE: usize = 10;


/// Contains the top ranked matches stored in the smart contract.
/// 
/// It's just a vector. So to avoid high costs sorting.
/// 
/// We limit the max number of entries to RANKSIZE.
/// 
/// Suggestion for change. Store the score of the lowest highscore in the ranking. 
/// Only update and sort the list when a value higher than such is included.
#[derive(BorshDeserialize, BorshSerialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Ranking{
    values: Vec<HighScore>,
    max_size: usize,
}

impl Serialize for Ranking {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: near_sdk::serde::Serializer {

        serializer.collect_seq(self.values.iter().map(|x| x))
    }
}

impl Default for Ranking{
    fn default() -> Self {
        // let values: Vec<HighScore> = Vector::new(StorageKey::Ranking);
        let max_size: usize = 10;
        let values: Vec<HighScore> = Vec::with_capacity(max_size);

        Self { 
            values,
            max_size,
        }
    }
}

impl Clone for Ranking{
    fn clone(&self) -> Self {
        let max_size: usize = self.max_size.clone();
        let mut values: Vec<HighScore> = Vec::with_capacity(max_size);
        
        for value in self.values.iter(){
            values.push(value.clone());
        }

        Self { 
            values,
            max_size,
        }
    }
}


impl Ranking{
    fn sort_and_resize(&mut self) {
        self.values.sort();
        self.values.truncate(self.max_size);
    }

    fn new_entry(&mut self, entry: HighScore) {
        self.values.push(entry);

        // Sort the highscores and resize it to RANKSIZE (If it has more values than RANKSIZE)
        self.sort_and_resize();
    }

    fn contains(&self, other: &HighScore) -> bool {
        for  entry in self.values.iter(){
            if *other == *entry {
                return true;
            }
        }

        false
    }

    pub fn set_max_highscore_players(&mut self, max_size: usize) -> Result<(), Errors>{
        let limit = 1000;

        if max_size > limit {
            return Err(Errors::ExcessiveMaxRankingPlayers(limit, max_size));
        }

        self.max_size = max_size;

        Ok(())
    }

    pub fn check_highscore(
        &mut self, 
        high_score: &Option<HighScore>,
    ) -> bool {
        match high_score {
            None => { 
                // Player didn't achieve a high score.
                false
            },
            Some(high_score) => {
                log!("New High Score for this Player.");

                // Ranking has a max number of top scores.
                // This will include an entry. Sort the top scores and remove any excess entries.
                self.new_entry(high_score.clone());

                // If the current high score is included in the top scores, a new one has been achieved.
                self.contains(&high_score)
            }
        }
    }

}