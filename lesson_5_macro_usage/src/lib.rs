//!
//! - format!
//! - println!
//! - panic!
//! - vec!
//! - setup_alloc!
//! 
//! 
//! 

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};


near_sdk::setup_alloc!();


/// This is used in ```print_examples```.
/// This function is only compiled in testing environments.
#[cfg(test)]
pub fn log(message: &str) {
    println!("{}", message);
}


/// This is used in ```print_examples```.
/// This function is only compiled outside of testing environments.
#[cfg(not(test))]
pub fn log(message: &str) {
    env::log(message.as_bytes());
}


#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // This contract is stateless
}

#[near_bindgen]
impl Contract{
    /// This is a function to show the differences between println and env:log.
    /// Run it with ```cargo test -- --nocapture``` then deploy and run it from near.
    /// Notice which messages show and which doesn't.
    pub fn print_examples() {
        log("\n\nprint_examples:\n");
        println!("This is a println! It doesn't show in Virtual Machine.");
        let a = String::from("something");
        println!("This is another println with several arguments, {}, {}, {}", 1 , "thingy", a);
        env::log(b"This is a log. It doesn't show in tests.");
        env::log("This is another log".as_bytes());
        env::log(
            format!("This is another log with several arguments: {}, {}, {}", 1, 2, "3")
            .as_bytes()
        );

        log("This is a message that shows in tests and the virtual machine.");
        log(&format!("This is another message with arguments, {}, {}, {}.",
            5,
            "6",
            format!("7"),
        ));
        
        log("\n\n---------------------------------------------------------------------------------\n\n");
    }


    pub fn format_examples() {
        log("\n\nformat_examples:\n");
        let message: String = format!("format returns a formatted String.");
        log(&message);

        let an_arg = "third";
        let message = format!("format can take arguments using {{}}: {}, {}, {}.", 1, "second", an_arg);
        log(&message);

        let (first, second, third) = (1, "second", an_arg);
        let message = format!("We can specify format arguments this way: {first}, {second}, {third}.");
        log(&message);

        let message = format!("We can specify the order of format args: {1}, {2}, {0}.", third, first, second);
        log(&message);

        let (first, second, third) = (1, 2, 3);
        let message = format!("We can make integers show an arbitrary number of digits: {:02}, {:04}, {:6}.", first, second, third);
        log(&message);

        let message = format!("Choosing number of digits and order: {2:02}, {0:4}, {1:06}.", second, third, first);
        log(&message);

        let (first, second, third) = (0.1, 1.23, -2.45);
        let message = format!("We can choose the precision of rational numbers: {:.2}, {:.4}, {:.6}", first, second, third);
        log(&message);

        let message = format!("We can choose the precision and the number of digits: {:2.2}, {:04.4}, {:06.6}", first, second, third);
        log(&message);

        let message = format!("We can choose the precision, the number of digits and the order of arguments: {1:02.2}, {2:4.4}, {0:06.6}", third, first, second);
        log(&message);

        let message = format!("Same as above: {first:2.2}, {second:04.4}, {third:6.6}");
        log(&message);

        log("\n\n----------------------------------------------------------------------\n\n");
    }

    pub fn panic_example() {
        log("\n\npanic_example:\n\n\n");

        log("Panic macros are written in the same way as println and format.");

        let second = 2;
        panic!("Panicking with a few args: {} {} {}", 1, second, 3);

    }

    pub fn vec_examples() {
        log("\n\nvec_examples:\n");

        let example = vec![1, 2, 3, 4];

        log(&format!("We can print vectors in debug mode:\n{:?}\n\n", example));
        
        log(&format!("We can print vectors in \"pretty debug mode\":\n{:#?}\n\n", example));

        log(&format!("We can do the same for tuples too:\n{:#?}\n\n", (1, 2, 3)));

        log(&format!("We can create vectors with default values:\n{:?}\n\n", vec![0; 5]));

        log("More information in \"collections\" lesson");

        log("\n\n-------------------------------------------------------------------------------\n\n");
    }
}

// Run tests with the following command:
// cargo test -- --nocapture --test-threads=1
//
//
// --nocapture makes it prints all output, even in successful tests.
// without --test-threads arg, all the tests will run async. Which means the output will be a mess.
//
#[cfg(test)]
mod tests{
    use super::*;
    use near_sdk::{
        MockedBlockchain,
        testing_env,
        test_utils::VMContextBuilder,
    };

    fn env_setup(){
        let builder: VMContextBuilder = VMContextBuilder::new();
        testing_env!(builder.build());

        // We're not using contract in this project. It's stateless.
        let _contract: Contract = Contract::default();
    }

    #[test]
    pub fn print_examples() {
        env_setup();
    
        Contract::print_examples();
    }

    #[test]
    pub fn format_examples() {
        env_setup();
    
        Contract::format_examples();
    }

    #[test]
    pub fn vec_examples() {
        env_setup();
    
        Contract::vec_examples();
    }

    #[test]
    #[should_panic(expected = "Panicking with a few args: 1 2 3")]
    pub fn panic_example() {
        env_setup();

        Contract::panic_example();
    }
}
