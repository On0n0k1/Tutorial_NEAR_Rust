# Lesson 5 - Macros

[back](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/)

Macros are a fundamental tool if you want to be an effective Rust developer. While it is not necessary to learn how to create them, you definitely need to learn how to use them.

---

## Smart Contract API

```rust
/// This function shows the differences between println and env::log
/// Run `cargo test -- --nocapture`, compile, deploy and run in NEAR.
/// Note how some messages will show up and some will not. 
pub fn print_examples();

/// Format examples. Check the output depending on how it is implemented.
pub fn format_examples();

/// Panic examples.
pub fn panic_example();

/// Using Vec (vectors) examples
pub fn vec_examples();
```

## Topics

 - [What is a Macro?](#what-is-a-macro)
 - ["function-like" Macros](#"function-like"-macros)
 - [Advantages](#advantages)
 - [Disadvantages](#disadvantages)
 - [Useful Macros](#useful-macros)
   - [format, println and panic](#format-println-and-panic)
   - [Compound types](#compound-types)
   - [setup_alloc](#setup_alloc)
 - [Extra: String and str](#extra-string-and-str) 

---

## What is a Macro?

[top](#topics)

Quite simply, a **macro** is a tool run at compile time that generates code. They're part of something called _metaprogramming_, which helps you write less code, which in turn also decreases the amount of code you have to maintain down the road.

The annotation `derive` is a Macro: 

```rust
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
```
In this case, the `derive` annotation is used to apply a trait to new types. 

There's another type of Macro, in which you annotate types with "attributes":

```rust
#[near_bindgen]
impl Contract{
```

Finally, let's go over some "function-like" macros. As the name implies, these resemble a function call where you can even provide arguments. Some examples would be:

```rust
// Prints a string using the provided format pattern
println!("{}", message);

// Format arguments as a String
format!("7");

// Panic and provide an error message formatted to match the given pattern
panic!("Panic with arguments: {} {} {}", 1, second, 3);

// Creates a Vec with the the list of arguments
vec![1, 2, 3, 4];

// Create boilerplate code for Smart Contract
near_sdk::setup_alloc!();

// Used in Tests.
// Uses the builder argument to create a Context for the virtual machine environment
testing_env!(builder.build());

``` 
---

## "function-like" Macros

[top](#topics)

These macros are used like functions, but there are some differences between them. For one, they are more flexible since they can take an unknown number of arguments. 

Let's use `vec` to see another difference: 

```rust
vec![1, 2, 3, 4];
```
`Vec!` uses square brackets `[]` instead of parentheses `()`, but remember that macros also use curly brackets `{}` in their definition. Macros can use any type of text inside these delimiters, and a developer can pass anything as an argument as long as the macro implementation understands it. 

---

## Advantages

[top](#topics)

 - Simplifies code.
 - They're run at compilation time, so if they are well implemented, they are very efficient and carry little or no overhead. 

---

## Disadvantages

[top](#topics)

 - A developer need to take a close look at the documentation for each macro that is in the code. 
 - They can make debugging harder. 
 - They increase overall compilation time.
 - They can "blow up" your code base with "ghost" (invisible) code. 

---

## Useful Macros

Here are some macros we find very useful.

### `format!`, `println!` and `panic!`

[top](#topics)

 - `format!` returns a formatted String.
 - `println!` prints a String using a formatting pattern.
 - `panic!` stops execution and prints a String as an error message.

```rust
println!("This is println!, it wont show up in the virtual machine");

// We create a "message" variable of type String with a value of "format returns a formatted string."
let message: String = format!("remember format {}", "returns a formatted string");

// Stops execution with an error message "Panic with some arguments: 1 2 3"
let second = 2;
panic!("Panic with some arguments: {} {} {}", 1, second, 3);

```
Using these macros is quite simple: your first argument would be a formatting string, which includes placeholders "{}" for arguments you'll be providing as arguments later. 

You can customize string formatting in several ways, so be sure to learn more about [formatting](https://doc.rust-lang.org/std/fmt/index.html). Here are some examples:

```rust
// format Examples
log("\n\nformat_examples:\n");
 
let message: String = format!("Format returns a formatted string");

let an_arg = "third";

// format! can receive sequential arguments using {}. 
let message = format!("format can receive argument using {{}}: {}, {}, {}.", 1, "second", an_arg);

let (first, second, third) = (1, "second", an_arg);

// we can specify arguments by using their names
let message = format!("We can also specify argument by variable names: {first}, {second}, {third}.");

// we can also specify them by position
let message = format!("We can also specify them by position: {1}, {2}, {0}.", third, first, second);

let (first, second, third) = (1, 2, 3);
// we can also specify formatting options, in this case, number-formatting (digits)
let message = format!("We can specify digits for numbers: {:02}, {:04}, {:6}.", first, second, third);

// specifying position and number-formatting
let message = format!("And specify digits and ordering: {2:02}, {0:4}, {1:06}.", second, third, first);

let (first, second, third) = (0.1, 1.23, -2.45);
// we can also specify precision for floating points
let message = format!("We can specify precision for floating points: {:.2}, {:.4}, {:.6}", first, second, third);

// specify precision and number of digits
let message = format!("And specify precision and number of digits: {:2.2}, {:04.4}, {:06.6}", first, second, third);

// specify precision, number of digits and argument position
let message = format!("We can specify precision, digits and ordering of arguments: {1:02.2}, {2:4.4}, {0:06.6}", third, first, second);

// same as the previous one, but using argument names
let message = format!("Same as the previous one, but using argument names: {first:2.2}, {second:04.4}, {third:6.6}");

```

---

### Compound types

[top](#topics)

Let's go over some ways to group data (formally called compound types).

Tuples are fixed-length (they cannot grow or shrink):

```rust
// an integer tuple
// you can have different types for each of the tuple's elements
// tuples use ()
let a_tuple: (u32, u32, u32) = (0, 1, 4);

// we can access tuple values by index number
println!("The 2nd value is {}", a_tuple.1);
```

Arrays are also fixed-length, and are stored on the stack. 
Unlike tuples, every element of an array must have the same type.

```rust
// declaring an array
// arrays use []
let an_array = [0, 1, 2];

// we access array values using indices
// remember we start at 0
println!("The 3rd value is {}", an_array[2]);

// here we declare an array with 10 integers that have a default value of 0
let mut another_array: [i32; 10] = [0; 10];

// let's change the first element, at index 0
another_array[0] = -1;

// let's get the first element's value, which we changed from 0 to -1 above
println!("The first element's value is {}", another_array[0]);
```
Arrays and Tuples are primitive types, and always keep in mind that these are fixed length. If we needed more flexibility, we can use collections. We have Rust collections and NEAR collections; the former are a part of the language, while the latter are stored on a data structure called a ["trie"](https://en.wikipedia.org/wiki/Trie). You need to learn about Rust collections in order to come up with quality code and logic, and you need to learn NEAR collections to come up with the most efficient way to save state on the blockchain.


 - Learn about [Rust collections]((https://doc.rust-lang.org/std/collections/)).
 - Learn about [NEAR collections](https://docs.rs/near-sdk/latest/near_sdk/collections/index.html) (use these when developing Smart Contracts).

Probably the most useful (and used) collection in Rust is `Vec`. [Learn more](https://doc.rust-lang.org/std/vec/struct.Vec.html) about it. Using this type, we can store data, known as elements, count them, access and change them too. 

:hand: **NOTE:** lowercase `vec!` is a macro to generate vectors (note the ! at the end), while uppercase `Vec` is a type (a struct actually).

We can create a vector specifying explicit values: 
```rust
// Vec with integers 1 2 3 4
let example = vec![1, 2, 3, 4];
```

We can also create a vector specifying the default value and the quantity of elements we want:
```rust
// Vec with 5 elements, default value of 0, which is then vector [0, 0, 0, 0, 0]
let example = vec![0;5];
```
Formatting and printing values can be a bother, but Rust comes to our rescue by providing out-of-the-box solutions with Debug and Pretty-print formatting utilities that work with `println!`, `format!` and `panic!`. 

```rust
let example = vec![1, 2, 3, 4];

// using debug formatting
log(&format!("Let's print a vector with debug formatting:\n{:?}\n\n", example));
// using pretty-print formatting
log(&format!("Let's print it using \"pretty print\":\n{:#?}\n\n", example));

// using debug formatting
log(&format!("We can do the same with tuples:\n{:#?}\n\n", (1, 2, 3)));
// using debug formatting
log(&format!("Let's create vectors with default values:\n{:?}\n\n", vec![0;5]));
```

When you specify `{:?}`, that means apply debug formatting.
You can also use `{:#?}` for pretty-print formatting, which makes the values more legible. Most of the time pretty-print will output an element per line.

Feel free to learn more about [module std::fmt](https://doc.rust-lang.org/std/fmt/index.html), and deep dive into utilities for formatting and printing strings.

You can also learn more about implementing the [Debug trait](https://doc.rust-lang.org/std/fmt/trait.Debug.html) on structs or enums.

---

### setup_alloc

[top](#topics)

This macro needs to be placed prior to declaring a Smart Contract. Its will generate the boilerplate code needed for everthing to work. 

```rust
near_sdk::setup_alloc!();
```

:warning: **NOTE:** Starting from version 4.x of the NEAR SDK, this macro will be deprecated. However, adding the `setup_alloc!()` macro is needed for version 3.x.

---

## Extra: `String` and `str`

[top](#topics)

`String` e `str` are two very different types. `String` is a type which keeps ownership of a string, but `str` (known as a string slice), is commonly used to keep references to strings; this type exists to minimize string copies at runtime. 

:warning: **Remember:**
 - The `str` type will be used as `&str`. This applies to a "string like this one in quotes" but also `&String`.
 - Anytime you need a reference to a `String` in a function, use `&str` and not `&String`. 
 - String literals like "this one in quotes" are actually `&'static str`. If this looks complicated, don't worry, you'll learn all about this when we discuss Lifetimes. In theory, this type of strings will never be dropped from memory, but that behavior will actually depend on optimizations done by the compiler. 


--- 
Lesson 5 :white_check_mark: ... **Done! Congratulations!**

Our [next lesson](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums) will be about Enums.

