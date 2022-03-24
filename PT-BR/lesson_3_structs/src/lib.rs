use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};


near_sdk::setup_alloc!();


// derive usa um macro para aplicar as traits no tipo StructExample.
// Implementamos isso porque o contrato precisa de BorshDeserialize e BorshSerialize.
// StructExample é um state do Contract, então declaramos derive aqui também.
#[derive(BorshDeserialize, BorshSerialize)]
// Um struct com tipos primitivos
pub struct StructExample{
    an_integer: u32,
    another_integer: i32,
    a_floating: f32,
    a_string: String,
}


// default é executado quando o codigo é carregado ao contrato e não é inicializado.
// Pode ser chamado manualmente para criar uma instância com valores padrão.
impl Default for StructExample{
    fn default() -> Self {
        // Cada atributo do StructExample é setado da forma Nome: Valor,
        // Não é preciso setar da forma nome: nome,
        let an_integer: u32 = 1;
        let a_floating: f32 = 0.5;


        StructExample {
            an_integer,
            another_integer: -1,
            a_floating,
            a_string: String::from("A default string"),
        }
    }
}

// Clone é uma forma de criar uma copia desse struct manualmente.
impl Clone for StructExample{
    // self é uma instancia de StructExample, Self (Letra maiúscula) é o Tipo StructExample.
    fn clone(&self) -> Self {
        let an_integer: u32 = self.get_an_integer();
        let another_integer: i32 = self.get_another_integer();
        let a_floating: f32 = self.get_a_floating();
        let a_string: String = self.get_a_string();

        // Self e StructExample são a mesma coisa (Em qualquer impl de StructExample)
        Self {
            an_integer,
            another_integer,
            a_floating,
            a_string,
        }

    }
}


// Aqui que métodos e funcões para o struct são implementados.
impl StructExample{

    // &self quer dizer que irá emprestar uma instância de StructExample, mas não irá alterá-la
    /// Retorna a copia de a_string
    pub fn get_a_string(&self) -> String {

        // Strings implementam a trait Clone mas não implementam Copy
        // Copy cria uma cópia da instância automaticamente quando ha uma instrução a = b
        // Clone cria uma cópia da instância quando o código chama clone()
        let a_string: String = self.a_string.clone();

        return a_string;
    }

    pub fn get_a_floating(&self) -> f32 {
        // f32 implementa Copy, então esta linha irá automaticamente criar uma copia de a_floating
        return self.a_floating;
    }

    pub fn get_another_integer(&self) -> i32 {
        // Não precisamos escrever "return" sempre.
        // Se a expressão não termina com ponto e virgula ";" retorna a expressão
        self.another_integer
    }

    pub fn get_an_integer(&self) -> u32 {
        self.an_integer
    }

    // &mut self quer dizer que irá emprestar uma instância de StructExample e fazer modificações a este.
    pub fn set_a_string(
        &mut self, 
        // detalhe importante: como não há referência & para a String, estamos tomando possessão (ownership) desta.
        // Quer dizer que a função é dona desta parte de memória.
        // Possessão de a_string_arg quer dizer que o código que o chamou não precisará usar a String novamente.
        //
        // Este é um dos superpoderes de Rust. Podemos criar código que cria o menor número de cópias possivel.
        a_string_arg: String,
    ) {
        // como emprestamos self mutavel, podemos alterar os valores internos
        self.a_string = a_string_arg;
    }

    pub fn set_a_floating(&mut self, arg: f32) {
        self.a_floating = arg;
    }

    pub fn set_an_integer(&mut self, an_integer: u32) {
        // u32 é unsigned, apenas valores positivos
        self.an_integer = an_integer;
    }

    pub fn set_another_integer(&mut self, another_integer: i32){
        // i32 é signed, pode ser positivo e negativo. Mas apenas metade do alcance máximo.
        self.another_integer = another_integer;
    }

    // Se o método não possui self, é apenas uma função comum associada ao tipo.
    pub fn just_a_function() {
        env::log(b"You just called this function");
        env::log(format!("1 + 1 = {}", 1 + 1).as_bytes());
    }

    // Se self está nos argumentos, a função toma possessão do tipo.
    // Quer dizer que o struct será deletado no fim da função (a não ser que retornemos ela).
    // É só um exemplo. Quase nunca terá que implementar uma função dessa forma.
    pub fn take_ownership(self) -> u32{
        env::log(b"Taking ownership of itself");

        let result = format!("an_integer is {}", self.an_integer);
        env::log(result.as_bytes());

        self.an_integer

        // self será liberado da memória aqui
    }
}


// #[near_bindgen] instrui near-sdk que esta struct representa o state principal do contrato.
// Podemos usar qualquer nome. Contract é só mais facil de entender.
#[near_bindgen]
// derive usa um macro para aplicar as traits no tipo StructExample.
// Implementamos isso porque o contrato precisa de BorshDeserialize e BorshSerialize.
// BorshSerialize converte o nosso tipo de retorno para json.
// BorshDeserialize converte os parâmetros em json para os tipos que nossa função chama.
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {
    struct_example: StructExample,
}

#[near_bindgen]
impl Contract{
    // Abaixo são as mesmas funções do struct sendo chamadas aqui.
    // A diferença sendo que estas são como interagimos com o contrato.
    //
    // O método para get_a_string precisa de &self, então usamos &self ou &mut self
    pub fn get_a_string(&self) -> String {
        self.struct_example.get_a_string()
    }

    pub fn get_a_floating(&self) -> f32 {
        self.struct_example.get_a_floating()
    }

