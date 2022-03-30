# Lesson 2: Ownership

[back](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/)

Let's learn about Ownership in Rust!

---

## Topics
 - [Functions](#functions)
 - [Background](#background)
 - [Ownership](#ownership)
 - [References](#references)
 - [Using References](#using-references)
 - [Examples](#examples)
 - [Unit tests](#unit-tests)

---

### Functions
[top](#topics)

```rust
/// Returns the length of the string
pub fn get_length(&self) -> u32;

/// Returns the length of the string and changes `name` to another value.
pub fn get_length_again(&mut self) -> u32;
```

---

### Background
[top](#topics)

We'll explain the concept of **Ownership** in the next section, but let's first focus on the problem it solves. 

Consider the following statement:

```
A = B;
```

We know `A`  is equal to `B`, since `A` is being assigned the value of `B`. But what is really happening? 

Are we creating a copy of the value of `B`, and assigning `A` that value? Creating a copy means to allocate memory, get that memory's address and set that memory's address to the value of `B`.  For an integer that seems simple, but what about a 2.000 character string? 

And if we are using a variable as an argument to a function... are we creating a copy of the variable and then dropping it after the function finishes? 

You will realize we need a way to reuse the same memory address in different parts of our application. The C language solved this through the use of pointers. Instead of storing the value of a variable, we store the memory address for that type of variable. 

But that solutions brings another problem. If a function has access to the memory address of a very important variable, then this function has a lot of power. What if this function was implemented without thinking of potential security issues? A hacker could use an implementation flaw or bug to break the application or gain access to the system. 

 - We need a method of handling memory that avoids unnecessary overhead and provides safety. 
 - And, we need to avoid that our chosen memory handling strategy provides more power than it needs to. 

**:hand: NOTE:** Rust also has pointers, but there are different kinds of pointers, with different advantages and disadvantages. C style pointers can be used but the code where they're used (code block) **must** to be annotated as **"unsafe"** ([More](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)).

---

### Ownership
[top](#topics)

The following statement:

```
A = B;
```

Can be done in two ways: 
 - If `B` implements a `Copy` trait, it will create a copy automatically. 
 - If `B` doesn't implement `Copy`, `A` will **own** `B`. The compiler will restrict the use of `B`, because the value of `B` was **"moved"** to `A`. 

 - Numbers (`u32`, `f32`, `i32`, ...) implement `Copy` and `Clone`. 
 - String implements `Clone`, but **not** `Copy`.

**:hand: NOTE:** [What’s the difference between Copy and Clone?](https://doc.rust-lang.org/std/marker/trait.Copy.html#whats-the-difference-between-copy-and-clone)

So, in order to create a copy of a String, we need to do it explicitly. 

Ownership guarantees that a variable is the owner of ("owns") a memory address for that variable. Keep in mind, that ownership can be transfered, but to "share" a variable, we can just use pointers or references. 

---

### References
[top](#topics)

References, or borrowing ("borrow") are a way of sharing memory addresses with limited permissions. These references can be mutable or immutable, and are specified as: 

```rust
let a = 10; // Create a variable with a value of 10
let b = &a; // Create a variable b that references variable a
```

```rust
let mut a = 10; // Create a mutable variable with a value of 10
let b = &mut a; // Create a variable that is a mutable reference to variable a
```

Variables are, by default, immutable or more like constants. That's why you need to explicitly declare `a` as mutable. 

 - Immutable references can access a value, but they can't change it.
 - Mutable reference can access **and** change a value.

Here are some rules to remember:
 - You can't change the original value while there's still a reference to it.
 - You can have many immutable references.
 - There can be only **one** mutable reference. 
 - You can't have immutable references if there is one mutable reference.

When we create a reference, we're basically having an "owner" variable "borrow" its value to another variable The "borrowing" ends on the last line the variable is used. 

---

### :warning: Using References
[top](#topics)

**Don't return references just yet!** While returning them is possible, you have to specify the "lifetime" of the return value. You're beginning to learn Rust, and the whole concept of lifetimes can be avoided by simply returning copies when necessary. If you want to know more, then [learn more about lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html) first.

Lifetimes are a **powerful** concept if used correctly. Tools like `serde` and `borsh` use it to convert JSON to the type we need without any copying. That means, memory allocation is only made for our JSON string and for the type we need, nothing more. 

---

### Examples
[top](#topics)

This is our Smart Contract ...
```rust
#[near_bindgen]
#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    name: String,
}

impl Default for Contract{
    fn default() -> Self {
        return Contract {
            name: String::from("A default string"),
        };
    }
}
```

... and here are the functions we will go over as we learn: 

```rust
fn this_takes_a_reference(name: &str) -> usize { 
        return name.len();
    }

fn this_takes_the_ownership(name: String) -> usize {
    name.len()
}

pub fn get_length(&self) -> u32 {
    let length_reference: usize = Self::this_takes_a_reference(&self.name);
    let length_ownership: usize = Self::this_takes_the_ownership(self.name.clone());

    assert_eq!(
        length_reference, 
        length_ownership, 
        "Both have the same size {} and {}", length_reference, length_ownership,
    );

    length_reference as u32
}

pub fn get_length_again(&mut self) -> u32 {
    let a_reference: &String = &self.name;
    let _another_reference: &String = &self.name;
    let _yet_another_reference: &String = &self.name;
    let length = Self::this_takes_a_reference(a_reference);
    self.name = String::from("Changed name");

    length as u32
}
```

But before getting into details, let's talk about `String` and `&str`.

---

### What is a `String`

A `String` is a variable that has an owner. It stores a string and will be freed from memory when the variable is dropped. Unlike other languages, keep in a mind a string in quotes, such as `"A text like this"` is not a String, but rather something called a string slice, or `&str`. A reference to a String is denoted as `&String` or `&mut String`.

---

### What is an `&str` (or string slice)

This type simplifies string use in our code. Think of it as an immutable reference to a String, but as it is allocated by the compiler, the compiler gets to decide how to best optimize its memory use.

---

### Using `String` in Functions

Let's take a look at an example of `String` and `&str`:

```rust
let variable: String = String::from("A Variable");
let reference: &str = "A Variable";
```

The function below takes a `&str` and returns its length. The "borrow" ends when the function finishes. 

```rust
fn this_takes_a_reference(name: &str) -> usize { 
    name.len()
}
```
Let's provide the function some arguments:

```rust
this_takes_a_reference(&variable);
this_takes_a_reference(reference);
```
The following functions takes a `String` as argument and returns its length. The functions becomes the **owner** of the memory and drops it when it finishes. 

```rust
fn this_takes_the_ownership(name: String) -> usize {
    name.len()
}
```
In order to use arguments with `this_takes_the_ownership` ...

```rust
this_takes_the_ownership(variable);
this_takes_the_ownership(String::from(reference));
```
... we need to convert `&str` to a `String` before passing it.  Also, this function acquired ownership but that wasn't really needed.

Both functions `this_takes_a_reference` and `this_takes_the_ownership` do the same thing, don't cause errors and return the same result. **But**, the first one is more efficient than the second one. Be mindful to prefer using `&str` instead of `String` on function declarations.

You also need to keep in mind that Smart Contract functions annotated with `#[near_bindgen]` need to use the `String` type in their arguments. That's only because the deserialization traits are implemented for `String`, but not for `&str`.

The following function ...

```rust
pub fn get_length(&self) -> u32 {
    let length_reference: usize = Self::this_takes_a_reference(&self.name);
    let length_ownership: usize = Self::this_takes_the_ownership(self.name.clone());

    assert_eq!(
        length_reference, 
        length_ownership, 
        "Both have the same length {} and {}", length_reference, length_ownership,
    );
}
```
... calls `this_takes_a_reference` and `this_takes_the_ownership`, making sure both return the same value (by ussing an `assert_eq`) before actually returning it. 

Let's take a look a another function:

```rust
pub fn get_length_again(&mut self) -> u32 {
    let a_reference: &String = &self.name;
    let _another_reference: &String = &self.name;
    let _yet_another_reference: &String = &self.name;
    let length = Self::this_takes_a_reference(a_reference);

    self.name = String::from("Changed name"); // change value of name

    length as u32
}
```
Calls `this_takes_a_reference` and changes the value of `name` stored in the Smart Contract. You can see that there can be many references to the same variable, but be sure to change these as specified [in the code's comments](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_2_ownership/src/lib.rs) to see how the compiler reacts.


Lesson 2 :white_check_mark: ... **Done! Congratulations!**

Our [next lesson](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_3_structs) will be about Rust's structs.
