# Lição 4 - Módulos

Esta lição discute sobre como módulos são importados. 

Podemos ter todo nosso código implementado no arquivo ```lib.rs```. Mas percebe-se como seria dificil de organizar um projeto grande dessa forma. 

 - Podemos declarar módulos externos; 
 - Declarar diretórios externos como módulos;
 - Controlar quais módulos são públicos;
 - Controlar o caminho para cada módulo público;
 - Também podemos organizar nossos testes no diretório ```./tests/```.

## Tópicos
 - 

## Como declarar um módulo externo

Um módulo externo deve ser declarado antes de usado/importado.

```rust
mod yet_another_module;
```

Essa linha diz ao compilador que existe um módulo com este nome no mesmo diretório. Existem três formas de se declarar um módulo. Se dois módulos ou mais com mesmo nome forem encontrados, um erro de ambiguidade será gerado.

O módulo acima é privado. Só pode ser usado onde foi declarado. O módulo abaixo é público:

```rust
pub mod a_module;
```

```a_module``` é público aqui. Ou seja, pode ser importado por outros. Isso inclui crates externas. O exemplo abaixo restringe isso.

```rust
pub(crate) fn hello() -> String{
    String::from("Hello from crate::a_module::specific_module")
}
```

```pub(crate)``` significa que esta função é pública apenas nessa crate. Ou seja, se ```lesson_4_modules``` for dependência de um outro projeto rust, o crate externo não terá acesso a essa função.

## Como declarar e usar diretórios

Podemos declarar diretórios como módulos também. Existem duas formas para isso. A primeira é:
 - Criar um diretório com o nome do seu módulo.
 - Criar um arquivo com nome ```mod.rs``` dentro desse diretório. Este arquivo possui a implementação do módulo.

![](../static/images/mod-diretorios.png)

```a_module``` é uma implementação de módulo. 

A segunda forma é:

 - Criar um diretório com o nome do seu módulo.
 - Criar um arquivo rust com o mesmo nome do seu módulo junto com o diretório. Este arquivo possui a implementação do módulo.

![](../static/images/mod-diretorios2.png)

```internal_module``` é outra implementação de módulo.

O arquivo rust fica dentro ou fora do diretório? Essa é a questão.

## Usando/Importando módulos

Qualquer item (módulos, funções, structs, traits, enums, et cetera) pode ser importado com a instrução use.

No exemplo abaixo, acessamos o caminho ```near_sdk``` (uma crate neste caso), e incluimos ```env``` e ```near_bindgen``` no nosso namespace.

```rust
use near_sdk::{env, near_bindgen};
```

Não é necessário usar a instrução use. Porém, se quiséssemos acessar o módulo ```env```, teremos que escrever ```near_sdk::env``` todas as vezes.

## Módulos para testes

## Conceitos Extras

Vários pequenos detalhes foram incluidos nesta lição. 