# Lição 5 - Uso de Macros

[voltar](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/)

Macros são a ferramenta principal para a utilização efetiva da linguagem rust. Não é necessário saber como criar macros. Mas é essencial entender o que são e como usá-los.

## API de Contrato

```rust
/// Esta função mostra as diferenças entre println e env::log
/// Execute com ```cargo test -- --nocapture```, compile, implante e execute em Near.
/// Note como algumas mensagens aparecem e outras não.
pub fn print_examples();

/// Exemplos de format. Compare o output com a implementação.
pub fn format_examples();

/// Exemplo de panico.
pub fn panic_example();

/// Exemplo de usos de vec.
pub fn vec_examples();
```

## Tópicos

 - [O que são macros](#o-que-são-macros)
 - [Macros "function-like"](#macros-"function-like")
 - [Vantagens](#vantagens)
 - [Desvantagens](#desvantagens)
 - [Exemplos](#exemplos)
   - [format, println e panic](#format-println-e-panic)
   - [vec](#vec)
   - [setup_alloc](#setup_alloc)
 - [Extra: String e str](#extra-string-e-str) 

## O que são macros

[topo](#lição-5---uso-de-macros)

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
panic!("Panico com alguns argumentos: {} {} {}", 1, second, 3);

// Recebe uma lista de argumentos e retorna um Vec com os valores alocados.
vec![1, 2, 3, 4];

// Gera código de fôrma (boilerplate) necessário para o funcionamento do contrato
near_sdk::setup_alloc!();

// Usado em testes.
// Usa o argumento Context para gerar o contexto de ambiente da máquina virtual.
testing_env!(builder.build());

``` 

## Macros "function-like"

[topo](#lição-5---uso-de-macros)

Esses macros são usados como funções, mas existem algumas diferenças.

Podem ter um número variado de parâmetros. Rust é uma linguagem "statically typed", ou seja, o compilador precisa saber exatamente onde memória é criada e onde é liberada. Porém, macros podem retornar código. Ou seja, um macro como ```println``` ou ```format``` recebem os argumentos e montam um bloco de código antes do compilador checar por erros.

Outra diferença que pode ser vista com o exemplo de ```vec```:

```rust
vec![1, 2, 3, 4];
```

Esta implementação vec é demarcada com ``` [ ] ``` em vez de ``` ( ) ```, macros também podem ser demarcados com ``` { } ```. Macros podem ter literalmente qualquer tipo de texto dentro dos demarcadores. Um desenvolvedor pode escrever qualquer coisa como argumento macro, contanto que a implementação entenda os parâmetros.

## Vantagens

[topo](#lição-5---uso-de-macros)

 - Simplifica o código.
 - É executado durante compilação. Se bem implementado, macros podem ser eficientes com pouco ou nenhum overhead.

## Desvantagens

[topo](#lição-5---uso-de-macros)

 - Um desenvolvedor precisa pesquisar a documentação de cada novo macro que encontram.
 - Podem ser mais dificeis de "debugar".
 - Aumentam o tempo de compilação.
 - Podem "inflar" o código com implementação "invisivel".

## Exemplos

A seguir recomendamos alguns macros "function-like" úteis.

### format, println e panic

[topo](#lição-5---uso-de-macros)

Os macros ```panic```, ```println``` e ```format``` são escritos da mesma forma.

 - ```format``` retorna um String.
 - ```println``` imprime o String no output padrão.
 - ```panic``` encerra a execução e retorna o String como mensagem de erro.

```rust
println!("Isso é um println!, não aparece na máquina virtual");

// Criamos uma variavel "message" e associamos uma String de valor "format retorna uma String formatada".
let message: String = format!("format retorna uma string formatada.");

// Encerra execução com a mensagem de erro "Panico com alguns argumentos: 1 2 3"
let second = 2;
panic!("Panico com alguns argumentos: {} {} {}", 1, second, 3);

```

A utilização destes macros é bem simples. Um str como primeiro argumento ([detalhes](#extra-string-e-str)). Cada "{}" é substituido pelos argumentos após o primeiro.

Podemos customizar a formatação de diversas formas. Mais informações, cheque o [link de documentação sobre formatação](https://doc.rust-lang.org/std/fmt/index.html). Abaixo incluimos algumas úteis utilizações:

```rust
// Exemplos de format
log("\n\nformat_examples:\n");
 
let message: String = format!("Format retorna uma String formatada");

let an_arg = "third";

// format pode receber argumentos usando {} 1 second third
let message = format!("format pode receber argumentos usando {{}}: {}, {}, {}.", 1, "second", an_arg);

let (first, second, third) = (1, "second", an_arg);

// Podemos especificar argumentos dessa forma: 1 second third
let message = format!("Podemos especificar argumentos dessa forma: {first}, {second}, {third}.");

// Podemos especificar a ordem de argumentos de format: 1 second third
let message = format!("Podemos especificar a ordem de argumentos de format: {1}, {2}, {0}.", third, first, second);

let (first, second, third) = (1, 2, 3);
// Podemos fazer inteiros mostrarem um número arbitrário de digitos: 01 0002      3
let message = format!("Podemos fazer inteiros mostrarem um número arbitrário de digitos: {:02}, {:04}, {:6}.", first, second, third);

// Escolhendo número de digitos e ordem: 01    2 000003
let message = format!("Escolhendo número de digitos e ordem: {2:02}, {0:4}, {1:06}.", second, third, first);

let (first, second, third) = (0.1, 1.23, -2.45);
// Podemos escolhar a precisão de números racionais: 0.10 1.230 -2.450000
let message = format!("Podemos escolhar a precisão de números racionais: {:.2}, {:.4}, {:.6}", first, second, third);

// Podemos escolher a precisão e número de digitos: 0.10 0001.2300 -00002.450000
let message = format!("Podemos escolher a precisão e número de digitos: {:2.2}, {:04.4}, {:06.6}", first, second, third);

// Podemos escolher a precisão, o número de digitos e a ordem dos argumentos: 00.10    1.2300 -00002.450000
let message = format!("Podemos escolher a precisão, o número de digitos e a ordem dos argumentos: {1:02.2}, {2:4.4}, {0:06.6}", third, first, second);

// Mesmo que o acima:  0.10 0001.2300      -2.450000
let message = format!("Mesmo que o acima: {first:2.2}, {second:04.4}, {third:6.6}");

```

### vec

[topo](#lição-5---uso-de-macros)

Uma rápida discussão sobre algumas formas de agrupar valores. 

Tuplas possuem tamanho imutavel:

```rust
// Uma tupla com inteiros
let tupla: (u32, u32, u32) = (0, 1, 4);

// Acessando um valor
// O segundo valor é 1
println!("O segundo valor é {}", tupla.1);
```

Arrays possuem tamanho imutável, são armazenados na pilha (stack).

```rust
// Uma forma de declarar um array
let lista = [0, 1, 2];

// Acessando um valor
// O terceiro valor é 2
println!("O terceiro valor é {}", lista[2]);

// um array com 10 inteiros inicializados com 0.
// Esse método só permite inicializar com valores iguais, não tente inicializar com uma função.
let mut lista: [i32; 10] = [0; 10];

// Alterando um valor
lista[0] = -1;

// Acessando um valor
// O primeiro valor é -1
println!("O primeiro valor é {}", lista[0]);
```

Arrays e tuplas são primitivos. Não podemos alterar o número de elementos desses grupos. Para armazenar conjuntos de valores de forma mais prática usamos coleções. Temos coleções rust, que são mais generalizadas, e coleções Near, que são armazenadas na "trie". O entendimento de coleções rust é importante para uma boa lógica de funções. O entendimento de coleções Near é importante para armazenamento eficiente de estado.

 - [Detalhes](https://doc.rust-lang.org/std/collections/) sobre coleções Rust para bom funcionamento de métodos.
 - [Detalhes](https://docs.rs/near-sdk/latest/near_sdk/collections/index.html) sobre coleções Near para bom armazenamento de estado.

A coleção rust mais utilizada é ```Vec```([detalhes](https://doc.rust-lang.org/std/vec/struct.Vec.html)). Com este tipo, podemos armazenar dados, observar a quantidade de elementos, acessar e alterar os elementos. O que é uma boa solução para a maior parte dos casos.

**Detalhe:** ```vec``` (letra minuscula) é o macro para criação de vetores ```Vec```. ```Vec``` (letra maiuscula) refere-se ao tipo do vetor.

Podemos criar um ```Vec``` da seguinte forma:

```rust
// Vec com os números 1 2 3 4
let example = vec![1, 2, 3, 4];
```

Podemos criar um ```Vec``` com vários valores iguais:

```rust
// Vec com os números 0 0 0 0 0
let example = vec![0;5];
```

Imprimir valores de conjuntos pode demandar muito tempo de processamento. Devido a isso, para usar um conjunto de elementos em um ```println```, ```format``` ou ```panic```, precisamos explicitar que é impresso em modo debug.

```rust
let example = vec![1, 2, 3, 4];

log(&format!("Podemos imprimir vetores com modo debug:\n{:?}\n\n", example));

log(&format!("Podemos imprimir vetores em \"formato legivel\":\n{:#?}\n\n", example));

log(&format!("Podemos fazer o mesmo com tuplas:\n{:#?}\n\n", (1, 2, 3)));

log(&format!("Podemos criar vetores com valores padrão:\n{:?}\n\n", vec![0;5]));
```

Formatação ```{:?}``` é "formatação debug".

Formatação ```{:#?}``` é "formatação pretty print". É o mesmo que o acima, porém escrito de uma forma mais legivel para um usuário. Normalmente simplesmente significa um elemento por linha.

Para implementar formatação debug em um struct ou enum, cheque o [link sobre formatação](https://doc.rust-lang.org/std/fmt/trait.Debug.html).


### setup_alloc

[topo](#lição-5---uso-de-macros)

Deve ser usado antes da declaração do contrato. Gera código que deveria ser escrito repetidas vezes em cada contrato.

```rust
near_sdk::setup_alloc!();
```

É necessário na versão de near_sdk atual (```3.1.0```). Nas versões seguintes, será deprecado.


## Extra: String e str

[topo](#lição-5---uso-de-macros)

```String``` e ```str``` são dois tipos diferentes. String é um tipo que mantem ownership de uma string. ```str``` é um tipo usado para referências a strings. Estes tipos existem para minimizar cópias de strings durante o runtime.

Lembrar: 
 - ```str``` é sempre usado como ```&str```. ```&str``` aplica para "strings como essa" e ```&String```;
 - Sempre que for precisar de uma referência para uma ```String``` em uma função, use ```&str```. Não use ```&String```.
 - "Strings como essa" são do tipo ```&'static str```. Mais detalhes na seção sobre lifetimes. Teoricamente, são strings que nunca são removidas da memória, mas isso depende da otimização do compilador.

A [proxima lição](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/lesson_6_enums) será sobre enums.


