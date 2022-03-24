# Lição 6 - 1 Uso de Enums

[voltar](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/lesson_6_enums/lesson_6_1_simple/)

Essa lição descreve enums e instruções ```match```.

## API de contrato

```rust

/// Podemos usar instruções match para Strings e &str.
/// 
/// Esta função é um exemplo. 
/// 
/// Retorna 1, 2, 3, 4, 5, se o argumento for o número.
/// 
/// Causa panic se for outro valor.
pub fn string_match_example(&self, arg: String) -> u32;

/// Retorna example_0.
pub fn get_example_0(&self) -> Example0;

/// Retorna example_1.
pub fn get_example_1(&self) -> Example1;

/// Retorna example_2.
pub fn get_example_2(&self) -> Example2User;

/// Chama Example0::get_number.
pub fn example_0_get_number(&self) -> u32;

/// Chama Example0::is_third.
pub fn example_0_is_third(&self) -> bool;

/// Chama Example1::get.
pub fn example_1_get(&self) -> String;

/// Chama Example1::is_novalue.
pub fn example_1_is_novalue(&self) -> bool;

/// Chama Example1::get_an_integer.
pub fn example_1_get_an_integer(&self) -> String;

/// Chama Example1::has_an_odd_number.
pub fn example_1_has_an_odd_number(&self) -> bool;

/// Chama Example2User::get_name.
pub fn example_2_get_name(&self) -> String;

/// Chama Example2User::has_permission.
pub fn example_2_has_permission(&self, permission: String) -> bool;

/// Chama Example2User::get_actions.
/// 
/// Quando retornamos um Vec, o serializer tentará usar serde::json.
/// A instrução #[result_serializer] nos permite selecionar borsh como serializador.
#[result_serializer(borsh)]
pub fn example_2_get_actions(&self) -> Vec<String>;
```

## Compilando, testando

Essa crate pertence ao workspace da lição 6. Instruções sobre compilação e execução de testes na pagina [anterior](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/lesson_6_enums/lesson_6_1_simple/).

Executar comandos como ```cargo test``` e ```cargo build``` afetará todos as crates na workspace. A não ser que uma crate seja especificada.


## Tópicos

 - O que são enums.
 - Instruções match.
 - Exemplo 0: enums sem valores.
 - Exemplo 1: enums com primitivos.
 - Exemplo 2: usuários como enums.
 - Instruções match para strings.

## O que são enums

Enquanto Structs armazenam diversos valores simultaneamente. Enums armazenam um valor de cada vez. As alternativas que um enum pode representar são descritos na definição.

```rust
pub enum Example0{
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}
```

Acima temos um exemplo de enum.
 - ```pub``` descreve que o enum está disponivel para ser usado em módulos externos.
 - ```Example0``` é o nome do enum.
 - ```First```, ```Second```, ```Third```, ```Fourth``` e ```Fifth``` são os nomes dos valores que este enum pode possuir.

Acima temos a declaração do enum, mas como criamos uma instância de enum? A seguir criamos um exemplo para os 5 valores possiveis.

```rust
let a = Example0::First;
let b = Example0::Second;
let c = Example0::Third;
let d = Example0::Fourth;
let e = Example0::Fifth;
```



## Instruções match




