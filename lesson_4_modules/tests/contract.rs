//! This module will test contract functionality.

// common will be a crate with functions useful for all the other crates
mod common;

// tests folder behaves like it's own crate.
// So when we import our own modules, 
// we refer to it as "lesson_4_modules::" instead of "crate::"
use lesson_4_modules::Contract;

use common::env_setup;


#[test]
pub fn get_phrase() {
    env_setup();

    let contract: Contract = Contract::default();

    // We used derive(Default) for contract.
    // derived default set Strings as an empty String.
    assert_eq!(contract.get_phrase(), "");
}

/// This function will set "Hello from crate::a_module".
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

/// This function will set "Hello from crate::a_module::specific_module".
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

/// This function will set "Hello from another_module".
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

/// This function will set "Hello from yet_another_module".
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

/// This function will set "Called a deep function".
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

/// This function will panic with message "A panic has just happened" when called.
// We can also test situations that should panic.
// expected makes sure we are panicking for the right reason.
#[test]
#[should_panic(expected = "A panic has just happened")]
pub fn this_will_panic() {
    env_setup();

    // Even though we don't use the state, it's good to initialize it before every test.
    let _contract: Contract = Contract::default();

    Contract::this_will_panic();
}
