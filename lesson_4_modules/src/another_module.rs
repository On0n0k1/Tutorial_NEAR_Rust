pub fn hello() -> String {
    String::from("Hello from another_module")
}

// function below is private
// #[allow(unused)] is a way to tell compiler/linter to ignore the warning. Even though, in this case, is a bad idea, since this function is useless.
// Comment the #[allow(unused)] to see the warning
#[allow(unused)]
fn this_is_private() -> String {
    String::from("This will never be used because it's private")
}

