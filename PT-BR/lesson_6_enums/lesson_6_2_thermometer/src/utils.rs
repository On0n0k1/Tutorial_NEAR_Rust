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

