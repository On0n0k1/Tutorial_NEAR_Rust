# Lição 6 - 1 Uso de Enums

[voltar](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/lesson_6_enums/)

Essa lição descreve enums e instruções ```match```.

---

## API de contrato

```rust
// /src/lib.rs

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

---

## Compilando, testando

[topo](#lição-6---1-uso-de-enums)

Essa crate pertence ao workspace da lição 6. Instruções sobre compilação e execução de testes na pagina [anterior](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/lesson_6_enums/lesson_6_1_simple/).

Executar comandos como ```cargo test``` e ```cargo build``` afetará todos as crates na workspace. A não ser que uma crate seja especificada.

---

## Tópicos

[topo](#lição-6---1-uso-de-enums)

 - [O que são enums](#o-que-são-enums)
 - [Instruções match](#instruções-match)
   - [String patterns](#string-patterns)
   - [Match precisa aplicar a todos possiveis patterns](#match-precisa-aplicar-a-todos-possiveis-patterns)
   - [Acessando apenas um valor de um enum](#acessando-apenas-um-valor-de-um-enum)
 - [Enums que "englobam" valores](#enums-que-"englobam"-valores)
 - [Funções devem especificar tipos](#funções-devem-especificar-tipos)
   - [Função is_no_value](#função-is_no_value)
   - [Função get_an_integer](#função-get_an_integer)
   - [Função has_an_odd_number](#função-hasanoddnumber)
 - [Exemplo de uso: Usuário](#exemplo-de-uso-usuário)
   - [Enums limitam as possibilidades](#enums-limitam-as-possibilidades)
   - [Função get_name](#função-get_name)
   - [Função has_permission](#função-has_permission)
   - [Função get_actions](#função-get_actions)
   - [Escolhendo serializador](#escolhendo-serializador)
 - [Proxima seção](#proxima-seção)

---

## O que são enums

[topo](#lição-6---1-uso-de-enums)

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

Para implementar ações de acordo com os valores do enum. Poderiamos usar instruções ```if|else```. Mas existe uma ferramenta muito mais potente para isso. A seguir, discutiremos sobre instruções ```match```.

---

## Instruções match

[topo](#lição-6---1-uso-de-enums)

Instruções match comparam um valor com diversos possiveis valores.

```rust
// /src/model.rs
impl Example0{

