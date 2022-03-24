# Lição 1: Contrato

[voltar](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/ES/)

Veja também:
 - Usos da [ferramenta cargo](https://github.com/On0n0k1/Tutorial_NEAR_Rust/blob/main/ES/static/tutorials/cargo.md).
 - Usos da [ferramenta near-cli](https://github.com/On0n0k1/Tutorial_NEAR_Rust/blob/main/ES/static/tutorials/nearcli.md).

---

## Tópicos
 - [Estrutura de um contrato NEAR](#estrutura-de-um-contrato-near)
 - [Importar Dependências](#importar-depend%C3%AAncias)
 - [Macro de Alocação](#macro-de-aloca%C3%A7%C3%A3o)
 - [Declaração de Contrato](#declara%C3%A7%C3%A3o-de-contrato)
 - [Declaração de API do Contrato](#declara%C3%A7%C3%A3o-de-api-do-contrato)
 - [Testes de Unidade](#testes-de-unidade)

---

## Estrutura de um contrato NEAR
[topo](#li%C3%A7%C3%A3o-1-contrato)

Um contrato NEAR na linguagem Rust pode ser resumido aos seguintes passos:
 - Importar crates, módulos e outras dependências necessárias.
 - Macro de alocação.
 - Declaração de contrato.
 - Declaração de api do contrato.
 - Testes de unidade.

O desenvolvedor é livre para adicionar o que julgar necessário ao projeto. Os passos acima são apenas para acelerar a memorização.

---

### Importar Dependências
[topo](#li%C3%A7%C3%A3o-1-contrato)

Isto é explicado em detalhes na "lição 4 - módulos". Só precisamos saber sobre as diferenças entre mod e use. Por exemplo:

```rust
use near_sdk::near_bindgen;
```

Acessa a crate near_sdk e inclui o macro near_bindgen neste namespace. Sem isso, precisariamos escrever "near_sdk::near_bindgen" todas as vezes que precisarmos deste elemento. Mas agora podemos simplesmente escrever "near_bindgen".

Por outro lado, a instrução:

```rust
mod outro_modulo;
```

Significa que existe um arquivo de nome "outro_modulo.rs" ou um diretório com o nome "outro_modulo" no mesmo diretório deste arquivo rust. Para mais detalhes, procure por lesson_4_modules.

Se houvesse pub antes da instrução, como os exemplos:

```rust
pub mod outro_modulo;
```

Ou:

```rust
mod outro_modulo;

pub use outro_modulo::alguma_dependencia;
```

Demonstram que "outro_modulo" e "alguma_dependencia" podem ser importados por um outro módulo ou crate (projeto rust) externos. 

Para mais detalhes, procure por lesson_4_modules. Importar e exportar módulos é uma característica da linguagem Rust. Não tem nenhum efeito direto na plataforma NEAR.

Agora, referindo ao contrato:

```rust
use near_sdk::{
    borsh::{
        self,
        BorshDeserialize,
        BorshSerialize,
    },
    near_bindgen,
};
```

Estamos acessando a crate "near_sdk" declarado em "Cargo.toml". Importando self, BorshDeserialize e BorshSerialize no módulo borsh. E importando near_bindgen. Seguem as descrições simplificadas:

 - **self**: Nem eu sei exatamente o que isso faz, mas é necessário para BorshDeserialize e BorshSerialize funcionarem corretamente.
 - **BorshDeserialize**: Quando chamamos uma função do nosso contrato, devemos enviar parâmetros. Mesmo que estes parâmetros sejam um json vazio, este deve ser deserializado. Este é o objetivo de BorshDeserialize, recebe um json em texto, transforma nos tipos que precisamos.
 - **BorshSerialize**: Caminho inverso do BorshDeserialize. Quando vamos retornar um resultado para o usuário, devemos transformar aquele valor para um json em texto.
 - **near_bindgen**: Isso é um marcador para um struct que diz **"Este é o contrato principal do nosso projeto"**. Damos o nome "Contract" apenas para facilitar o entendimento, não é obrigatório. Porém deve-se ter pelo menos um struct com **near_bindgen** em cada contrato.

---

### Macro de alocação
[topo](#li%C3%A7%C3%A3o-1-contrato)

```rust
near_sdk::setup_alloc!();
```

Macros parecem com funções. Mas são executadas antes da compilação. São ferramentas para gerar código de acordo com os parâmetros. Macros não existem no runtime do programa.

Neste caso, "setup_alloc" gera o código "boilerplate" (forma) para o funcionamento do nosso contrato. Só deve ser executado uma vez, antes da declaração do contrato.

Aviso: Nas próximas versões esta instrução será deprecada. Não será necessário usar mais.

---

### Declaração de contrato
[topo](#li%C3%A7%C3%A3o-1-contrato)

```rust
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract{
    counter: i32,
}
```

Será mais fácil descrever de dentro para fora.
 - **counter**: é um número. i32 quer dizer que é "signed", pode ser positivo ou negativo. 32 quer dizer que é um número de 32 bits.
 - **pub struct Contract**: é a declaração de um struct de nome Contract. "pub" quer dizer que este struct é público.
 - **#[derive(BorshDeserialize, BorshSerialize)]**: Simplificando, aplica as traits BorshDeserialize e BorshSerialize neste struct. Descritos acima.
 - **[near_bindgen]**: É um marcador que diz "Este é o Contrato". As funções deste struct são as funções do contrato. Quando executamos uma função do contrato, executamos uma função deste struct.

Logo a seguir temos também:

```rust
impl Default for Contract{
    fn default() -> Self{
        Contract { counter: 0 }
    }
}
```

Default é uma trait (característica) de "padrão". É praticamente um construtor sem parâmetros para o nosso struct. Mas, Default é uma trait padronizada da linguagem Rust. near_sdk usa essa trait no funcionamento do nosso contrato. Então precisamos aplicar ao nosso contrato, senão teremos um erro de compilação.

default é uma função da trait Default que retorna um struct do mesmo tipo Self. Self nesta declaração é o mesmo que Contract. A função retorna uma instância de Contract com o valor de counter igual a 0.

Se implementarmos este contrato em uma conta NEAR, e depois executarmos uma primeira função que não seja de inicialização. A máquina irá inicializar o contrato com  default antes de executar nossa função.

---

### Declaração de API do contrato
[topo](#li%C3%A7%C3%A3o-1-contrato)

A seguir se encontram as funções do smart contract.

```rust
#[near_bindgen]
impl Contract{

    pub fn get(&self) -> i32 {
        self.counter
    }

    pub fn increment(&mut self) -> i32 {
        self.counter += 1;
        self.counter
    }

    pub fn decrement(&mut self) -> i32 {
        self.counter -= 1;
        self.counter
    }
}
```

 - ```#[near_bindgen]``` é um marcador que diz "estas são as funções do contrato".
 - ```impl Contract``` é onde declaramos as funções e métodos associados ao struct Contract.
 - ```&self``` e ```&mut self``` são descritos na lição seguinte. Só precisamos saber que esse tipo de função é invocada no formato ```nome_do_struct.nome_da_funcao```, self neste caso se refere a instância existente deste struct.
 - ```-> i32``` significa que a função retorna um inteiro i32.
 - No fim da função temos uma linha ```self.counter``` sem ponto-virgula ";". Isso é o mesmo que ```return self.counter```.

Com estes detalhes, vemos que a função ```get``` retorna o valor atual de counter armazenado no struct do contrato. ```increment``` incrementa o valor de counter em 1. ```decrement``` reduz o valor de counter em 1.

---

### Testes de unidade
[topo](#li%C3%A7%C3%A3o-1-contrato)

Descrito com mais detalhes na lição 4 - módulos, não é necessário ter todos os testes do projeto aqui. Podemos incluir testes no fim de cada módulo rust. Podemos também criar um diretório ```tests```. Todos os arquivos ".rs" na pasta ```tests``` será considerado um módulo de testes.

```rust
#[cfg(test)]
mod tests{
```

```mod tests``` é simplesmente um módulo local com nome tests. Nada de especial.

```#[cfg(test)]``` este é bem interessante. ```cfg``` é uma instrução que diz ao compilador "Compile o módulo abaixo de mim apenas se a condição entre parenteses for verdadeira.". ```(test)``` é verdadeiro quando executamos ```cargo test```. Se não estivermos realizando testes de unidade, este módulo não existe.

Se em vez de ```#[cfg(test)]``` tivéssemos:

```rust
#[cfg(not(test))]
mod another_module{
```

Teriamos a situação oposta, este módulo não seria compilado durante testes de unidade.

```rust
use super::*;
use near_sdk::{
    AccountId,
    env,
    MockedBlockchain,
    testing_env,
    test_utils::VMContextBuilder,
    json_types::ValidAccountId,
};
```
Acima importamos as dependências usadas nos testes abaixo.

```rust
fn env_setup(){
    let mut builder: VMContextBuilder = VMContextBuilder::new();
    let account_id: AccountId = String::from("stiltztinkerstein");

    builder.current_account_id(
        ValidAccountId::try_from(
            account_id.clone()
        ).unwrap()
    );

    testing_env!(builder.build());

    assert_eq!(
        env::current_account_id(),
        account_id, 
        "Erro assert.\n env: {}\naccount: {}\n", 
        env::current_account_id(), 
        &account_id,
    );
}
```

Antes de cada teste, precisamos iniciar uma simulação do ambiente de blockchain. Uma das formas de fazer isso é utilizando ```VMContextBuilder```. Basta criar uma instância desse tipo, alterar os atributos que queremos, e usar o builder como argumento para o macro ```testing_env```.

Para não termos que escrever estas linhas em cada teste, criamos uma função para ser usada.

```assert_eq``` não é necessário. Só mostra que o atributo de ambiente ```env::current_account_id``` é o mesmo id de conta que escolhi para o builder.

A seguir teremos os três testes: 

```rust
#[test]
pub fn get() {
    env_setup();

    let contract: Contract = Contract::default();
    
    assert_eq!(
        contract.get(),
        0
    );
}

#[test]
pub fn increment() {
    env_setup();

    let mut contract: Contract = Contract::default();

    contract.increment();

    assert_eq!(
        contract.get(),
        1
    );
}

#[test]
pub fn decrement() {
    env_setup();

    let mut contract: Contract = Contract::default();

    contract.decrement();

    assert_eq!(
        contract.get(),
        -1
    );
}
```

Percebe-se um padrão em cada um dos testes:
 - Inicializar o ambiente;
 - Inicializar o contrato;
 - Executar a função que queremos testar;
 - Confirmar que a função deu o resultado que esperamos;

A função ```get``` foi testada primeiro. Isto porque esta será usada nos testes seguidos. Se esta função não funcionasse da forma que esperassemos, temos que ver este erro primeiro na lista. Bom para evitar confusões nos testes. 

A [próxima lição](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/ES/lesson_2_ownership) será sobre ownership.
