use near_sdk::{
    borsh::{ BorshDeserialize, BorshSerialize, self},
    // collections::Vector,
    log, 
    serde::{Deserialize, Serialize},
};

use crate::model::{
    score::HighScore,
    // StorageKey,
};

// Just for this exampĺe, we just want max 10 values in the ranking.
const RANKSIZE: usize = 10;

#[derive(BorshDeserialize, BorshSerialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Ranking{
    // values: Vector<HighScore>,
    values: Vec<HighScore>,
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
        let values: Vec<HighScore> = Vec::with_capacity(RANKSIZE);

        Self { values }
    }
}

impl Clone for Ranking{
    fn clone(&self) -> Self {
        let mut values: Vec<HighScore> = Vec::with_capacity(RANKSIZE);
        
        for value in self.values.iter(){
            values.push(value.clone());
        }

        Self { values }
    }
}




impl Ranking{
    fn sort_and_resize(&mut self) {
        // let mut values: Vec<HighScore> = self.values.to_vec();
        // values.sort();

        // for index in 0..values.len(){
        //     self.values.replace(index as u64, &values[index]);
        // }

        // Remove excessive values.
        // values.truncate(RANKSIZE);


        self.values.sort();
        self.values.truncate(RANKSIZE);
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


// impl std::fmt::Display for Ranking {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut description: String = format!("Ranking: \n");

//         for entry in self.values.iter(){
//             // description = format!("   \n{}{}\n", description, String::from_utf8(entry.try_to_vec().unwrap()).unwrap());
//             description = format!("    {}{}\n", description, entry);
//         }

//         description = format!("{}\n", description);

//         write!(f, "{}", description)
//     }
// }