    /// Observa o valor de si mesmo e retorna um número entre 1 e 5.
    /// 
    /// Note o &self, significando que a função acessa o valor, mas não altera.
    /// 
    pub fn get_number(&self) -> u32 {
        log("Calling Example0::get_number");

        // Instruções match são semelhantes a uma 
        match self {
            Example0::First => {1},
            Example0::Second => {2},
            Example0::Third => {3},
            Example0::Fourth => {4},
            Example0::Fifth => {5},
        }
    }
```

O exemplo acima simplesmente compara o valor do enum e retorna um inteiro.
 - ```Example0::First``` retorna 1;
 - ```Example0::Second``` retorna 2;
 - ```Example0::Third``` retorna 3;
 - ```Example0::Fourth``` retorna 4;
 - ```Example0::Fifth``` retorna 5;

É semelhante a uma instrução switch em linguagens como c, python, java e javascript. Porém, instruções ```switch``` comparavam ```booleans```, instruções ```match``` de rust comparam patterns.

---

### String patterns

[topo](#lição-6---1-uso-de-enums)

Podemos usar instruções match para ```String``` e ```&str```:

```rust
// /src/lib.rs

impl Contract{
    /// Podemos usar instruções match para Strings e &str.
    /// 
    /// Esta função é um exemplo. 
    /// 
    /// Retorna 1, 2, 3, 4, 5, se o argumento for o número.
    /// 
    /// Causa panic se for outro valor.
    pub fn string_match_example(&self, arg: String) -> u32 {

        // Trata a referencia &String como &str
        return match &arg as &str {
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            value => panic!("Received {}. Must be 1, 2, 3, 4 or 5.", value),
        }
    }
```

No exemplo acima, a instrução match compara os padrões de cima para baixo.
 - &arg é "1"? Não.
 - &arg é "2"? Não.
 - &arg é "3"? Não.
 - &arg é "4"? Não.
 - &arg é "5"? Não.
 - ```value``` é um nome de variável. Este pattern é sempre verdadeiro. Patterns serão explicados em detalhes na próxima lição.

Qualquer String que não seja "1", "2", "3", "4" ou "5", alcançará o ultimo branch. Ou seja, resultará em panic com o valor da String.

---

### match precisa aplicar a todos possiveis patterns

[topo](#lição-6---1-uso-de-enums)

No [primeiro exemplo](#instruções-match), o enum possui 5 possiveis valores. Apague um dos branches e o compilador resultará em erro.

```rust
match self {
    Example0::First => {1},
    Example0::Second => {2},
    Example0::Third => {3},
    Example0::Fourth => {4},
    // Branch apagada, resultará em erro.
    // Instruções match precisam considerar todos possiveis valores.
    // Example0::Fifth => {5},
}
```


O [segundo exemplo](#string-patterns) compara um String. Strings podem possuir infinitos valores. Por isso, a ultima branch é:

```rust
value => panic!("Received {}. Must be 1, 2, 3, 4 or 5.", value),
```

O pattern ```value``` pode ser qualquer nome de variável. Este pattern é o mais simples possivel. Portanto, este aplica para qualquer valor possivel. Muitas vezes este pattern é demonstrado com "underline" "_", iniciar o nome de uma variável com "underline" descreve ao compilador que não temos intenção de utiliza-la.

Um exemplo. Digamos que escrevamos o match de Strings da seguinte forma:

```rust
// /src/model.rs
pub fn string_match_example(&self, arg: String) -> u32 {

    // Mesmo que o exemplo anterior. Mas não utiliza value.
    return match &arg as &str {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        value => {
            // Compilador irá enviar um aviso acima: "Variável não usada"
            panic!("Invalid value. Must be 1, 2, 3, 4 or 5.");
        },
    }
}
```

Para corrigir o aviso em "value". Basta fazer a modificação:

```rust
// /src/model.rs
pub fn string_match_example(&self, arg: String) -> u32 {

    // Mesmo que o exemplo anterior. Mas não utiliza _value.
    return match &arg as &str {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        _value => {
            // Aviso corrigido.
            panic!("Invalid value. Must be 1, 2, 3, 4 or 5.");
        },
    }
}
```

Como convenção, simplesmente usamos "_" como o nome de patterns irrelevantes:

```rust
// /src/model.rs
pub fn string_match_example(&self, arg: String) -> u32 {

    // Mesmo que o exemplo anterior. Mas não utiliza _.
    return match &arg as &str {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        _ => {
            // Aviso corrigido.
            panic!("Invalid value. Must be 1, 2, 3, 4 or 5.");
        },
    }
}
```

---

### Acessando apenas um valor de um enum

[topo](#lição-6---1-uso-de-enums)

Digamos que tenhamos uma função que só precisa aplicar a um enum. Instruções match podem ser usadas também. A função abaixo retorna verdadeiro apenas se o valor do enum for ```Example::Third```:

```rust
// /src/model.rs
/// true se o valor for Exemplo0::THIRD
pub fn is_third(&self) -> bool {
    log("Calling Example0::is_third");

    match self {
        Example0::Third => true,
        _ => false,
    }
}
```

---

## Enums que "englobam" valores

[topo](#lição-6---1-uso-de-enums)

Enums podem armazenar valores:

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

No exemplo acima. A primeira alternativa não tem valor, as 4 seguintes são tuplas com os respectivos tipos. A ultima alternativa é semelhante a um "struct da linguagem C".

---

## Funções devem especificar tipos

[topo](#lição-6---1-uso-de-enums)

Funções para enums podem ser dificeis de implementar, devido a uma regra de linguagens de programação de tipo estático "statically typed":

 - Os tipos de argumentos devem ser especificados.
 - Os tipos de retorno devem ser especificados.

Isso inclui funções genéricas. Que serão explicados adiante. O compilador **deve** saber qual o tipo do argumento que recebe e qual tipo de valor irá retornar. Funções genéricas são apenas uma forma de gerar funções que seguem essas regras.

Ao implementar uma simples função get ou set, um desenvolvedor pode ter dificuldades lidando com o compilador. A segunda alternativa é um inteiro, a terceira é um número real, a quarta é um string...

Porém, é possivel implementar estas funções. A pergunta que precisamos fazer é: Como usar o mesmo tipo para o argumento e retorno?

Explicaremos implementação de genéricos na lição sobre traits. No caso de uso adiante, podemos simplesmente retornar um String:

```rust
// /src/model.rs

// O método a seguir retorna apenas um tipo, isso é aceitável para o compilador.
pub fn get(&self) -> String {
    log("Calling Example1::get");

    match self{
        Example1::NoValue => String::from(""),
        Example1::AnInteger(valor) => format!("{}", valor),
        Example1::AFloat(valor) => format!("{}", valor),
        Example1::AString(valor) => format!("{}", valor),
        Example1::ATuple(valor0, valor1) => format!("({}, {})", valor0, valor1),
        Example1::ACLikeStruct { first, second } => format!("{{\nfirst: {},\nsecond: \"{}\",\n}}\n", first, second),
    }
}
```

```format!``` é um macro que formata uma String. Macros são explicados na lição 5. Como a intenção é simplesmente imprimir o valor na tela. Retornar um String é aceitável, neste caso de uso.

A lição sobre traits explicará como usar as traits borsh e serde. Podemos usar essas ferramentas para converter um struct para uma fatia (slice) de bytes. Podemos também converter uma fatia de bytes ou String para um struct (se compativel).

A lição sobre traits também explicará sobre como usar genéricos. Que é uma forma de implementar a mesma função para diversos tipos simultaneamente.

---

### Função is_no_value

[topo](#lição-6---1-uso-de-enums)

As vezes, precisamos checar apenas um valor. Retorna verdadeiro se o valor interno for ```NoValue```.

```rust
// /src/model.rs
/// true se o enum for Example1::NoValue.
pub fn is_no_value(&self) -> bool{
    log("Calling Example1::is_no_value");

    match self{
        Example1::NoValue => true,
        _ => false,
    }
}
```

---

### Função get_an_integer

[topo](#lição-6---1-uso-de-enums)

Retorna um inteiro, se a alternativa for ```Example1::AnInteger```.

Retorna um ```Option```, será explicado em detalhes na proxima seção. Simplificando, é um enum que representa um valor que pode existir ou não. Se o valor existir, é ```Option::Some(valor)``` ou simplesmente ```Some(valor)```. Se o valor não existir, é então ```Option::None``` ou simplesmente ```None```.

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

Considere a instrução:

```rust
match self{
```

Esta instrução deveria estar coletando ```ownership``` de si mesmo, porque não está fazendo isso?

Isso é devido ao argumento:

```rust
pub fn get_an_integer(&self) -> Option<i32>{
```

Como apenas ```&self``` está sendo usado, o compilador entende que este valor é apenas uma referência.

Finalizando:

```rust
match self{
    Example1::AnInteger(valor) => Some(valor.clone()),
    _ => None
}
```

A função retorna ```Option<i32>```. Portanto, esta instrução ```match``` rust retorna ```Some(valor)``` ou ```None```. Representando se o valor foi encontrado ou não.

---

### Função has_an_odd_number

[topo](#lição-6---1-uso-de-enums)

Retorna verdadeiro se possui um inteiro impar.

O propósito desta função é demonstrar um uso mais detalhado de instruções match.

```rust
// /src/model.rs
/// Retorna true se possui algum numero inteiro impar,
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
            // Não temos interesse no segundo valor que é String
            first%2 == 1
        },
    }
}
```

As únicas alternativas que possuem inteiros são ```Example1::AnInteger```, ```Example1::ATuple``` e ```Example1::ACLikeStruct```. Todas as outras alternativas retornam falso.

Note que variáveis com nome ```_valor``` e ```_``` são valores que não pretendemos usar. A convenção é simplesmente usar underline ```_```. Usando outro nome apenas para demonstrar que todas as variáveis que começam com o caractere underline ```_``` são variáveis que consideramos "irrelevantes". O compilador ignora avisos de "variável não utilizada" nestes casos.

---

## Exemplo de uso: Usuário

[topo](#lição-6---1-uso-de-enums)

O próximo exemplo descreve um exemplo de aplicação para um enum.

```rust
// /src/model.rs
pub enum Example2User{
    Admin{ name: String, id: u32, pass: String, actions: Vec<String> },
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

Neste caso, existem 3 tipos de pessoas que podem acessar o nosso sistema:
 - Funcionários que podem acessar o sistema, podem fazer algumas alterações limitadas, mas não podem alterar regras críticas.
 - Administradores que tem permissões para alterar regras críticas.
 - Clientes que não tem permissões para alterar dados do sistema. Mas podem alterar os próprios dados, além de acessar os serviços do sistema.

Como descrito acima, funcionarios, administradores e clientes são tipos/objetos diferentes. Possuem finalidades e estados diferentes. Mas, no contexto de usuário, são usados da mesma forma.

Traits também são usados para agrupamento. Também devem ser considerados no planejamento de projeto. Aqui está a diferença entre agrupamento com enums e agrupamento com traits:
 - Enums nos permitem agrupar diversos tipos diferentes para uma única funcionalidade.
 - Traits nos permitem agrupar uma funcionalidade para diversos tipos diferentes.

---

### Enums limitam as possibilidades

[topo](#lição-6---1-uso-de-enums)

Isso é bom em alguns casos, ruim em outros. Importante entender essa característica. 

Por exemplo, digamos que tenhamos um jogo de xadrez. Existem diversas peças diferentes no tabuleiro. Eu implementei o meu projeto de xadrez da seguinte forma:

```rust
// https://github.com/On0n0k1/NCD.L1--Chess/blob/main/src/pieces/piece.rs
pub enum Piece{
    BISHOP( Bishop ),
    EMPTY(  Empty  ),
    KING(   King   ),
    KNIGHT( Knight ),
    PAWN(   Pawn   ),
    QUEEN(  Queen  ),
    ROOK(   Rook   ),
}
```

Cada peça tem sua funcionalidade, mas o tabuleiro não tem interesse por isso. O tabuleiro precisa de receber uma lista de possiveis movimentos para calcular cheque-mate. Independente de qual peça esteja naquela posição.

Neste caso, a limitação de enums é util, porque sabemos que existem 6 peças de xadrez diferentes (mais um para espaço vazio), e nunca será necessário expandir este enum.

Enums também são úteis para erros. Consideremos, como exemplo, um app de uma biblioteca. Uma função para coletar informação sobre um livro retorna os seguintes erros:

```rust
pub enum MessageError{
    BookNotFound(String),
    InvalidArg(String),
    MissingArg(String),
    NoPermission,

}
```

Os erros são:
 - ```BookNotFound```: Livro não encontrado. O String é o nome do livro procurado.
 - ```InvalidArg```: Algum dos argumentos recebidos não pode ser reconhecido. O String é o argumento referido.
 - ```MissingArg```: Para o pedido especificado, um argumento necessário estava ausente. String é o argumento.
 - ```NoPermission```: Não possui permissão para acessar o livro. Talvez seja um caso do usuário ser menor de idade, e o livro possuir conteúdo adulto.

Isso é outra vantagem da "limitação" de enums. Um desenvolvedor sabe exatamente todos os erros possíveis que podem ocorrer com esse chamado de função.

---

### Função get_name

[topo](#lição-6---1-uso-de-enums)

A função get_name do ```Example2User``` simplesmente retorna o ```String``` "name" armazenado. As variáveis com nome "_" são variáveis que iremos ignorar.

```rust
// /src/model.rs
/// Retorna nome do usuário.
/// 
/// O bloco que chama o método não precisa de saber o que o usuário é.
pub fn get_name(&self) -> String {
    log("Calling Example2User::get_name");

    match self {
        Example2User::Admin { name, id: _, pass: _, actions: _ } => { name.clone() },
        Example2User::Client { name, id: _, orders: _ } => { name.clone() },
        Example2User::Employee( employee ) => { employee.name.clone() },
    }
}
```

---

### Função has_permission

[topo](#lição-6---1-uso-de-enums)

Esta função retorna se o usuário possui permissão ou não para uma dada ação no sistema.
 - Administradores sempre possuem permissão.
 - Clientes nunca possuem permissão.
 - Funcionários possuem uma lista alterável de permissões.

Usar Strings não é uma boa idéia para permissões. Enums seriam melhores, mas o exemplo já está complexo o suficiente.


```rust
// /src/model.rs
pub fn has_permission(&self, permission: String) -> bool{
    // imprime na tela que a função foi chamada
    log("Calling Example2User::has_permission");

    match self{
        Example2User::Client { name: _, id: _, orders: _ } => { false },
        Example2User::Admin { name: _, id: _, pass: _, actions: _ } => { true },
        Example2User::Employee(employee) => {

            // Vec implementa a trait Iterator.
            // Isso disponibiliza o método .iter ao vetor.
            // Este método nos permite iterar referencias de String.
            // Nenhuma cópia de String é feita.
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

Funcionário possui um ```Vec``` de Strings representando suas permissões. ```Vec``` será explicado em detalhes na lição sobre coleções. É uma lista alterável de valores.

Já explicamos sobre as traits ```Clone``` e ```Copy```. Agora explicaremos sobre a trait ```Iterator``` ([detalhes oficiais](https://doc.rust-lang.org/std/iter/trait.Iterator.html)). Esta trait nos permite usar o vetor em uma instrução ```for```.

O método ```iter()```, disponilizado pela trait ```Iterator```, gera um iterador de referências. Ou seja, cada ```employee_permission``` é uma referência a um elemento pertencente ao ```Vec```. Não podemos alterar os valores, mas não gastamos computação gerando cópias.

A instrução:

```rust
if permission == *employee_permission {
    return true;
}
```

Note o operador "*". ```employee_permission``` é do tipo ```&String```. Precisamos acessar o valor do String, não da referência. Se fosse ```&&String```, acessariamos o valor com ```**employee_permission```.

Ou seja, se o argumento da função ```permission``` for igual à String acessada, retorna verdadeiro. Senão, continua iterando.

---

### Função get_actions

[topo](#lição-6---1-uso-de-enums)

A função ```get_actions``` retorna a lista de ações mais recentes realizadas pelo usuário.

Essa função da uma breve introdução ao enum ```Result``` que será explicado em detalhes na proxima sub-lição. Simplificando, é um enum que representa uma ação que pode causar um erro.

Neste exemplo, decidimos que a função deve retornar um erro se o usuário for cliente.

```rust
// /src/model.rs
pub fn get_actions(&self) -> Result<Vec<String>, String> {
    log("Calling Example2User::get_actions");
    
    // Se for client, retorna um erro (Como exemplo).
    // Se for admin ou employee, retorna referencia para o Vec.
    let actions = match self{
        Example2User::Client { name: _, id: _, orders: _ } => { return Err(format!("User is Client")); },
        Example2User::Admin { name: _, id: _, pass: _, actions, } => { actions },
        Example2User::Employee( employee ) => { &employee.actions },
    };

    let mut result: Vec<String> = Vec::new();
    // Usa a referência para criar uma cópia do Vec.
    for action in actions{
        result.push(action.clone());
    }

    Ok(result)
}
```

Note que a função retorna ```Result<Vec<String>, String>```. O primeiro dos tipos ```Vec<String>``` é o que consideramos uma operação de sucesso. O segundo dos tipos ```String``` é o tipo que será retornado se a operação resultar em erro.

Note o bloco em que este erro é chamado:

```rust
return Err(format!("User is Client"));
```

E o bloco em que a operação é um sucesso:

```rust
Ok(result)
```

```Result::Ok(value)``` ou simplesmente ```Ok(value)``` é uma operação de sucesso. ```Result::Err(err)``` ou simplesmente ```Err(err)``` é uma operação de falha. 

---

**Detalhe**: o valor entre parènteses não precisa de ser ```value``` ou ```err```. 

```err``` é apenas convenção para erros em ```Result```.

---

**Outro detalhe**:

```rust
let mut result: Vec<String> = Vec::new();
// Usa a referência para criar uma cópia do Vec.
for action in actions{
    result.push(action.clone());
}
```

Não podemos simplesmente retornar o vetor encontrado. Este pertence ao enum. Neste caso, precisamos criar uma cópia. 
 - Criamos um ```Vec<String>``` vazio. 
 - Iteramos cada um dos valores.
 - Em cada iteração, cria uma cópia ```action.clone()``` de um elemento, e inclui este valor ao fim do vetor ```result.push(action.clone());```

---

### Escolhendo serializador

[topo](#lição-6---1-uso-de-enums)

Uma instrução diferente pode ser notada na implementação de ```example_2_get_actions``` do contrato:

```rust
// /src/lib.rs
#[result_serializer(borsh)]
pub fn example_2_get_actions(&self) -> Vec<String>{
```

Isso é para evitar um erro que irá confundir muitos desenvolvedores começando na plataforma. A instrução ```#[result_serializer(borsh)]``` diz ao near_sdk para usar o serializador borsh na saida dessa função.

Existem duas ferramentas de serialização/deserialização disponiveis para ```near_sdk```: serde e borsh. Como padrão, ```near_sdk``` usa serde para deserializar o coleções como Vec. Mas o problema é que, para isso não causar erros, precisamos implementar as traits ```Serialize``` e ```Deserialize``` ao nosso contrato.

Simplesmente selecionando borsh como serializador de resultado da função, evitamos diversos erros. ```borsh``` é mais rápido que ```serde```. Tente evitar usar ```serde``` o máximo possivel.

---

## Proxima Seção

[topo](#lição-6---1-uso-de-enums)

A próxima seção será sobre Enums da biblioteca Standard. Option e Result.

A próxima lição será sobre Traits.
