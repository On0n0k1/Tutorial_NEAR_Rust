# Lição 2: Ownership

[voltar](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/)

Este curto tutorial possui o objetivo de explicar sobre ownership.

## Funções de Contrato

```rust
/// Retorna o comprimento da String armazenada
pub fn get_length(&self) -> u32;

/// Retorna o comprimento da String e altera o nome armazenado para "Changed name"
pub fn get_length_again(&mut self) -> u32;
```

## Hipótese

Ownership é explicado na seção seguinte. Esta discute o problema que ownership soluciona.

Consideremos a instrução exemplo a seguir:

```
A = B;
```

Sabemos que A é igual a B. A recebe o valor de B. Mas o que está acontecendo? 

Estamos criando uma cópia do valor de B e associando A a este valor? Criar uma cópia significa requisitar alocação de memória, adiquirir o endereço da memória e igualar o valor desse endereço ao valor de B. Para um número inteiro isso parece simples, mas e se fosse uma string de 2 mil caracteres?

E se usarmos uma variável como parâmetro de função. Estariamos criando uma cópia da variável e depois apagando a cópia no fim da função?

Percebe-se que precisamos de uma forma de reutilizar o mesmo endereço de memória em várias partes diferentes do programa. A linguagem C solucionou isso através do uso de ponteiros. Em vez de armazenarmos o valor da variável, nós armazenamos o endereço de memória daquele tipo de variável.

Mas isso nos trás outro problema. Se uma função tem acesso ao endereço de memória de uma variável importante, essa função agora tem muito poder. E se o método foi implementado de uma forma insegura? Um hacker poderia aproveitar essa falha de segurança para acessar um sistema.

 - Precisamos de uma forma de reutilizar memória para evitar sobrecarregar o sistema com operações desnecessárias.
 - Mas precisamos evitar que esse uso de memória dê mais poder as instruções do que é necessário.

**Extra:** Ponteiros existem em Rust também. Mas existem vários tipos de ponteiros, com diferentes vantagens e desvantagens. Ponteiros semelhantes à linguagem C podem ser utilizados também, mas os blocos que os utilizam precisam ser marcados como "unsafe" ([Mais Informações](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)).

## Ownership

A instrução a seguir:

```
A = B;
```

Pode agir de duas formas diferentes:
 - Se **B** implementa a trait Copy, irá criar uma cópia automaticamente.
 - Se **B** não implementa Copy, **A** será dona de **B** agora. O compilador não deixará realizarmos instruções com **B**, porque o valor de **B** foi "movido" para **A**.

Dos tipos primitivos: 
 - Números (```u32```, ```f32```, ```i32```, ...) implementam ```Copy``` e ```Clone```. 
 - String implementa ```Clone```, mas não implementa ```Copy```. 

Em outras palavras, para criarmos uma cópia de um String, precisamos fazer isso manualmente.

Ownership garante que apenas uma variável é dona ("owns") de um endereço de váriável. Essa possessão pode ser transferida. Mas para compartilharmos uma variável, usamos ponteiros ou referências.

## Referências

Referências, ou empréstimos ("borrows") são uma forma de compartilhar um endereço de memória com permissões limitadas. As referências podem ser mutáveis ou imutáveis. São escritas, respectivamente, da seguinte forma:

```rust
let a = 10; // Criando uma variável com valor 10
let b = &a; // Criando uma variável que é uma referência à variavel a
```

```rust
let mut a = 10; // Criando uma variável mutável com valor 10
let b = &mut a; // Criando uma variável que é uma referência mutável à variavel a
```

Variáveis são, por padrão, imutáveis, constantes. Por isso declarei ```a``` mutavel no segundo exemplo.

 - Referências imutáveis permitem acessar o valor, mas não permitem alterá-lo.
 - Referências mutáveis permitem acessar o valor e permitem alterá-lo.

Algumas regras a considerar:
 - Não se pode alterar a variável original enquanto uma referência ainda existe.
 - Várias referências imutáveis podem existir.
 - Só uma referência mutável pode existir.
 - Não podem existir referências imutáveis se uma referência mutável existe.

Quando criamos uma referência, digamos que a variável dona está "emprestando" ("borrow") para a outra. A linha em que o empréstimo é utilizado por ultimo é a linha em que o empréstimo é devolvido.

## Importante

