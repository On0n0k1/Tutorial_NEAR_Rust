// internal_module is private
// no access by external modules
mod internal_module;

// we export a_deep_function as public so other modules can access it
pub use internal_module::a_deep_module::a_deep_function;

pub fn hello() -> String {
    String::from("Hello from yet_another_module")
}
