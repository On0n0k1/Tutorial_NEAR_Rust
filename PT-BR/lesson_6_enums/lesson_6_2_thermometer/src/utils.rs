use near_sdk::serde::{
    Deserialize, Serialize,
};

use crate::entry::Entry;


#[allow(unused_imports)]
use near_sdk::env;

#[cfg(test)]
pub fn log(msg: &str){
    println!("{}", msg);
}

#[cfg(not(test))]
pub fn log(msg: &str) {
    env::log(msg.as_bytes());
}


/// Usado para saida da função de contrato view_get.
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ViewGet{
    Single(Entry),
    Multiple(Vec<Entry>),
}
