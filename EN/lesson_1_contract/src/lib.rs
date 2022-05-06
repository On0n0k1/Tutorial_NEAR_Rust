//! Source <https://github.com/near-examples/rust-counter>
//! 
//! 
//! 

// Documentation Tips:
// Double-slash comments // don't show up in documentation.
// Three-slash comments /// show un as description for what comes next (mod, fn, struct, enum, trait...)
// Bang comments //! can appear at the beginning of the file, and they provide documentation about the entire module.


// imports
use near_sdk::{
    // Arguments received and return values will be converted to/from JSON with borsh
    borsh::{
        self,
        BorshDeserialize,
        BorshSerialize,
    },
    // env,
    // Creates boilerplatecode needed for NEAR virtual machine
    near_bindgen,
};

near_sdk::setup_alloc!();

/// Smart Contract. This struct contains the state in the VM.
/// Functions here are Smart Contract functions.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract{
    /// Counter.
    counter: i32
}

// Default is used for auto-initialization
impl Default for Contract{
    fn default() -> Self{
        Contract { counter: 0 }
    }
}

#[near_bindgen]
impl Contract{

    /// Returns the counter value
    pub fn get(&self) -> i32 {
        // return self.counter;
        self.counter
    }

    /// Increments the counter by 1
    pub fn increment(&mut self) -> i32 {
        self.counter += 1;
        self.counter
    }

    /// Decreases the counter by 1
    pub fn decrement(&mut self) -> i32 {
        self.counter -= 1;
        self.counter
    }
}


// Unit tests go here
// cfg(test) means this mod will be compiled when doing unit testings
#[cfg(test)]
mod tests{
    // super::* imports all modules
    use super::*;
    // import some near_sdk modules needed for these tests 
    use near_sdk::{
        // an account id, like "stiltztinkerstein.near"
        AccountId,
        // has functions related to the execution environment
        // e.g.: we wanted to know the user account that executed this contract
        // we would use a function found here
        env,
        // Mocks (simulates) the Blockchain
        MockedBlockchain,
        // Macro that sets up the test environment with a valid context
        testing_env,
        // Used to create a test context
        test_utils::VMContextBuilder,
        // A valid account id
        // An account id is a string, but the entire string does not represent the actual valid id
        json_types::ValidAccountId,
    };

    /// This function is not a test. It is used by the tests to setup our mock test environment
    fn env_setup(){
        // Initializes a context builder for our tests 
        let mut builder: VMContextBuilder = VMContextBuilder::new();

        // attributes that can be modified using the builder
        // current_account_id
        // signer_account_id
        // signer_account_pk
        // precessor_account_id
        // block_index
        // block_timestamp
        // epoch_height
        // account_balance
        // account_locked_balance
        // storage_usage
        // attached_deposit
        // prepaid_gas
        // random_seed
        // is_view

        // account id string
        let account_id: AccountId = String::from("stiltztinkerstein");

        builder.current_account_id(
            // try_from tries to convert a string to a valid account id
            // panics if the id is invalid
            ValidAccountId::try_from(
                account_id.clone()
            ).unwrap()
        );

        // sets up the mock
        testing_env!(builder.build());

        // if the first two arguments are not equal, 
        // return the error message
        assert_eq!(
            env::current_account_id(),
            account_id, 
            "Erro assert.\n env: {}\naccount: {}\n", 
            env::current_account_id(), 
            &account_id,
        );
    }


    /// As it is annotated as #[test] this will execute automatically
    /// when we do unit testing
    #[test]
    pub fn get() {
        env_setup();

        let contract: Contract = Contract::default();
        
        assert_eq!(
            contract.get(),
            0
        );
    }

    #[test]
    pub fn increment() {
        env_setup();

        let mut contract: Contract = Contract::default();

        contract.increment();

        assert_eq!(
            contract.get(),
            1
        );
    }

    #[test]
    pub fn decrement() {
        env_setup();

        let mut contract: Contract = Contract::default();

        contract.decrement();

        assert_eq!(
            contract.get(),
            -1
        );
    }
}
