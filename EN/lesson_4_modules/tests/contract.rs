//! This module tests the Smart Contract

// the 'common' module contains utility functions to be used in all our tests
mod common;

// The 'test' directory is its own crate, 
// So when we import our own modules we need to refer to them as
// "lesson_4_modules::" instead of "crates::"
use lesson_4_modules::Contract;

use common::env_setup;


#[test]
pub fn get_phrase() {
    env_setup();

    let contract: Contract = Contract::default();

    // we used derive(Default) in our contract
    // and the default for String is an empty string
    assert_eq!(contract.get_phrase(), "");
}


/// Assigns "Hello from crate::a_module" to phrase and test the value afterwards
#[test]
pub fn hello() {
    env_setup();

    let mut contract: Contract = Contract::default();

    contract.hello();

    assert_eq!(
        contract.get_phrase(),
        "Hello from crate::a_module",
    );
}


/// Assigns "Hello from crate::a_module::specific_module"  to phrase and test the value afterwards
#[test]
pub fn hello1() {
    env_setup();

    let mut contract: Contract = Contract::default();

    contract.hello1();

    assert_eq!(
        contract.get_phrase(),
        "Hello from crate::a_module::specific_module",
    );
}


/// Assigns "Hello from another_module"  to phrase and test the value afterwards
#[test]
pub fn hello2() {
    env_setup();

    let mut contract: Contract = Contract::default();

    contract.hello2();

    assert_eq!(
        contract.get_phrase(),
        "Hello from another_module",
    );
}


/// Assigns "Hello from yet_another_module"  to phrase and test the value afterwards
#[test]
pub fn hello3() {
    env_setup();

    let mut contract: Contract = Contract::default();

    contract.hello3();

    assert_eq!(
        contract.get_phrase(),
        "Hello from yet_another_module",
    );
}

/// Assigns "Called a deep function"  to phrase and test the value afterwards
#[test]
pub fn hello4() {
    env_setup();

    let mut contract: Contract = Contract::default();

    contract.hello4();

    assert_eq!(
        contract.get_phrase(),
        "Called a deep function",
    );
}


// We can also test error situations, known as 'panic'
// This function will raise an error (panic) with the message "A panic just happened"
// Our test checks if indeed panic happened, and will pass if this was the case
#[test]
#[should_panic(expected = "A panic has just happened")]
pub fn this_will_panic() {
    env_setup();

    // Embora não utilizemos o state, é bom inicializar antes de cada teste.
    let _contract: Contract = Contract::default();

    Contract::this_will_panic();
}
