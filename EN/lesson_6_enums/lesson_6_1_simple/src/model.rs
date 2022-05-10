use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{ Serialize, Deserialize },
};    


// As duas funções a seguir são declaradas para
// termos duas implementações diferentes de uma mesma função "log".
// As mensagens chamadas com essa função log aparecerão 
// em testes e na máquina virtual.


#[cfg(test)]
pub fn log(message: &str){
    println!("{}", message);
}

#[cfg(not(test))]
pub fn log(message: &str){
    near_sdk::env::log(message.as_bytes());
}


/// Um struct possui varios valores diferentes simultâneos, um enum só pode possuir um valor.
/// Os possiveis valores de um enum são descritos em sua declaração.
/// 
/// Os possiveis valores de Exemplo0 são:
///  - Example0::FIRST
///  - Example0::SECOND
///  - Example0::THIRD
///  - Example0::FOURTH
///  - Example0::FIFTH
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Example0{
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

impl Default for Example0{
    fn default() -> Self {
        Example0::First
    }
}


// Semelhante a structs, implementamos métodos no namespace de Exemplo0 a seguir:
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

    /// true se o valor for Exemplo0::THIRD
    pub fn is_third(&self) -> bool {

        log("Calling Example0::is_third");

        // match compara os valores iniciando do topo
        // se colocarmos um nome de variavel, o branch acerta
        // e a variavel possui o valor no bloco associado.
        //
        // Uma variável que começa com o caracter _ é uma variável que
        // não pretendemos utilizar.
        //
        // Devido a isso, _ sempre será "matched", as alternativas
        // abaixo nunca serão acessadas.
        match self {
            Example0::Third => true,
            _ => false,
            // Exemplo0::SECOND => {
            //     // Descomente esse bloco e receberá um aviso
            //     // Essa branch nunca será alcançada
            //     // Porque a branch acima aplica a qualquer pattern.
            //     false
            // },
        }
    }
}


/// Um enum permite um objeto representar vários tipos diferentes:
/// 
/// Este exemplo possui o objetivo de mostrar que usar um enum como conteiner de valores não é uma boa ideia.
/// 
/// Os métodos de enum devem retornar resultados simples.
/// Tentar retirar os valores armazenados em enums para serem usados fora adiciona complexidade desnecessária ao código.
/// 
/// Use enums para agrupar tipos diferentes que compartilham uma funcionalidade semelhante.
/// 
/// 
#[derive(Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Example1{
    NoValue,
    AnInteger(i32),
    AFloat(f32),
    AString(String),
    ATuple(i32, u32),
    ACLikeStruct{first: u32, second: String},
}

impl Default for Example1{
    fn default() -> Self {
        Example1::NoValue
    }
}


impl Example1{
    // Porem, vale lembrar que um método ou função deve retornar apenas um tipo de resultado especificado.
    //
    // Um desenvolvedor pode tentar criar uma função get que retorna o valor armazenado.
    // Isso pode ser dificil de implementar.
    // 
    // A forma mais simples de uma função get seria converter para um mesmo tipo.
    // Seguem alguns exemplos:
    //  - Retornar o valor como String
    //  - Usar Borsh ou serde para serializar o valor para bytes, deserializando após o recebimento.
    //  - Implementar genéricos. serão explicados em outra lição.
    //  - Retornar um ponteiro? A possibilidade disso ser necessário é baixa. O custo de complexidade é muito alto.
    // 

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

    /// true se o enum for Example1::NoValue.
    pub fn is_no_value(&self) -> bool{
        log("Calling Example1::is_no_value");

        match self{
            Example1::NoValue => true,
            _ => false,
        }
    }


    /// Retorna um inteiro, se o enum for essa alternativa.
    ///
    /// Option será explicado em detalhes na próxima seção.
    ///
    /// Option é um enum da biblioteca padrão (standard).
    /// Representa a possibilidade de possuir um valor ou não.
    /// Option pode ser Option::Some(valor) ou Option::None.
    pub fn get_an_integer(&self) -> Option<i32>{
        log("Calling Example1::get_an_integer");

        // valor será uma referência, clonamos o valor para não retornar uma referência.
        match self{
            Example1::AnInteger(valor) => Some(valor.clone()),
            _ => None
        }
    }


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
}


