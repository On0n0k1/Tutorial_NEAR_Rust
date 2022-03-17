use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

mod model;

use model::{
    Example0,
    Example1,
    Example2User,
    log,
};


#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    example_0: Example0,
    example_1: Example1,
    example_2_user: Example2User,
}


#[near_bindgen]
impl Contract{
    pub fn get_example_0(&self) -> Example0 {
        self.example_0.clone()
    }

    pub fn get_example_1(&self) -> Example1 {
        self.example_1.clone()
    }

    pub fn get_example_2_user(&self) -> Example2User {
        self.example_2_user.clone()
    }
}

