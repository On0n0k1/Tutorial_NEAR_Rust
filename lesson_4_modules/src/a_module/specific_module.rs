// Having (crate) after pub makes the function (or module, or trait, ...) available for local modules, but not for outside crates.
pub(crate) fn hello() -> String{
    String::from("Hello from crate::a_module::specific_module")
}

/// panic is when something goes wrong and everything has to stop now. We can panic on demand when some unexpected behavior and it mo
pub fn this_will_panic() {
    panic!("A panic has just happened");
}


// cfg(test) tells the compiler this only exist in testing environments
#[cfg(test)]
mod tests{
    // load everything inside this file, outside "tests" module
    use super::*;

    #[test]
    fn hello_test(){
        // Hello is public only for modules in this crate.
        // We will try importing it in the tests folder, and it will cause an error.
        // Because tests folder acts as it's own crate.
        assert_eq!(
            hello(),
            "Hello from crate::a_module::specific_module",
        );
    }


    #[test]
    #[should_panic(expected = "A panic has just happened")]
    fn this_will_panic_test() {
        this_will_panic();
    }
}