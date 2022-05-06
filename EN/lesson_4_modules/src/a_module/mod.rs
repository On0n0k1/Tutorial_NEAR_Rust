//! Declaring modules:
//! Use a directory name for the module you want and put a mod.rs file in it, 
//! or create a .rs file with the same filename as the module you want
pub mod specific_module;


pub fn hello() -> String {
    String::from("Hello from crate::a_module")
}
