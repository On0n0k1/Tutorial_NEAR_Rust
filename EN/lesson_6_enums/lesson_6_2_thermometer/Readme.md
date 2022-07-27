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
     - [Panic](#panic-new_entry)
   - [list_update_entries](#list_update_entries)
     - [Examples](#examples-list_update_entries)
     - [Panic](#panic-list_update_entries)
   - [clear_entries](#clear_entries)
     - [Arguments](#arguments-clear_entries)
     - [Examples](#examples-clear_entries)
     - [Panic](#panic-clear_entries)
   - [view_get_format](#view_get_format)
   - [view_get](#view_get)
     - [Arguments](#arguments-view_get)
     - [Examples](#examples-view_get)
 - [Implementation](#implementation)
   - [Project Documentation](#project-documentation)
     - [Comentários sobre Arquivo](#comentários-sobre-arquivo)
     - [Comentários e Documentação](#comentários-e-documentação)
     - [Exemplos/Testes em Documentação](#exemplostestes-em-documentação)
   - [Organização de Módulos](#organização-de-módulos)
   - [Controle de Acesso de Usuários](#controle-de-acesso-de-usuários)
   - [Acesso Cross-Contract](#acesso-cross-contract)
   - [Handling Output](#controle-de-output)
   - [Handling Input](#controle-de-input)
   - [Implementing Traits](#implementação-de-traits)

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

**Retorna**: Vec com todas as entries associadas ao id de conta.

#### Examples list_update_entries

[top](#topics)

O exemplo abaixo retorna todas as entries associadas ao usuário "my-sensor-id".

```bash
near call my-contract list_update_entries '{}' --accountID my-sensor-id
```

O exemplo abaixo retorna todas as entries associadas a outro usuário. Apenas owner tem permissão para isso.

```bash
near call my-contract list_update_entries '{"account_id": "my-sensor-id.testnet"}' --accountID my-contract
```

#### Pânico list_update_entries

[topo](#lição-6---2-termômetro)

 - Se usuário não tiver permissão de acesso;
 - Se usuário não for owner e estiver tentando atualizar as entries de outro usuário.
 - Se usuário não for encontrado;


---

### clear_entries

[topo](#lição-6---2-termômetro)

Função call. Apenas owner pode chamar esta função. Pode ser cross-contract. Apaga todas as entries associadas a um usuário.

O motivo da função permitir cross-contract é para facilitar automação de contrato. Contratos externos não podem incluir ou remover usuários permitidos. Mas podem adicionar entries, podem coletar dados e remover dados.

Usuários não tem permissão de utilizar essa função para evitar ações suspeitas. Caso um dos sensores for acessado por um terceiro, este terá o acesso mais limitado possivel ao sistema. Sensores deste projeto existem apenas para incluir entries. Nada mais.

#### Argumentos clear_entries

[topo](#lição-6---2-termômetro)

 - **account_id**: Opcional. String. ID de usuário para remover todas as entries. Se omitido, remove todas as entries do owner.

#### Exemplo clear_entries

[topo](#lição-6---2-termômetro)

O exemplo abaixo remove todas as entries associadas ao id "my-sensor-id".

```rust
near call my-contract clear_entries '{"account_id": "my-sensor-id.testnet"}' --accountID my-contract
```

#### Pânico clear_entries

[topo](#lição-6---2-termômetro)

 - Se o usuário não for owner;
 - Se id de conta não for encontrado;

---

### view_get_format

[topo](#lição-6---2-termômetro)

Função view. Retorna o formato de temperatura armazenado como String.

```bash
near view my-contract view_get_format '{}'
```

--- 

### view_get

[topo](#lição-6---2-termômetro)

Função view. Retorna um ou mais valores associados a um id de conta.

Note que esta função é uma função view. Não realiza computação. E ainda assim retorna dois tipos de resultado possiveis.

O tipo ```ViewGet``` é declarado em './src/utils.rs'.

```rust
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ViewGet{
    Single(Entry),
    Multiple(Vec<Entry>),
}
```

Com esta declaração, percebe-se:

 - serde é usado para serializar e deserializar o tipo para json;
 - ```#[serde(untagged)]``` faz a serialização mostrar os valores contidos nas tuplas no json. Neste caso ```Entry``` ou ```Vec<Entry>```.

Dessa forma, o desenvolvedor pode usar as vantagens de um enum sem afetar a experiência do usuário. Que apenas vê o resultado.

**Aviso**: Este exemplo existe apenas para demonstrar a possibilidade de retornar diversos tipos. Implementar isso em outras linguagens pode aumentar a complexidade de código desnecessariamente. Tome cuidado com as necessidades do sistema.

#### Argumentos view_get

[topo](#lição-6---2-termômetro)

 - index: u64. Opcional. Index da entry a ser retornada. Se omitida, retorna todas as entries.
 - account_id: String. ID de usuário para retornar entries.

#### Exemplo view_get

[topo](#lição-6---2-termômetro)

A instrução abaixo retorna o primeiro elemento (se existir) associado a conta de usuário "sensor-id".

```bash
near view my-contract view_get '{"index": 0, "account_id": "sensor-id.testnet"}'
```

A instrução abaixo retorna todas as entries associadas ao id de conta "sensor-id".

```bash
near view my-contract view_get '{"account_id": "sensor-id.testnet"}'
```

---

## Implementação

[topo](#lição-6---2-termômetro)

Esta seção explicará como as funcionalidades descritas acimas foram alcançadas.

 - Documentação de projetos;
 - Organização de módulos;
 - Controle de acesso de usuários;
 - Acesso cross-contract;
 - Controle de input;
 - Controle de output;
 - Implementação de traits;

---

### Documentação de projetos

[topo](#lição-6---2-termômetro)

Como descrito [acima](#documentação) o comando:

```bash
cargo doc --open --lesson_6_2_thermometer
```

Gera um website com a toda a documentação do nosso projeto. A seguir serão descritos alguns detalhes sobre documentação:


---

#### Comentários sobre arquivo

[topo](#lição-6---2-termômetro)

Se o comentário é iniciado com ```//! ```, a documentação descreve sobre todo o arquivo ```.rs```, ou seja, o módulo.

Abaixo vemos um fragmento do inicio do módulo ```Day```, localizado no caminho ```./src/entry/schedule/date/day.rs```.

```rust
//! Módulo com todas as funcionalidades necessárias para a 
//! representação de dia no contrato.
//! 
//! Usamos um inteiro u8 para representar um dia. Mas 
//! precisamos garantir que este valor é válido.
//! 
//! Devido a isso, o tipo Day é representado por um struct 
//! tupla Day(u8).
//! 
//! Quando serializado para json, o valor é visto como um 
//! número u8. Ou seja, o usuário não perceberá essa 
//! complexidade.
//! 
```

Comentários como este devem existir no início do arquivo.

Recomenda-se que estes comentários tenha um resumo sobre toda funcionalidade disponivel no módulo.

---

#### Comentários e Documentação

[topo](#lição-6---2-termômetro)

Comentários com "// " não são incluidos na documentação. Comentários com "/// " descrevem o tipo abaixo.

Como exemplo, abaixo está a implementação da função ```Day::assert_valid```, que é uma função privada.

```rust
/// # Panics
/// Se dia for invalido.
fn assert_valid(&self, current_month: &Month, current_year: &Year) {
    let &Day(day) = self;

    // Coleta o valor do ano.
    let mut current_year: i32 = current_year.get();

    // Se for negativo, converte para positivo
    if current_year < 0 {
        current_year = -current_year;
    }

    // A cada 4 anos, o mês de janeiro possui 29 dias, ao invez de 28.
    // true se for um "leap year".
    let leap_year: bool = (current_year % 4) == 0;
    // converte true para 1, false para 0.
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

    // panic se o valor do dia for maior que o valor referente ao mês.
    assert!(day <= max_day,
        "Invalid values for day. Day: {}, Month: {}, Year: {}. Day for given month and year can not be higher than {}.",
            day,
            current_month,
            current_year,
            max_day,
    )
}
```

A função acima impede que um usuário escolha um dia incorreto.

 - Se for informado o dia 31 para outubro, não ocorrerá erro;
 - Se for informado o dia 31 setembro, haverá erro. Pois não existe o dia 31 de setembro;
 - Se for informado o dia 29 de fevereiro em 2024, não haverá erro por ser ano bissexto (leap year);
 - Se for informado o dia 29 de fevereiro em 2025, haverá erro por não ser ano bissexto (leap year);

Note que os comentários escritos com "//" descrevem a implementação do código e não aparecem na documentação.

Note como os comentários escritos com "///" descrevem o elemento abaixo (neste caso, a função ```assert_valid```). Este exemplo apenas descreve que a função entra em pânico caso o dia seja inválido. Funções privadas não serão usadas por outros, então não há necessidade de documentar com extremo detalhe.

A seguir há um exemplo da função ```Month::new``` no caminho ```./src/entry/schedule/month/Month.rs```.

```rust
/// Constroi uma instância de Mês:
/// 
/// Os possiveis valores de String na esquerda são 
/// convertidos para os seguintes valores na direita:
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
/// Se o argumento não for nenhum dos possiveis acima.
/// 
pub fn new(month: &str) -> Self {
    let lower_case: String = month.to_ascii_lowercase();
    
    match &lower_case[..]{
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

A documentação da função acima é detalhado. Isso porque, não apenas os usuários necessitam de informação sobre como mês é reconhecido como argumento, assim como é essencial que desenvolvedores tenham acesso ao máximo de informação possivel caso queiram modificar/adaptar este projeto para outros casos de uso.

Resumidamente, mês é convertido de uma String. Vários Strings são válidos para cada possivel valor. Por exemplo: "january", "jan", "JAN", "Janeiro", "enero" e "ene" são todos possiveis Strings que convertem para um ```Month::January(String::from("January"))```. Não é case-sensitive.

---

#### Exemplos/Testes em Documentação

[topo](#lição-6---2-termômetro)

Exemplos existentes em documentação são incluidos nos testes. Segue adiante um fragmento da documentação para o módulo no caminho "./src/entry/schedule/date/day.rs"

```rust

//! ## Examples
//! 
//! ```rust
//! # use lesson_6_2_thermometer::schedule::date::day::Day;
//! # use lesson_6_2_thermometer::schedule::date::month::Month;
//! # use lesson_6_2_thermometer::schedule::date::year::Year;
//! 
//! // not leap year
//! let month = Month::new("feb");
//! let year = Year::new(1971);
//! 
//! let day = Day::new(28, &month, &year);
//! assert_eq!(u8::from(&day), 28);
//! assert_eq!(format!("{}", day), "28");
//! assert_eq!(String::from(&day), "28");
//! 
//! // leap year
//! let month = Month::new("feb");
//! let year = Year::new(1972);
//! 
//! let day = Day::new(29, &month, &year);
//! assert_eq!(u8::from(&day), 29);
//! assert_eq!(format!("{}", day), "29");
//! assert_eq!(String::from(&day), "29");
//! 
//! ```
```

O bloco acima é testado sempre que testes de unidade são executados. 

O objetivo deste exemplo é demonstrar as implementações de traits implementadas ao tipo Day.

Linhas com "#" não aparecem na documentação. Existem para permitir o funcionamento correto dos testes. Assim como reduzem a complexidade do exemplo.

---

### Organização de Módulos

[topo](#lição-6---2-termômetro)

Ao analizar a organização dos arquivos. Um desenvolvedor pode ficar confuso sobre como os módulos foram organizados. Alguns diretórios possuem um arquivo de nome "mod.rs" e outros não. Isso é porque duas formas diferentes de declarar módulos foram usadas.

A pergunta a ser feita é "Como um diretório pode ser considerado um módulo?". Existem duas respostas:

 - Um arquivo rust com o nome do diretório existindo no mesmo caminho que o diretório.
 - O arquivo rust com nome "mod.rs" dentro do diretório.

Como exemplo:

 - O módulo "entry" se encontra no caminho ```./src/entry/mod.rs```;
 - O módulo "date" se encontra no caminho ```./src/schedule/date.rs```. O diretório se encontra no mesmo caminho ```./src/schedule/```;
 - O módulo "temperature" se encontra no caminho ```./src/temperature/mod.rs```;
 - O módulo "time" se encontra no caminho ```./src/schedule/time.rs```. O diretório se encontra no mesmo caminho;

---

### Controle de Acesso de Usuários

[topo](#lição-6---2-termômetro)

Funções call precisam ser assinadas por uma conta NEAR. Podemos controlar o acesso checando o nome da conta que fez a chamada. Quando o contrato é inicializado, apenas o owner tem permissão de acessar o contrato. Outras contas podem ser incluidas através da função ```Contract::allow_user```.

Cada conta adicionada não terá permissões administrativas. Mas terão o próprio espaço de armazenamento de dados. Terão permissão para incluir entries. E terão permissão para atualizar os valores armazenados em uma conta.

O motivo da limitação de acesso de outras contas é devido a possibilidade de terceiros conseguirem acesso aos dispositivos sem permissão e usá-los para acessar o smart contract. Caso isso aconteça, a unica ação que um dispositivo infringido pode fazer é incluir e listar entries. Uma ação que é limitada pela quantidade de gás disponível na conta do usuário sensor.

As funções que controlam acesso ao contrato são as funções privadas:

 - ```Contract::assert_owner_only```: Entra em pânico se o caller não for o owner. Owner é a mesma conta em que o contrato foi implantado (deployed).
 - ```Contract::assert_user_allowed```: Entra em pânico se o caller não for um usuário incluido na lista de permitidos. Owner se encontra na lista de permitidos.


```rust
// Garante que apenas owner está chamando a função.
fn assert_owner_only(&self){
    let predecessor: AccountId = env::predecessor_account_id();
    let owner_id: AccountId = AccountId::from(env::current_account_id());

    assert_eq!(predecessor, owner_id, "Only owner's account is allowed to make this function call.");
}

// Garante que apenas usuários permitidos podem chamar funções.
fn assert_user_allowed(&self) {
    let predecessor_id: AccountId = env::predecessor_account_id();
    let owner_id: AccountId = env::current_account_id();

    // Se a conta dono do contrato está chamando a função.
    if owner_id == predecessor_id {
        return;
    }

    // Se não for a conta dono, e não estiver incluido na lista de permitidos, causa panic.
    assert!(self.users.contains(&predecessor_id), "User not allowed to make this call.");
}
```

Através do módulo ```near_sdk::env```, temos acesso a informações relacionadas ao ambiente da máquina virtual, e informações sobre a mensagem recebida. Segue a descrição de alguns dados disponibilizados pelo módulo:

 - ```env::predecessor_account_id```: ID da ultima conta que assinou a chamada call;
 - ```env::signer_account_id```: ID da primeira conta que assinou a chamada call;
 - ```env::current_account_id```: ID da conta atual. A conta que possui o contrato;

Nas situações mais comuns, "**predecessor_account_id**" e "**signer_account_id**" são a mesma conta. Mas contratos podem chamar outros contratos (chamadas "cross-contract"), cada conta assina a chamada seguinte. Por exemplo, digamos que uma conta **A** chame um contrato **B**, que chama um contrato **C**, que chama um contrato **D**:

```
A -> B -> C -> D
```

Na situação acima. Para todas as chamadas, o "**signer_account_id**" é a conta A. 
O "**predecessor_account_id**" é:
 
 - Para o ambiente **B**, o "**predecessor_account_id**" é **A**;
 - Para o ambiente **C**, o "**predecessor_account_id**" é **B**;
 - Para o ambiente **D**, o "**predecessor_account_id**" é **C**;

Para ambos exemplos acimas. Coletamos "**owner_account_id**" e "**predecessor_account_id**". Se ambos são iguais, o **chamador** é o "**owner**". Se "**predecessor_account_id**" estiver incluido na lista de permitidos, então é um usuário permitido.

Poderiamos ter usado "**signer_account_id**". Mas isso anularia a possibilidade de chamadas cross-contract. Um desenvolvedor pode decidir adicionar mais funcionalidades a este contrato. Esta decisão manterá a oportunidade de integração com a funcionalidade de outros contratos.

---

### Acesso Cross-Contract

[topo](#lição-6---2-termômetro)

Como descrito acima. Chamada "cross-contract" é quando um contrato faz uma chamada "call" para um outro contrato. Cada conta assina a chamada seguinte. Em alguns casos isso não é desejado. Como funções que fazem alterações críticas ou caras no sistema. Funções em que o desenvolvedor não deseja que sejam chamadas automaticamente.

Isso é implementado através da função ```Contract::no_cross_contract```. Descrito a seguir:

```rust
// Garante que o chamado é direto. Não pode ser um contrato chamando outro contrato.
fn assert_no_cross_contract(&self){
    let signer_id: AccountId = env::signer_account_id();
    let predecessor_id: AccountId = env::predecessor_account_id();
    assert_eq!(signer_id, predecessor_id, "Cross-contract calls not allowed.");
}
```

Basta garantir que o "signer_account_id" é o mesmo que o "predecessor_account_id".

Isto é usado nas funções ```Contract::allow_user```, ```Contract::remove_user``` e ```Contract::set_format```. Usado na função ```Contract::set_format``` . As outras funções porque são operações administrativas que podem incluir usuários indesejados, ou remover usuários e grande quantidade de dados do sistema.
 
 - ```Contract::set_format```: pode resultar em altos consumos de gás para uma grande quantidade de dados;
 - ```Contract::allow_user```: pode incluir usuários indesejados ao sistema. É uma função pouco usada, mas essencial;
 - ```Contract::remove_user```: pode remover usuários e todos os dados associados a um respectivo usuário. Mal uso dessa função pode causar danos irreversiveis aos dados;

---

### Controle de Output

[topo](#lição-6---2-termômetro)

Uma mesma função pode retornar tipos diferentes através do uso de enums. Para isso, primeiro criamos o enum que representa todos os possiveis tipos que podem ser retornados:

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

 - ```#[derive(Deserialize, Serialize)]``` aplica as traits ```near_sdk::serde::Deserialize``` e ```near_sdk::serde::Serialize``` ao enum. São necessárias para converter um json para o tipo (deserializar), e converter um tipo para json (serializar).
 - ```#[serde(crate) = "near_sdk::serde"]``` irá informar o compilador que a crate "serde" se encontra em "near_sdk::serde". Sem essa instrução, o compilador tentará encontrar "serde" em uma crate única.
 - ```#[serde(untagged)]``` é um atributo da crate "serde" que informa à crate para não usar tags para este enum. Sem este atributo, o valor é descrito como ```{ Single: valor }``` ou ```{ Multiple: [valor1, valor2, ...] }```. Com este atributo o valor é descrito como ```valor```, ou ```[valor1, valor2, ...]```, respectivamente.

Mais detalhes sobre serde e configurações no [site oficial](https://serde.rs/enum-representations.html) da crate.

Com o enum declarado e configurado. Basta retornar o tipo em uma função de contrato:

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

### Controle de Input

[topo](#lição-6---2-termômetro)

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

### Implementação de Traits

[topo](#lição-6---2-termômetro)

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

