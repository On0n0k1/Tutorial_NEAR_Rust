use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

mod model;

#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    
}

#[near_bindgen]
impl Contract{

}

