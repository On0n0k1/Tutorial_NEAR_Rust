use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


use crate::{
    schedule::Schedule,
    temperature::Temperature,
};


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Entry{
    schedule: Schedule,
    temperature: Temperature,
}


