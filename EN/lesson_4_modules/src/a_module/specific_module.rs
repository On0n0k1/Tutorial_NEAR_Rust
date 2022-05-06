// Specifying (crate) after pub allows a function (or module, trait, etc) to be available to 
// other local modules (the same crate), but not to external crates
pub(crate) fn hello() -> String{
    String::from("Hello from crate::a_module::specific_module")
}

/// use the panic! macro to raise an error and halt/terminate the application
/// we can call panic! whenever we need it
pub fn this_will_panic() {
    panic!("A panic has just happened");
}


// cfg(test) tells the compiler these is a module with tests (test harness)
// and you can then use 'cargo test'
#[cfg(test)]
mod tests{
    // load everything 
    use super::*;

    #[test]
    fn hello_test(){
        // hello() is public for modules in the same crate
        // we'll try to import it from the test directory, which would cause an error
        // the test directory is its own crate
        assert_eq!(
            hello(),
            "Hello from crate::a_module::specific_module",
        );
    }


    #[test]
    // we can also test scenarios where an error is raised (panic)
    // and so we can ensure we panic for the right reason
    #[should_panic(expected = "A panic has just happened")]
    fn this_will_panic_test() {
        this_will_panic();
    }
}