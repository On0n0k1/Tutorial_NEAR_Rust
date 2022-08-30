# Lesson 6 - Thermometer

[back](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums/)

Time to build a Rust application with all of the concepts we have covered! We will also learn about: 

 - Project Documentation.
 - User Access Control.
 - Cross-Contract Calls.
 - Input Control.
 - Output Control.
 - Using Traits to convert types.

---

## Topics

 - [Overview](#overview)
 - [Pre-requisites](#pre-requisites)
   - [Rust](#rust)
   - [Visual Studio Code](#visual-studio-code)
   - [near-cli](#near-cli)
 - [Bash commands: compiling](#bash-commands-compiling)
   - [Documentation](#documentation)
   - [Tests](#tests)
   - [Creating a sub-account for the Smart Contract](#creating-a-sub-account-for-the-smart-contract)
   - [Creating sub-accounts for Sensors](#creating-sub-accounts-for-sensors)
   - [Deleting sub-accounts](#deleting-sub-accounts)
 - [Smart Contract](#smart-contract)
   - [Initialization](#initialization)
   - [Deployment](#deployment)
   - [allow_user](#allow_user)
   - [remove_user](#remove_user)
   - [set_format](#set_format)
   - [new_entry](#new_entry)
     - [Examples](#examples-new_entry)
   - [list_update_entries](#list_update_entries)
     - [Examples](#examples-list_update_entries)
   - [clear_entries](#clear_entries)
     - [Examples](#examples-clear_entries)
   - [view_get_format](#view_get_format)
   - [view_get](#view_get)
     - [Examples](#examples-view_get)
 - [Project Development](#project-development)
   - [Project Documentation](#project-documentation)
     - [File comments](#file-comments)
     - [Comments in code](#comments-in-code)
   - [Modules](#modules)
   - [User access control](#user-access-control)
   - [Cross-Contract calls](#cross-contract-calls)
   - [Handling Output](#handling-output)
   - [Handling Input](#handling-input)
   - [Implementing Traits](#implementing-traits)

---

## Overview

[top](#topics)
In our previous lesson, we learned about `enum` and how to use the `match` keyword. We can also use enums to handle I/O (input/output) of data.

Let's image we are an embedded developer working on a device that monitors temperature, and so we need input from several thermometers at the same time. We realized the following constraints: 

 - **Connecting** all devices to a single computer is impractical.  
 - **Keeping** a server up 24x7 receiving sensor data is also impractical.
 - **Have** a server up in the cloud is also impractical, because the developer doesn't want to pay a centralized entity (and give even **more** money to a billionarie).

So, the developer decides to create a smart contract to store the data. Some clear advantages are:

 - **Easy to implement**. The user only requires an account to store a smart contract, and so an account-per-sensor seems a good solution.
 - **Easy to automate**. The only thing that changes is the smart contract's name (per sensor). With a few scripts, a developer can have a fully working system in minutes.
 - **Easy to extend**. In this example we only focus on temperature readings but the smart contract can be easily changed to receive and save other data.

Now, does the contract only store data? There's no computation on the data received? Actually, computing could be made locally in the device, and there's no need to waste [gas](https://docs.near.org/concepts/basics/transactions/gas) on computing that can be done off-chain. Plus, there's a lot of libraries out there for data-science, no need to re-invent the wheel. 

:warning: **Always "think in gas" (gas usage) when it comes to on-chain data or computation**.

In summary, sensors send data to the smart contract, who stores on-chain the sensor name, temperature and date/time of the measurement. Then, any consumer (e.g. our own computer) can later on access on-demand this data from all contracts and do further processing and analysis. 

---

## Pre-requisites

[top](#topics)
 - Install Rust.
 - Download and Install Visual Studio Code.
 - Install near-cli.


### Rust
The easiest way to install Rust is to follow the steps in https://www.rust-lang.org/tools/install, where you'll find instructions for all major operating systems (even Windows Subsytem for Linux). There's also [other installation methods](https://forge.rust-lang.org/infra/other-installation-methods.html) available.

Once you have Rust installed, you'll need to add a target so you can compile to webassembly (WASM). Run the following command in your shell:

```bash
rustup target add wasm32-unknown-unknown
```
If you ever need to uninstall Rust, that's pretty easy too:
```bash
rustup self uninstall
```

### Visual Studio Code
 - Download and install from https://code.visualstudio.com
 - Once you have installed Visual Studio, add this extension to have additional tooling that will help you with Rust programming  https://marketplace.visualstudio.com/items?itemName=Zerotaskx.rust-extension-pack

:hand: **NOTE:** an old version of the extension mentioned above added a dependency (extension) called `rust` which has been deprecated in favor of `rust-analyzer`. You should always use `rust-analyzer` as it is kept up-to-date (and be sure **not** to have both `rust` and `rust-analyzer` installed and running!)


### near-cli
An npm-installable tool for interacting with the NEAR backend (RPC server). For near-cli to work, you need [node.js](https://nodejs.org/) and npm installed (npm is installed with node.js). If you need to learn more about node.js, be sure to visit their [guides](https://nodejs.org/en/docs/guides/), but in short, node.js is a javascript runtime environment (which lets you develop applications using javascript and run them outside of the browser as regular app). 

Once you have node (and npm) installed, you can install near-cli using npm: 

```bash
npm install -g near-cli
```
The `-g` option will install near-cli globally (for all users).

There's quite a lot of options that you **need** to learn in order to use near-cli, so be sure to [read the docs](https://docs.near.org/docs/tools/near-cli).

---

## Bash commands: compiling
[top](#topics)

You can compile the project using: 

```bash
cargo build --target wasm32-unknown-unknown --release -p lesson_6_2_thermometer
```
 - `--target wasm32-unknown-unknown`: Compile to webassembly.
 - `--release`: Output production-ready code; compile using most compiler optimizations.
 - `-p`: Remember this crate is part of the workspace defined as `lesson_6_enums`. Using the flag `-p` tells the compiler that only `lesson_6_2_thermometer` should be compiled.

---

### Documentation
[top](#topics)
Did you know you can generate documentation using cargo? This command will generate documentation (as a webiste) for all modules on this crate:

```bash
cargo doc --open -p lesson_6_2_thermometer
```
 - `-p`: Remember, you need to specify that only `lesson_6_2_thermometer` is the crate we want to generate documentation for.

---

### Tests

[top](#topics)
Execute tests (including those found in documentation) by running:

```bash
cargo test -p lesson_6_2_thermometer
```
Rust has a tool called `rustdoc` that extracts code samples from documentation comments and executes them. Learn more about [rustdoc](https://doc.rust-lang.org/rustdoc/) for more information on writing doc tests.

---

### Creating a sub-account for the Smart Contract

[top](#topics)
This is the account where the smart contract will be deployed. 

Let's use near-cli to create sub-accounts. Using the `near` command, you need to specify two options:
- your **existing** NEAR testnet account. So, replace `your-main-account.testnet` with **your testnet account name**.
- the name you want for the **new** sub-account where the smart contract will be deployed. So, replace `smart-contract-account-name` with the name you want.

```bash
near create-account smart-contract-account-name.your-main-account.testnet --masterAccount your-main-account.testnet --initialBalance 90
```
`--masterAccount`: master account, able to create sub-accounts. 
`--initialBalance`: amount of NEAR to be transferred to the sub-account from the master account. If you don't specify an amount, 100 NEAR will be sent from the master account. 

If you need a refresher about accounts on NEAR, be sure to [read the docs](https://docs.near.org/concepts/basics/account) once more.

---

### Creating sub-accounts for Sensors

[top](#topics)
We won't deploy any contracts to these sub-accounts but they will be used to sync with the master account.

You need to replace:

 - `sensor-sub-account`: account name for the sensor.
 - `smart-contract-account-name`: account name where the smart contract will be deployed.
 - `your-main-account`: master account name.

```bash
near create-account sensor-sub-account.smart-contract-account-name.your-main-account.testnet --masterAccount smart-contract-account-name.your-main-account.testnet --initialBalance 10
```
`--masterAccount`: master account, able to create sub-accounts. 
`--initialBalance`: amount of NEAR to be transferred to the sub-account from the master account. If you don't specify an amount, 100 NEAR will be sent from the master account. 

---

### Deleting sub-accounts

[top](#topics)
You need to delete sensor sub-accounts prior to deleting the smart contract account. You specify the account to delete, and also the account that will receive any NEAR found on the account to be deleted.

```bash
near delete sub-account-to-delete.testnet receiver-beneficiary.testnet
```

 - `sub-account-to-delete.testnet`: account name to delete
 - `receiver-beneficiary.testnet`: account name to receive any funds from the deleted account. If you specify an invalid name, any funds from the deleted account will be distributed among validators.

---

## Smart Contract
[top](#topics)

Before reviewing the code, keep in mind that just after being deployed, the only user than can "call" functions in the smart contract is its **owner** (the account where you deployed it). So, you need to specify that account for the first calls. 

Other users (sensors) can, and will, later be included on a list of allowed users, so that each user (sensor) will have their own data storage. 

---

### Initialization

[top](#topics)

The contract initializes using Kelvin (as temperature unit) and only a single user (the owner).

---

### Deployment
[top](#topics)

Deploy your contract to NEAR's testnet by running: 
```bash
near deploy --accountId smart-contract-account-name.your-main-account.testnet --wasmFile .\target\wasm32-unknown-unknown\release\lesson_6_2_thermometer.wasm
```
:hand: **NOTE:** depending on how you built your project, the path to the _.wasm_ file might be different on your machine. 

:warning: Remember that the account where you'll deploy your contract to has to have enough NEAR for storage. Read about [storage staking](https://docs.near.org/concepts/storage/storage-staking).

---

### add_user

[top](#topics)

```bash
near call my-contract add_user '{"account_id": "sensor-account-id.testnet"}' --accountId my-contract

```
Adds the specified user to the list of allowed users. Only the owner can call this function, and it cannot be used cross-contract.


Parameters:
 - account_id: String. Account name to add to the allowed user list.

Panics:
 - If cross-contract call.
 - If caller is not the owner.
 - If account name is invalid.
 - If account name already exists.

---

### remove_user

[top](#topics)

```bash
near call my-contract remove_user '{"account_id": "sensor-account-id.testnet"}' --accountId my-contract
```
Remove the specified user from the list of allowed users, all data stored for this user will be lost. You can't remove the owner. Only the owner can call this function, and it cannot be used cross-contract.

Parameters:
 - account_id: String. Account name to remove from the allowed user list.

Panics: 
 - If cross-contract call.
 - If the caller is not the owner.
 - If account name is invalid.
 - If account name doesn't exist.
 - If acount name to remove is the owner.

---

### set_default_temperature_unit

[top](#topics)

```bash
near call my-contract set_default_temperature_unit '{"unit_name": "Fahrenheit}' --accountId my-contract
```

```bash
near call my-contract set_default_temperature_unit '{"unit_name": "Kelvin}' --accountId my-contract
```

```bash
near call my-contract set_default_temperature_unit '{"unit_name": "Celsius}' --accountId my-contract
```
Changes the default temperature unit (Fahrenheit, Kelvin, Celius), but it will not update/convert any values previously stored.

All temperature readings are converted into an internal system unit (the default). That allows sensors with different temperature units to simply send their measurements. Only the owner can call this function, and it cannot be used cross-contract.

 - The **call** function `list_update_entries` converts all stored values to the new default temperature unit.
 - The **view** function `view_get` returns all stored values.

Panics:
 - If account name is invalid.
 - If cross-contract call.
 - If the caller is not the owner.

---

### new_entry

[top](#topics)

This **call** function stored a new temperature measurement from any allowed user (can be cross-contract call too). 

Parameters:
 - **time**: optional. A tuple `(u8, u8, f32)` representing hour, minute and second. If ommited, the default value will be the current time (UTC) when the function was called.
 - **date**: optional. A tuple `(i32, String, u8)` representing year, month and day. If ommited, the default value will be the current date when the function was called.
 - **temp_value**: A `f32`, which is the measured temperature value, which cannot be less than absolute zero. 
 - **temp_format**: Optional. A `String`, representing the temperature unit. If ommited, the default system unit will be used. If the unit is different to the default system unit, a conversion will be made to the default system unit prior to storing.

#### Examples new_entry

[top](#topics)

Store a new measurement of 100 using the default system temperature unit, and current date and time:
```bash
near call my-contract new_entry '{"temp_value": 100 }' --accountID my-sensor-id
```

Store a new measurement of 100 degrees Celsius, using the current date and time:
```bash
near call my-contract new_entry '{"temp_value": 100, "temp_format": "Celsius"}' --accountID my-sensor-id
```

Store a new measurement of 50.5 degrees Fahrenheit, using the provided date and the current time:
```bash
near call my-contract new_entry '{"temp_value": 50.5, "temp_format": "Fahrenheit", "date: [2022, "feb", 11]"}' --accountID my-sensor-id
```

Store a new measurement of 11.5 degrees Fahrenheit, using the provided date and the provided time:

```bash
near call my-contract new_entry '{"temp_value": 11.5, "temp_format": "f", "date": [2018, "mar", 27], "time": [10, 50, 9.3453]}' --accountID my-sensor-id
```

Store a new measurement of -45.4 degrees Celsius, using the current date and the provided time:

```bash
near call my-contract new_entry '{"temp_value": -45.4, "temp_format": "c", "time": [23, 41, 4.443]}' --accountID my-sensor-id
```

O comando abaixo armazena uma temperatura de 44.13 Kelvin. Horário do recebimento da mensagem. Data atual do recebimento da mensagem.

```bash
near call my-contract new_entry '{"temp_value": 44.13, "temp_format": "kelvin"}' --accountID my-sensor-id
```

### list_update_entries

[top](#topics)

This **call** fuction returns all temperature readings (measurements) for a specified account, converting to the default temperature unit, if needed.

All allowed users can access their own data, but only the owner can access other user's data. This restriction is in place to keep in check gas usage. Any user or account can still take advantage of **view** functions to gather all data. 

Parameters:
 - account_id: Optional. A `String` representing the account to retrieve data for. If not specified, it will return data for the caller account.

**Returns**: A `Vec` with all temperature readings (measurements) associated with a user.

#### Examples list_update_entries

[top](#topics)

This will return all temperature readings associated with the user (account) "my-sensor-id":

```bash
near call my-contract list_update_entries '{}' --accountID my-sensor-id
```

This will return all temperature readings associated with another user (specified with account_id). Only the owner can call retrieve data for another user:

```bash
near call my-contract list_update_entries '{"account_id": "my-sensor-id.testnet"}' --accountID my-contract
```

#### Panic list_update_entries

[top](#topics)

 - If the user is not on the allowed user list.
 - If the user is not owner and is trying to update other user's data.
 - If the user is not found.

---

### clear_entries

[top](#topics)

This **call** function clears all user data (temperature readings) for a user. Can be cross-contract call and only the owner can call this function.

The reason this function can be cross-contract is to make it easier for automation. Other contracts can't add or remove users, but they can add temperature readings and collect user data.

Users don't have permissions to use this function in order to increase security. If one sensor was hacked, the hacker would only have the most limited functionality. Sensors only exist to provide temperature readings. 

#### Parameters clear_entries

[top](#topics)

 - **account_id**: Optional. A `String`, representing the account to remove data from. If ommited, **all data for the owner** will be removed.

#### Examples clear_entries

[top](#topics)

This will remove all temperature readings (measurements) associated with the user "my-sensor-id".

```rust
near call my-contract clear_entries '{"account_id": "my-sensor-id.testnet"}' --accountID my-contract
```

#### Panic clear_entries

[top](#topics)

 - If user is not the owner.
 - If account is not found.

---

### view_get_format

[top](#topics)

A **view** function. Returns a `String` representing the default temperature unit.

```bash
near view my-contract view_get_format '{}'
```

--- 

### view_get

[top](#topics)

A **view** function. Returns temperature readings (measurements) associated with an account.

:hand: **NOTE:** this is a **view** function and so there's not gas involved. However, it can still return two different result types. Take a look at `ViewGet` in `./src/utils.rs`.


```rust
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ViewGet{
    Single(TemperatureReading),
    Multiple(Vec<TemperatureReading>),
}
```
Let's take a closer look at the declaration:
 - serde is used for JSON serialization and deserialization.
 - `#[serde(untagged)]` allows not using an explicit JSON tag for a type, and so we can have any type of variant in the enum. In our enum `ViewGet`, we can then have two variants:`Single(TemperatureReading)` and `Multiple(Vec<TemperatureReading>)`. 
 
 Learn about serde's [enum representations](https://serde.rs/enum-representations.html), or specifically more about [untagged](https://serde.rs/enum-representations.html#untagged).

This is a way to allow the developer to take advantage of enums to simply focus on getting a result.

:hand: **NOTE:**: this example was just made this way to show the possibility of returning different variants as a result. It could lead to increasing system complexity!

#### Parameters view_get

[top](#topics)

 - index: u64. Optional. The index for the temperature reading to return. If omitted, return all temperature readings. 
 - account_id: A `String` representing the account that has associated temperature readings to return.

#### Examples view_get

[top](#topics)

This will return the first (with an index of 0) temperature reading, if found, associated with the account "sensor-id":

```bash
near view my-contract view_get '{"index": 0, "account_id": "sensor-id.testnet"}'
```
This will return all temperature readings associated with the account "sensor-id":

```bash
near view my-contract view_get '{"account_id": "sensor-id.testnet"}'
```

---

## Project Development

[top](#topics)

In this section, let's explain how we developed this project by going over a checklist:

 - Project documentation.
 - Module organization.
 - Access control for users.
 - Cross-contract calls.
 - Handling input;
 - Handling output;
 - Implemmenting Traits.

---

### Project documentation

[top](#topics)
In order to create documentation, we can take advantage of `cargo` by running: 

```bash
cargo doc --open --lesson_6_2_thermometer
```
This will create a website with all documentation for our project.

However, there are some rules we need to follow. Let's go over them next:

#### File comments
Use `//!` at the beginning of files for module-level documentation.

Here's an example from our `Day` module, found in `./src/schedule/date/day.rs`.

```rust
//! Module with all functions related to a day
//! 
//! We use an u8 for the day, but we also need to 
//! check the day is valid. So, we'll need to make
//! day a struct Day(u8).
//! 
//! When serialized to JSON, the value would just be
//! an u8, so there won't be any additional complexity
//! for the user.
```
These comments should be a summary of module functionality and features.

#### Comments in code
Comments in code `//` are not included in the generated documentation. 
Comments using `///` are used to describe types following them.

As an example, here's the code for the function `Day::assert_valid` which is a private function:
```rust
/// # Panics
/// - if day is invalid
fn assert_valid(&self, current_month: &Month, current_year: &Year) {
    let &Day(day) = self;

    let mut current_year: i32 = current_year.get();

    // Se for negativo, converte para positivo
    if current_year < 0 {
        current_year = -current_year;
    }

    // true if "leap year".
    let leap_year: bool = (current_year % 4) == 0;
    // convert true = 1, false = 0.
    let leap_year: u8 = leap_year as u8;

    // source: https://www.rapidtables.com/calc/time/months-of-year.html
    let max_day: u8 = match current_month {
        &Month::January(_) => 31,
        &Month::February(_) => 28 + leap_year,
        &Month::March(_) => 31,
        &Month::April(_) => 30,
        &Month::May(_) => 31,
        &Month::June(_) => 30,
        &Month::July(_) => 31,
        &Month::August(_) => 31,
        &Month::September(_) => 30,
        &Month::October(_) => 31,
        &Month::November(_) => 30,
        &Month::December(_) => 31,
    };

    // check if day is within valid range
    assert!(day <= max_day,
        "Invalid values for day. Day: {}, Month: {}, Year: {}. Day for given month and year can not be higher than {}.",
            day,
            current_month,
            current_year,
            max_day,
    )
}
```
Comments using `//` provide insight into how the code is implemented, and they will not be included in the generated documentation.

Comments using `///` provide more information on the function's behavior. Here, we see the function can panic if the 
day is invalid. 

A seguir há um exemplo da função ```Month::new``` no caminho ```./src/entry/schedule/month/Month.rs```.

```rust
/// Create a month instance.
/// 
/// All possible values on the left are converted
/// to an enum value on the right:
/// 
///  - "january", "jan", "janeiro", "enero", "ene" => Month::January("January")
///  - "february", "feb", "fevereiro", "fev", "febrero" => Month::February("February")
///  - "march", "mar", "março", "marzo" => Month::March("March")
///  - "april", "apr", "abril", "abr" => Month::April("April")
///  - "may", "maio", "mayo" => Month::May("May")
///  - "june", "jun", "junho", "junio" => Month::June("June")
///  - "july", "jul", "julho", "julio" => Month::July("July")
///  - "august", "aug", "agosto", "ago" => Month::August("August")
///  - "september", "sep", "setembro", "set", "septiembre" => Month::September("September")
///  - "october", "octo", "oct", "outubro", "out", "octubre", "octu" => Month::October("October")
///  - "november", "nov", "novembro", "noviembre" => Month::November("November")
///  - "december", "dec", "dezembro", "dez", "diciembro", "dic" => Month::December("December")
/// 
/// # Panics
/// - if an invalid argument is provided. Month name is not valid.
/// 
pub fn new(month: &str) -> Self {
    let lower_case: String = month.to_ascii_lowercase();
    
    match &lower_case[..] {
        "january" | "jan" | "janeiro" | "enero" | "ene" => Month::January(String::from("January")),
        "february" | "feb" | "fevereiro" | "fev" | "febrero" => Month::February(String::from("February")),
        "march" | "mar" | "março" | "marzo" => Month::March(String::from("March")),
        "april" | "apr" | "abril" | "abr" => Month::April(String::from("April")),
        "may" | "maio" | "mayo" => Month::May(String::from("May")),
        "june" | "jun" | "junho" | "junio" => Month::June(String::from("June")),
        "july" | "jul" | "julho" | "julio" => Month::July(String::from("July")),
        "august" | "aug" | "agosto" | "ago" => Month::August(String::from("August")),
        "september" | "sep" | "setembro" | "set" | "septiembre" => Month::September(String::from("September")),
        "october" | "octo" | "oct" | "outubro" | "out" | "octubre" | "octu" => Month::October(String::from("October")),
        "november" | "nov" | "novembro" | "noviembre" => Month::November(String::from("November")),
        "december" | "dec" | "dezembro" | "dez" | "diciembre" | "dic" => Month::December(String::from("December")),
        invalid => panic!("Invalid value for month: {}.", invalid),
    }
}
```
The documentation above the function has a lot of details; the reason being that not only users need information about what the function does and the arguments it needs, but also to inform developers how they can change or modify the function to extend to other use cases.

---

### Modules

[top](#topics)

Modules in Rust can be a bit confusing... some directories have a `mod.rs` file while others do not. That's just because there is more than one way to declare a module in Rust. 

You declare a Rust module by either:
 - A rust file with a matching directory name on the level.
 - Having a `mod.rs` file inside a directory.

Examples:

 - Module `entry` is located in ```./src/entry/mod.rs```
 - Module `temperature` is defined by ```./src/temperature/mod.rs```
 - Module `date` is located in ```./src/schedule/date.rs```, inside a directory on the path ```./src/schedule/```
 - Module `time` is located in ```./src/schedule/time.rs```, inside a directory on the path ```./src/schedule/```

Learn more about [module organization](https://aloso.github.io/2021/03/28/module-system.html).

---

### User access control

[top](#topics)

Function calls needs to be mde from a NEAR account. We can control access by checking for the account's name who made the call. When the contract is initialized, only the owner can make calls. Additional accounts can be included using the function `Contract::add_user`. 

Each account we add does not have admin permissions, but they do have some storage space for data. It will also have the permission to add new entries, as well as the permission to update their values. 

A reason for limiting access by account is due to the possibility of a bad actor gaining acesss to a device and using them to also access the smart contract. If this happens, with the security put in place, that bad actor could only include new entries or update the values, which are very limited actions due to the amount of gas they spend for an account related to a sensor. 

Functions that control access are private functions: 
 - ```Contract::assert_owner_only```: panics if the caller is not the owner; the owner is the account that was used to deploy the smart contract.
 - ```Contract::assert_user_allowed```: panics if the caller is not a user in the allowed user list. Owner is, of course, on the allowed user list.


```rust
// assert the owner is the caller
fn assert_owner_only(&self){
    let predecessor: AccountId = env::predecessor_account_id();
    let owner_id: AccountId = AccountId::from(env::current_account_id());

    assert_eq!(predecessor, owner_id, "Only owner's account is allowed to make this function call.");
}

// check user permissions
fn assert_user_allowed(&self) {
    let predecessor_id: AccountId = env::predecessor_account_id();
    let owner_id: AccountId = env::current_account_id();

    // is the caller the owner? call assert_owner_only
    if owner_id == predecessor_id {
        return;
    }

    // check if user is in the allowed list
    assert!(self.users.contains(&predecessor_id), "User not allowed to make this call.");
}
```

The module `near_sdk::env` provides all the information related to the virtual machine's environment as well as all the message details. Here's a quick glance at some of the information available: 

 - `env::predecessor_account_id`: Id of the account who is _currently_ calling the function.
 - `env::signer_account_id`: Id of the account who _first signed_ the transaction that initiated the call(s).
 - `env::current_account_id`: Id of the current account, who is the owner of the _currently executing_ smart contract.

In the most simple scenarios, `predecessor_account_id` and `signer_account_id` are the same. However, do keep in mind that smart contracts can call other contract's functions (in a chain like manner); when this happens, we call them _cross contract calls_. 

Let's say account **A** calls contract **B**, and contract **B** calls contract **C**, who in turn, calls yet another contract **D**:

```
A -> B -> C -> D
```

In the scenario above, the `signer_account_id` will always be **Account A**. 
Let's find out who is the `predecessor_account_id`: 
 
 - For **B**, the `predecessor_account_id` is **A**.
 - For **C**, the `predecessor_account_id` is **B**.
 - For **D**, the `predecessor_account_id` is **C**.

In our contract, we check if the `owner_account_id` and `predecessor_account_id` are the same. If they are, then the function caller is the **owner**; otherwise, we check if `predecessor_account_id` is included in the allowed user list. 

We could have used the `signer_account_id` but that will rule out any possibility of cross contract calls. A developer could add more features to this contract, so we have to keep our smart contract flexible (yet secure). 

[Learn more](https://docs.near.org/tutorials/crosswords/beginner/actions#predecessor-signer-and-current-account) about predecessor, signer, and current account. 


---

### Cross-Contract calls

[top](#topics)
A "cross contract call" is when a smart contract calls another smart contract's function. Sometimes, you want to prevent cross contract calls, such as when having function that make critical changes to the system. 

For our smart contract, we have implemented a function called `Contract::no_cross_contract`: 


```rust
// don't allow cross-contract calls
fn assert_no_cross_contract(&self){
    let signer_id: AccountId = env::signer_account_id();
    let predecessor_id: AccountId = env::predecessor_account_id();
    assert_eq!(signer_id, predecessor_id, "Cross-contract calls not allowed.");
}
```

Our check simply compaers if the `signer_account_id` is the same as `predecessor_account_id`.

We prevent cross contract calls in some of our functions, such as `Contract::add_user`, `Contract::remove_user` and `Contract::set_default_temperature_unit` since these are admin operations that could add or remove users, as well as affect our system's data.
 
 - `Contract::set_default_temperature_unit`: could result in high gas use due to changing a lot of data.
 - `Contract::add_user`: could include unneeded users to the system.
 - `Contract::remove_user`: could remove users as well as their data. Unproper usage of this function could bring the whole system down.

---

### Handling Output

[top](#topics)
The same function can return diferent data types if we handle them using `enums`. We first need to create an enum that has all the variants:

```rust
// ./src/utils.rs
use near_sdk::serde::{
    Deserialize, Serialize,
};

use crate::entry::Entry;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ViewGet{
    Single(Entry),
    Multiple(Vec<Entry>),
}
```

 - `#[derive(Deserialize, Serialize)]` implements the traits `near_sdk::serde::Deserialize` and `near_sdk::serde::Serialize` on our enum. These are needed to serialize and deserialize to and from JSON.
 - `#[serde(crate) = "near_sdk::serde"]` hints to the compiler to use the serde crate found in the NEAR SDK.
 - `#[serde(untagged)]` is an attribute that hints serde  **not** to use JSON tags such as `{ Single: value }` or `{ Multiple: [value1, value2, ...] }`, but just use the actual values like so: `value`, and `[value1, value2, ...]`.

[Learn more](https://serde.rs/enum-representations.html) about using Serde and enums. 

Com o enum declarado e configurado. Basta retornar o tipo em uma função de contrato:
Since we declared and decorated our enum with the proper attributes, we can simply return it from our functions:

```rust
pub fn view_get(
    &self, 
    index: Option<u64>, 
    account_id: String,
) -> ViewGet {
    match index{
        None => {
            let result = self.entries
                .get(&account_id)
                .unwrap()
                .to_vec();

            ViewGet::Multiple(result)
        },
        Some(index) => {
            let result = self.entries
                .get(&account_id)
                .unwrap()
                .get(index)
                .unwrap();

            ViewGet::Single(result)
        }
    }
}
```

Note que a função acima é uma função view. Conversão de estado para json é preparado durante compilação. Ou seja, não consome gás.

A função simplesmente retorna o enum ```ViewGet``` declarado anteriormente. Se o argumento "index" existe no json, retorna ```ViewGet::Single(valor)``` com o "valor" encontrado. Se argumento "index" foi omitido, retorna ```ViewGet::Multiple(valores)``` com os "valores" encontrados.

Uso do tipo Option para argumentos é descrito logo a seguir.

---

### Handling Input

[top](#topics)

Note a função abaixo. Existem vários argumentos da função do tipo ```Option```.

```rust
pub fn new_entry(
    &mut self, 
    time: Option<(u8, u8, f32)>,
    date: Option<(i32, String, u8)>,
    temp_value: f32, 
    temp_format: Option<String>,
){
    self.assert_user_allowed();
    let user: AccountId = env::predecessor_account_id();

    log("Called new_entry.");

    log("Creating Entry.");
    let entry: Entry = Entry::new(time, date, &self.temp_format, temp_value, temp_format);

    log("Acquiring entries for this user.");
    let mut entries = match self.entries.get(&user){
        None => panic!("Unexpected Behavior: Failed to find entries for this user."),
        Some(value) => value,
    };
    
    log("Pushing entry to Vector.");
    entries.push(&entry);
    assert!(self.entries.insert(&user, &entries).is_some(), "Failed to replace vector");

    log("Operation Successful.");
}
```

Option é um tipo da biblioteca standard que pode ter duas alternativas, ```Some(valor)``` ou ```None```. Se usarmos este tipo nos argumentos das nossas funções, o usuário não precisa incluir este argumento na chamada de função.

 - Se incluir, o valor do argumento é ```Some(valor)```;
 - Se não incluir, o valor do argumento é ```None```;

Usamos instruções match para considerar as duas possibilidades. Por exemplo, o ``` account_id``` é opcional para a maioria das funções de contrato.

```rust
// ./src/contract.rs

pub fn list_update_entries(
    &mut self, 
    account_id: Option<String>,
) -> Vec<Entry> {
    self.assert_user_allowed();

    // let account_id: AccountId = env::predecessor_account_id();
    let account_id = match account_id{
        None => {
            env::predecessor_account_id()
        },
        Some(value) => {
            let predecessor = env::predecessor_account_id();
```

O fragmento da função ```Contract::list_update_entries``` acima possui o argumento ```account_id``` que é um Option. Se o valor para ```account_id``` existir na mensagem, usa-o. Senão usa o ```account_id``` da conta que chamou o contrato. Essa operação é repetida em diversos outras funções.

No tópico sobre [controle de output](#controle-de-output) acima. Foi descrito como usar um enum para possuir diversos tipos diferentes para o mesmo retorno de função. Podemos usar um enum com as mesmas configurações para aceitar diversos tipos de input também. Não é necessário incluir nenhuma configuração serde adicional.

```rust
// ./src/utils.rs
use near_sdk::serde::{
    Deserialize, Serialize,
};

use crate::entry::Entry;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ViewGet{
    Single(Entry),
    Multiple(Vec<Entry>),
}
```

Basta alterar os valores internos do enum de acordo com suas necessidades, e usar o tipo como argumento da função de contrato.

---

### Implementing Traits

[top](#topics)

O "dia", contido em "data", contido em "schedule" é representado da seguinte forma.

```rust
// ./src/schedule/date/day.rs

#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Day(u8);
```

É apenas um ```u8``` contido em um tipo próprio. Embora esse tipo tenha sido criado para aplicarmos as limitações que o valor de dia pode possuir. Queremos utilizar este valor como um número nos outros casos. Para isso, aplicamos as seguintes traits:

```rust
/// Nos permite usar u8::from(nossoDay)
impl From<&Day> for u8{
    fn from(day: &Day) -> u8 {
        let &Day(result) = day;

        result
    }
}

/// Nos permite usar u8::from(nossoDay)
impl From<&Day> for String{
    fn from(day: &Day) -> String {
        u8::from(day).to_string()
    }
}


// Usado para converter o struct para String. Se usarmos instruções como format!, println! ou panic!, esta trait é usada.
impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
```

A trait ```From``` permite a conversão de um tipo para outro. Devido a essas implementações, para uma variável Day com nome day, é possivel converter para u8 e String, respectivamente, com ```u8::from(&day)``` e ```String::from(&day)```.

A trait ```std::fmt::Display``` parece complicado, mas simplesmente permite o uso do tipo em macros como ```panic```, ```format``` e ```println```. Sem esta implementação, uma instrução como ```println!("O valor de day é {}", day)``` resultaria em pânico.

---

Lesson 6 - Thermometer :white_check_mark: ... **Done! Congratulations!**

Let's learn next all we can about `Result` on our [next lesson](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums/lesson_6_3_game_score/).