/// Tipo criado para o exemplo abaixo.
/// 
/// Criado apenas para mostrar um exemplo de implementação de struct em match.
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Employee{
    pub name: String,
    pub id: u32,
    pub pass: String,
    pub permissions: Vec<String>,
    pub actions: Vec<String>,
}

impl Default for Employee{
    fn default() -> Self {
        log("Calling Employee::default");

        Employee { 
            name: String::from("a name"), 
            id: 11, 
            pass: String::from("some random pass"), 
            permissions: vec![
                String::from("Can access google"),
                format!("Can access 9gag"),
            ], 
            actions: vec![
                String::from("Did something"),
                String::from("Did something else"),
            ],
        }
    }
}

/// Exemplo mais prático. 
/// 
/// Representa o Usuário de um aplicativo.
/// 
/// Digamos que um usuário possa ser os seguintes tipos:
///  - Cliente
///  - Funcionario
///  - Administrador
/// 
/// Podemos controlar as permissões de cada com um enum.
/// 
/// Seria melhor termos tipos struct pra cada valor, mas estamos com pressa.
/// 
/// Todos possuem nome e id, alem disso, cada um possui:
///  - Admin: passe (codificado, claro) para acesso. Lista de ações no sistema.
///  - Employee: passe (codificado) para acesso. Lista de ações. Lista de permissões no sistema.
///  - Client: apenas lista de pedidos.
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Example2User{
    Admin{ name: String, id: u32, pass: String, actions: Vec<String> },
    Client{ name: String, id: u32, orders: Vec<String> },
    Employee( Employee ),
}

impl Default for Example2User{
    fn default() -> Self {
        log("Calling Example2User::default");

        Example2User::Employee(Employee::default())
    }
}


impl Example2User{

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

