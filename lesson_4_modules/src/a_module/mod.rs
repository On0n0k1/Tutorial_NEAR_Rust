//! Entre usar mod.rs ou um arquivo rust com mesmo nome. 
//! É apenas uma questão de manter o arquivo rust no diretorio pai ou filho.
//! 
//! 

pub mod specific_module;


pub fn hello() -> String {
    String::from("Hello from crate::a_module")
}
