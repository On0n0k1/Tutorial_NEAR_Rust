// internal_module é privado para módulos externos.
mod internal_module;

// Mas fizemos a_deep_function publico para outros na declaração aqui.
// Bom para organizar código.
pub use internal_module::a_deep_module::a_deep_function;

pub fn hello() -> String {
    String::from("Hello from yet_another_module")
}
