use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};


// A Macro that generates boilerplate code for NEAR. For v.4 setup_alloc() will be deprecated. 
near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    name: String,
}

impl Default for Contract{
    fn default() -> Self {
        // Initialize name with a value of "A default string"
        return Contract {
            name: String::from("A default string"),
        };
    }
}


#[near_bindgen]
impl Contract{
    // &str is a string reference
    // strings between " " are string literals or 'static &str
    // and having a &str as a parameter allows for &String as well as String literals such as "a string like this"
    /// Returns the length of a string
    fn this_takes_a_reference(name: &str) -> usize { 
        return name.len();
    }

    // This functions does the same as the previous one, but uses a String parameter
    // We can create a String by using the function String::from("our text")
    /// Returns the size of a string
    fn this_takes_the_ownership(name: String) -> usize {
        // return usze, which is u32 in 32-bit systems, and u64 in 64-bit sytems
        name.len()
    }

    /// Returns the size of the string stored in the name variable
    pub fn get_length(&self) -> u32 {
        // Let's call both functions to show they do the same thing
        //
        // Borrowing: Adding & to each argument is allowing the function to use the variable's value, but not modify it
        let length_reference: usize = Self::this_takes_a_reference(&self.name);

        // Ownership: this function takes ownership of the string, and therefore we need to clone it 
        // notice there's no &
        let length_ownership: usize = Self::this_takes_the_ownership(self.name.clone());

        // Let's use assert_eq to test both functions returned the same value
        // if they are different, the code will panic
        assert_eq!(
            // first value to compare
            length_reference, 
            // second value to compare
            length_ownership, 
            // if they're not equal, panic with the provided message
            "The are not the same size {} and {}", length_reference, length_ownership,
        );

        // Convert to u32, this is simple format for json serialization
        // types can be converted using traits, by implementing "into" and "from"
        length_reference as u32
    }


    /// Return the size of the string stored in the name variable, but change its value
    pub fn get_length_again(&mut self) -> u32 {
        // we can declare variables that store references to another value
        let a_reference: &String = &self.name;
        let _another_reference: &String = &self.name;
        let _yet_another_reference: &String = &self.name;


        // We can have many immutable references
        // but we can't change a variable's value while these references exist
        // If we needed a mutable reference, you can't have any existing immutable references

        // Uncomment the following line to raise an error: existing borrow
        // self.name = String::from("Changed name");

        let length = Self::this_takes_a_reference(a_reference);

        // this next line is Ok, since a reference is no longer used (borrow)
        // and the compiler can drop the borrow
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

        // fields that can be changed via the builder
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
            "Erro assert. env: {} account: {}", 
            env::current_account_id(), 
            &account_id,
        );
    }

    #[test]
    pub fn get_length() {
        env_setup();
    
        let mut contract: Contract = Contract::default();
    
        // Both functions to the same thing, so both must return the same value
        assert_eq!(
            contract.get_length(),
            contract.get_length_again()
        );

        // get_length_again also modified the stored string value or the name field
        assert_eq!(
            contract.name,
            "Changed name"
        );
    }
}