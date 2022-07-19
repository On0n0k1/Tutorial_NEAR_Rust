# Lesson 6 - Using Enums

[back](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums/)

In this lesson, we will learn all about enums and the ```match``` keyword.

---

## Contract API

```rust
// /src/lib.rs

/// We can use this function to match Strings and string slices &str
/// return 1, 2, 3, 4, 5, if the parameter given is one of those values
/// otherwise, panic!
pub fn string_match_example(&self, arg: String) -> u32;

/// Returns example_0.
pub fn get_example_0(&self) -> Example0;

/// Returns example_1.
pub fn get_example_1(&self) -> Example1;

/// Returns example_2.
pub fn get_example_2(&self) -> Example2User;

/// Calls Example0::get_number.
pub fn example_0_get_number(&self) -> u32;

/// Calls Example0::is_third.
pub fn example_0_is_third(&self) -> bool;

/// Calls Example1::get.
pub fn example_1_get(&self) -> String;

/// Calls Example1::is_novalue.
pub fn example_1_is_novalue(&self) -> bool;

/// Calls Example1::get_an_integer.
pub fn example_1_get_an_integer(&self) -> String;

/// Calls Example1::has_an_odd_number.
pub fn example_1_has_an_odd_number(&self) -> bool;

/// Calls Example2User::get_name.
pub fn example_2_get_name(&self) -> String;

/// Calls Example2User::has_permission.
pub fn example_2_has_permission(&self, permission: String) -> bool;

/// Calls Example2User::get_actions.
/// 
/// When we return a Vec, the serialiyer will try to use serde::json.
/// Using #[result_serializer] allows to specify borsh as the serializer.
#[result_serializer(borsh)]
pub fn example_2_get_actions(&self) -> Vec<String>;
```

---

## Compiling and testing

This crate belongs to the Lesson 6 workspace. You can find how to compile and test it in the [intro page for the lesson](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums/lesson_6_1_simple/).

:hand: **NOTE:**  All commands like `cargo test` and `cargo build` will affect all crates in the workspace, unless you provide a specific crate. 

---

