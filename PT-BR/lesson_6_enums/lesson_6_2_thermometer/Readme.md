# Lição 6 - 2 Termômetro

[voltar](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/lesson_6_enums/)

Neste exemplo, será visto uma aplicação prática para os conceitos descritos nas lições anteriores. Além dos conceitos anteriores, será demonstrado tópicos como:

 - Documentação de projetos. Testes em documentação;
 - Controle de acesso de usuários em smart contracts;
 - Controle de input de dados;
 - Controle de output de dados;

## Idéia

Na seção anterior, foram descritos enums e como utilizar instruções match. Além das funcionalidades descritas, também podemos utilizar enums para controlar a entrada/saida de dados. 

Digamos que um desenvolvedor de aplicativos embarcados (embedded) quer monitorar os dados de diversos termômetros simultaneamente. Logo de início, foram notadas as seguintes restrições:

 - **Conectar** todos os dispositivos a um computador é inviável.
 - **Manter** a máquina como servidor 24 horas recebendo input dos sensores também é inviavel.
 - **Criar** um servidor nuvem como aws também é inviável porque o desenvolvedor não quer patrocinar o foguete de algum bilhonário.

Portanto, o desenvolvedor decide criar um smart contract para armazenar os dados. A vantagem de uma alternativa dessas são:

 - **Fácil de implementar**. O usuário só precisa de uma conta para armazenar o smart contract. Uma conta para cada sensor utilizado.
 - **Fácil de automatizar**. A única coisa que muda entre cada aplicação é o nome dos contratos. Com bons scripts, desenvolvedores podem implementar o sistema em minutos.
 - **Fácil de expandir**. Este exemplo utiliza valores de temperatura. Mas o contrato pode ser facilmente alterado para receber qualquer tipo de dados.

Mas o contrato apenas coleta dados? Não executa nenhuma computação sobre os dados? A computação de dados pode ser executada localmente pelo desenvolvedor. Não há necessidade de desperdiçar gás em operações que podem ser facilmente executadas localmente.

O contrato resolve o problema de implantação e comunicação entre os dispositivos. Já existem bibliotecas eficientes para ciência de dados. Não há necessidade de reinventar a roda. 

Resumindo. Sensores enviam dados para o smart contract. O Smart Contract armazena os dados de acordo com o nome do sensor, formato de temperatura, data e tempo de recebimento. A máquina do usuário acessa o contrato e coleta os dados armazenados para processamento.

A seguir será descrito o funcionamento do contrato.

## Documentação

Documentação sobre contrato implementada. Gere um website com todos os módulos executando o comando:

```bash
cargo doc --open -p lesson_6_2_thermometer
```

 - p: Como o projeto está incluido como componente de workspace do diretório anterior. É necessário especificar a crate a gerar.

## Testes

Execute testes de unidade com a instrução:

```bash
cargo test -p lesson_6_2_thermometer
```

Isso irá testar os exemplos na documentação também.


## Contrato

Antes de observar o funcionamento das funções. Vale saber que após implantação, o contrato apenas permite execução de chamadas "call" para o dono (owner). Portanto, as chamadas call administrativas devem ser assinadas com o mesmo nome de conta do Smart Contract.

Outros usuários (os sensores) podem ser incluídos na lista de usuários permitidos. Cada usuário possuirá a própria lista de armazenamento de dados.

---

### Inicialização

O contrato inicializa com formato de temperatura Kelvin. O único usuário incluido na lista de permissões é o dono (owner).

---

### allow_user:

```bash
near call my-contract allow_user '{"account_id": "sensor-account-id.testnet"}' --accountId my-contract
```


Função call. Apenas owner tem permissão de executar esta função. Não pode ser cross-contract.

Inclui o id de conta informado na lista de usuários permitidos.

Argumentos:

 - account_id: String. Nome de usuário para incluir na lista de permissões.

Pânico:

 - Se id de conta for um id inválido.
 - Se usuário ja estiver incluido.

---

### remove_user:

```bash
near call my-contract remove_user '{"account_id": "sensor-account-id.testnet"}' --accountId my-contract
```

Função call. Apenas owner tem permissão de executar esta função. Não pode ser cross-contract.

Exclui o id de conta informado da lista de usuários permitidos. Todos os dados armazenados relacionados a este usuário são perdidos.

Owner não pode ser removido.

Argumentos:

 - account_id: String. Nome de usuário para excluir da lista de permissões.

