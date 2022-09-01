//! Utilities module
//! 
//!  - log: print a message on testing or production environments (different target output)
//!  - ViewGet: allows having different return typs for the same function.
//! 

use near_sdk::serde::{
    Deserialize, Serialize,
};

use crate::entry::TemperatureReading;


#[allow(unused_imports)]
use near_sdk::env;

/// Prints using println when in a test environment. 
#[cfg(test)]
pub fn log(msg: &str){
    println!("{}", msg);
}

/// Prints using env::log when in a production environment. 
#[cfg(not(test))]
pub fn log(msg: &str) {
    env::log(msg.as_bytes());
}


/// Used as return type for view_get function
/// 
///  - If using an index returns a single temperature reading.
///  - If not using an index, then return a list of all temperature readings.
/// 
/// Not efficient when a contract has a lot of readings for a user/sensor, 
/// but best practice would be for the user to collect values locally and remove old ones
/// to save on both storage and computing fees
/// 
/// Using #[serde(untagged)] will keep our JSON very lean.
/// 
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ViewGet{
    Single(TemperatureReading),
    Multiple(Vec<TemperatureReading>),
}
