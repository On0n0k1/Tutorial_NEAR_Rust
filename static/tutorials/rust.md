# Instalação Rust

Este tutorial descreve a instalação das ferramentas recomendadas para criação de projetos NEAR em Rust.

 - [Scripts de Instalação](#scripts-de-instala%C3%A7%C3%A3o)
 - [Rustlings](#rustlings)
 - [Rust](#rust)

Instalaremos várias ferramentas diferentes com uma única linha de comando. Às que destacaremos nesse tutorial são: **rustup**, **cargo** e **rustc**.

 - **rustc**: compila o código rust para linguagem de máquina;
 - **cargo**: permite realizar comandos especificos para o projeto como compilar, criar documentação, realizar testes de unidade, executar projeto como binário.
 - **rustup**: existem varias diferentes formas de se compilar um projeto. Diversas arquiteturas diferentes. Diversas versões. rustup controla estas versões.

Uma analogia para desenvolvedores javascript:
 - rustup age de forma semelhante a nvm.
 - cargo age de forma semelhante a npm.
 - rustc age de forma semelhante a node.

É apenas uma analogia, existem algumas diferenças que serão encontradas em pratica.

## Scripts de instalação
[topo](#instala%C3%A7%C3%A3o-rust)

Recomendaremos duas alternativas para instalação de Rust. A primeira é recomendada para estudo, a segunda é recomendada para quem simplesmente quer instalar rustup.

 - rustlings: Instala rust e instala a ferramenta rustlings, que possui exercicios para desenvolvedores praticarem seu entendimento.
 - rust: Instala rust e todas as ferramentas necessárias para desenvolvimento.

## Rustlings
[topo](#instala%C3%A7%C3%A3o-rust)

Acesse o [repositório](https://github.com/rust-lang/rustlings) e siga as instruções de instalação.

![Pagina de web rust](/static/images/rustlings.png)

#### Rustlings para MacOS/Linux
[topo](#instala%C3%A7%C3%A3o-rust)

```bash
curl -L https://git.io/install-rustlings | bash
# Ou se quiser escolher o diretorio de instalação
curl -L https://git.io/install-rustlings | bash -s mypath/
```

### Rustlings para Windows
[topo](#instala%C3%A7%C3%A3o-rust)

Executar em um powershell com permissões de administrador.

```bash
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

E depois executar:

```bash
Start-BitsTransfer -Source https://git.io/JTL5v -Destination $env:TMP/install_rustlings.ps1; Unblock-File $env:TMP/install_rustlings.ps1; Invoke-Expression $env:TMP/install_rustlings.ps1
```

## Rust
[topo](#instala%C3%A7%C3%A3o-rust)

Acesse a [pagina oficial](https://www.rust-lang.org/tools/install) e execute o script. 

![Pagina de web rust](/static/images/rust.png)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Este comando detecta o sistema e instala todas as ferramentas necessárias.