Não retorne referências. Retornar referências é possivel, mas é preciso marcar o tempo-de-vida (lifetime) do valor retornado. Não recomendamos estudar isso enquanto está aprendendo a linguagem rust. Todo o conceito de lifetimes pode ser evitado simplesmente retornando cópias quando necessário. Para os interessados, referências [aqui](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html).

Lifetimes são um conceito bem poderoso quando usado corretamente. As ferramentas serde e borsh usam isso para converter texto json para o tipo que precisamos com zero cópia. Ou seja, existe alocação de memória para o String json, para o tipo que precisamos, e nada mais. O processador não precisa esperar alocação de memória, ou seja, muito rápido.

## Exemplos

Para o contrato descrito a seguir:

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

Temos os exemplos a seguir:

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
        "Ambos tamanhos não são o mesmo {} e {}", length_reference, length_ownership,
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

Antes de iniciarmos com os detalhes falaremos sobre String e &str.

### O que é String

Uma String é uma variável que possui dono. Armazena um "string" e irá ser liberado da memória quando a variável ser liberada. Mas "Um texto entre aspas como este não é um String, é um &str". Uma referência a um String é um &String ou &mut String.

### O que é &str

Isso é um tipo criado para simplificar o uso de Strings em nosso código. Age como uma referência imutável à um String. Mas este será alocado pelo compilador, e o compilador decide como melhor otimizá-lo na memória.

### Strings em Funções

Consideraremos as duas variáveis abaixo para o exemplo:

```rust
let variavel: String = String::from("Uma Variavel");
let referencia: &str = "Uma Variavel";
```

A função abaixo recebe um &str e retorna o comprimento da string. O empréstimo é devolvido no fim da função.

```rust
fn this_takes_a_reference(name: &str) -> usize { 
    return name.len();
}
```

Para utilizar a função com os parâmetros:

```rust
this_takes_a_reference(&variavel);
this_takes_a_reference(referencia);
```

A função abaixo recebe um String como parâmetro e retorna o comprimento. A função se torna dona do endereço de memória e o deleta no fim.

```rust
fn this_takes_the_ownership(name: String) -> usize {
    name.len()
}
```

Para utilizar a função com os parâmetros:

```rust
this_takes_the_ownership(variavel);
this_takes_the_ownership(String::from(referencia));
```

Precisamos de transformar o &str em um String antes de usar como parâmetro. A função adiquiriu ownership quando não precisava também.

Ambas as funções ```this_takes_a_reference``` e ```this_takes_the_ownership``` fazem a mesma coisa, não causam erros, e retornam o mesmo resultado. Mas a primeira é bem mais eficiente do que a segunda.

Por isso, é boa prática usar ```&str``` em vez de ```String``` nas declarações de função.

Eu lamento ter que adicionar mais um detalhe nessa explicação, mas funções de contrato, aquelas em que precisamos de marcar com ```#[near_bindgen]``` precisam de receber String como parâmetro. Isso é porque as traits de deserialização são implementadas para String, mas não são implementadas para referências de string.

A função:

```rust
pub fn get_length(&self) -> u32 {
    let length_reference: usize = Self::this_takes_a_reference(&self.name);
    let length_ownership: usize = Self::this_takes_the_ownership(self.name.clone());

    assert_eq!(
        length_reference, 
        length_ownership, 
        "Ambos tamanhos não são o mesmo {} e {}", length_reference, length_ownership,
    );
}
```

Chama ```this_takes_a_reference``` e ```this_takes_the_ownership```, garantindo que ambas retornam o mesmo valor antes de retorná-lo. Como ```this_takes_the_ownership``` não pega o valor emprestado, criamos uma cópia para ser usada como necessário.

A função:

```rust
pub fn get_length_again(&mut self) -> u32 {
    let a_reference: &String = &self.name;
    let _another_reference: &String = &self.name;
    let _yet_another_reference: &String = &self.name;
    let length = Self::this_takes_a_reference(a_reference);
    self.name = String::from("Changed name");

    length as u32
}
```

Simplesmente chama ```this_takes_a_reference``` e altera o "nome" armazenado no contrato. Este exemplo mostra que podem haver várias referências para uma variável. Faça as alterações recomendadas nos comentários para ver as reações do compilador.

A [próxima Lição](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/lesson_3_structs) será sobre structs.
