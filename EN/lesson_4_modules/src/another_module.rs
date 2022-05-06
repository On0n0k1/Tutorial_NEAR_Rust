pub fn hello() -> String {
    String::from("Hello from another_module")
}


// A função abaixo é privada.
// this is a private function
// #[allow(unused)] tells the compiler to ignore this 'unused code' warning (no warning)
// Comment #[allow(unused)] so the compiler raises a warning
#[allow(unused)]
fn this_is_private() -> String {
    String::from("This will never be used because it's private")
}

