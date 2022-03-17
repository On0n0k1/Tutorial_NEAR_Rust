use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env};


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
    env::log(message.as_bytes());
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
#[derive(BorshDeserialize, BorshSerialize, Clone)]
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
#[derive(Clone, BorshDeserialize, BorshSerialize)]
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
    // Isso será dificil de implementar.
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
        match self{
            Example1::NoValue => String::from(""),
            Example1::AnInteger(valor) => format!("{}", valor),
            Example1::AFloat(valor) => format!("{}", valor),
            Example1::AString(valor) => format!("{}", valor),
            Example1::ATuple(valor0, valor1) => format!("({}, {})", valor0, valor1),
            Example1::ACLikeStruct { first, second } => format!("{{\nfirst: {},\nsecond: \"{}\",\n}}\n", first, second),
        }
    }


    // Também pode-se criar uma função para retornar cada tipo.

    /// true se o valor do enum
    pub fn is_no_value(&self) -> bool{
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
        // valor será uma referência, clonamos o valor para não retornar uma referência.
        match self{
            Example1::AnInteger(valor) => Some(valor.clone()),
            _ => None
        }
    }


    /// Retorna true se possui algum numero inteiro impar,
    pub fn has_an_odd_number(&self) -> bool {
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
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Employee{
    pub name: String,
    pub id: u32,
    pub pass: String,
    pub permissions: Vec<String>,
    pub actions: Vec<String>,
}

impl Default for Employee{
    fn default() -> Self {
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
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub enum Example2User{
    Admin{ name: String, id: u32, pass: String, actions: Vec<String> },
    Client{ name: String, id: u32, orders: Vec<String> },
    Employee( Employee ),
}

impl Default for Example2User{
    fn default() -> Self {
        Example2User::Employee(Employee::default())
    }
}


impl Example2User{

    /// Retorna nome do usuário.
    /// 
    /// O bloco que chama o método não precisa de saber o que o usuário é.
    pub fn get_name(&self) -> String {
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

        match self{
            Example2User::Client { name: _, id: _, orders: _ } => { false },
            Example2User::Admin { name: _, id: _, pass: _, actions: _ } => { true },
            Example2User::Employee(employee) => {

                // Vec implementa a trait IntoIterator.
                // Isso disponibiliza o método .iter ao vetor.
                // Este método nos permite iterar referencias de String.
                // Nenhuma cópia de String é feita.
                for employee_permission in employee.permissions.iter(){
                    if permission.eq_ignore_ascii_case(employee_permission){
                        return true;
                    }
                }

                false
            }
        }
    }

    /// Retorna a lista de ações se for Admin ou Employee.
    /// 
    /// Como exemplo, digamos que o design de projeto necessita de retornar
    /// um erro, se o usuário for um Client.
    /// 
    /// Result é semelhante a Option. Mas é usado para representar ações que podem causar erros.
    /// Explicado na proxima sub-seção.
    pub fn get_actions(&self) -> Result<Vec<String>, String> {
        
        // Se for client, retorna um erro (Como exemplo).
        // Se for admin ou employee, retorna referencia para o Vec.
        let actions = match self{
            Example2User::Client { name: _, id: _, orders: _ } => { return Err(format!("O usuário é cliente")); },
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

        // Example1::NoValue => String::from(""),
        // Example1::AnInteger(valor) => format!("{}", valor),
        // Example1::AFloat(valor) => format!("{}", valor),
        // Example1::AString(valor) => format!("{}", valor),
        // Example1::ATuple(valor0, valor1) => format!("({}, {})", valor0, valor1),
        // Example1::ACLikeStruct { first, second } => format!("{{\nfirst: {},\nsecond: {},\n}}\n", first, second),
    }
}