# Lesson 3: Structs

[back](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/)

Let's go over `struct` and how ownership is used in our struct functions.

---

## Topics

 - [Introduction](#introduction)
 - [Contract functions](#contract-functions)
 - [Project](#project)
   - [Contract Structure](#contract-structure)
   - [`Clone` trait](#clone-trait)
   - [`just_a_function`](#just_a_function)
   - [Macros](#macros)
   - [`take_ownership`](#take_ownership)

---

## Introduction
[back](#topics)

A `struct` is similar to a class in other object-oriented programming languages. The difference is that a `struct` doens't support inheritance, but we can implement one or more traits on them, and these traits specify behavior. 

We'll dive into traits in future lessons, but for now, consider traits as sets of functions that represent behavior supported by types. The trait `Clone` allows using the `.clone()` function to create copies of an instance. The trait `BorshDeserialize` lets you build an instance of a type by using a JSON formatted string. 

Later on, we'll learn how to create trait functions that can be applied to any type. For now, we'll just focus on structs.

---

## Smart Contract functions
[top](#topics)

```rust
// gets and sets
pub fn get_a_string(&self) -> String;

pub fn get_a_floating(&self) -> f32;

pub fn get_another_integer(&self) -> i32;

pub fn get_an_integer(&self) -> u32;

pub fn set_a_string(&mut self, a_string_arg: String);

pub fn set_a_floating(&mut self, a_floating: f32);

pub fn set_an_integer(&mut self, an_integer: u32);

pub fn set_another_integer(&mut self, another_integer: i32);

// A function that doesn't change the contract's state
pub fn just_a_function();

// A function using StructExample that takes ownership of itself and is dropped at the end
pub fn take_ownership(&self) -> u32;
```
See their implementations for details.

---

## Project

[top](#topics)

We'll first create a `struct` called `StructExample`.

```rust
pub struct StructExample{
    an_integer: u32,
    another_integer: i32,
    a_floating: f32,
    a_string: String,
}
```
This type has the following fields: 
 - `an_integer`: an unsigned 32-bit integer.
 - `another_integer`: a signed 32-bit integer.
 - `a_floating`: a floating 32-bit number.
 - `a_string`: a String, described in the previous lesson.

In other languages, we might have to write `long int` for i32, or `long long int` for i64. However, in Rust, we just need to specify the `i` for "signed" (positive or negative) and `u` for "unsigned" (positive). All types such as `u8`, `u16`, `u32`, `u64` and `u128` are all valid "unsigned" types.

---

### Contract Structure

[top](#topics)

Here's the Smart Contract code:

```rust
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {
    struct_example: StructExample,
}
```
See how this Smart Contract is annotated with `derive`? When you `derive`, it means the compiler is able to provide a basic implementation for some traits, but you can manually implement them if more complex behavior is needed. Here, `derive` for `Default` will make sure **all fields** of `Contract` have a `.default()` function. This also means that `StructExample` must also implement the `Default` trait. 

:hand: Be sure to read about [Serialization Protocols](https://www.near-sdk.io/contract-interface/serialization-interface) if you want a deeper understanding of how BorshSerialize and BorshDeserialize works in NEAR. 

```rust
impl Default for StructExample{
    fn default() -> Self {
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
```
We used some random values for this example. We also don't need to specify `an_integer: an_integer` or `a_floating: a_floating` when the names of the variables are the same.

---

### `Clone` trait
[top](#topics)

Let's implement the `Clone` trait on our `StructExample`:

```rust
impl Clone for StructExample{
    // self is an instance of StructExample, Self (uppercase) is of type StructExample.
    fn clone(&self) -> Self {
        let an_integer: u32 = self.get_an_integer();
        let another_integer: i32 = self.get_another_integer();
        let a_floating: f32 = self.get_a_floating();
        let a_string: String = self.get_a_string();

        // Self and StructExample are the same
        Self {
            an_integer,
            another_integer,
            a_floating,
            a_string,
        }
    }
}
```
:hand: **NOTE:** Remember, I'm intentionally writing code in a more complex way just to show the different ways our implementation could be made. 

There really isn't much to say on `get` and `set` functions, you can just check the comments. 

Let's go over `just_a_function` and `take_ownership`:

---

### just_a_function

[top](#topics)

```rust
pub fn just_a_function() {
    env::log(b"You just called this function");
    env::log(format!("1 + 1 = {}", 1 + 1).as_bytes());
}
```
This function outputs two lines of text. 

The `log` function receives a sequence of bytes as an argument. 
So, in our first call, you can use "b" as a way to indicate that the following string should be treated as bytes. 

The second time, we use the macro `format!` to format a String. The String type has a function `.as_bytes()` that converts its value to bytes. If you want to learn more, then be sure to read about [as_bytes()](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes).

---

### Macros
[top](#topics)

For now, let's consider a **macro** as a function that will execute prior to code being compiled. These functions generate code for you. After the code is generated, the compiler runs and error checking happens. The most common scenario for a **macro** is to allow for functions with a variable number of parameters. 

Another way to see macros would be as a way to trade code complexity for ease of use. 

---

### take_ownership
[top](#topics)

```rust
pub fn take_ownership(self) -> u32{
    env::log(b"Taking ownership of itself");

    let result = format!("an_integer is {}", self.an_integer);
    env::log(result.as_bytes());

    self.an_integer

    // self will be dropped / freed here
}
```

This is an interesting piece of code: 
 - Prints "Taking ownership of itself" on screen. 
 - Prints the value of `an_integer`, which is a contract variable. 
 - Finally, returns the value of `an_integer`.

However, as we used `self` instead of `&self`, as well as `&mut self` as an argument, this function will take ownership of itself and will "self-destruct" after finishing execution. 

:hand: **NOTE:** a beginner will probably get a confusing error from the compiler while attempting to write the code above, such as "value used here after move".


---
Lesson 3 :white_check_mark: ... **Done! Congratulations!**

Our [next lesson](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_4_modules) will be about Rust's modules.