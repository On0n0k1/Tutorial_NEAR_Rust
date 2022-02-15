# Tutorial_NEAR_Rust

Tutorial em etapas para desenvolvimento de contratos inteligentes em rust. Neste conjunto de tutoriais serão discutidos todas as principais características da linguagem, assim como seu uso na plataforma NEAR.

## Contato

Para dúvidas, reclamações ou sugestões, por favor me adicione no discord On0n0k1#3800. Se este tutorial facilitar a sua vida, considere comprar um café para mim enviando uma fração de NEAR para stiltztinkerstein.near .

## O que é a linguagem Rust

De forma bem resumida, é uma linguagem de programação de baixo nível com as seguintes características:

 - Execução aproximadamente tão rápida quanto linguagem c ou c++.
 - Não tem os riscos de vazamento de memória que outras linguagens de baixo nível possuem.
 - É dificil de começar a aprender.
 - Não **usa** e nem **precisa** de coleta de lixo de memória. Pois no periodo de compilação, o compilador sabe exatamente quando variáveis são criadas e liberadas.
 - Processamento em paralelo é fácil.
 - Processamento assíncrono é de dificuldade semelhante a outras linguagens populares.
 - Muito mais simples organização de projeto e dependências do que python e javascript.
 - Ganhou repetidos anos consecutivos como a linguagem mais popular do stackoverflow.

## Usos da linguagem Rust

Um desenvolvedor Rust pode:

 - Criar apps decentralizados em plataformas web3 como NEAR.
 - Pode criar aplicativos que não precisam de uma máquina virtual para serem executados. Precisa do compilador Rust para compilar, mas não precisa para executar.
 - Criar servidores compactos e rápidos em conteineres docker.
 - Criar aplicações potentes como funções lambda para serem implantados em servidores aws (web3 é melhor porém).
 - Usar o linker para criar bibliotecas que podem ser usadas por um compilador como c.
 - Compilar bibliotecas que podem ser importadas em um browser javascript ou em um runtime nodejs com o formato WebAssembly.
 - Compilar bibliotecas potentes e eficientes para Python usando a crate PyO3.
 - Compilar código para dispositivos embarcados (embedded).
 - Competir em um mercado de trabalho que possui 1 ou 2 inscritos por vaga (incluindo internacional).

## Aprendendo a linguagem Rust

No meu ponto de vista, aprender a linguagem rust é semelhante a idéia de domar um dragão em um mundo de fantasia. É demorado, é dificil, existem muitas alternativas diferentes e mais simples do que essa. Mas, se conseguir, você vai ter um terrível dragão ao seu lado.

Existem estudos que destacaram que o tempo para escrever uma certa quantidade de linhas de código em linguagens de baixo nível (como c) é até 30 vezes mais devagar do que as de alto nível (como python e javascript). Pela minha prática, é mais demorado ainda para uma pessoa aprendendo Rust escrever código do que c. 

Mas, com prática, ficamos mais ágeis em tudo. Com o tempo acostumamos com o que o compilador precisa e espera de nós. Podemos também configurar snippets para gerar códigos de "boilerplate" (forma) automaticamente. Então, é apenas uma questão de entendimento, memorização e paciência para o desenvolvedor. Houveram vezes em que eu escrevi 800 linhas de código Rust em 2 dias.

Quase sempre teremos que dar pausas para estudar o nosso método e garantir que estamos fazendo as decisões corretas. Porém, cada tentativa seguinte será mais fácil que a anterior.

## Comparações com javascript e Python

Porém uma pessoa astuta perguntaria "Porque eu iria aprender uma linguagem dessas se eu ja posso resolver os mesmos problemas com as linguagens que sei?" . É uma ótima pergunta, se eu ja posso conseguir o resultado escrevendo algumas linhas de código em python no terminal, porque eu iria querer aprender Rust?

Facilidade de uso e resolução de problemas. Este é o foco principal dessas linguagens. Como conseguir a solução para o nosso problema da forma mais simples possivel. Os processadores ficavam cada vez mais rápido cada geração, então bastava comprar as gerações de hardware mais recentes.

Porém a lei de Moore não se aplica mais. Os desenvolvedores estão precisando de algoritmos mais eficientes. Esta necessidade nos faz olhar para nossas linhas de código e perguntar "O que está instrução está fazendo exatamente?"

Quando escrevemos uma instrução em python "a = 3". A máquina virtual python está criando um objeto numero, criando um ponteiro mutex que aponta para o numero, e associando "a" a este ponteiro. Por isso que python normalmente é limitado a um core do processador. Quando tentamos aproveitar mais a capacidade de processamento de nossa máquina, a complexidade de código em javascript e python cresce exponencialmente.

O foco da linguagem Rust não é o resultado final dessa execução. E sim, o caminho que o processador e memória levam até alcançar este resultado. Um(a) desenvolvedor(a) rust experiente sabe olhar um bloco de código e dizer:
 - "Essa memória vai ser liberada nessa linha de código";
 - "O processador irá pedir pra liberar um espaço de memória nessa linha e criar uma cópia dessa variável aqui.";
 - "Essa função irá pegar esse endereço emprestado, usar este valor nessa parte, e retornar o endereço para o dono ao fim.";

## Instalação

Antes de começarmos, devemos realizar os seguintes passos:

 - Instalar [near-cli](https://github.com/On0n0k1/Tutorial_NEAR_Rust/blob/main/static/tutorials/nearcli.md) para interagir com a plataforma NEAR.
 - Instalar [rust](https://github.com/On0n0k1/Tutorial_NEAR_Rust/blob/main/static/tutorials/rust.md) para compilar e testar os projetos.
