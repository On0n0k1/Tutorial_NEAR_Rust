use near_sdk::{
    borsh::{ BorshDeserialize, BorshSerialize, self},
    collections::Vector,
};

use crate::model::{
    score::HighScore,
    StorageKey,
};

// Just for this exampĺe, we just want max 10 values in the ranking.
const RANKSIZE: usize = 10;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Ranking{
    values: Vector<HighScore>,
}

impl Default for Ranking{
    fn default() -> Self {
        let values: Vector<HighScore> = Vector::new(StorageKey::Ranking);

        Self { values }
    }
}


impl Ranking{
    fn sort_and_resize(&mut self) {
        let mut values: Vec<HighScore> = self.values.to_vec();
        values.sort();

        for index in 0..values.len(){
            self.values.replace(index as u64, &values[index]);
        }

        // Remove excessive values.
        values.truncate(RANKSIZE);
    }

    pub fn new_entry(&mut self, entry: &HighScore) {
        self.values.push(entry);

        // Sort the highscores and resize it to RANKSIZE (If it has more values than RANKSIZE)
        self.sort_and_resize();
    }

    pub fn contains(&self, other: &HighScore) -> bool {
        for  entry in self.values.iter(){
            if *other == entry {
                return true;
            }
        }

        false
    }

}


impl std::fmt::Display for Ranking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut description: String = format!("Ranking: \n");

        for entry in self.values.iter(){
            description = format!("   {}{}\n", description, String::from_utf8(entry.try_to_vec().unwrap()).unwrap());
        }

        description = format!("{}\n", description);

        write!(f, "{}", description)
    }
}