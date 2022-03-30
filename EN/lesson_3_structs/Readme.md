# Lsesson 3 - Structs

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

A `struct` is similar to a class in other object-oriented programming languages. The difference is that a `struct` doens't support inheritance, but we can implement one or more `trait`s on them, and these traits specify behavior. 

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
 - an_integer: an unsigned 32-bit integer.
 - another_integer: a signed 32-bit integer.
 - a_floating: a floating 32-bit number.
 - a_string: a String, described in the previous lesson.

Em muitas linguagens teriamos que escrever algo como ```long int``` para i32, ```long long int``` para i64. Em rust e assemblyscript, simplesmente usamos i para "signed" (positivo e negativo) e u para "unsigned" (positivo). u8, u16, u32, u64 e u128 são todos tipos válidos de inteiros "unsigned".

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

A função ```log``` recebe uma sequência de bytes como parâmetro. Devido a isso, a primeira linha mostra que podemos incluir a letra "b" antes das aspas para tratar a string como bytes.

Na segunda instrução usamos o macro ```format!``` para formatar uma String dinamicamente. O tipo String possui um método ```.as_bytes``` que converte o tipo para bytes. Para mais detalhes sobre String, cheque os [docs oficiais](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes).

---

### Macros
[voltar](#li%C3%A7%C3%A3o-3---structs)

Serão explicados mais adiante. Para simplificar o entendimento inicial, considere **macros** como funções que são executadas antes do código ser compilado. São funções que geram código. Só depois do **macro** gerar código que o compilador checa por erros. O uso mais comum de **macros** é para agir como funções que recebem um número variado de parâmetros.

Outra forma de vermos **macros** é: uma forma de trocar complexidade de código por praticidade de uso.

---

### take_ownership
[voltar](#li%C3%A7%C3%A3o-3---structs)

```rust
pub fn take_ownership(self) -> u32{
    env::log(b"Taking ownership of itself");

    let result = format!("an_integer is {}", self.an_integer);
    env::log(result.as_bytes());

    self.an_integer

    // self será liberado da memória aqui
}
```

Acho esse exemplo interessante. 
 - Imprime "Taking ownership of itself" na tela. 
 - Imprime o valor de ```an_integer``` no contrato. 
 - E retorna o valor de ```an_integer```.

Mas como declaramos ```self``` em vez de ```&self``` ou ```&mut self``` como argumento, o método tomará possessão (ownership) de si mesmo e se auto-destruira no fim. 

Um usuário iniciante provavelmente receberá um aviso de erro muito confuso se tentar escrever o método dessa forma. Um erro similar a "valor não pode ser usado pois um move aconteceu aqui.".


Lesson 3 :white_check_mark: ... **Done! Congratulations!**

Our [next lesson](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_4_modules) will be about Rust's modules.