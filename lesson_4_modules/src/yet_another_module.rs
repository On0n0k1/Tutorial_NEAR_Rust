// internal_module is private for outside modules
mod internal_module;

// but we make a_deep_function public for others as if it was declared here.
// that's great for organizing code.
pub use internal_module::a_deep_module::a_deep_function;

pub fn hello() -> String {
    String::from("Hello from yet_another_module")
}