## Topics

 - [What are enums](#what-are-enums)
 - [The `match` keyword](#the-match-keyword)
   - [String patterns](#string-patterns)
   - [`match` needs to match on all patterns](#match-needs-to-match-on-all-patterns)
   - [Using just one enum value](#using-just-one-enum-value)
 - [Enums that "box" values](#enums-that-box-values)
 - [Functions must specify return type](#functions-must-specify-return-type)
   - [Function is_no_value](#function-is_no_value)
   - [Function get_an_integer](#function-get_an_integer)
   - [Function has_an_odd_number](#function-has_an_odd_number)
 - [Usage scenario: user](#usage-scenario-user)
   - [Enums limit choices](#enums-limit-choices)
   - [Function get_name](#function-get_name)
   - [Function has_permission](#function-has_permission)
   - [Function get_actions](#function-get_actions)
   - [Choosing a serializer](#choosing-a-serializer)
 
---

## What are enums

[top](#topics)

While `Structs` are a type *composed* of other types, `Enums` are a type that can have only *one value at a time*. The values an enum can take are specified in its definition.

Here's an example of a simple `enum`:
```rust
pub enum Example0{
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}
```
 - `pub` allows an enum to be used in external modules.
 - `Example0` is the name of the `enum`.
 - `First`, `Second`, `Third`, `Fourth` and `Fifth` are the actual values this `enum` can have. 

So now that we have declared an `enum`, how do we create an instance? Here's an example for each possible value: 

```rust
let a = Example0::First;
let b = Example0::Second;
let c = Example0::Third;
let d = Example0::Fourth;
let e = Example0::Fifth;
```
In order to check for a value, we could use `if | else` but Rust provide a much powerful tool. Let's learn about the `match` keywork next. 

---

## The `match` keyword

[top](#topics)
The `match` keyword allows to compare a value with all possible values for an enum.

```rust
// /src/model.rs
impl Example0{

    /// Check its assigned value and returns a number between 1 and 5.
    /// 
    /// Note the &self reference, meaning this function access its value 
    /// but it doesnt modify it (mutate)
    /// 
    pub fn get_number(&self) -> u32 {
        log("Calling Example0::get_number");

        // Here we match all possible enum values and 
        // return something, based on the enum value
        // remember: since this is the last statement of the function,
        // a return is implicit, you don't have to use the return keyword
        match self {
            Example0::First => {1},
            Example0::Second => {2},
            Example0::Third => {3},
            Example0::Fourth => {4},
            Example0::Fifth => {5},
        }
    }
```
The example above simply matches an enum value to a number, and returns the matching number (an integer in this case)

 - `Example0::First` returns 1;
 - `Example0::Second` returns 2;
 - `Example0::Third` returns 3;
 - `Example0::Fourth` returns 4;
 - `Example0::Fifth` returns 5;

`match` is similar a `switch` in other languages like C, Python, Java and Javascript, however, while `switch` compares for booleans, Rust's `match` compares against patterns.

---

### String patterns

[top](#topics)

We can use `match` on `String` and `&str`: 

```rust
// /src/lib.rs

impl Contract{
    /// We can use this function to match Strings and string slices &str
    /// return 1, 2, 3, 4, 5, if the parameter given is one of those values
    /// otherwise, panic!
    pub fn string_match_example(&self, arg: String) -> u32 {

        // treat &String as &str
        match &arg as &str {
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            value => panic!("Received {}. Must be 1, 2, 3, 4 or 5.", value),
        }
    }
```
In the example above, `match` goes over each pattern in order.
 - &arg is "1"? No.
 - &arg is "2"? No.
 - &arg is "3"? No.
 - &arg is "4"? No.
 - &arg is "5"? No.
 - `value` is a variable whose value will be any other value not matching the above cases.
 You can think of it as the last case in the match, which actually will match anything.
 Any string that isn't "1", "2", "3", "4" or "5", will be `value`, and so the function will panic.

---

### `match` needs to match on all patterns

[top](#topics)

On our first example, our enaum had 5 possible values. If you comment out one, the compiler will complain! (throw an error)

```rust
match self {
    Example0::First => {1},
    Example0::Second => {2},
    Example0::Third => {3},
    Example0::Fourth => {4},
    // if we comment the last possible value for the Example0 enum, 
    // the compiler will error out!
    // match needs to account for ALL possible values for the enum
    // Example0::Fifth => {5},
}
```

Our second examples compares a String. As a String can have basically any value, we need to have a 'last resort' case which matches 'any other value given', like so: 

```rust
value => panic!("Received {}. Must be 1, 2, 3, 4 or 5.", value),
```

You can have any variable name, it doesn't have to be `value`. It is also the case that sometimes you don't care for the actual value of the varibale but you **do** have to account for this 'whatever else' case when using `match`. In Rust, you'll see the underscore `_` being used as a variable name in those cases. Also, any variable that starts with an underscore `_` tells the compiler that we might not even use that variable later on (so the compiler will ignore its non-usage and not give you a *not used warning*, which is how it behaves by default). 

Let's say we write our `match` like this:

```rust
// /src/model.rs
pub fn string_match_example(&self, arg: String) -> u32 {

    match &arg as &str {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        value => {
            // We are not using our value in panic! below
            // so the compiler will warn you that value is not being used
            panic!("Invalid value. Must be 1, 2, 3, 4 or 5.");
        },
    }
}
```
To avoid the warning, we just need to modify things a bit... by adding an underscore: 

```rust
// /src/model.rs
pub fn string_match_example(&self, arg: String) -> u32 {

    match &arg as &str {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        _value => {
            // see? we added a _ , making it _value and not just value
            // we are STILL not using it in our panic! below
            panic!("Invalid value. Must be 1, 2, 3, 4 or 5.");
        },
    }
}
```

Do bear in mind that convention is imnportant, so we use "_" for patterns where we truly don't care about the 'whatever else' value: 

```rust
// /src/model.rs
pub fn string_match_example(&self, arg: String) -> u32 {

    match &arg as &str {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        _ => {
            // see? we used the _ for anything
            panic!("Invalid value. Must be 1, 2, 3, 4 or 5.");
        },
    }
}
```

---

### Using just one enum value

[top](#topics)

Sometimes we have a function that only needs to do something based on just one case or value of our `enum`. We can also use `match`! 

In the following example, we only match `true` when our enum is `Example0::Third`. 

```rust
// /src/model.rs
/// true but checking only for Example0::Third
pub fn is_third(&self) -> bool {
    log("Calling Example0::is_third");

    match self {
        Example0::Third => true,
        _ => false,
    }
}
```

---

## Enums that "box" values

[top](#topics)

This is a powerful fact you need to keep in mind... in Rust, `enum` can also have different underlying types; all possible enum values don't have to be same type!

```rust
// /src/model.rs
pub enum Example1{
    NoValue,
    AnInteger(i32),
    AFloat(f32),
    AString(String),
    ATuple(i32, u32),
    ACLikeStruct{first: u32, second: String},
}
```
:hand: **NOTE:** Read the above carefully. We have primitive types (i32, f32, String) as `enum` choices, as well as tuples, **and** we can also have a C-like `struct`. All of these are part of one `enum`, `Example1`.

---

## Functions must specify return type

[top](#topics)

Functions that use enums can sometimes be difficult to implement, due to static typing. 

- Argument types must be specified.
- Return type must be specified.
 
That also includes generic functions. The compiler **must** know the argument types as well as the return type. Generic function are but a way to create function that also follow those rules.

For our enum, a developer might run into trouble with the compiler, since we have different underlying types for our enum choices (an integer, a string, a struct).
However, we can certainly implement functions! We just need to focus on thing: what type should we choose for our return value? 

In our case, let's choose a String:

```rust
// /src/model.rs

// Our method returns a String
pub fn get(&self) -> String {
    log("Calling Example1::get");

    match self{
        Example1::NoValue => String::from(""),
        Example1::AnInteger(value) => format!("{}", value),
        Example1::AFloat(value) => format!("{}", value),
        Example1::AString(value) => format!("{}", value),
        Example1::ATuple(value0, value1) => format!("({}, {})", value0, value1),
        Example1::ACLikeStruct { first, second } => format!("{{\nfirst: {},\nsecond: \"{}\",\n}}\n", first, second),
    }
}
```

`format!` is a macro that creates a String. We went over macros in [Lesson #5](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_5_macro_usage/), so feel free to go back if you need a recap. Anyway, since we only want to print a value on screen, we can simply return a String.

In our lesson about Traits, we wil go over how to use borsh and serde. We use these to convert a struct into bytes, as well as convert from bytes or a string, into a struct (if they are compatible). In that lesson, we wil also cover how to use Generics, which allows to write functions that work for multiple types.

---

### Function is_no_value
[top](#topics)
Sometimes, we need to only check for one value. This function returns true if the enum value is `NoValue`.
```rust
// /src/model.rs
/// true if enum is Example1::NoValue.
pub fn is_no_value(&self) -> bool{
    log("Calling Example1::is_no_value");

    match self{
        Example1::NoValue => true,
        _ => false,
    }
}
```
---

### Function get_an_integer
[top](#topics)
Returns an integer if the enum's value is `Example1::AnInteger`.

```rust
// /src/model.rs
pub fn get_an_integer(&self) -> Option<i32>{
    log("Calling Example1::get_an_integer");

    match self{
        Example1::AnInteger(valor) => Some(valor.clone()),
        _ => None
    }
}
```
:hand: **NOTE:** if you looked at the code above you'll see the return value is actually something called an `Option`. We'll learn more about `Option` later on, but you can think of right now as an enum that *can* have a value, *or not*. If there is a value, you can retrieve it using `Some(value)` (where `value` is just a variable name), and if there is no value, then Option would be set to `None` (`Option::None`).
There is no **NULL** in Rust. 

By the way, take a look at this code:

```rust
match self {
```

Shouldn't this code be taking ownership of itself, and if not, why is that?

Well, that's because of how this argument is being passed:

```rust
pub fn get_an_integer(&self) -> Option<i32>{
```
As `&self` is being used, the compiler knows this value is just a reference (or borrow).

So, in summary:

```rust
match self{
    Example1::AnInteger(valor) => Some(valor.clone()),
    _ => None
}
```
The functions returns `Option<i32>` and therefore, our `match` must provide a return of either `Some(value)` or `None`. 

---

### Function has_an_odd_number

[top](#topics)
Returns true only if the argument is an odd integer.
Take a closer look at the code, since this is a more detailed usage scenario for `match`.

```rust
// /src/model.rs
/// Returns true only if the argument is an odd integer.
pub fn has_an_odd_number(&self) -> bool {
    log("Calling Example1::has_an_odd_number");

    match self {
        Example1::NoValue => false,
        Example1::AnInteger(valor) => {
            if valor%2 == 1{
                return true;
            }
                
            return false;
        },
        Example1::AFloat(_valor) => false,
        Example1::AString(_valor) => false,
        Example1::ATuple(valor0, valor1) => {
            return (valor0%2 == 1) || (valor1%2 == 1);
        },
        Example1::ACLikeStruct { first, second: _ } => {
            // we don't care about 'second' since it is a string
            first%2 == 1
        },
    }
}
```
So, the only alternatives that have integers are `Example1::AnInteger`, `Example1::ATuple` and `Example1::ACLikeStruct`. All other return false.

:warning: Variables starting with an underscore `_`, such as `_value` and `_`, hold **values** we don't really care about. The convention is to just an underscore `_`, but any variable names starting with an underscore will be ignored by the compiler when checking and raising any 'unused variable' warnings.

---

## Usage scenario: User

[top](#topics)
Let's see how we could use an enum in an application.

```rust
// /src/model.rs
pub enum Example2User {
    Admin { name: String, id: u32, pass: String, actions: Vec<String> },
    Client{ name: String, id: u32, orders: Vec<String> },
    Employee( Employee ),
}

pub struct Employee{
    pub name: String,
    pub id: u32,
    pub pass: String,
    pub permissions: Vec<String>,
    pub actions: Vec<String>,
}
```
We have three types of people who can access our system:
 - Employees: they can access the system and make limited modifications to data but can't alter business or system rules.
 - Admins: they have permissions to alter any business or system rules (basically do anything).
 - Clients: they can't change any system data, but are allowed to change their own data.
 
Employees, Admins and Clients are different types, and have different functions and data. But in the context of a User, they can be thought of in a similar manner.

You can group types using Enums but also Traits; however, consider that:
 - You use enums for grouping different types as a single concept or entity that will be used by a function.
 - You use traits for applying a single set of behaviors to different types. 

---

### Enums limit choices

[top](#topics)
Restricting choices can be good or bad depending on the case, so let's consider a few examples in order to get a grasp of the concepts.

Think about Chess. There are different pieces on the board, and so we made an enum for all the piece names:

```rust
// https://github.com/On0n0k1/NCD.L1--Chess/blob/main/src/pieces/piece.rs
pub enum Piece {
    BISHOP( Bishop ),
    EMPTY(  Empty  ),
    KING(   King   ),
    KNIGHT( Knight ),
    PAWN(   Pawn   ),
    QUEEN(  Queen  ),
    ROOK(   Rook   ),
}
```
Each piece has its own behavior, but the board itself doesn't care about it. However, the board does need to know each piece's possible movement in order to know if there's a check-mate. 

In this case, the restriction imposed by using enums is useful, since there are only so many piece types, plus one for empty space, and this will never change (so there won't be a need to add to the enum).

Enums are also useful for error handling. Let's think about an application used in a library. A function that retrieves information about a book could also return the following errors:

```rust
pub enum MessageError{
    BookNotFound(String),
    InvalidArg(String),
    MissingArg(String),
    NoPermission,
}
```
The errors are: 
 - `BookNotFound`: book not found, with the string being the name of the book that was not found.
 - `InvalidArg`: an argument was not recognized, with the string being the argument. 
 - `MissingArg`: a mandatory argument is needed but wasn't provided. The string is the required argument.
 - `NoPermission`: You don't have permission to access this book. Maybe the user is underage, and this book has adult content.

It becomes apparent that an advantage of restrictions imposed by enums is that a developer can well know all of the errors that can or should occur when calling a function.

---

### Function get_name

[top](#topics)
The function get_name of `Example2User` simply returns the 'name' of the user (which is a String). Remember, the underscores are values we choose to ignore. 

```rust
// /src/model.rs
/// returns the name of the user
pub fn get_name(&self) -> String {
    log("Calling Example2User::get_name");

    match self {
        Example2User::Admin { name, id: _, pass: _, actions: _ } => { name.clone() },
        Example2User::Client { name, id: _, orders: _ } => { name.clone() },
        Example2User::Employee( employee ) => { employee.name.clone() },
    }
}
```
Also, when there are multiple fields that have values you don't care about, you can simply use '`..`', like so: 
```rust
pub fn get_name(&self) -> String {
    log("Calling Example2User::get_name");

    match self {
        Example2User::Admin { name, .. } => { name.clone() },
        Example2User::Client { name, .. } => { name.clone() },
        Example2User::Employee( employee ) => { employee.name.clone() },
    }
}
```
---
### Function has_permission

[top](#topics)
Returns true if a user has permissions for an action in the application.
 - Admins always have all permissions granted.
 - Clients never have them.
 - Employees are granted permissions. Granted permissions are stored in a list.

:warning: Using String is never a good idea for permissions. This would be an ideal use case for enums, but we are doing it this way to keep complexity low.


```rust
// /src/model.rs
pub fn has_permission(&self, permission: String) -> bool{
    log("Calling Example2User::has_permission");

    match self{
        Example2User::Client { name: _, id: _, orders: _ } => { false },
        Example2User::Admin { name: _, id: _, pass: _, actions: _ } => { true },
        Example2User::Employee(employee) => {

            // Vec implements the Iterator trait, which means we have 
            // the .iter() function available to go over all 
            // value in the Vec, without needing to copy them
            for employee_permission in employee.permissions.iter(){
                if permission == *employee_permission {
                    return true;
                }
            }

            false
        }
    }
}
```
Employees have a `Vec` (vector) of strings where permissions are stored. We'll get to `Vec` when learn more about collections, but think of Vec (vector) as a modifiable list.

We already learned about `Clone` and `Copy`, so it is time to introduce [iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html) and iter(). This trait allows us to go over each value in a vector using `for`.

The `iter()` function generates a reference iterator. That means, that each `employee_permission` is a reference to an element contained in `Vec`. We **can't alter or modify** the values, but we also don't waste computing resources generating any copies of these values. 

Take a look at this code, and keep an eye out for the * operator:
```rust
if permission == *employee_permission {
    return true;
}
```
`employee_permission` is a &String (remember, iter() provides references) but we need to access the **actual** value of the string, not its reference. So, we use the * operator to *de-reference* the variable to get the actual value. 

Going back to the function, the argument `permission` will be compared to the permissions found in vec and if there's a match, the function returns true immediately. 

---

### Function get_actions

[top](#topic)
The function `get_actions` return a list of actions recently performed by the user. 

We'll use this function to introduce the `Result` enum, which we'll go in-depth in the next lesson, but for now think of `Result` as an action outcome that may have also caused an error. 

In our example, let's make this function return an error if the user is of type Client.

```rust
// /src/model.rs
pub fn get_actions(&self) -> Result<Vec<String>, String> {
    log("Calling Example2User::get_actions");
    
    // If Client, return an error.
    // If admin or employee, we can return a reference to the vec actions
    let actions = match self {
        Example2User::Client { name: _, id: _, orders: _ } => { 
                return Err(format!("User is Client")); 
        },
        Example2User::Admin { name: _, id: _, pass: _, actions, } => { actions },
        Example2User::Employee( employee ) => { &employee.actions },
    };

    // create an empty, but mutable vector
    let mut result: Vec<String> = Vec::new();
    // let's create a copy of Vec with the actions from our last match
    for action in actions {
        result.push(action.clone());
    }

    Ok(result)
}
```
:hand:**NOTE:** The function returns `Result<Vec<String>, String>`. The first type `Vec<String>` is what we choose to be the return of a sucessful outcome. The second type, `String`, is what we choose to return if there was an error (note we say error, not panic, they are very different). 

There's code to return an error, as a String (the second type): 
```rust
return Err(format!("User is Client"));
```

And there's code to return a `Vec<String>` if everything went OK:

```rust
Ok(result)
```
:warning: **REMEMBER:** `Result::Ok(value)` or simply `Ok(value)` is for a succesful outcome, and `Result::Err(err)` or just `Err(err)` is used when an error occurred. 
Also note that the variable names for these don't have to be `value` or `err`, that's just a convention when using `Result`.

---

:hand:**NOTE:** in our example we can't just return the found vec, as it is owned by the enum. So, we have to create a copy: 

```rust
let mut result: Vec<String> = Vec::new();

for action in actions{
    result.push(action.clone());
}
```

 - Create an empty `Vec<String>`. 
 - We iterate over the elements in the `actions` vec.
 - In each iteration, we create a copy of the element using `action.clone()`, and we append this to our new vec using `result.push();`.

---

### Choosing a serializer

[top](#topics)

Our `example_2_get_actions` function has something different about it, can you see it? 
```rust
// /src/lib.rs
#[result_serializer(borsh)]
pub fn example_2_get_actions(&self) -> Vec<String>{
```
I hope you spotted `#[result_serializer(borsh)]`. By using it, we're being explicit that we want to use `borsh` as our serializer for our function return. 

There are two options available for the near_sdk when it comes to serialization/deserialization: `serde` and `borsh`. If you want to use `serde`, then you need to implement the traits `Serialize` and `Deserialize` in your Smart Contract. However, if choosing `borsh`, you don't have to, and `borsh` has higher performance. 

Why not take a small detour from the lessons, and learn more about them? The best way is to read about [serialization protocols](https://www.near-sdk.io/contract-interface/serialization-interface) in the NEAR docs. 

---

Lesson 6 - Using Enums :white_check_mark: ... **Done! Congratulations!**

Let's put everything we've learned into developing an app on our [next lesson](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums/lesson_6_2_thermometer/).