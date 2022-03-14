use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};
use near_sdk::serde::{Deserialize, Serialize};


near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(Clone, Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Contract {

}

#[near_bindgen]
impl Contract{

}
