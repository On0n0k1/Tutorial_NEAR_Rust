# Lição 6 - Enums

[voltar](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/PT-BR/)

Esta lição é dividida em 4 partes relacionadas a enums. Cada tópico é uma crate. Aproveitaremos este momento para explicar sobre workspaces e como usá-los.

As sessões são as seguintes:
 - [Parte 1 - Declarando e usando enums](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/PT-BR/lesson_6_enums/lesson_6_1_simple/).
 - [Parte 2 - Termometro implementado com enums](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/PT-BR/lesson_6_enums/lesson_6_2_thermometer/).
 - Parte 3 - Como implementar erros de forma prática e eficiente.

---

## Workspaces

[topo](#lição-6---enums)

Simplesmente, workspace é um diretório com várias crates. Quando compilados, todas as crates compartilham o mesmo diretório target. Mas cada crate resulta em um arquivo compilado.

Algumas utilidades para uso de workspaces são descritos a seguir:
 - Dependências locais. Criamos uma crate customizada para o nosso projeto, e outra crate depende dela.
 - Organização de projeto. Queremos executar testes e compilar todos os projetos simultaneamente.
 - Coordenação de contratos. Um projeto que consiste em vários contratos responsáveis por diferentes funções. Um workspace pode incluir os contratos e uma crate de testes cross-contract (exemplo: [workspaces-rs](https://github.com/near/workspaces-rs)).

---

## Cargo.toml

[topo](#lição-6---enums)

O manifest possui a seção ```[workspace]```.

```toml
[workspace]
members=["lesson_6_1_simple"]
```

```members``` descreve cada crate do projeto.

**Detalhe**: Se uma crate existir neste diretório, esta ainda será considerada. Para não incluir a crate, devemos incluir o atributo ```exclude```. Como no exemplo a seguir (fonte: https://doc.rust-lang.org/cargo/reference/workspaces.html, acesso em 17-mar-2022):


```toml
[workspace]
members = ["member1", "path/to/member2", "crates/*"]
exclude = ["crates/foo", "path/to/other"]
```

No exemplo acima, foram incluidos 3 caminhos, e excluidos 2.

---

## CLI em workspaces

[topo](#lição-6---enums)

Se executarmos comandos como ```cargo build``` ou ```cargo test``` em uma crate que pertence a uma workspace, todos as crates serão afetadas também.

Para especificar o comando para apenas uma crate, adicionamos a "flag" ``` -p ```, ```--package``` ou ```--workspace``` ao comando.

Para testar ```lesson_6_1_simple``` execute:

```bash
cargo test -p lesson_6_1_simple -- --nocapture --test-threads=1
```

```--nocapture``` faz imprimir o output de todos os testes.

```--test-threads=1``` faz todos os testes serem executados em um thread. Tornando o output legivel.

```bash
cargo build -p lesson_6_1_simple --target wasm32-unknown-unknown --release
```

Os arquivos '.wasm' estarão em './lesson_6_enums/target/wasm32-unknown-unknown/release/'.

```bash
cargo doc --lib --document-private-items -p lesson_6_1_simple --open
```

Gera documentação da sub-lição 6-1 e abre no browser padrão.

 - ```--lib``` especifica que a crate é um library.
 - ```--document-private-items``` pede para gerar documentação de todos os items.
 - ```--open``` abre o website no navegador padrão.

Documentação estará em './target/doc/lesson_6_1_simple/index.html'.


---

 - A proxima seção será sobre [declaração e uso de enums](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/PT-BR/lesson_6_enums/lesson_6_1_simple/).
 - A proxima lição será sobre traits.

