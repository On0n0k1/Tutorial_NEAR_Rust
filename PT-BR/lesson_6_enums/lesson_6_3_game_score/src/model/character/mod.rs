use near_sdk::{
    borsh::{ BorshDeserialize, BorshSerialize, self,},
    env,
};


mod class;
mod stats;

pub(crate) use class::Class;
pub(crate) use stats::Stats;

use crate::model::{
    Errors,
    score::{
        Score,
        // HighScore,
    },
};


use super::score::HighScore;


pub type Name = String;
pub type EXP = u32;
pub type Level = u32;


// Attributes are ordered according to priority here, not alphabetic order
#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct Character{
    name: Name,
    class: Class,
    level: Level,
    xp: EXP,
    stats: Stats,
    high_score: Score,
}


impl Character {

    fn update_level(&mut self) {
        let level: u32 = self.level;

        // just an exponential function to work as an example of how level scaling could work.
        // It gets exponentially higher each level. Level 1 requirement is 100.
        let next_level: u32 = 100 + level * 10 + 3 * level * level;

        if self.xp >= next_level { 
            self.level += 1;
            self.xp -= next_level;
            self.stats.update(self.level);
        }
    }

    pub fn new(name: Name, class: Class) -> Result<Character, Errors> {
        let level: Level = 1;
        let xp: EXP = 0;
        let stats: Stats = Stats::from(&class);
        let high_score: Score = 0;

        let first_character = &name[0..1];
        match first_character{
            "" => Err(Errors::InvalidCharacterName(name)),
            _ => Ok(Character {
                name,
                class,
                level,
                xp,
                stats,
                high_score,
            })
        }

        
    }

    pub fn get_name(&self) -> Name {
        self.name.clone()
    }

    pub fn get_class(&self) -> Class {
        self.class.clone()
    }

    pub fn get_level(&self) -> Level {
        self.level.clone()
    }

    pub fn get_xp(&self) -> EXP {
        self.xp.clone()
    }

    pub fn get_stats(&self) -> Stats {
        self.stats.clone()
    }

    pub fn get_high_score(&self) -> Score {
        self.high_score.clone()
    }

    pub fn reward_exp(&mut self, exp: EXP) {
        self.xp += exp;
        self.update_level();
    }

    /// Makes a comparison with the highscore. 
    /// 
    /// 
    /// If a highscore is achieved, return it.
    /// 
    /// Else return None.
    /// 
    pub fn check_highscore(&mut self, score: Score) -> Option<HighScore> {

        if score > self.high_score {
            self.high_score = score;
            let player = env::predecessor_account_id();

            let high_score: HighScore = HighScore::new(score, &self, player);

            return Some(high_score);
        }

        None
    }

}
