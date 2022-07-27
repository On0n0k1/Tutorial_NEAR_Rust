use near_sdk::{
    borsh::{ BorshDeserialize, BorshSerialize, self},
    env,
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
    lowest_high_score: Option<HighScore>,
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
        let lowest_high_score: Option<HighScore> = None;

        Self { 
            values,
            max_size,
            lowest_high_score,
        }
    }
}

impl Clone for Ranking{
    fn clone(&self) -> Self {
        let max_size: usize = self.max_size.clone();
        let mut values: Vec<HighScore> = Vec::with_capacity(max_size);
        let lowest_high_score: Option<HighScore> = self.lowest_high_score.clone();
        
        for value in self.values.iter(){
            values.push(value.clone());
        }

        Self { 
            values,
            max_size,
            lowest_high_score,
        }
    }
}


impl Ranking{

    fn sort_and_resize(&mut self) {
        self.values.sort();
        self.values.truncate(self.max_size);
    }

    /// This is only called when the ranking list is full.
    /// Add the entry to the list, sort it, then remove all the excess elements.
    /// Finally, set the lowest high score value to the element at the end of the list.
    fn new_entry(&mut self, entry: HighScore) {
        self.values.push(entry);

        // Sort the highscores and resize it to RANKSIZE (If it has more values than RANKSIZE)
        self.sort_and_resize();

        let lowest_high_score = self.values.last();

        match lowest_high_score {
            None => env::panic_str("Smart contract implementation error. This should never happen. Called Ranking::new_entry and got a None."),
            Some(lowest) => {
                self.lowest_high_score = Some(lowest.clone());
            }
        }
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

                // Compiler will apply branchless optimization to all these if/else statements.
                if self.lowest_high_score.is_none() {
                    // This is the first entry, so just include it.
                    self.new_entry(high_score.clone());

                    return true;
                } else {
                    // This is not the first entry.
                    // The list may be full or not.
                    let ranking_is_full: bool = self.values.len() == self.max_size;

                    if !ranking_is_full {
                        // If the list is not full, just include it.
                        self.new_entry(high_score.to_owned());

                        return true;
                    } else {
                        // .unwrap will never panic because of the first "if" above. It is always Some.
                        // We are cloning because unwrap will take ownership of this mutable reference.
                        let lowest_high_score = self.lowest_high_score
                            .clone()
                            .unwrap();

                        if lowest_high_score < *high_score {
                            self.new_entry(high_score.clone());

                            return true;
                        }
                        
                        false
                    }
                }
            }
        }
    }

}