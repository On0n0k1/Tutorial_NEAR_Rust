//! Each rust file in the tests directory will be run individually


// Uncomment the line below to get a 'private' warning from the compiler
// this happens because this module is public within the same crate, but 'tests' is its own crate
// use lesson_4_modules::a_module::specific_module::hello;

#[test]
fn oneplusone() {
    assert_eq!(1 + 1, 2);
}
