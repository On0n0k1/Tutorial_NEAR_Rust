//! Examples for the following macros:
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


/// This function is used in print_examples and compiled when test option is used.
#[cfg(test)]
pub fn log(message: &str) {
    println!("{}", message);
}


/// This function is used in print_examples and NOT compiled when test option is used.
#[cfg(not(test))]
pub fn log(message: &str) {
    env::log(message.as_bytes());
}


#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // This smart contract doesn't have state
}

#[near_bindgen]
impl Contract{
    /// This function shows differences between println! and env::log
    /// Run with `cargo test -- --nocapture`, compile, deply and execute on NEAR
    /// You'll notice some of these will print out, but others will not
    pub fn print_examples() {
        log("\n\nprint_examples:\n");
        println!("This println! will not output in the virtual machine");
        let a = String::from("something");
        println!("Another println! with some arguments, {}, {}, {}", 1, "thingy", a);

        env::log(b"This is a log, it will not output in test.");
        env::log("another log".as_bytes());

        env::log(
            format!("A log with some arguments: {}, {}, {}", 1, 2, "3")
            .as_bytes()
        );

        log("A message that shows up in the virtual machine during when testing.");

        log(
            &format!("Another message with arguments, {}, {}, {}.",
                5,
                "6",
                format!("7"),
            )
        );
        
        log("\n\n---------------------------------------------------------------------------------\n\n");
    }


    /// examples for format!
    /// check the message output for each format! use
    pub fn format_examples() {
        log("\n\nformat_examples:\n");
        let message: String = format!("format returns a formatted string");
        log(&message);

        let an_arg ="third";
        let message = format!("format can receive argument using {{}}: {}, {}, {}.", 1, "second", an_arg);
    
        log(&message);

        let (first, second, third) = (1, "segundo", an_arg);
        let message = format!("We can specify arguments by variable name: {first}, {second}, {third}.");

        log(&message);

        let message = format!("We can specify the ordering of arguments for format: {1}, {2}, {0}.", third, first, second);

        log(&message);

        let (first, second, third) = (1, 2, 3);
        let message = format!("We can show integers with specified digits: {:02}, {:04}, {:6}.", first, second, third);
        log(&message);

        let message = format!("Specifying digits and argument ordering: {2:02}, {0:4}, {1:06}.", second, third, first);
        log(&message);

        let (first, second, third) = (0.1, 1.23, -2.45);
        let message = format!("Specify precision for floating points: {:.2}, {:.4}, {:.6}", first, second, third);
        log(&message);

        let message = format!("Specifying both precision and digits: {:2.2}, {:04.4}, {:06.6}", first, second, third);
        log(&message);

        let message = format!("Specify precision, digits and ordering of arguments: {1:02.2}, {2:4.4}, {0:06.6}", third, first, second);
        log(&message);

        let message = format!("Same as previous one but with variable names: {first:2.2}, {second:04.4}, {third:6.6}");

        log(&message);

        log("\n\n----------------------------------------------------------------------\n\n");
    }

    /// Example for panic!
    pub fn panic_example() {
        log("\n\npanic_example:\n\n\n");

        log("The panic! macro is used just like println! and format!");

        let second = 2;
        panic!("panic! with arguments: {} {} {}", 1, second, 3);

    }

    /// Example for vec!
    pub fn vec_examples() {
        log("\n\nvec_examples:\n");

        let example = vec![1, 2, 3, 4];

        log(&format!("Let's print a vector with debug formatting:\n{:?}\n\n", example));
        
        log(&format!("Let's print it using \"pretty print\":\n{:#?}\n\n", example));

        log(&format!("We can do the same with tuples:\n{:#?}\n\n", (1, 2, 3)));

        log(&format!("Let's create vectors with default values:\n{:?}\n\n", vec![0;5]));

        log("\n\n-------------------------------------------------------------------------------\n\n");
    }
}

// Run test with this command
// cargo test -- --nocapture --test-threads=1
//
//
// --nocapture will print everything as output, 
// Without --test-threads, all tests will run async and so output will be confusing
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

        // our contract is stateless, we declare it but we don't use it
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
    #[should_panic(expected = "Pânico com alguns argumentos: 1 2 3")]
    pub fn panic_example() {
        env_setup();

        Contract::panic_example();
    }
}
