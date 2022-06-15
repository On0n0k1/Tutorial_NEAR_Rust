//! When a user loads a player type. This type is what they will receive.
//! 
//! 

use near_sdk::{
    serde::{ Deserialize, Serialize },
};

use crate::model::{
    character::Character,
    player,
    score::HighScore,
};


#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct View {
    pub name: player::Name,
    pub high_score: Option<HighScore>,
    pub characters: Vec<Character>,
}



