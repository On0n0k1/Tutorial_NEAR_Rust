# Lição 6 - 2 Termômetro

[voltar](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/lesson_6_enums/)

Neste exemplo, será visto uma aplicação prática para os conceitos descritos nas lições anteriores. Além dos conceitos anteriores, serão demonstrados tópicos como:

 - Documentação de projetos;
 - Controle de acesso de usuários;
 - Acesso cross-contract;
 - Controle de input;
 - Controle de output;
 - Traits para conversão de tipos;

---

## Indice

[topo](#lição-6---2-termômetro)

 - [Idéia](#idéia)
 - [Instalação](#instalação)
   - [Visual Studio Code](#visual-studio-code)
   - [Rust](#rust)
   - [Near-CLI](#near-cli)
 - [Comandos Bash: Compilação](#comandos-bash-compilação)
   - [Documentação](#documentação)
   - [Testes](#testes)
   - [Criação de Sub-contas para Smart Contracts](#criação-de-sub-contas-para-smart-contracts)
   - [Criação de Sub-contas para Sensores](#criação-de-sub-conta-para-sensores)
   - [Deletar Sub-conta](#deletar-sub-conta)
 - [Contrato](#contrato)
   - [Inicialização](#inicialização)
   - [allow_user](#allow_user)
   - [remove_user](#remove_user)
   - [set_format](#set_format)
   - [new_entry](#new_entry)
     - [Exemplos](#exemplos-new_entry)
     - [Pânico](#pânico-new_entry)
   - [list_update_entries](#list_update_entries)
     - [Exemplos](#exemplos-list_update_entries)
     - [Pânico](#pânico-list_update_entries)
   - [clear_entries](#clear_entries)
     - [Argumentos](#argumentos-clear_entries)
     - [Exemplo](#exemplo-clear_entries)
     - [Pânico](#pânico-clear_entries)
   - [view_get_format](#view_get_format)
   - [view_get](#view_get)
     - [Argumentos](#argumentos-view_get)
     - [Exemplo](#exemplo-view_get)
 - [Implementação](#implementação)
   - [Documentação de Projetos](#documentação-de-projetos)
     - [Comentários sobre Arquivo](#comentários-sobre-arquivo)
     - [Comentários e Documentação](#comentários-e-documentação)
     - [Exemplos/Testes em Documentação](#exemplostestes-em-documentação)
   - [Organização de Módulos](#organização-de-módulos)
   - [Controle de Acesso de Usuários](#controle-de-acesso-de-usuários)
   - [Acesso Cross-Contract](#acesso-cross-contract)
   - [Controle de Output](#controle-de-output)
   - [Controle de Input](#controle-de-input)
   - [Implementação de Traits](#implementação-de-traits)
 - [Fim](#fim)

---

## Idéia

[topo](#lição-6---2-termômetro)

Na seção anterior, foram descritos enums e como utilizar instruções match. Além das funcionalidades descritas, também podemos utilizar enums para controlar a entrada/saida de dados. 

Digamos que um desenvolvedor de aplicativos embarcados (embedded) quer monitorar os dados de diversos termômetros simultaneamente. Logo de início, foram notadas as seguintes restrições:

 - **Conectar** todos os dispositivos a um computador é inviável.
 - **Manter** a máquina como servidor 24 horas recebendo input dos sensores também é inviável.
 - **Criar** um servidor nuvem como aws também é inviável porque o desenvolvedor não quer patrocinar o foguete de algum bilhonário.

Portanto, o desenvolvedor decide criar um smart contract para armazenar os dados. A vantagem de uma alternativa dessas são:

 - **Fácil de implementar**. O usuário só precisa de uma conta para armazenar o smart contract. Uma conta para cada sensor utilizado.
 - **Fácil de automatizar**. A única coisa que muda entre cada aplicação é o nome dos contratos. Com bons scripts, desenvolvedores podem implementar o sistema em minutos.
 - **Fácil de expandir**. Este exemplo utiliza valores de temperatura. Mas o contrato pode ser facilmente alterado para receber qualquer tipo de dados.

Mas o contrato apenas coleta dados? Não executa nenhuma computação sobre os dados? A computação de dados pode ser executada localmente pelo desenvolvedor. Não há necessidade de desperdiçar gás em operações que podem ser facilmente executadas localmente.

O contrato resolve o problema de implantação e comunicação entre os dispositivos. Já existem bibliotecas eficientes para ciência de dados. Não há necessidade de reinventar a roda. 

Resumindo. Sensores enviam dados para o smart contract. O Smart Contract armazena os dados de acordo com o nome do sensor, formato de temperatura, data e tempo de recebimento. A máquina do usuário acessa o contrato e coleta os dados armazenados para processamento.

---

## Instalação

[topo](#lição-6---2-termômetro)

 - Visual Studio Code;
 - Rust;
 - Near-cli;

### Visual Studio Code

 - Link para instalação: https://code.visualstudio.com/ ;
 - Instale a extensão para rust: https://marketplace.visualstudio.com/items?itemName=Zerotaskx.rust-extension-pack ;


**Opcional**: O pacote de extensão acima instala uma extensão para a linguagem rust chamada ```Rust```. Esta extensão pode causar bugs de linting, principalmente quando analizando bibliotecas webassembly. Eu, pessoalmente, costumo desativar está extensão e ativar outra chamada ```rust-analyzer```.

**Extra**: Não tenha ```rust``` e ```rust-analyzer``` ativos simultaneamente. ```crates``` e ```Better Toml``` podem ser mantidos com ```rust-analyzer```, porém.

---

### Rust

Um script para instalação existe em: https://www.rust-lang.org/tools/install.

Após executar o script em um terminal, execute a seguinte instrução para permitir compilação para webassembly:

```bash
rustup target add wasm32-unknown-unknown
```

Para desinstalar rust e todas as ferramentas associadas:

```bash
rustup self uninstall
```

---

### Near-cli

[topo](#lição-6---2-termômetro)

[(Mais detalhes)](https://docs.near.org/docs/tools/near-cli)

É uma ferramenta npm. **Instale npm e node**. É recomendado instalar através da ferramenta **npx**, para manter controle sobre diversas versões no mesmo sistema. Existem diversos tutoriais para windows e linux. Não explicarei em mais detalhes.

Com **npm** e **node** instalado, instale **near-cli** globalmente com o seguinte comando:

```bash
npm install -g near-cli
```

---

## Comandos Bash: Compilação

[topo](#lição-6---2-termômetro)

Compile o projeto com o comando:

```bash
cargo build --target wasm32-unknown-unknown --release -p lesson_6_2_thermometer
```
 - ```--target wasm32-unknown-unknown```: Compila para webassembly;
 - ```--release```: Otimizado para produção;
 - ```-p```: Esta crate pertence ao workspace lesson_6_enums. Este comando especifica apenas lesson_6_2_thermometer para ser compilado;

---

### Documentação

[topo](#lição-6---2-termômetro)

Documentação sobre contrato implementada. Gere um website com todos os módulos executando o comando:

```bash
cargo doc --open -p lesson_6_2_thermometer
```

 - p: Como o projeto está incluido como componente de workspace da lição "lesson_6_enums". é necessário especificar a crate.

---

### Testes

[topo](#lição-6---2-termômetro)

Execute testes de unidade com a instrução:

```bash
cargo test -p lesson_6_2_thermometer
```

Isso irá testar os exemplos na documentação também.

---

### Criação de Sub-contas para Smart Contracts

[topo](#lição-6---2-termômetro)

Onde o contrato será implantado. Substitua ```your-main-account.testnet``` pelo nome de sua conta NEAR. Substitua ```your-account-name``` pelo nome da conta que quiser para armazenamento do seu contrato.

```bash
near create-account your-account-name.your-main-account.testnet --masterAccount your-main-account.testnet --initialBalance 90
```

 - ```--masterAccount```: Conta "mestre", que tem permissões administrativas sobre a sub-conta.
 - ```--initialBalance```: Quantidade de NEAR transferido pela conta "mestre" na criação desta sub-conta.

---

### Criação de sub-conta para Sensores

[topo](#lição-6---2-termômetro)

Não haverão contratos nessas sub-contas. Mas serão usadas pelos dispositivos para comunicar com a conta mestre. Substitua:

 - ```your-sub-account```: nome da conta do sensor;
 - ```your-account-name```: nome da sub-conta que possuirá o contrato;
 - ```your-main-account```: nome da sua conta mestre;

```bash
near create-account your-sub-account.your-account-name.your-main-account.testnet --masterAccount your-account-name.your-main-account.testnet --initialBalance 10
```

 - ```--masterAccount```: Conta "mestre", que tem permissões administrativas sobre a sub-conta.
 - ```--initialBalance```: Quantidade de NEAR transferido pela conta "mestre" na criação desta sub-conta.

**Recomendação**: Como exercício de prática, aprimore este contrato fazendo com que o contrato crie/delete as sub-contas automaticamente ao executar as funções ```allow_user```/```remove_user```. Não existe documentação sobre isso na versão "3.1.0". Terá que clonar manualmente o repositório, gerar documentação com o comando ```cargo doc --open``` e encontrar a seção do módulo ```env``` com detalhes sobre a instrução de criação de conta. Boa sorte!

---

### Deletar sub-conta

[topo](#lição-6---2-termômetro)

É recomendado deletar as sub-contas dos sensores antes do smart contract. O primeiro argumento é a conta a deletar, o segundo argumento é a conta que irá receber todo o NEAR armazenado.

```bash
near delete sub-conta-a-deletar.testnet conta-a-receber.testnet
```

 - ```sub-conta-a-deletar.testnet```: nome da sub-conta que pretende deletar;
 - ```conta-a-receber.testnet```: nome da sub-conta que irá receber os fundos. Se o nome for inválido, todos os fundos armazenados serão perdidos permanentemente;

---

## Contrato

[topo](#lição-6---2-termômetro)

Antes de observar o funcionamento das funções. Vale saber que após implantação, o contrato apenas permite execução de chamadas "call" para o dono (owner). Portanto, as chamadas call administrativas devem ser assinadas com o mesmo nome de conta do Smart Contract.

Outros usuários (os sensores) podem ser incluídos na lista de usuários permitidos. Cada usuário possuirá a própria lista de armazenamento de dados.

---

### Inicialização

[topo](#lição-6---2-termômetro)

O contrato inicializa automaticamente com o formato de temperatura Kelvin. O único usuário incluido na lista de permissões é o dono (owner).

---

### allow_user:

[topo](#lição-6---2-termômetro)

```bash
near call my-contract allow_user '{"account_id": "sensor-account-id.testnet"}' --accountId my-contract
```


Função call. Apenas owner tem permissão de executar esta função. Não pode ser cross-contract.

Inclui o id de conta informado na lista de usuários permitidos.

Argumentos:

 - account_id: String. Nome de usuário para incluir na lista de permissões.

Pânico:

 - Se for uma chamada cross-contract;
 - Se não for owner;
 - Se id de conta for um id inválido.
 - Se usuário ja estiver incluido.

---

### remove_user:

[topo](#lição-6---2-termômetro)

```bash
near call my-contract remove_user '{"account_id": "sensor-account-id.testnet"}' --accountId my-contract
```

Função call. Apenas owner tem permissão de executar esta função. Não pode ser cross-contract.

Exclui o id de conta informado da lista de usuários permitidos. Todos os dados armazenados relacionados a este usuário são perdidos.

Owner não pode ser removido.

Argumentos:

 - account_id: String. Nome de usuário para excluir da lista de permissões.

Pânico:
 
 - Se for uma chamada cross-contract;
 - Se o usuário chamando a função não for owner;
 - Se o nome de usuário for inválido;
 - Se o usuário informado não existir na lista de permissões;
 - Se o nome de usuário informado for o owner;

---

### set_format

[topo](#lição-6---2-termômetro)

```bash
near call my-contract set_format '{"temp_format": "Fahrenheit}' --accountId my-contract
```

```bash
near call my-contract set_format '{"temp_format": "Kelvin}' --accountId my-contract
```

```bash
near call my-contract set_format '{"temp_format": "Celsius}' --accountId my-contract
```

Função call. Apenas owner pode executar esta função. Não pode ser cross-contract.

Altera o formato de temperatura do sistema para o formado. 

Todo input de temperatura é convertido para o formato do sistema. Isso permite que diversos sensores diferentes podem ser usados simultaneamente.

Alterar o formato de temperatura não irá alterar os valores armazenados anteriormente.

 - A função call ```list_update_entries``` converte os valores armazenados antes de retornar.
 - A função de coleta view ```view_get``` retorna os valores armazenados sem conversão de valores.

Pânico:
 - Se o nome de usuário for inválido;
 - Se o usuário informado não existir na lista de permissões;
 - Se o nome de usuário informado for o owner;

---

### new_entry

[topo](#lição-6---2-termômetro)

Função call. Todos usuários permitidos podem executar esta função. Pode ser cross-contract. Adiciona um input de temperatura associado ao usuário que executou a função.

Argumentos:
 - **time**: Opcional. Tupla com estrutura ```(u8, u8, f32)``` com os valores para hora minuto e segundo, respectivamente. Se omitido, o contrato utilizará o valor do momento em que a função foi executada (UTC).
 - **date**: Opcional. Tupla com estrutura ```(i32, String, u8)``` com os valores para ano, mês e dia, respectivamente. Se omitido, o contrato utilizará o dia em que a função foi executada.
 - **temp_value**: f32, o valor de temperatura coletado. Não pode ser menor do que zero absoluto.
 - **temp_format**: Opcional, String. Se omitido, o contrato utilizará o formato de temperatura do sistema. Se o formato for diferente do formato do sistema, realizará conversão da temperatura antes de armazenar.

#### Exemplos new_entry

[topo](#lição-6---2-termômetro)

O comando abaixo armazena uma temperatura de 100.0 utilizando o formato de temperatura do sistema, no dia do sistema, no horário do sistema.

```bash
near call my-contract new_entry '{"temp_value": 100 }' --accountID my-sensor-id
```

O comando abaixo armazena uma temperatura de 100 Celsius. Utiliza o dia do recebimento da mensagem. Utiliza o horário do recebimento da mensagem.

```bash
near call my-contract new_entry '{"temp_value": 100, "temp_format": "Celsius"}' --accountID my-sensor-id
```

O comando abaixo armazena uma temperatura de 50.5 Fahrenheit. Dia 11 de fevereiro, 2022. Horário do recebimento da mensagem. **Não causará panic se a data for diferente da atual**.

```bash
near call my-contract new_entry '{"temp_value": 50.5, "temp_format": "Fahrenheit", "date: [2022, "feb", 11]"}' --accountID my-sensor-id
```

O comando abaixo armazena uma temperatura de 11.5 Fahrenheit. Data 27 de março, 2018. Horário 10:50:9.3453.

```bash
near call my-contract new_entry '{"temp_value": 11.5, "temp_format": "f", "date": [2018, "mar", 27], "time": [10, 50, 9.3453]}' --accountID my-sensor-id
```

O comando abaixo armazena uma temperatura de -45.4 Celsius. Horário 23:41:4.443. Data do recebimento da mensagem.

```bash
near call my-contract new_entry '{"temp_value": -45.4, "temp_format": "c", "time": [23, 41, 4.443]}' --accountID my-sensor-id
```

O comando abaixo armazena uma temperatura de 44.13 Kelvin. Horário do recebimento da mensagem. Data atual do recebimento da mensagem.

```bash
near call my-contract new_entry '{"temp_value": 44.13, "temp_format": "kelvin"}' --accountID my-sensor-id
```

#### Pânico new_entry

[topo](#lição-6---2-termômetro)

 - Se o **usuário** não tem permissão de acesso;
 - Se **hora** (time) não for um valor negativo ou maior do que 23;
 - Se **minuto** (time) não for um valor negativo ou maior do que 59;
 - Se **segundo** (time) for um valor negativo ou maior do que 59.9999...;
 - Se **dia** (date) for um valor inválido para o mês e ano;
 - Se **mês** (date) for um String inválido para mês;
 - Se **temp_format** for um String inválido;

---

### list_update_entries

[topo](#lição-6---2-termômetro)

Função call. Pode ser cross-contract. Retorna todas as entries associadas a um id de conta.


Todos os usuários permitidos podem acessar os próprios dados. Mas apenas owner tem permissão de acessar dados de outros usuários. Essa restrição existe para manter controle sobre consumo de gás no contrato. Outros usuários ainda podem coletar os dados utilizando a função view.

Argumentos:
 - account_id: Opcional. String. ID de usuário a ser coletado. Se omitido, retornará os dados do usuário que executou a função.

**Retorna**: Vec com todas as entries associadas ao id de conta.

#### Exemplos list_update_entries

[topo](#lição-6---2-termômetro)

O exemplo abaixo retorna todas as entries associadas ao usuário "my-sensor-id".

```bash
near call my-contract list_update_entries '{}' --accountID my-sensor-id
```

O exemplo abaixo retorna todas as entries associadas a outro usuário. Apenas owner tem permissão para isso.

```bash
near call my-contract list_update_entries '{"account_id": "my-sensor-id.testnet"}' --accountID my-contract
```

#### Pânico list_update_entries

[topo](#lição-6---2-termômetro)

 - Se usuário não tiver permissão de acesso;
 - Se usuário não for owner e estiver tentando atualizar as entries de outro usuário.
 - Se usuário não for encontrado;


---

### clear_entries

[topo](#lição-6---2-termômetro)

Função call. Apenas owner pode chamar esta função. Pode ser cross-contract. Apaga todas as entries associadas a um usuário.

O motivo da função permitir cross-contract é para facilitar automação de contrato. Contratos externos não podem incluir ou remover usuários permitidos. Mas podem adicionar entries, podem coletar dados e remover dados.

Usuários não tem permissão de utilizar essa função para evitar ações suspeitas. Caso um dos sensores for acessado por um terceiro, este terá o acesso mais limitado possivel ao sistema. Sensores deste projeto existem apenas para incluir entries. Nada mais.

#### Argumentos clear_entries

[topo](#lição-6---2-termômetro)

 - **account_id**: Opcional. String. ID de usuário para remover todas as entries. Se omitido, remove todas as entries do owner.

#### Exemplo clear_entries

[topo](#lição-6---2-termômetro)

O exemplo abaixo remove todas as entries associadas ao id "my-sensor-id".

```rust
near call my-contract clear_entries '{"account_id": "my-sensor-id.testnet"}' --accountID my-contract
```

#### Pânico clear_entries

[topo](#lição-6---2-termômetro)

 - Se o usuário não for owner;
 - Se id de conta não for encontrado;

---

### view_get_format

[topo](#lição-6---2-termômetro)

Função view. Retorna o formato de temperatura armazenado como String.

```bash
near view my-contract view_get_format '{}'
```

--- 

### view_get

[topo](#lição-6---2-termômetro)

Função view. Retorna um ou mais valores associados a um id de conta.

Note que esta função é uma função view. Não realiza computação. E ainda assim retorna dois tipos de resultado possiveis.

O tipo ```ViewGet``` é declarado em './src/utils.rs'.

```rust
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ViewGet{
    Single(Entry),
    Multiple(Vec<Entry>),
}
```

Com esta declaração, percebe-se:

 - serde é usado para serializar e deserializar o tipo para json;
 - ```#[serde(untagged)]``` faz a serialização mostrar os valores contidos nas tuplas no json. Neste caso ```Entry``` ou ```Vec<Entry>```.

Dessa forma, o desenvolvedor pode usar as vantagens de um enum sem afetar a experiência do usuário. Que apenas vê o resultado.

**Aviso**: Este exemplo existe apenas para demonstrar a possibilidade de retornar diversos tipos. Implementar isso em outras linguagens pode aumentar a complexidade de código desnecessariamente. Tome cuidado com as necessidades do sistema.

#### Argumentos view_get

[topo](#lição-6---2-termômetro)

 - index: u64. Opcional. Index da entry a ser retornada. Se omitida, retorna todas as entries.
 - account_id: String. ID de usuário para retornar entries.

#### Exemplo view_get

[topo](#lição-6---2-termômetro)

A instrução abaixo retorna o primeiro elemento (se existir) associado a conta de usuário "sensor-id".

```bash
near view my-contract view_get '{"index": 0, "account_id": "sensor-id.testnet"}'
```

A instrução abaixo retorna todas as entries associadas ao id de conta "sensor-id".

```bash
near view my-contract view_get '{"account_id": "sensor-id.testnet"}'
```

---

## Implementação

[topo](#lição-6---2-termômetro)

Esta seção explicará como as funcionalidades descritas acimas foram alcançadas.

 - Documentação de projetos;
 - Organização de módulos;
 - Controle de acesso de usuários;
 - Acesso cross-contract;
 - Controle de input;
 - Controle de output;
 - Implementação de traits;

---

### Documentação de projetos

[topo](#lição-6---2-termômetro)

Como descrito [acima](#documentação) o comando:

```bash
cargo doc --open --lesson_6_2_thermometer
```

Gera um website com a toda a documentação do nosso projeto. A seguir serão descritos alguns detalhes sobre documentação:


---

#### Comentários sobre arquivo

[topo](#lição-6---2-termômetro)

Se o comentário é iniciado com ```//! ```, a documentação descreve sobre todo o arquivo ```.rs```, ou seja, o módulo.

Abaixo vemos um fragmento do inicio do módulo ```Day```, localizado no caminho ```./src/entry/schedule/date/day.rs```.

```rust
//! Módulo com todas as funcionalidades necessárias para a 
//! representação de dia no contrato.
//! 
//! Usamos um inteiro u8 para representar um dia. Mas 
//! precisamos garantir que este valor é válido.
//! 
//! Devido a isso, o tipo Day é representado por um struct 
//! tupla Day(u8).
//! 
//! Quando serializado para json, o valor é visto como um 
//! número u8. Ou seja, o usuário não perceberá essa 
//! complexidade.
//! 
```

Comentários como este devem existir no início do arquivo.

Recomenda-se que estes comentários tenha um resumo sobre toda funcionalidade disponivel no módulo.

---

#### Comentários e Documentação

[topo](#lição-6---2-termômetro)

Comentários com "// " não são incluidos na documentação. Comentários com "/// " descrevem o tipo abaixo.

Como exemplo, abaixo está a implementação da função ```Day::assert_valid```, que é uma função privada.

```rust
/// # Panics
/// Se dia for invalido.
fn assert_valid(&self, current_month: &Month, current_year: &Year) {
    let &Day(day) = self;

    // Coleta o valor do ano.
    let mut current_year: i32 = current_year.get();

    // Se for negativo, converte para positivo
    if current_year < 0 {
        current_year = -current_year;
    }

    // A cada 4 anos, o mês de janeiro possui 29 dias, ao invez de 28.
    // true se for um "leap year".
    let leap_year: bool = (current_year % 4) == 0;
    // converte true para 1, false para 0.
    let leap_year: u8 = leap_year as u8;

    // source: https://www.rapidtables.com/calc/time/months-of-year.html
    let max_day: u8 = match current_month {
        &Month::January(_) => 31,
        &Month::February(_) => 28 + leap_year,
        &Month::March(_) => 31,
        &Month::April(_) => 30,
        &Month::May(_) => 31,
        &Month::June(_) => 30,
        &Month::July(_) => 31,
        &Month::August(_) => 31,
        &Month::September(_) => 30,
        &Month::October(_) => 31,
        &Month::November(_) => 30,
        &Month::December(_) => 31,
    };

    // panic se o valor do dia for maior que o valor referente ao mês.
    assert!(day <= max_day,
        "Invalid values for day. Day: {}, Month: {}, Year: {}. Day for given month and year can not be higher than {}.",
            day,
            current_month,
            current_year,
            max_day,
    )
}
```

A função acima impede que um usuário escolha um dia incorreto.

 - Se for informado o dia 31 para outubro, não ocorrerá erro;
 - Se for informado o dia 31 setembro, haverá erro. Pois não existe o dia 31 de setembro;
 - Se for informado o dia 29 de fevereiro em 2024, não haverá erro por ser ano bissexto (leap year);
 - Se for informado o dia 29 de fevereiro em 2025, haverá erro por não ser ano bissexto (leap year);

Note que os comentários escritos com "//" descrevem a implementação do código e não aparecem na documentação.

Note como os comentários escritos com "///" descrevem o elemento abaixo (neste caso, a função ```assert_valid```). Este exemplo apenas descreve que a função entra em pânico caso o dia seja inválido. Funções privadas não serão usadas por outros, então não há necessidade de documentar com extremo detalhe.

A seguir há um exemplo da função ```Month::new``` no caminho ```./src/entry/schedule/month/Month.rs```.

```rust
/// Constroi uma instância de Mês:
/// 
/// Os possiveis valores de String na esquerda são 
/// convertidos para os seguintes valores na direita:
/// 
///  - "january", "jan", "janeiro", "enero", "ene" => Month::January("January")
///  - "february", "feb", "fevereiro", "fev", "febrero" => Month::February("February")
///  - "march", "mar", "março", "marzo" => Month::March("March")
///  - "april", "apr", "abril", "abr" => Month::April("April")
///  - "may", "maio", "mayo" => Month::May("May")
///  - "june", "jun", "junho", "junio" => Month::June("June")
///  - "july", "jul", "julho", "julio" => Month::July("July")
///  - "august", "aug", "agosto", "ago" => Month::August("August")
///  - "september", "sep", "setembro", "set", "septiembre" => Month::September("September")
///  - "october", "octo", "oct", "outubro", "out", "octubre", "octu" => Month::October("October")
///  - "november", "nov", "novembro", "noviembre" => Month::November("November")
///  - "december", "dec", "dezembro", "dez", "diciembro", "dic" => Month::December("December")
/// 
/// # Panics
/// Se o argumento não for nenhum dos possiveis acima.
/// 
pub fn new(month: &str) -> Self {
    let lower_case: String = month.to_ascii_lowercase();
    
    match &lower_case[..]{
        "january" | "jan" | "janeiro" | "enero" | "ene" => Month::January(String::from("January")),
        "february" | "feb" | "fevereiro" | "fev" | "febrero" => Month::February(String::from("February")),
        "march" | "mar" | "março" | "marzo" => Month::March(String::from("March")),
        "april" | "apr" | "abril" | "abr" => Month::April(String::from("April")),
        "may" | "maio" | "mayo" => Month::May(String::from("May")),
        "june" | "jun" | "junho" | "junio" => Month::June(String::from("June")),
        "july" | "jul" | "julho" | "julio" => Month::July(String::from("July")),
        "august" | "aug" | "agosto" | "ago" => Month::August(String::from("August")),
        "september" | "sep" | "setembro" | "set" | "septiembre" => Month::September(String::from("September")),
        "october" | "octo" | "oct" | "outubro" | "out" | "octubre" | "octu" => Month::October(String::from("October")),
        "november" | "nov" | "novembro" | "noviembre" => Month::November(String::from("November")),
        "december" | "dec" | "dezembro" | "dez" | "diciembre" | "dic" => Month::December(String::from("December")),
        invalid => panic!("Invalid value for month: {}.", invalid),
    }
}
```

A documentação da função acima é detalhado. Isso porque, não apenas os usuários necessitam de informação sobre como mês é reconhecido como argumento, assim como é essencial que desenvolvedores tenham acesso ao máximo de informação possivel caso queiram modificar/adaptar este projeto para outros casos de uso.

Resumidamente, mês é convertido de uma String. Vários Strings são válidos para cada possivel valor. Por exemplo: "january", "jan", "JAN", "Janeiro", "enero" e "ene" são todos possiveis Strings que convertem para um ```Month::January(String::from("January"))```. Não é case-sensitive.

---

#### Exemplos/Testes em Documentação

[topo](#lição-6---2-termômetro)

Exemplos existentes em documentação são incluidos nos testes. Segue adiante um fragmento da documentação para o módulo no caminho "./src/entry/schedule/date/day.rs"

```rust

//! ## Examples
//! 
//! ```rust
//! # use lesson_6_2_thermometer::schedule::date::day::Day;
//! # use lesson_6_2_thermometer::schedule::date::month::Month;
//! # use lesson_6_2_thermometer::schedule::date::year::Year;
//! 
//! // not leap year
//! let month = Month::new("feb");
//! let year = Year::new(1971);
//! 
//! let day = Day::new(28, &month, &year);
//! assert_eq!(u8::from(&day), 28);
//! assert_eq!(format!("{}", day), "28");
//! assert_eq!(String::from(&day), "28");
//! 
//! // leap year
//! let month = Month::new("feb");
//! let year = Year::new(1972);
//! 
//! let day = Day::new(29, &month, &year);
//! assert_eq!(u8::from(&day), 29);
//! assert_eq!(format!("{}", day), "29");
//! assert_eq!(String::from(&day), "29");
//! 
//! ```
```

O bloco acima é testado sempre que testes de unidade são executados. 

O objetivo deste exemplo é demonstrar as implementações de traits implementadas ao tipo Day.

Linhas com "#" não aparecem na documentação. Existem para permitir o funcionamento correto dos testes. Assim como reduzem a complexidade do exemplo.

---

### Organização de Módulos

[topo](#lição-6---2-termômetro)

Ao analizar a organização dos arquivos. Um desenvolvedor pode ficar confuso sobre como os módulos foram organizados. Alguns diretórios possuem um arquivo de nome "mod.rs" e outros não. Isso é porque duas formas diferentes de declarar módulos foram usadas.

A pergunta a ser feita é "Como um diretório pode ser considerado um módulo?". Existem duas respostas:

 - Um arquivo rust com o nome do diretório existindo no mesmo caminho que o diretório.
 - O arquivo rust com nome "mod.rs" dentro do diretório.

Como exemplo:

 - O módulo "entry" se encontra no caminho ```./src/entry/mod.rs```;
 - O módulo "date" se encontra no caminho ```./src/schedule/date.rs```. O diretório se encontra no mesmo caminho ```./src/schedule/```;
 - O módulo "temperature" se encontra no caminho ```./src/temperature/mod.rs```;
 - O módulo "time" se encontra no caminho ```./src/schedule/time.rs```. O diretório se encontra no mesmo caminho;

---

### Controle de Acesso de Usuários

[topo](#lição-6---2-termômetro)

Funções call precisam ser assinadas por uma conta NEAR. Podemos controlar o acesso checando o nome da conta que fez a chamada. Quando o contrato é inicializado, apenas o owner tem permissão de acessar o contrato. Outras contas podem ser incluidas através da função ```Contract::allow_user```.

Cada conta adicionada não terá permissões administrativas. Mas terão o próprio espaço de armazenamento de dados. Terão permissão para incluir entries. E terão permissão para atualizar os valores armazenados em uma conta.

O motivo da limitação de acesso de outras contas é devido a possibilidade de terceiros conseguirem acesso aos dispositivos sem permissão e usá-los para acessar o smart contract. Caso isso aconteça, a unica ação que um dispositivo infringido pode fazer é incluir e listar entries. Uma ação que é limitada pela quantidade de gás disponível na conta do usuário sensor.

As funções que controlam acesso ao contrato são as funções privadas:

 - ```Contract::assert_owner_only```: Entra em pânico se o caller não for o owner. Owner é a mesma conta em que o contrato foi implantado (deployed).
 - ```Contract::assert_user_allowed```: Entra em pânico se o caller não for um usuário incluido na lista de permitidos. Owner se encontra na lista de permitidos.


```rust
// Garante que apenas owner está chamando a função.
fn assert_owner_only(&self){
    let predecessor: AccountId = env::predecessor_account_id();
    let owner_id: AccountId = AccountId::from(env::current_account_id());

    assert_eq!(predecessor, owner_id, "Only owner's account is allowed to make this function call.");
}

// Garante que apenas usuários permitidos podem chamar funções.
fn assert_user_allowed(&self) {
    let predecessor_id: AccountId = env::predecessor_account_id();
    let owner_id: AccountId = env::current_account_id();

    // Se a conta dono do contrato está chamando a função.
    if owner_id == predecessor_id {
        return;
    }

    // Se não for a conta dono, e não estiver incluido na lista de permitidos, causa panic.
    assert!(self.users.contains(&predecessor_id), "User not allowed to make this call.");
}
```

Através do módulo ```near_sdk::env```, temos acesso a informações relacionadas ao ambiente da máquina virtual, e informações sobre a mensagem recebida. Segue a descrição de alguns dados disponibilizados pelo módulo:

 - ```env::predecessor_account_id```: ID da ultima conta que assinou a chamada call;
 - ```env::signer_account_id```: ID da primeira conta que assinou a chamada call;
 - ```env::current_account_id```: ID da conta atual. A conta que possui o contrato;

Nas situações mais comuns, "**predecessor_account_id**" e "**signer_account_id**" são a mesma conta. Mas contratos podem chamar outros contratos (chamadas "cross-contract"), cada conta assina a chamada seguinte. Por exemplo, digamos que uma conta **A** chame um contrato **B**, que chama um contrato **C**, que chama um contrato **D**:

```
A -> B -> C -> D
```

Na situação acima. Para todas as chamadas, o "**signer_account_id**" é a conta A. 
O "**predecessor_account_id**" é:
 
 - Para o ambiente **B**, o "**predecessor_account_id**" é **A**;
 - Para o ambiente **C**, o "**predecessor_account_id**" é **B**;
 - Para o ambiente **D**, o "**predecessor_account_id**" é **C**;

Para ambos exemplos acimas. Coletamos "**owner_account_id**" e "**predecessor_account_id**". Se ambos são iguais, o **chamador** é o "**owner**". Se "**predecessor_account_id**" estiver incluido na lista de permitidos, então é um usuário permitido.

Poderiamos ter usado "**signer_account_id**". Mas isso anularia a possibilidade de chamadas cross-contract. Um desenvolvedor pode decidir adicionar mais funcionalidades a este contrato. Esta decisão manterá a oportunidade de integração com a funcionalidade de outros contratos.

---

### Acesso Cross-Contract

[topo](#lição-6---2-termômetro)

Como descrito acima. Chamada "cross-contract" é quando um contrato faz uma chamada "call" para um outro contrato. Cada conta assina a chamada seguinte. Em alguns casos isso não é desejado. Como funções que fazem alterações críticas ou caras no sistema. Funções em que o desenvolvedor não deseja que sejam chamadas automaticamente.

Isso é implementado através da função ```Contract::no_cross_contract```. Descrito a seguir:

```rust
// Garante que o chamado é direto. Não pode ser um contrato chamando outro contrato.
fn assert_no_cross_contract(&self){
    let signer_id: AccountId = env::signer_account_id();
    let predecessor_id: AccountId = env::predecessor_account_id();
    assert_eq!(signer_id, predecessor_id, "Cross-contract calls not allowed.");
}
```

Basta garantir que o "signer_account_id" é o mesmo que o "predecessor_account_id".

Isto é usado nas funções ```Contract::allow_user```, ```Contract::remove_user``` e ```Contract::set_format```. Usado na função ```Contract::set_format``` . As outras funções porque são operações administrativas que podem incluir usuários indesejados, ou remover usuários e grande quantidade de dados do sistema.
 
 - ```Contract::set_format```: pode resultar em altos consumos de gás para uma grande quantidade de dados;
 - ```Contract::allow_user```: pode incluir usuários indesejados ao sistema. É uma função pouco usada, mas essencial;
 - ```Contract::remove_user```: pode remover usuários e todos os dados associados a um respectivo usuário. Mal uso dessa função pode causar danos irreversiveis aos dados;

---

### Controle de Output

[topo](#lição-6---2-termômetro)

Uma mesma função pode retornar tipos diferentes através do uso de enums. Para isso, primeiro criamos o enum que representa todos os possiveis tipos que podem ser retornados:

```rust
// ./src/utils.rs
use near_sdk::serde::{
    Deserialize, Serialize,
};

use crate::entry::Entry;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ViewGet{
    Single(Entry),
    Multiple(Vec<Entry>),
}
```

 - ```#[derive(Deserialize, Serialize)]``` aplica as traits ```near_sdk::serde::Deserialize``` e ```near_sdk::serde::Serialize``` ao enum. São necessárias para converter um json para o tipo (deserializar), e converter um tipo para json (serializar).
 - ```#[serde(crate) = "near_sdk::serde"]``` irá informar o compilador que a crate "serde" se encontra em "near_sdk::serde". Sem essa instrução, o compilador tentará encontrar "serde" em uma crate única.
 - ```#[serde(untagged)]``` é um atributo da crate "serde" que informa à crate para não usar tags para este enum. Sem este atributo, o valor é descrito como ```{ Single: valor }``` ou ```{ Multiple: [valor1, valor2, ...] }```. Com este atributo o valor é descrito como ```valor```, ou ```[valor1, valor2, ...]```, respectivamente.

Mais detalhes sobre serde e configurações no [site oficial](https://serde.rs/enum-representations.html) da crate.

Com o enum declarado e configurado. Basta retornar o tipo em uma função de contrato:

```rust
pub fn view_get(
    &self, 
    index: Option<u64>, 
    account_id: String,
) -> ViewGet {
    match index{
        None => {
            let result = self.entries
                .get(&account_id)
                .unwrap()
                .to_vec();

            ViewGet::Multiple(result)
        },
        Some(index) => {
            let result = self.entries
                .get(&account_id)
                .unwrap()
                .get(index)
                .unwrap();

            ViewGet::Single(result)
        }
    }
}
```

Note que a função acima é uma função view. Conversão de estado para json é preparado durante compilação. Ou seja, não consome gás.

A função simplesmente retorna o enum ```ViewGet``` declarado anteriormente. Se o argumento "index" existe no json, retorna ```ViewGet::Single(valor)``` com o "valor" encontrado. Se argumento "index" foi omitido, retorna ```ViewGet::Multiple(valores)``` com os "valores" encontrados.

Uso do tipo Option para argumentos é descrito logo a seguir.

---

### Controle de Input

[topo](#lição-6---2-termômetro)

Note a função abaixo. Existem vários argumentos da função do tipo ```Option```.

```rust
pub fn new_entry(
    &mut self, 
    time: Option<(u8, u8, f32)>,
    date: Option<(i32, String, u8)>,
    temp_value: f32, 
    temp_format: Option<String>,
){
    self.assert_user_allowed();
    let user: AccountId = env::predecessor_account_id();

    log("Called new_entry.");

    log("Creating Entry.");
    let entry: Entry = Entry::new(time, date, &self.temp_format, temp_value, temp_format);

    log("Acquiring entries for this user.");
    let mut entries = match self.entries.get(&user){
        None => panic!("Unexpected Behavior: Failed to find entries for this user."),
        Some(value) => value,
    };
    
    log("Pushing entry to Vector.");
    entries.push(&entry);
    assert!(self.entries.insert(&user, &entries).is_some(), "Failed to replace vector");

    log("Operation Successful.");
}
```

Option é um tipo da biblioteca standard que pode ter duas alternativas, ```Some(valor)``` ou ```None```. Se usarmos este tipo nos argumentos das nossas funções, o usuário não precisa incluir este argumento na chamada de função.

 - Se incluir, o valor do argumento é ```Some(valor)```;
 - Se não incluir, o valor do argumento é ```None```;

Usamos instruções match para considerar as duas possibilidades. Por exemplo, o ``` account_id``` é opcional para a maioria das funções de contrato.

```rust
// ./src/contract.rs

pub fn list_update_entries(
    &mut self, 
    account_id: Option<String>,
) -> Vec<Entry> {
    self.assert_user_allowed();

    // let account_id: AccountId = env::predecessor_account_id();
    let account_id = match account_id{
        None => {
            env::predecessor_account_id()
        },
        Some(value) => {
            let predecessor = env::predecessor_account_id();
```

O fragmento da função ```Contract::list_update_entries``` acima possui o argumento ```account_id``` que é um Option. Se o valor para ```account_id``` existir na mensagem, usa-o. Senão usa o ```account_id``` da conta que chamou o contrato. Essa operação é repetida em diversos outras funções.

No tópico sobre [controle de output](#controle-de-output) acima. Foi descrito como usar um enum para possuir diversos tipos diferentes para o mesmo retorno de função. Podemos usar um enum com as mesmas configurações para aceitar diversos tipos de input também. Não é necessário incluir nenhuma configuração serde adicional.

```rust
// ./src/utils.rs
use near_sdk::serde::{
    Deserialize, Serialize,
};

use crate::entry::Entry;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ViewGet{
    Single(Entry),
    Multiple(Vec<Entry>),
}
```

Basta alterar os valores internos do enum de acordo com suas necessidades, e usar o tipo como argumento da função de contrato.

---

### Implementação de Traits

[topo](#lição-6---2-termômetro)

O "dia", contido em "data", contido em "schedule" é representado da seguinte forma.

```rust
// ./src/schedule/date/day.rs

#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Day(u8);
```

É apenas um ```u8``` contido em um tipo próprio. Embora esse tipo tenha sido criado para aplicarmos as limitações que o valor de dia pode possuir. Queremos utilizar este valor como um número nos outros casos. Para isso, aplicamos as seguintes traits:

```rust
/// Nos permite usar u8::from(nossoDay)
impl From<&Day> for u8{
    fn from(day: &Day) -> u8 {
        let &Day(result) = day;

        result
    }
}

/// Nos permite usar u8::from(nossoDay)
impl From<&Day> for String{
    fn from(day: &Day) -> String {
        u8::from(day).to_string()
    }
}


// Usado para converter o struct para String. Se usarmos instruções como format!, println! ou panic!, esta trait é usada.
impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
```

A trait ```From``` permite a conversão de um tipo para outro. Devido a essas implementações, para uma variável Day com nome day, é possivel converter para u8 e String, respectivamente, com ```u8::from(&day)``` e ```String::from(&day)```.

A trait ```std::fmt::Display``` parece complicado, mas simplesmente permite o uso do tipo em macros como ```panic```, ```format``` e ```println```. Sem esta implementação, uma instrução como ```println!("O valor de day é {}", day)``` resultaria em pânico.

---

## Fim

[topo](#lição-6---2-termômetro)

A próxima seção mostrará aplicações úteis para Result.

A próxima lição será sobre traits.


