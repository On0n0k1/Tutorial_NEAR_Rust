//! This example focuses in module management.
//! 
//! How modules are imported and used.
//! 
//! 
//! 
//!
//! The order for importing crates, modules is usually:
//!  - standard (std), 
//!  - other crates (like near-sdk), 
//!  - crates in this workspace, 
//!  - then local modules.
//!
//! 
//! "mod file" tells the compiler that the file in this local directory must be compiled too.
//! in order to use the module we use "use"
//! 
//! "pub mod" means that other modules can use said module, else it's private.
//! "pub use" can also be used for the same purpose.
//! The difference being that your files can be organized in a way, and the library modules in another, more convenient, way.
//! 

// This line is saying that there is a file or folder named a_module and should be compiled.
// "mod.rs" is required in the directory for it to be recognized as a module.
pub mod a_module;
// This line is saying that the file "another_module.rs" is part of this project and should be compiled.
mod another_module;

// This line is another way to declare modules, it doesn't use mod.rs, but I think it's more messy.
mod yet_another_module;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};

// We can use "as" to give a nickname for the function or module imported
use a_module::hello as hello;
use a_module::specific_module::hello as hello1;

// pub use makes the function available for outside crates.
pub use another_module::hello as hello2;
pub use yet_another_module::hello as hello3;

// The actual location of a_deep_function is yet_another_module::internal_module::a_deep_module::a_deep_function.
// But that address is private, and yet_another_module calls "pub use" for that function. In other words, cleaner.
use yet_another_module::a_deep_function as hello4;

// Uncomment the following line to get a "this is private" error warning
// use another_module::this_is_private;

pub use a_module::specific_module::this_will_panic;


near_sdk::setup_alloc!();

// This function will make our code less messy.
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
    /// Return stored String.
    pub fn get_phrase(&self) -> String {
        self.stored_phrase.clone()
    }

    /// This function will set "Hello from crate::a_module".
    pub fn hello(&mut self) {        
        self.stored_phrase = hello();
        log(&hello());
    }

    /// This function will set "Hello from crate::a_module::specific_module".
    pub fn hello1(&mut self) {
        self.stored_phrase = hello1();
        log(&hello1());
    }

    /// This function will set "Hello from another_module".
    pub fn hello2(&mut self) {
        self.stored_phrase = hello2();
        log(&hello2());
    }

    /// This function will set "Hello from yet_another_module".
    pub fn hello3(&mut self) {
        self.stored_phrase = hello3();
        log(&hello3());
    }

    /// This function will set "Called a deep function".
    pub fn hello4(&mut self) {
        self.stored_phrase = hello4();
        log(&hello4());
    }

    /// This function will panic with message "A panic has just happened" when called.
    pub fn this_will_panic() {
        this_will_panic();
    }
}

// tests are in the tests folder. Really useful for big projects.
