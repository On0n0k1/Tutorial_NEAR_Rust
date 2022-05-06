//! This example focused on module management
//! 
//! 
//!
//! The search order for crates and modules is normally:
//! 
//!  - default (std), 
//!  - other crates (like near-sdk), 
//!  - crates in the workspace, 
//!  - local modules.
//!
//! 
//! 'mod modname' tells the compiler to compile files in the directory, or file, with that name
//! and to use bring into scope the module we then use the keyword 'use'
//! 
//! 'pub mod' allows other modules to access and use the module
//! 'pub use' works like the above (bring into scope)

// Here we declare there's a module called 'a_module' and it is public
// (which can be either a file with that name, or a directory with that name and a mod.rs file inside the directory)
pub mod a_module;
// Here we declare 'another_module' (in our example, a file called 'another_module.rs')
mod another_module;

// and one more file based module
mod yet_another_module;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};

// we can use the 'as' keyword to alias functions
use a_module::hello as hello;
use a_module::specific_module::hello as hello1;

// and we can use 'pub use' to export function as public to external crates
pub use another_module::hello as hello2;
pub use yet_another_module::hello as hello3;


// the actual location (path) of 'a_deep_function' is 'yet_another_module::internal_module::a_deep_module::a_deep_function'
// but this full path is private. 
// however, remember we used 'pub use' for this function on 'yet_another_module' and so we can use that shorter path 
use yet_another_module::a_deep_function as hello4;

// uncomment the following line to get a compiler error (private access)
// use another_module::this_is_private;

pub use a_module::specific_module::this_will_panic;


near_sdk::setup_alloc!();


// this function will help to keep cleaner code
pub fn log(message: &str) {
    env::log(message.as_bytes());
}


#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    stored_phrase: String,
}


#[near_bindgen]
impl Contract{
    /// Returns the stored string value
    pub fn get_phrase(&self) -> String {
        self.stored_phrase.clone()
    }

    /// Logs "Hello from crate::a_module" and 
    /// stores that string value 
    pub fn hello(&mut self) {        
        self.stored_phrase = hello();
        log(&hello());
    }

    /// Logs "Hello from crate::a_module::specific_module" and 
    /// stores that string value 
    pub fn hello1(&mut self) {
        self.stored_phrase = hello1();
        log(&hello1());
    }

    /// Logs "Hello from another module" and 
    /// stores that string value 
    pub fn hello2(&mut self) {
        self.stored_phrase = hello2();
        log(&hello2());
    }

    /// Logs "Hello from yet_another_module" and 
    /// stores that string value 
    pub fn hello3(&mut self) {
        self.stored_phrase = hello3();
        log(&hello3());
    }

    /// Logs "Called a deep function" and 
    /// stores that string value 
    pub fn hello4(&mut self) {
        self.stored_phrase = hello4();
        log(&hello4());
    }

    /// this function will panic when called, with a message "A panic just happened"
    pub fn this_will_panic() {    
        this_will_panic();
    }
}

// Tests can be found in the 'tests' directory. This best practice helps with keeping our code organized. 
