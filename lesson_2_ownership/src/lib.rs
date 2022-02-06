use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};


// Hint about documentation:
// comments with // doesn't show in generated docs.
// comments with /// show as description to the following mod, fn, struct, enum, trait...
// comments with //! can only be at the start of the file, and represents the description of the entire module.

near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    name: String,
}

impl Default for Contract{
    fn default() -> Self {
        // Giving it a starting string just as example
        return Contract {
            name: String::from("A default string"),
        };
    }
}


#[near_bindgen]
impl Contract{
    // &str is a reference to a string
    // strings with comma " " are 'static &str
    // so having &str in args accepts both &String and "any static string like this"
    /// Returns the length of a String.
    fn this_takes_a_reference(name: &str) -> usize { 
        return name.len();
    }

    // This does the same as above, but takes a String as arg
    // If we wanted to use a static string like "this one",
    // we would have to convert it to a String like this: String::from("this one")
    /// Returns the length of a String.
    fn this_takes_the_ownership(name: String) -> usize {
        // returns usize, usize is u32 in 32 bit systems and u64 in 64 bit systems
        name.len()
    }

    /// Return the length of stored String.
    pub fn get_length(&self) -> u32 {
        // will call both methods to show both do the same thing.
        //
        // Adding & before the arg variable name is the same as saying: 
        // "I'm giving permission for this function to look at this value, but I am not giving permission for it to be changed".
        let length_reference: usize = Self::this_takes_a_reference(&self.name);

        // this_takes_the_ownership wants to own a String, so we have to create a new copy for it.
        let length_ownership: usize = Self::this_takes_the_ownership(self.name.clone());

        // Calling assert_eq to prove that both values are the same,
        // if values are different, panic
        assert_eq!(
            // first arg to compare
            length_reference, 
            // second arg to compare
            length_ownership, 
            // if both are not equal, panic with this error message
            "Both lengths are not the same {} and {}", length_reference, length_ownership,
        );

        // lets return a u32 because it will be converted to json automatically
        // types can be converted using the into and from traits too
        length_reference as u32
    }

    /// Return the length of stored String. Changes stored name to "Changed name"
    pub fn get_length_again(&mut self) -> u32 {
        // we can also declare variables that store references to a value elsewhere
        let a_reference: &String = &self.name;
        let _another_reference: &String = &self.name;
        let _yet_another_reference: &String = &self.name;

        // We can have several immutable references at the same time.
        // But we can't mutate the variable while immutable references exist
        // If we need to get a mutable reference, there must be no immutable references existing.

        // Uncomment the following line to get an error due to existing references.
        // self.name = String::from("Changed name");

        let length = Self::this_takes_a_reference(a_reference);

        // The following line is ok though, because the references above are not used again.
        // So the compiler knows it can free them back there.
        self.name = String::from("Changed name");

        length as u32
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use near_sdk::{
        AccountId,
        env,
        MockedBlockchain,
        testing_env,
        test_utils::VMContextBuilder,
        json_types::ValidAccountId,
    };

    fn env_setup(){
        let mut builder: VMContextBuilder = VMContextBuilder::new();

        // attributes we can set with the builder:
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

        let account_id: AccountId = String::from("stiltztinkerstein");

        builder.current_account_id(
            ValidAccountId::try_from(
                account_id.clone()
            ).unwrap()
        );

        testing_env!(builder.build());

        assert_eq!(
            env::current_account_id(),
            account_id, 
            "Assert Error. env: {} account: {}", 
            env::current_account_id(), 
            &account_id,
        );
    }

    #[test]
    pub fn get_length() {
        env_setup();
    
        let mut contract: Contract = Contract::default();
    
        // both functions do the same thing, so both should return the same value
        assert_eq!(
            contract.get_length(),
            contract.get_length_again()
        );

        // get_length_again also changes the stored string
        assert_eq!(
            contract.name,
            "Changed name"
        );
    }
}