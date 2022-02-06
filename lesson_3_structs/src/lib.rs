//!

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};


near_sdk::setup_alloc!();


// derive implements a macro for the given traits
// We implement this because the Contract needs BorshDeserialize and BorshSerialize
// StructExample is used in Contract, so we declare it here too.
#[derive(BorshDeserialize, BorshSerialize)]
// A struct with some primitive types
pub struct StructExample{
    an_integer: u32,
    another_integer: i32,
    a_floating: f32,
    a_string: String,
}


// default is used when the code is first deployed
// It can be called manually to create a new instance with these default values
impl Default for StructExample{
    fn default() -> Self {
        // we can set the value with name: value
        // or we can declare a variable with the same name and the value
        let an_integer: u32 = 1;
        let a_floating: f32 = 0.5;


        StructExample {
            an_integer,
            another_integer: -1,
            a_floating,
            a_string: String::from("A default string"),
        }
    }
}

// Clone is just a way to manually create a copy of this struct.
impl Clone for StructExample{
    // self is the instance of StructExample, Self (Capital letter) is the type StructExample.
    fn clone(&self) -> Self {
        let an_integer: u32 = self.get_an_integer();
        let another_integer: i32 = self.get_another_integer();
        let a_floating: f32 = self.get_a_floating();
        let a_string: String = self.get_a_string();

        // Self and StructExample are the same thing (In any impl for StructExample)
        Self {
            an_integer,
            another_integer,
            a_floating,
            a_string,
        }

    }
}


// This is where methods and functions for struct are implemented
impl StructExample{
    // This method returns a copy of a_string
    // &self means it will borrow the StructExample instance, but will not mutate it.
    pub fn get_a_string(&self) -> String {

        // Strings implement the trait Clone but doesn't implement Copy
        // Copy creates a copy of the instance automatically
        // Clone creates a copy of the instance when the code calls clone()
        let a_string: String = self.a_string.clone();

        return a_string;
    }

    pub fn get_a_floating(&self) -> f32 {
        // f32 implements copy, so this line 
        // will automatically create a copy of a_floating
        return self.a_floating;
    }

    pub fn get_another_integer(&self) -> i32 {
        // We don't need to type "return" all the time
        // If the instruction doesn't end with semi-colon ";" returns the expression
        self.another_integer
    }

    pub fn get_an_integer(&self) -> u32 {
        self.an_integer
    }

    // &mut self means that it will borrow this instance of StructExample and make changes to it
    pub fn set_a_string(
        &mut self, 
        // important detail: since there's no reference & for the String, we are taking ownership of it.
        // Which means this function now owns this piece of memory. The processor didn't spent time and memory creating another String with this value.
        // Owning a_string_arg means that the code block that called it won't use this String again.
        //
        // That's one of rust's superpowers. We can create code that create the least number of copies possible.
        a_string_arg: String,
    ) {
        // since we borrowed mutable, we can change internal values
        self.a_string = a_string_arg;
    }

    pub fn set_a_floating(&mut self, arg: f32) {
        self.a_floating = arg;
    }

    pub fn set_an_integer(&mut self, an_integer: u32) {
        // u32 is unsigned, only positive values
        self.an_integer = an_integer;
    }

    pub fn set_another_integer(&mut self, another_integer: i32){
        // i32 is signed, can be positive and negative. Half the max range though.
        self.another_integer = another_integer;
    }

    // if the method doesn't have self, it's just a regular function associated to the type
    pub fn just_a_function() {
        env::log(b"You just called this function");
        env::log(format!("1 + 1 = {}", 1 + 1).as_bytes());
    }

    // If self is in the arguments, the function is taking ownership of the type.
    // Which means the struct will be deleted at the end of the function (unless we return it).
    // This is just to explain how it works. You will almost never want to implement a function like this.
    pub fn take_ownership(self) -> u32{
        env::log(b"Taking ownership of itself");

        let result = format!("an_integer is {}", self.an_integer);
        env::log(result.as_bytes());

        self.an_integer

        // self will be freed from memory here
    }
}


//#[near_bindgen] Tells near-sdk that this represents the main state of the contract
// We could name this struct anything, Contract is just easier to tell.
#[near_bindgen]
// derive implements a macro for the given traits
// We implement this because the Contract needs BorshDeserialize and BorshSerialize
// BorshSerialize converts our return types into json
// BorshDeserialize converts the input args into the types we need for our function calls.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    struct_example: StructExample,
}


// In regular rust we don't have to implement default
// But in Near we do because it's run when the contract is deployed (and not initialized)
impl Default for Contract{
    fn default() -> Self{
        let struct_example = StructExample::default();

        Contract { struct_example }
    }
}


