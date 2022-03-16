use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
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
pub enum Exemplo0{
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}


// Semelhante a structs, implementamos métodos no namespace de Exemplo0 a seguir:
impl Exemplo0{

    /// Observa o valor de si mesmo e retorna um número entre 1 e 5.
    /// 
    /// Note o &self, significando que a função acessa o valor, mas não altera.
    /// 
    pub fn get_number(&self) -> u32 {
        // Instruções match são semelhantes a uma 
        match self {
            Exemplo0::First => {1},
            Exemplo0::Second => {2},
            Exemplo0::Third => {3},
            Exemplo0::Fourth => {4},
            Exemplo0::Fifth => {5},
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
            Exemplo0::Third => true,
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
pub enum Exemplo1{
    NoValue,
    AnInteger(i32),
    AFloat(f32),
    AString(String),
    ATuple(i32, u32),
    ACLikeStruct{first: u32, second: String},
}

impl Exemplo1{
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
    //  - Retornar um ponteiro? A possibilidade disso ser necessário é quase nula. O custo de complexidade é muito alto.
    // 

    // O método a seguir retorna apenas um tipo, isso é aceitável para o compilador.
    pub fn get(&self) -> String {
        match self{
            Exemplo1::NoValue => String::from(""),
            Exemplo1::AnInteger(valor) => format!("{}", valor),
            Exemplo1::AFloat(valor) => format!("{}", valor),
            Exemplo1::AString(valor) => format!("{}", valor),
            Exemplo1::ATuple(valor0, valor1) => format!("({}, {})", valor0, valor1),
            Exemplo1::ACLikeStruct { first, second } => format!("{{\nfirst: {},\nsecond: {},\n}}\n", first, second),
        }
    }



    // Também pode-se criar uma função para retornar cada tipo.

    /// true se o valor do enum
    pub fn is_no_value(&self) -> bool{
        match self{
            Exemplo1::NoValue => true,
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
            Exemplo1::AnInteger(valor) => Some(valor.clone()),
            _ => None
        }
    }


    /// Retorna true se possui algum numero inteiro impar,
    pub fn has_an_odd_number(&self) -> bool {
        match self {
            Exemplo1::NoValue => false,
            Exemplo1::AnInteger(valor) => {
                if valor%2 == 1{
                    return true;
                }
                    
                return false;
            },
            Exemplo1::AFloat(_valor) => false,
            Exemplo1::AString(_valor) => false,
            Exemplo1::ATuple(valor0, valor1) => {
                return (valor0%2 == 1) || (valor1%2 == 1);
            },
            Exemplo1::ACLikeStruct { first, second: _ } => {
                first%2 == 1
            },
        }
    }

}

