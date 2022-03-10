# Lição 5 - Uso de Macros

Macros são a ferramenta principal para a utilização efetiva da linguagem rust. Não é necessário saber como criar macros. Mas é essencial entender o que são e como usá-los.

## Tópicos

 - O que são macros
 - Macros "function-like"
 - Vantagens
 - Desvantagens

## O que são macros

Macros são ferramentas que geram código. Macros são executados em tempo de compilação. 

As instruções derive são um macro:

```rust
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
```

Instruções derive servem para aplicar traits a novos tipos. A instrução a seguir é outro tipo de macro chamado "atributo":

```rust
#[near_bindgen]
impl Contract{
```

Mas discutiremos macros "function-like" (semelhantes a funções). Como o nome descreve, são usados com a mesma lógica do chamado de funções. Alguns exemplos:

```rust
// Imprime o string formatado ao output padrão
println!("{}", message);

// Formata os argumentos em um String
format!("7");

// Formata uma String e entra em pânico com a mensagem formatada
// "Entrar em pânico" Encerra o runtime retornando uma mensagem de erro.
panic!("Panicking with a few args: {} {} {}", 1, second, 3);

// Recebe uma lista de argumentos e retorna um Vec com os valores alocados.
vec![1, 2, 3, 4];

// Gera código de fôrma (boilerplate) necessário para o funcionamento do contrato
near_sdk::setup_alloc!();

// Usado em testes.
// Usa o argumento Context para gerar o contexto de ambiente da máquina virtual.
testing_env!(builder.build());

``` 

## Macros "function-like"

Esses macros são usados como funções, mas existem algumas diferenças.

Podem ter um número variado de parâmetros. Rust é uma linguagem "statically typed", ou seja, o compilador precisa saber exatamente onde memória é criada e onde é liberada. Porém, macros podem retornar código. Ou seja, um macro como ```println``` ou ```format``` recebem os argumentos e montam um bloco de código antes do compilador checar por erros.

Outra diferença que pode ser vista com o exemplo de ```vec```:

```rust
vec![1, 2, 3, 4];
```

Esta implementação vec é demarcada com ``` [ ] ``` em vez de ``` ( ) ```, macros também podem ser demarcados com ``` { } ```. Macros podem ter literalmente qualquer tipo de texto dentro dos demarcadores. Um desenvolvedor pode criar uma linguagem de programação dentro de macros.

## Vantagens

 - Simplifica o código.
 - É executado durante compilação. Se bem implementado, macros podem ser eficientes com pouco ou nenhum overhead.

## Desvantagens

 - Um desenvolvedor precisa pesquisar a documentação de cada novo macro que encontram.
 - Podem ser mais dificeis de "debugar".
 - Aumentam o tempo de compilação.
 - Podem "inflar" o código com implementação "invisivel".

## Exemplos

A seguir recomendamos alguns macros "function-like" úteis.

### format, println e panic

Os macros ```panic```, ```println``` e ```format``` recebem argumentos na mesma estrutura.

 - ```format``` retorna um String.
 - ```println``` imprime o String no output padrão.
 - ```panic``` encerra a execução e retorna o String como mensagem de erro.

```rust
// Isso é um println!, não aparece na máquina virtual
println!("This is a println! It doesn't show in Virtual Machine.");

// Criamos uma variavel "message" e associamos uma String de valor "format retorna uma String formatada".
let message: String = format!("format returns a formatted String.");

// Encerra execução com a mensagem de erro "Panicking with a few args: 1 2 3"
let second = 2;
panic!("Panicking with a few args: {} {} {}", 1, second, 3);

```

A utilização destes macros é bem simples. Um str como primeiro argumento, cada "{}" é substituido pelos argumentos após o primeiro.

Podemos customizar a formatação de diversas formas. Mais informações, cheque o [link de documentação sobre formatação](https://doc.rust-lang.org/std/fmt/index.html). Abaixo incluimos algumas úteis utilizações:

```rust
// Exemplos de format
log("\n\nformat_examples:\n");
// Format retorna uma String formatada
let message: String = format!("format returns a formatted String.");

let an_arg = "third";
// format can take arguments using {} 1 second third
// format pode receber argumentos usando {} 1 second third
let message = format!("format can take arguments using {{}}: {}, {}, {}.", 1, "second", an_arg);

let (first, second, third) = (1, "second", an_arg);
// We can specify format arguments this way: 1 second third
// Podemos especificar argumentos dessa forma: 1 second third
let message = format!("We can specify format arguments this way: {first}, {second}, {third}.");

// We can specify the order of format args: 1 second third
// Podemos especificar a ordem de argumentos de format: 1 second third
let message = format!("We can specify the order of format args: {1}, {2}, {0}.", third, first, second);

let (first, second, third) = (1, 2, 3);
// We can make integers show an arbitrary number of digits: 01 0002      3
// Podemos fazer inteiros mostrarem um número arbitrário de digitos: 01 0002      3
let message = format!("We can make integers show an arbitrary number of digits: {:02}, {:04}, {:6}.", first, second, third);

// Choosing number of digits and order: 01    2 000003
// Escolhendo número de digitos e ordem: 01    2 000003
let message = format!("Choosing number of digits and order: {2:02}, {0:4}, {1:06}.", second, third, first);

let (first, second, third) = (0.1, 1.23, -2.45);
// We can choose the precision of rational numbers: 0.10 1.230 -2.450000
// Podemos escolhar a precisão de números racionais: 0.10 1.230 -2.450000
let message = format!("We can choose the precision of rational numbers: {:.2}, {:.4}, {:.6}", first, second, third);

// We can choose the precision and the number of digits:  0.10 0001.2300 -000002.450000
// Podemos escolher a precisão e número de digitos: 0.10 0001.2300 -00002.450000
let message = format!("We can choose the precision and the number of digits: {:2.2}, {:04.4}, {:06.6}", first, second, third);

// We can choose the precision, the number of digits and the order of arguments: 00.10    1.2300 -000002.450000
// Podemos escolher a precisão, o número de digitos e a ordem dos argumentos: 00.10    1.2300 -00002.450000
let message = format!("We can choose the precision, the number of digits and the order of arguments: {1:02.2}, {2:4.4}, {0:06.6}", third, first, second);

// Same as above:  0.10 0001.2300      -2.450000
// Mesmo que o acima:  0.10 0001.2300      -2.450000
let message = format!("Same as above: {first:2.2}, {second:04.4}, {third:6.6}");

```