#[near_bindgen]
impl Contract{
    // The following are the same functions from the struct being called here
    // The difference being that these are how we interact with the contract.
    //
    // The method for get_a_string requires &self, so we can use &self or &mut self
    pub fn get_a_string(&self) -> String {
        self.struct_example.get_a_string()
    }

    pub fn get_a_floating(&self) -> f32 {
        self.struct_example.get_a_floating()
    }

    pub fn get_another_integer(&self) -> i32 {
        self.struct_example.get_another_integer()
    }

    pub fn get_an_integer(&self) -> u32 {
        self.struct_example.get_an_integer()
    }

    // The method for set_a_string requires &mut self, so we can only use &mut self
    pub fn set_a_string(&mut self, a_string_arg: String) {
        self.struct_example.set_a_string(a_string_arg);
    }

    pub fn set_a_floating(&mut self, a_floating: f32) {
        self.struct_example.set_a_floating(a_floating);
    }

    pub fn set_an_integer(&mut self, an_integer: u32) {
        self.struct_example.set_an_integer(an_integer);
    }

    pub fn set_another_integer(&mut self, another_integer: i32) {
        self.struct_example.set_another_integer(another_integer);
    }

    // Just_a_function doesn't have &self, &mut self or even self
    // It's just a function in the namespace of that struct. We don't need to use self as well.
    pub fn just_a_function(){
        // :: is used for referencing to namespaces. It's not an instance method, just a function.
        StructExample::just_a_function();
    }

    pub fn take_ownership(&self) -> u32 {
        // The function take_ownership frees the object from the memory.
        // The compiler will not let us call it directly when the owner is Contract.
        // Most of the issues of ownership can be solved by just cloning the object.
        let a_copy: StructExample = self.struct_example.clone();

        // a_copy will be removed at the end
        let result = a_copy.take_ownership();

        // Uncomment the line below and a move error will show up.
        // That's because we called a function from an instance that doesn't exist anymore.
        // env::log(format!("This line of code will generate an error warning {}", a_copy.get_a_floating()).as_bytes());

        result
    }
}


// cfg tells the compiler to only consider this module in a test environment. It doesn't exist otherwise.
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
        //
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
            "Assert Error. env: {} account: {}
            ", 
            env::current_account_id(), 
            &account_id,
        );
    }

    #[test]
    pub fn get_a_string() {
        env_setup();
    
        let contract: Contract = Contract::default();
    
        // This value is the one from default implementation,
        assert_eq!(
            contract.get_a_string(),
            "A default string"
        );
    }

    #[test]
    pub fn get_a_floating(){
        env_setup();
    
        let contract: Contract = Contract::default();
    
        // This value is the one from default implementation,
        assert_eq!(
            contract.get_a_floating(),
            0.5,
        );

    }

    #[test]
    pub fn get_another_integer(){
        env_setup();
    
        let contract: Contract = Contract::default();
    
        // This value is the one from default implementation,
        assert_eq!(
            contract.get_another_integer(),
            -1,
        );

    }

    #[test]
    pub fn get_an_integer(){
        env_setup();
    
        let contract: Contract = Contract::default();
    
        // This value is the one from default implementation,
        assert_eq!(
            contract.get_an_integer(),
            1,
        );
    }

    #[test]
    pub fn set_a_string() {
        env_setup();

        let mut contract: Contract = Contract::default();

        contract.set_a_string(String::from("A new string"));

        assert_eq!(
            contract.get_a_string(),
            String::from("A new string"),
        );
    }

    #[test]
    pub fn set_a_floating() {
        env_setup();

        let mut contract: Contract = Contract::default();

        contract.set_a_floating(-10.5432);
        
        assert_eq!(
            contract.get_a_floating(),
            -10.5432,
        );
    }

    #[test]
    pub fn set_an_integer() {
        env_setup();

        let mut contract: Contract = Contract::default();

        contract.set_an_integer(5);

        assert_eq!(
            contract.get_an_integer(),
            5,
        );
    }

    #[test]
    pub fn set_another_integer() {
        env_setup();

        let mut contract: Contract = Contract::default();

        contract.set_another_integer(7);

        assert_eq!(
            contract.get_another_integer(),
            7
        );
    }

    #[test]
    pub fn just_a_function(){
        env_setup();

        // We declare this to start the contract, but we don't need to use it's state here
        // Start the variable with _ if you never intend to use it.
        let _contract: Contract = Contract::default();

        // Notice the :: due to the function not needing any state.
        Contract::just_a_function();
    }

    #[test]
    // take_ownership is just an example for a gimmick in rust. Not much use to it.
    pub fn take_ownership(){
        env_setup();

        let contract: Contract = Contract::default();

        let an_integer = contract.get_an_integer();

        assert_eq!(
            contract.take_ownership(),
            an_integer,
        );
    }

}
