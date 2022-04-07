use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[allow(unused_imports)]
use near_sdk::{
    collections::Vector,
    near_bindgen,
    BorshStorageKey,
};

near_sdk::setup_alloc!();

use crate::{
    temperature::{
        temp_format::TempFormat,
        Temperature,
    },
    utils::log,
};


#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    TEMP,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    temp_format: String,
    temp: Vector<Temperature>,
    temp_length: u32,
}


impl Default for Contract {
    fn default() -> Self {
        let temp_format = format!("{}", TempFormat::default());
        let temp = Vector::new(StorageKey::TEMP);

        Contract{
            temp_format,
            temp,
            temp_length: 0,
        }
    }
}


#[near_bindgen]
impl Contract{
    pub fn set_format(&mut self, temp_format: String) {
        log("Called set_format");
        let temp_format = String::from(TempFormat::from(temp_format));

        log(
            &format!("Setting default temperature format to {}", &temp_format)
        );

        self.temp_format = temp_format;
    }

    pub fn push_temp(&mut self, value: f32, temp_format: String){

    }
    // pub fn push_temp()

}

