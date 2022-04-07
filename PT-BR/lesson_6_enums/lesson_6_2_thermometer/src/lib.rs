// use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::near_bindgen;
// use near_sdk::serde::{Serialize, Deserialize};


// near_sdk::setup_alloc!();

pub(crate) mod contract;
pub(crate) mod entry;
pub(crate) mod schedule;
pub(crate) mod temperature;
pub(crate) mod utils;

// use model::Experience;
pub use contract::Contract;



// #[derive(BorshDeserialize, BorshSerialize)]
// #[derive(Deserialize, Serialize)]
// #[serde(crate = "near_sdk::serde")]
// #[serde(untagged)]
// pub enum TwoArgs{
//     Integer(u8),
//     String(String),
// }

// #[near_bindgen]
// #[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
// pub struct Contract {

// }

// #[near_bindgen]
// impl Contract{
//     // pub fn test_experience(experience: String) -> String {
//     //     Experience::new(&experience).get()
//     // }
    
//     // pub fn test_experience(experience: TwoArgs) -> String {
//     //     let experience: String = match experience {
//     //         TwoArgs::Integer(value) => format!("{}", value),
//     //         TwoArgs::String(value) => value.clone(),
//     //     };

//     //     Experience::new(&experience).get()
//     // }
// }