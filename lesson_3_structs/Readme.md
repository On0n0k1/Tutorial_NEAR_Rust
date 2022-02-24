# Lição 3 - Structs

Esta lição discute sobre o formato de structs e como ownership é usada nos métodos.

## Tópicos

 - [Descrição](#descri%C3%A7%C3%A3o)
 - [Funções de Contrato](#fun%C3%A7%C3%B5es-de-contrato)
 - [Projeto](#projeto)
   - [Estrutura de Contrato](#estrutura-de-contrato)
   - [Trait Clone](#trait-clone)
   - [just_a_function](#just_a_function)
   - [macros](#macros)
   - [take_ownership](#take_ownership)

## Descrição
[voltar](#li%C3%A7%C3%A3o-3---structs)

Structs são um pouco semelhantes a classes em algumas linguagens orientadas a objetos. A diferença é que ```structs``` não podem herdar outros ```structs```, só podem implementar ```traits```.

Aprofundaremos o conceito de traits em uma lição adiante. Mas considere traits como um conjunto de funções representando uma característica que vários tipos diferentes podem implementar. A trait ```Clone``` permite usar o método ```.clone()``` para criar uma cópia do elemento. A trait ```BorshDeserialize``` permite tentar construir uma instância do tipo utilizando um string no formato json.

Mais adiante aprenderemos a criar funções que aplicam a qualquer tipo de variável que implementa uma trait especifica. Porém, essa lição será apenas sobre structs.

## Funções de Contrato
[voltar](#li%C3%A7%C3%A3o-3---structs)

```rust
// gets e sets
pub fn get_a_string(&self) -> String;

pub fn get_a_floating(&self) -> f32;

pub fn get_another_integer(&self) -> i32;

pub fn get_an_integer(&self) -> u32;

pub fn set_a_string(&mut self, a_string_arg: String);

pub fn set_a_floating(&mut self, a_floating: f32);

pub fn set_an_integer(&mut self, an_integer: u32);

pub fn set_another_integer(&mut self, another_integer: i32);

// Uma função que não altera nem acessa o estado do contrato
pub fn just_a_function();

// Uma função StructExample que toma ownership de si mesmo, se deletando no final.
pub fn take_ownership(&self) -> u32;
```

Cheque a implementação para mais detalhes.

## Projeto
[voltar](#li%C3%A7%C3%A3o-3---structs)

Criamos um tipo de nome ```StructExample```.

```rust
pub struct StructExample{
    an_integer: u32,
    another_integer: i32,
    a_floating: f32,
    a_string: String,
}
```
O tipo possui os seguintes valores internos:
 - an_integer: um inteiro positivo de 32 bits;
 - another_integer: um inteiro positivo ou negativo de 32 bits;
 - a_floating: um número real de 32 bits;
 - a_string: um string, descrito na lição anterior;

Em muitas linguagens teriamos que escrever algo como ```long int``` para i32, ```long long int``` para i64. Em rust e assemblyscript, simplesmente usamos i para "signed" (positivo e negativo) e u para "unsigned" (positivo). u8, u16, u32, u64 e u128 são todos tipos válidos de inteiros "unsigned".

### Estrutura de Contrato
[voltar](#li%C3%A7%C3%A3o-3---structs)

O contrato possui a seguinte estrutura:

```rust
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {
    struct_example: StructExample,
}
```

Este exemplo de contrato deriva a trait ```Default``` em vez de implementar manualmente. Essa forma de implementar executa o método ```.default()``` para todos os valores internos. Devido a isso, o tipo ```struct_example``` deve implementar ```Default``` também.

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

Escolhemos alguns valores aleatórios para servir de exemplo. Vemos que não é necessário escrever algo como ```an_integer: an_integer``` ou ```a_floating: a_floating``` quando os nomes são iguais.

### Trait Clone
[voltar](#li%C3%A7%C3%A3o-3---structs)

Implementamos a trait Clone para o tipo:

```rust
impl Clone for StructExample{
    // self é uma instancia de StructExample, Self (Letra maiúscula) é o tipo StructExample.
    fn clone(&self) -> Self {
        let an_integer: u32 = self.get_an_integer();
        let another_integer: i32 = self.get_another_integer();
        let a_floating: f32 = self.get_a_floating();
        let a_string: String = self.get_a_string();

        // Self e StructExample são a mesma coisa (Em qualquer impl de StructExample)
        Self {
            an_integer,
            another_integer,
            a_floating,
            a_string,
        }
    }
}
```

**Vale relembrar que estou intencionalmente escrevendo o código de forma mais complexa apenas para demonstrar as diversas liberdades que temos na implementação de nossos projetos.**

Não há muito o que adicionar sobre as implementações dos ```get```s e ```set```s. Recomendo checar os comentários. Mas discutiremos sobre ```just_a_function``` e ```take_ownership``` a seguir:

### just_a_function
[voltar](#li%C3%A7%C3%A3o-3---structs)

```rust
pub fn just_a_function() {
    env::log(b"You just called this function");
    env::log(format!("1 + 1 = {}", 1 + 1).as_bytes());
}
```
Esta função imprime duas linhas de texto. 

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

A [próxima lição](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/lesson_4_modules) será sobre módulos.