    pub fn get_another_integer(&self) -> i32 {
        self.struct_example.get_another_integer()
    }

    pub fn get_an_integer(&self) -> u32 {
        self.struct_example.get_an_integer()
    }

    // O método para set_a_string precisa de &mut self, então só podemos usar &mut self
    pub fn set_a_string(&mut self, a_string_arg: String) {
        self.struct_example.set_a_string(a_string_arg);
    }

    pub fn set_a_floating(&mut self, a_floating: f32) {
        self.struct_example.set_a_floating(a_floating);
    }

    pub fn set_an_integer(&mut self, an_integer: u32) {
        self.struct_example.set_an_integer(an_integer);
    }

    pub fn set_another_integer(&mut self, another_integer: i32) {
        self.struct_example.set_another_integer(another_integer);
    }

    // just_a_function não tem &self, &mut self nem self
    // É apenas uma função no namespace da struct. Não precisamos de usar self também.
    pub fn just_a_function(){
        // :: é usado para referenciar namespaces. Não é um metodo de uma instância de struct, apenas uma função.
        StructExample::just_a_function();
    }

    pub fn take_ownership(&self) -> u32 {
        // A função take_ownership libera o objeto da memória.
        // The compiler will not let us call it directly when the owner is Contract.
        // O compilador não irá nos deixar chamar isso diretamente enquanto o dono for Contract.
        // Most of the issues of ownership can be solved by just cloning the object.
        // A maioria dos problemas de possessão (ownership) podem ser resolvidos apenas clonando objetos
        let a_copy: StructExample = self.struct_example.clone();

        // a_copy será removido no fim dessa função abaixo.
        let result = a_copy.take_ownership();

        // Descomente a linha abaixo e um erro de "move" irá aparecer.
        // Isso é porque chamamos uma função de uma instância que não existe mais.
        // env::log(format!("Essa linha de código irá gerar um erro {}", a_copy.get_a_floating()).as_bytes());

        result
    }
}


// cfg diz ao compilador para considerar esse modulo apenas em um ambiente de teste. Este não existe fora dessas condições.
#[cfg(test)]
mod tests{
    use super::*;
    use near_sdk::{
        AccountId,
        env,
        MockedBlockchain,
        testing_env,
        test_utils::VMContextBuilder,
        json_types::ValidAccountId,
    };

    fn env_setup(){
        let mut builder: VMContextBuilder = VMContextBuilder::new();

        // atributos que podemos alterar com esse builder
        //
        // current_account_id
        // signer_account_id
        // signer_account_pk
        // precessor_account_id
        // block_index
        // block_timestamp
        // epoch_height
        // account_balance
        // account_locked_balance
        // storage_usage
        // attached_deposit
        // prepaid_gas
        // random_seed
        // is_view

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
            "Assert Error. env: {} account: {}
            ", 
            env::current_account_id(), 
            &account_id,
        );
    }

    #[test]
    pub fn get_a_string() {
        env_setup();
    
        let contract: Contract = Contract::default();
    
        // Este valor é da implementação padrão.
        assert_eq!(
            contract.get_a_string(),
            "A default string"
        );
    }

    #[test]
    pub fn get_a_floating(){
        env_setup();
    
        let contract: Contract = Contract::default();
    
        // Este valor é da implementação padrão.
        assert_eq!(
            contract.get_a_floating(),
            0.5,
        );

    }

    #[test]
    pub fn get_another_integer(){
        env_setup();
    
        let contract: Contract = Contract::default();
    
        // Este valor é da implementação padrão.
        assert_eq!(
            contract.get_another_integer(),
            -1,
        );

    }

    #[test]
    pub fn get_an_integer(){
        env_setup();
    
        let contract: Contract = Contract::default();
    
        // Este valor é da implementação padrão.
        assert_eq!(
            contract.get_an_integer(),
            1,
        );
    }

    #[test]
    pub fn set_a_string() {
        env_setup();

        let mut contract: Contract = Contract::default();

        contract.set_a_string(String::from("A new string"));

        assert_eq!(
            contract.get_a_string(),
            String::from("A new string"),
        );
    }

    #[test]
    pub fn set_a_floating() {
        env_setup();

        let mut contract: Contract = Contract::default();

        contract.set_a_floating(-10.5432);
        
        assert_eq!(
            contract.get_a_floating(),
            -10.5432,
        );
    }

    #[test]
    pub fn set_an_integer() {
        env_setup();

        let mut contract: Contract = Contract::default();

        contract.set_an_integer(5);

        assert_eq!(
            contract.get_an_integer(),
            5,
        );
    }

    #[test]
    pub fn set_another_integer() {
        env_setup();

        let mut contract: Contract = Contract::default();

        contract.set_another_integer(7);

        assert_eq!(
            contract.get_another_integer(),
            7
        );
    }

    #[test]
    pub fn just_a_function(){
        env_setup();

        // Declaramos isso no início do contrato, mas não precisamos de usa-lo aqui
        // Começar a variável com _ se não tiver intenção de usá-la
        let _contract: Contract = Contract::default();

        // Note o :: devido a função não precisar de um state.
        Contract::just_a_function();
    }

    #[test]
    // take_ownership é só um exemplo de uma gambiarra em rust. Não ha muito uso aqui.
    pub fn take_ownership(){
        env_setup();

        let contract: Contract = Contract::default();

        let an_integer = contract.get_an_integer();

        assert_eq!(
            contract.take_ownership(),
            an_integer,
        );
    }

}