Pânico:
 
 - Se o nome de usuário for inválido;
 - Se o usuário informado não existir na lista de permissões;
 - Se o nome de usuário informado for o owner;

---

### set_format

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

Função call. Todos usuários permitidos podem executar esta função. Pode ser cross-contract. Adiciona um input de temperatura associado ao usuário que executou a função.

Argumentos:
 - **time**: Opcional. Tupla com estrutura ```(u8, u8, f32)``` com os valores para hora minuto e segundo, respectivamente. Se omitido, o contrato utilizará o valor do momento em que a função foi executada (UTC).
 - **date**: Opcional. Tupla com estrutura ```(i32, String, u8)``` com os valores para ano, mês e dia, respectivamente. Se omitido, o contrato utilizará o dia em que a função foi executada.
 - **temp_value**: f32, o valor de temperatura coletado. Não pode ser menor do que zero absoluto.
 - **temp_format**: Opcional, String. Se omitido, o contrato utilizará o formato de temperatura do sistema. Se o formato for diferente do formato do sistema, realizará conversão da temperatura antes de armazenar.

#### Exemplos

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

#### Pânico

 - Se o **usuário** não tem permissão de acesso;
 - Se **hora** (time) não for um valor negativo ou maior do que 23;
 - Se **minuto** (time) não for um valor negativo ou maior do que 59;
 - Se **segundo** (time) for um valor negativo ou maior do que 59.9999...;
 - Se **dia** (date) for um valor inválido para o mês e ano;
 - Se **mês** (date) for um String inválido para mês;
 - Se **temp_format** for um String inválido;

---

### list_update_entries

Função call. Pode ser cross-contract. Retorna todas as entries associadas a um id de conta.


Todos os usuários permitidos podem acessar os próprios dados. Mas apenas owner tem permissão de acessar dados de outros usuários. Essa restrição existe para manter controle sobre consumo de gás no contrato. Outros usuários ainda podem coletar os dados utilizando a função view.

Argumentos:
 - account_id: Opcional. String. ID de usuário a ser coletado. Se omitido, retornará os dados do usuário que executou a função.

Retorna: Vec com todas as entries associadas ao id de conta.

#### Exemplos

O exemplo abaixo retorna todas as entries associadas ao usuário "my-sensor-id".

```bash
near call my-contract list_update_entries '{}' --accountID my-sensor-id
```

O exemplo abaixo retorna todas as entries associadas a outro usuário. Apenas owner tem permissão para isso.

```bash
near call my-contract list_update_entries '{"account_id": "my-sensor-id.testnet"}' --accountID my-contract
```

#### Pânico

 - Se usuário não tiver permissão de acesso;
 - Se usuário não for owner e estiver tentando atualizar as entries de outro usuário.
 - Se usuário não for encontrado;


---

### clear_entries

Função call. Apenas owner pode chamar esta função. Pode ser cross-contract. Apaga todas as entries associadas a um usuário.

O motivo da função permitir cross-contract é para facilitar automação de contrato. Contratos externos não podem incluir ou remover usuários permitidos. Mas podem adicionar entries, podem coletar dados e remover dados.

Usuários não tem permissão de utilizar essa função para evitar ações suspeitas. Caso um dos sensores for acessado por um terceiro, este terá o acesso mais limitado possivel ao sistema. Sensores deste projeto existem apenas para incluir entries. Nada mais.

#### Argumentos

 - **account_id**: Opcional. String. ID de usuário para remover todas as entries. Se omitido, remove todas as entries do owner.

#### Exemplo

O exemplo abaixo remove todas as entries associadas ao id "my-sensor-id".

```rust
near call my-contract clear_entries '{"account_id": "my-sensor-id.testnet"}' --accountID my-contract
```

#### Pânico

 - Se o usuário não for owner;
 - Se id de conta não for encontrado;

---

### view_get_format

Função view. Retorna o formato de temperatura armazenado como String.

```bash
near view my-contract view_get_format '{}'
```

--- 

### view_get

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

#### Argumentos

 - index: u64. Opcional. Index da entry a ser retornada. Se omitida, retorna todas as entries.
 - account_id: String. ID de usuário para retornar entries.

#### Exemplo

A instrução abaixo retorna o primeiro elemento (se existir) associado a conta de usuário "sensor-id".

```bash
near view my-contract view_get '{"index": 0, "account_id": "sensor-id.testnet"}'
```

A instrução abaixo retorna todas as entries associadas ao id de conta "sensor-id".

```bash
near view my-contract view_get '{"account_id": "sensor-id.testnet"}'
```