    /// Checa se usuário possui permissão para ação.
    /// 
    /// Não é uma boa ideia usar String para permissões. Devido a possivel erros de caracteres, etc. Enums seriam melhor.
    /// 
    /// Mas o código ja está complexo o suficiente.
    /// 
    /// Neste exempĺo:
    ///  - Clientes não possuem permissão. Sempre retorna falso.
    ///  - Administradores sempre possuem permissão. Sempre retorna true.
    ///  - Empregados podem ou não possuir permissão. Checa por permissões.
    /// 
    pub fn has_permission(&self, permission: String) -> bool{
        log("Calling Example2User::has_permission");

        match self{
            Example2User::Client { name: _, id: _, orders: _ } => { false },
            Example2User::Admin { name: _, id: _, pass: _, actions: _ } => { true },
            Example2User::Employee(employee) => {

                // Vec implementa a trait IntoIterator.
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

    /// Retorna a lista de ações se for Admin ou Employee.
    /// 
    /// Como exemplo, digamos que o sistema precisa retornar
    /// um erro, se o usuário for um Client.
    /// 
    /// Result é semelhante a Option. Mas é usado para representar ações que podem causar erros.
    /// Explicado na proxima seção.
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
}


#[cfg(test)]
mod tests{
    // use std::result;

    use super::*;

    #[test]
    fn example0() {
        let first = Example0::First;
        let second = Example0::Second;
        let third = Example0::Third;
        let fourth = Example0::Fourth;
        let fifth = Example0::Fifth;

        assert_eq!(first.get_number(), 1);
        assert_eq!(second.get_number(), 2);
        assert_eq!(third.get_number(), 3);
        assert_eq!(fourth.get_number(), 4);
        assert_eq!(fifth.get_number(), 5);

        assert_eq!(first.is_third(), false);
        assert_eq!(second.is_third(), false);
        assert_eq!(third.is_third(), true);
        assert_eq!(fourth.is_third(), false);
        assert_eq!(fifth.is_third(), false);
    }

    // NoValue,
    // AnInteger(i32),
    // AFloat(f32),
    // AString(String),
    // ATuple(i32, u32),
    // ACLikeStruct{first: u32, second: String},


    /// Cria 6 instâncias diferentes de Example1 para servir de exemplo.
    fn example1_create() -> (
        Example1,
        Example1,
        Example1,
        Example1,
        Example1,
        Example1,
    ){
        // Retorna uma tupla com um exemplo de cada um dos valores.
        (
            Example1::NoValue,
            Example1::AnInteger(10),
            Example1::AFloat(3.5),
            Example1::AString(String::from("A String")),
            Example1::ATuple(-5, 5),
            Example1::ACLikeStruct{first: 1, second: String::from("second")},
        )
    }

    #[test]
    /// Garante que a função get retorna as Strings esperadas.
    fn example1_get(){
        let (
            no_value,
            an_integer,
            a_float,
            a_string,
            a_tuple,
            a_c_like_struct
        ) = example1_create();

        let no_value = no_value.get();
        let an_integer = an_integer.get();
        let a_float = a_float.get();
        let a_string = a_string.get();
        let a_tuple = a_tuple.get();
        let a_c_like_struct = a_c_like_struct.get();
        

        assert!(no_value.eq_ignore_ascii_case(""));
        assert!(an_integer.eq_ignore_ascii_case("10"));
        assert!(a_float.eq_ignore_ascii_case("3.5"));
        assert!(a_string.eq_ignore_ascii_case("A String"));
        assert!(a_tuple.eq_ignore_ascii_case("(-5, 5)"));
        assert!(a_c_like_struct.eq_ignore_ascii_case(&format!("{{\nfirst: 1,\nsecond: \"second\",\n}}\n")));
    }

    /// Garante que apenas retorna true para no_value.
    #[test]
    fn example1_is_novalue(){
        let (
            no_value,
            an_integer,
            a_float,
            a_string,
            a_tuple,
            a_c_like_struct
        ) = example1_create();

        let no_value = no_value.is_no_value();
        let an_integer = an_integer.is_no_value();
        let a_float = a_float.is_no_value();
        let a_string = a_string.is_no_value();
        let a_tuple = a_tuple.is_no_value();
        let a_c_like_struct = a_c_like_struct.is_no_value();

        assert_eq!(no_value, true);
        assert_eq!(an_integer, false);
        assert_eq!(a_float, false);
        assert_eq!(a_string, false);
        assert_eq!(a_tuple, false);
        assert_eq!(a_c_like_struct, false);
    }

    #[test]
    fn example1_get_an_integer(){

        fn asserting(value: Option<i32>, expected: Option<i32>){
            let comparison = match (value, expected){
                (None, None) => true,
                (Some(first), Some(second)) => first == second,
                _ => false,
            };

            assert_eq!(comparison, true, "Failed comparison between {:?} == {:?}\n", value, expected);
        }

        let (
            no_value,
            an_integer,
            a_float,
            a_string,
            a_tuple,
            a_c_like_struct
        ) = example1_create();

        let no_value = no_value.get_an_integer();
        let an_integer = an_integer.get_an_integer();
        let a_float = a_float.get_an_integer();
        let a_string = a_string.get_an_integer();
        let a_tuple = a_tuple.get_an_integer();
        let a_c_like_struct = a_c_like_struct.get_an_integer();

        asserting(no_value, None);
        asserting(an_integer, Some(10));
        asserting(a_float, None);
        asserting(a_string, None);
        asserting(a_tuple, None);
        asserting(a_c_like_struct, None);
    }

    // Admin{ name: String, id: u32, pass: String, actions: Vec<String> },
    // Client{ name: String, id: u32, orders: Vec<String> },
    // Employee( Employee ),

    /// Cria 3 instâncias diferentes de Example2User para serem usadas nos testes.
    fn example2_user_create() -> [Example2User; 3] {
        [
            Example2User::Admin { 
                name: String::from("Lucas"), 
                id: 0, 
                pass: String::from("12345"), 
                actions: vec![
                    String::from("Signed in 24 dec 2022, 06h33m49.67s"),
                    String::from("Logged off 24 dec 2022, 09h22m01.18s"),
                ]
            },
            Example2User::Employee(Employee {
                 name: String::from("Lucas"), 
                 id: 1, 
                 pass: String::from("123456"), 
                 permissions: vec![
                     String::from("Access client logs"),
                     String::from("Access stock"),
                 ], 
                 actions : vec![
                    String::from("Signed in 25 dec 2022, 08h11m32.01s"),
                    format!("Accessed Logs from {} {} {} {}, {:02}h{:02}m{:02}.{:02}s", "Lucas", 25, "dec", 2022, 09, 45, 19, 05),
                    String::from("Logged off 25 dec 2022, 11h44m51.92s"),
                ]
            }),
            Example2User::Client { 
                name: String::from("Lucas"), 
                id: 3, 
                orders: Vec::from([
                    format!("Successful transaction. ID: {}.", 4241235)
                ]) 
            }
        ]
    }

    #[test]
    fn example2_user_get_name(){
        // Cria 3 instâncias de example2 e aplica-os aos 3 tipos abaixo.
        let [admin, employee, client] = example2_user_create();

        // Executa get_name para as 3 instâncias.
        let (result_admin, result_employee, result_client) = (
            admin.get_name(),
            employee.get_name(),
            client.get_name(),
        );

        // Garante que o valor adiquirido para os 3 é "Lucas"
        // Detalhe extra: Estamos comparando um String com um &str,
        // isso é possivel porque implementam a trait partial_eq para os tipos.
        assert_eq!(result_admin, "Lucas");
        assert_eq!(result_employee, "Lucas");
        assert_eq!(result_client, "Lucas");
    }

    #[test]
    fn example2_has_permission(){
        // Cria 3 instâncias de example2 e aplica-os aos 3 tipos abaixo.
        let [admin, employee, client] = example2_user_create();

        // Executa has_permission para as 3 instâncias.
        let (result_admin, result_employee, result_client) = (
            admin.has_permission(String::from("Access client logs")),
            employee.has_permission(String::from("Access client logs")),
            client.has_permission(String::from("Access client logs")),
        );

        assert_eq!(result_admin, true);
        assert_eq!(result_employee, true);
        assert_eq!(result_client, false);
    }

    // pub fn get_actions(&self) -> Result<Vec<String>, String> {
    
    #[test]
    fn example2_get_actions(){

        // Função para comparar vetores
        fn vec_eq(first: Result<Vec<String>, String>, second: Result<Vec<String>, String>) -> bool {
            let (first, second) = match (first, second) {
                (Err(first), Err(second)) => {
                    return first == second;
                },
                (Ok(first), Ok(second)) => {
                    (first.clone(), second.clone())
                },
                (_, _) => {
                    return false;
                }
            };

            // Ambos os vetores devem ter o mesmo número de elementos
            assert_eq!(first.len(), second.len(), "len is different");

            let length: usize = first.len();

            for counter in 0..length {
                assert_eq!(first[counter], second[counter], "Failed comparison between {} and {}", first[counter], second[counter]);
            }

            return true;
        }

        // Cria 3 instâncias de example2 e aplica-os aos 3 tipos abaixo.
        let [admin, employee, client] = example2_user_create();

        // Executa has_permission para as 3 instâncias.
        let (result_admin, result_employee, result_client) = (
            admin.get_actions(),
            employee.get_actions(),
            client.get_actions(),
        );

        // Garante que a função retorna um Ok contendo os respectivos valores.
        assert!(
            vec_eq(
                result_admin, 
                Ok(vec![
                    String::from("Signed in 24 dec 2022, 06h33m49.67s"),
                    String::from("Logged off 24 dec 2022, 09h22m01.18s"),
                ])
            )
        );

        // Mesmo para employee.
        assert!(
            vec_eq(
                result_employee,
                Ok(vec![
                    String::from("Signed in 25 dec 2022, 08h11m32.01s"),
                    format!("Accessed Logs from {} {} {} {}, {:02}h{:02}m{:02}.{:02}s", "Lucas", 25, "dec", 2022, 09, 45, 19, 05),
                    String::from("Logged off 25 dec 2022, 11h44m51.92s"),
                ]),
            )
        );

        // No caso de client, garante que retorna um erro.
        assert!(
            vec_eq(
                result_client,
                Err(format!("User is Client")),
            )
        )

    }
}