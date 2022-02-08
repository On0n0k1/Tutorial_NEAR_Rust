//! Between using mod.rs or a rust file with same name. It's all a matter of keeping the rust file in the parent or child folder.


pub mod specific_module;


pub fn hello() -> String {
    String::from("Hello from crate::a_module")
}
