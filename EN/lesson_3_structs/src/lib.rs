use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};


near_sdk::setup_alloc!();


// derive is a macro that generates code for the specifiec traits between ()
// we use it so the contract gets trait implementations of BorshDeserialize and BorshSerialize
// StructExample is the state of a contract, so we need these two traits in place
#[derive(BorshDeserialize, BorshSerialize)]
// A struct with primitive types
pub struct StructExample{
    an_integer: u32,
    another_integer: i32,
    a_floating: f32,
    a_string: String,
}


// default is executed when the code loads and prior to initialization
// it can be called manually to create an instance with default values
impl Default for StructExample{
    fn default() -> Self {
        // you can set field values using name: value
        let an_integer: u32 = 1;
        let a_floating: f32 = 0.5;

        // and here we return the struct with the default field values we specified
        StructExample {
            an_integer,
            another_integer: -1,
            a_floating,
            a_string: String::from("A default string"),
        }
    }
}

// Clone creates a copy of the struct
impl Clone for StructExample{
    // self is an instance of StructExample, and Self (uppercase) is the actual StructExample type.
    fn clone(&self) -> Self {
        let an_integer: u32 = self.get_an_integer();
        let another_integer: i32 = self.get_another_integer();
        let a_floating: f32 = self.get_a_floating();
        let a_string: String = self.get_a_string();

        // Self and StructExample are the same here
        Self {
            an_integer,
            another_integer,
            a_floating,
            a_string,
        }

    }
}

// Let's implement the functions for StructExample
impl StructExample{

    // &self means borrowing a StructExample instance, but it can't modify it
    /// Returns a String copy of the a_string field value
    pub fn get_a_string(&self) -> String {

        // String implements the Clone trait, but not Copy
        // Copy creates an instance (a copy) automatically with an instruction like a = b 
        // Clone creates an instance (a copy) when the code calls clone()
        let a_string: String = self.a_string.clone();

        return a_string;
    }

    pub fn get_a_floating(&self) -> f32 {
        // f32 implements Copy, so this will create a copy of a_floating automatically
        return self.a_floating;
    }

    pub fn get_another_integer(&self) -> i32 {
        // you dont' need to write the 'return' statement every time
        // if the line is the last one and it doesn't have ';' at the end, assume return
        self.another_integer
    }

    pub fn get_an_integer(&self) -> u32 {
        self.an_integer
    }

    // '&mut self' will borrow a StructExample instance and also make changes to it
    pub fn set_a_string(
        &mut self, 
        // For the following a_string_arg, the function will take ownership since we are not borrowing it using &
        // This means this function now owns this piece of memory and 
        // that code that called this function won't need this string again
        //
        // This is one of Rust's superpowers: It allows to minimze the amount of copies we need
        a_string_arg: String,
    ) {
        // as we borrow a mutable self, we can change its field values
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
        // i32 is signed, values can be positive or negative. This halves the max value allowed
        self.another_integer = another_integer;
    }

    // if the function doesn't have 'self', then it is a function associated with the type itself
    pub fn just_a_function() {
        env::log(b"You just called this function");
        env::log(format!("1 + 1 = {}", 1 + 1).as_bytes());
    }

    // if 'self' is an argument, the function will take ownership of the instance
    // this meand the struct will be dropped at the end of the function (unless we return it)
    // this is just an example, you probably won't ever need to do this
    pub fn take_ownership(self) -> u32{
        env::log(b"Taking ownership of itself");

        let result = format!("an_integer is {}", self.an_integer);
        env::log(result.as_bytes());

        self.an_integer

        // self will be dropped/free from memory here
    }
}


// #[near_bindgen] instructs near-sdk that this struct is the state of our Smart Contract
// we can use any name, but 'Contract' is easy to understand
#[near_bindgen]
// derive is a macro that generates the specified trait implementations on our StructExample
// we use it so our contract supports BorshDeserialize and BorshSerialize
// These traits define how data structures are translated into bytes which are needed for passing data
// into methods of the smart contract or storing data in state.
//
// For method parameters, JSON (default) and Borsh are supported.
// For storing data on-chain Borsh is used.
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {
    struct_example: StructExample,
}

#[near_bindgen]
impl Contract{
    // The functions below redirect to the struct functions. 
    // The difference being is that these ones below, represent how we interact with the actual Smart Contract
    
    
    // get_* functions need &self, and set_* need to make changes so we use &mut self
    // using '&self' for get_* functions
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

    // using 'mut &self' for get_* functions
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

    // just_a_function doesn't use '&self', '&mut self' or 'self'
    // so is is just a function in the same namespace as our struct
    pub fn just_a_function(){
        // you can use :: to reference items in namespaces. Our function here is not an instance of our struct. 
        StructExample::just_a_function();
    }

    pub fn take_ownership(&self) -> u32 {
        // A função take_ownership libera o objeto da memória.
        // The compiler will not let us call it directly when the owner is Contract.
        // O compilador não irá nos deixar chamar isso diretamente enquanto o dono for Contract.
        // Most of the issues of ownership can be solved by just cloning the object.
        // A maioria dos problemas de possessão (ownership) podem ser resolvidos apenas clonando objetos
        let a_copy: StructExample = self.struct_example.clone();

        // a_copy será removido no fim dessa função abaixo.
        let result = a_copy.take_ownership();

        // Descomente a linha abaixo e um erro de "move" irá aparecer.
        // Isso é porque chamamos uma função de uma instância que não existe mais.
        // env::log(format!("Essa linha de código irá gerar um erro {}", a_copy.get_a_floating()).as_bytes());

        result
    }
}


// cfg instructs the compiler to consider the 'tests' module as a test harness. You can then use 'cargo test' to use this module 
// for running functions with a #[test] attribute, automatically
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
    
        // test Value from the default implementation
        assert_eq!(
            contract.get_a_string(),
            "A default string"
        );
    }

    #[test]
    pub fn get_a_floating(){
        env_setup();
    
        let contract: Contract = Contract::default();
    
        // test Value from the default implementation
        assert_eq!(
            contract.get_a_floating(),
            0.5,
        );

    }

    #[test]
    pub fn get_another_integer(){
        env_setup();
    
        let contract: Contract = Contract::default();
    
        // test Value from the default implementation
        assert_eq!(
            contract.get_another_integer(),
            -1,
        );

    }

    #[test]
    pub fn get_an_integer(){
        env_setup();
    
        let contract: Contract = Contract::default();
    
        // test Value from the default implementation
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

        // We declare a Contract instance with default field values
        // You can use an underscore _ in front of a variable name if you know in advance
        // you are not going to use it immediately (that way the compiler won't complain)
        let _contract: Contract = Contract::default();

        
        // Note we use :: since this function doesn't require state
        Contract::just_a_function();
    }

    #[test]
    // take_ownership is just an example of another way to do things in Rust
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
