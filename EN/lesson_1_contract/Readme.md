# Lesson 1: Smart Contracts

[back](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/)

Also see:
 - Using [cargo](https://github.com/On0n0k1/Tutorial_NEAR_Rust/blob/main/PT-BR/static/tutorials/cargo.md).
 - Using [near-cli](https://github.com/On0n0k1/Tutorial_NEAR_Rust/blob/main/PT-BR/static/tutorials/nearcli.md).

---

## Topics
 - [Structure of a NEAR Smart Contract](#structure-of-a-near-smart-contract)
 - [Importing Dependencies](#importing-dependencies)
 - [Allocation macro](#allocation-macro)
 - [Smart Contract declaration](#smart-contract-declaration)
 - [Smart Contract API](#smart-contract-api)
 - [Unit tests](#unit-tests)

---

## Structure of a NEAR Smart Contract
[top](#topics)

Creating a NEAR Smart Contract in Rust can be summarized as:
 - Import crates, modules and other needed dependencies.
 - Allocation Macro (sdk 3.x, but not 4.x)
 - Smart Contract code.
 - Smart Contract API.
 - Unit tests.

The developer is free to add anything to the above list as needed; the steps outlined are just to help memorize the basic steps required to start.

---

### Importing Dependencies
[top](#topics)

Explained in detail in Lesson #4, we need at this point to know the differences between `use` and `mod`. 

```rust
use near_sdk::near_bindgen;
```

Access the `near_sdk` crate and include the macro `near_bindgen` in this namespace. Without this, we would need to write `near_sdk::near_bindgen` every time we needed the macro! Now, we can just write instead a shorter `near_bindgen`. 

Now, let's go over `mod`: 

```rust
mod another_module;
```

This statement means there's a file called "another_module.rs" or a directory with a name "another_module", located in the same place as this Rust file.

If you see a `pub` modifier before the statement, like this:

```rust
pub mod another_module;
```

Or this:

```rust
mod another_module;

pub use another_module::some_dependency;
```

Then it means "another_module" or "some_dependency" can be imported by another external module or crate. 
Importing and Exporting modules is a Rust feature and doesn't have anything to do with the NEAR platform per-se. 

As for Smart Contracts: 

```rust
use near_sdk::{
    borsh::{
        self,
        BorshDeserialize,
        BorshSerialize,
    },
    near_bindgen,
};
```

We are accessing a crate `near_sdk` declared in `Cargo.toml`, and then we're importing `self`, `BorshDeserialize` and `BorshSerialize` from the borsh module. We're also importing `near_bindgen`. 

 - `self`: Needed for BorshDeserialize and BorshSerialize to work correctly.
 - `BorshDeserialize`: When we call a function in our Smart Contract, we sometimes need to provide arguments. Unless they are an empty JSON, these arguments need to be deserialized. This is what BorshDeserialize does: converts from JSON to an actual type we can use.
 - `BorshSerialize`: The reverse of BorshDeserialize. When we want to send back a result, we need to convert from a type or value to valid JSON. 
 - `near_bindgen`: An annotation (actually, a macro) used on a struct to indicate that **"This is a Smart Contract"**. We need to have at least one `struct` annotated with `near_bindgen` for each contract.

---

### Allocation Macro
[top](#topics)

```rust
near_sdk::setup_alloc!(); // (sdk v.3.x)
```

Macros look like functions, however, they are not executed prior to compilation, but rather used as code generators that can be given configuration arguments. Macros don't exist on the program's final binary output. 

In this case, `setup_alloc` generates the necessary boilerplate code so our Smart Contract can work. This macro only runs once, before the Smart Contract declaration.

:warning: **Heads up**: Adding `setup_alloc` applies to **version 3.x** of the NEAR SDK. Starting from v.4.x you might not need it. Be sure to double check the official NEAR SDK documentation.

---

### Smart Contract declaration
[top](#topics)

```rust
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract{
    counter: i32,
}
```
This code might be easier to explain from the inside-out: 
 - `counter`: is a number. the "i" in i32 means "signed", so it can be either positive or negative. 32 in this case is the number of bits it has.
 - `pub struct Contract`: a struct declaration that specifies the name of the Smart Contract. "pub" means this Smart Contract is publicly accessible.
 - `#[derive(BorshDeserialize, BorshSerialize)]`: Apply the _traits_ BorshDeserialize and BorshSerialize to this struct. Think for now of _traits_ as similar to the concept of interfaces (behavior-wise).
 - `[near_bindgen]`: An annotation that indicates "this is a Smart Contract". The functions on this struct are also Smart Contract functions. When we execute a Smart Contract function, we are basically executing a function on this struct. 

Next, we have: 
```rust
impl Default for Contract{
    fn default() -> Self{
        Contract { counter: 0 }
    }
}
```

`Default` is a "pattern" trait. Think of it as a default constructor (a constructor without parameters) for our struct. `Default` is there to provide a type with a useful default value. `near_sdk` implements this trait for our Smart Contracts, so we need to apply it or we might run into a compilation error. 

`default` is a function of the `Default` trait that returns a struct of the same type. _`Self`_ refers to the Smart Contract itself. The functions return an instance of `Contract` with a `counter` value of `0`.

If we deploy this contract to a NEAR Account, and then we execute a contract function (one that is not an initializer), NEAR's virtual machine would initialize the contract using `default` before executing our contract's function.

---

### Smart Contract API
[top](#topics)

Now, we've come to the actual functions of our Smart Contract.

```rust
#[near_bindgen]
impl Contract{

    pub fn get(&self) -> i32 {
        self.counter
    }

    pub fn increment(&mut self) -> i32 {
        self.counter += 1;
        self.counter
    }

    pub fn decrement(&mut self) -> i32 {
        self.counter -= 1;
        self.counter
    }
}
```

 - `#[near_bindgen]` is an annotation that indicates "these are the functions of the contract".
 - `impl Contract` is where we declare the functions for our Smart Contract.
 - `&self` and `&mut self` are described in the next lesson. For now, we just need to know that for this type of function that follows the "dot call convention" of `struct_name.function_name`, _`self`_ refers to an instance of the struct.
 - `-> i32` means the return value is a signed 32-bit integer.
 - At the end of the function, the **last line** `self.counter` doesnt end with `;`. the Rust compiler knows how to handle this, as it is the same as having a complete `return self.counter;` statement.

We can now see that the function `get` returns the actual value of `counter` which is being stored on the Smart Contract's `struct` data structure. As for our two contract functions, `increment` adds one to the counter's value, while `decrement` subtracts one from the counter's value.

---

### Unit tests
[top](#topics)

We'll deep dive in more detail in Lesson #4, as we don't need to have all our unit tests for now. We can include unit tests at the end of each Rust module. We can also create a directory `tests`, where all files ending in `.rs` will be considered tests in a test module. 

```rust
#[cfg(test)]
mod tests {
```

`mod tests` is simply a module with a name of `tests`. Nothing special!

`#[cfg(test)]` is quite interesting. `cfg` is a compiler instruction that tells it to "compile the following module if the condition between parenthesis is true". In our case, `(test)` will be true when we run `cargo test`. If we are not running unit tests, this module won't be compiled. 

if, instead of `#[cfg(test)]`, we had:

```rust
#[cfg(not(test))]
mod another_module {
```
Then we would have the opposite scenario, where `another_module` won't be compiled when doing unit testing.

Let's see how we go about unit testing: 
```rust
use super::*;
use near_sdk::{
    AccountId,
    env,
    MockedBlockchain,
    testing_env,
    test_utils::VMContextBuilder,
    json_types::ValidAccountId,
};
```
We first need to import the above dependencies for our unit tests. And now:   

```rust
fn env_setup(){
    let mut builder: VMContextBuilder = VMContextBuilder::new();
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
        "Error.\n env: {}\naccount: {}\n", 
        env::current_account_id(), 
        &account_id,
    );
}
```

Before each unit test, we need to set up a mock blockchain environment. One way is to use `VMContextBuilder`, as we just use it to create and configure the desired mock environment, and then use `builder` (which is a VMContextBuilder instance) as an argument to the `testing_env` macro. 

So we don't have to write this code on each test, we can create a function that is re-used.

`assert_eq` isn't really necessary. It just checks the environment variable `env::current_account_id` is the same as the account id specified for the `builder`.

We have three tests:
```rust
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
```

You can probably see there's a pattern in these tests: 
 - Set up the environment.
 - Initialize the contract. 
 - Execute (exercise) the function we want to test. 
 - Confirm that the function returns the value we expected (or not). 

The function `get` was tested first and this is because it will be used in the following tests. If this function would have not passed the test, we would have to fix it first, before moving on to the other functions. 

Lesson 1 :white_check_mark: ... **Done! Congratulations!**

Our [next lesson](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_2_ownership) will be about Rust's concept of **Ownership**.
