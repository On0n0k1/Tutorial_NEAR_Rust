//! Any rust file in the tests folder is tested individually.
//! No need to include them with mod.

// Uncomment the line below and you'l get a "private" error warning.
// That's because it's only public within the module. And tests acts like it's own module.
// use lesson_4_modules::a_module::specific_module::hello;


#[test]
fn oneplusone() {
    assert_eq!(1 + 1, 2);
}

