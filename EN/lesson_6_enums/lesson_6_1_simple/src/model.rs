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


// Similar to structs, let's implement some function on our enum Example0
impl Example0{

    /// Check its own value and returns a number from 1..5.
    /// 
    /// We use a reference &self, meaning we only access the value, not modify it
    /// 
    pub fn get_number(&self) -> u32 {
        log("Calling Example0::get_number");

        // and here we match enum choices/options to return values
        match self {
            Example0::First => {1},
            Example0::Second => {2},
            Example0::Third => {3},
            Example0::Fourth => {4},
            Example0::Fifth => {5},
        }
    }

    /// true if own enum value is set to Example0::Third
    pub fn is_third(&self) -> bool {

        log("Calling Example0::is_third");

        // match compares value in the order specified, 
        // and if assign a variable to a match, that variable will have the return value
        // of the match block.
        //
        // A variable starting with an underscore, is a variable whose value we don't care about
        // and that we will probably not be using later on
        //
        // So, a _ for our last case, as seen below, will match to anything else
        match self {
            Example0::Third => true,
            _ => false,
            // Exemplo0::SECOND => {
            //     // uncomment the above
            //     // and you'll get an error beause _ is above it 
            //     // and that means this SECOND enum option will never be reached
            //     false
            // },
        }
    }
}


/// An enum allows different typs to be grouped into a single concept or entity
/// 
/// This example shows that using enums for complex types (composability) is not really a good idea.
/// 
/// Function in enums should return simple results.
/// Having to 'extract' the underlying values in enums so they can be used only adds complexity.
/// 
/// Use enums to group different types that seem to share similar concepts.
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
    // Remember that functions must return one specific result type
    //
    // A developer might be tempted to create a get function to return the stored value but that 
    // can be difficult to implement.
    // 
    // The simplest way to go about it is to have your function convert enum options as needed into a single (common) return type
    // Here are some examples:
    //  - Return a value as a String
    //  - Use borsh or serde to serialize the value to bytes, to later deserialize after receiving the result
    //  - Implement generic. We'll go over this in a future lesson.
    //  - Return a "pointer"? There's a VERY low probability that is really necessary, and the complexity increases a lot.
    // 

    // Our function will return just a String type 
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

    /// true if enum is Example1::NoValue.
    pub fn is_no_value(&self) -> bool{
        log("Calling Example1::is_no_value");

        match self{
            Example1::NoValue => true,
            _ => false,
        }
    }


    /// Return an integer, if the enum value is that choice
    ///
    /// Option will be exlained in a future lesson. 
    ///
    /// Option is an enum of the std library.
    /// It means that we can have a value or not
    /// Option could be Option::Some(value) or Option::None.
    /// There's no NULL in Rust!
    pub fn get_an_integer(&self) -> Option<i32>{
        log("Calling Example1::get_an_integer");

        // value is a reference, so we clone the value to return an actual value and not a reference.
        match self{
            Example1::AnInteger(valor) => Some(valor.clone()),
            _ => None
        }
    }


    /// Returns true if the enum represents an odd number
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
                // We don't care about the second value, so we use an underscore
                first%2 == 1
            },
        }
    }
}


/// A simple struct with Default
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

/// A user for an application.
/// 
/// A user can be any of these three:
///  - Client
///  - Employee
///  - Administrator
/// 
/// We cantrol permission of each of them using enums
/// 
/// All have name and id, and each has in addition:
///  - Admin: password (encrypted) and  a list of action they can do in the app. 
///  - Employee: password (encrypted), a list of actions they can do and a list of permissions for the app.
///  - Client: only list of orders.
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
    /// Returns the user name
    /// 
    /// The caller doesn't need to know the type of the user.
    pub fn get_name(&self) -> String {
        log("Calling Example2User::get_name");

        match self {
            Example2User::Admin { name, id: _, pass: _, actions: _ } => { name.clone() },
            Example2User::Client { name, id: _, orders: _ } => { name.clone() },
            Example2User::Employee( employee ) => { employee.name.clone() },
        }
    }

    /// Check if an user has a permission.
    /// 
    /// Having Strings to keep permissions is not a good idea, due to possible errors. A better choice would be enums
    /// 
    ///  - Clientes don't have permissions. Always return false.
    ///  - Administrators have all permissions (can do anything). Always returns true.
    ///  - Employees might have the permission. 
    /// 
    pub fn has_permission(&self, permission: String) -> bool{
        log("Calling Example2User::has_permission");

        match self{
            Example2User::Client { name: _, id: _, orders: _ } => { false },
            Example2User::Admin { name: _, id: _, pass: _, actions: _ } => { true },
            Example2User::Employee(employee) => {

                // Vec implement the Iterator trait, 
                // so we have the iter() function available.
                // This functions allows to iterate over references to strings,
                // but no copies are made while iterating 
                for employee_permission in employee.permissions.iter(){
                    if permission == *employee_permission {
                        return true;
                    }
                }

                false
            }
        }
    }

    /// Returns a list of actions for Admin or Employee. 
    /// 
    /// Result is similar to Option, but is used to provide more detail into the outcome while also accounting for errors
    pub fn get_actions(&self) -> Result<Vec<String>, String> {
        log("Calling Example2User::get_actions");
        
        // If user is Client, return error Err()
        // If user is Admin or Employee, return reference to Vec.
        let actions = match self{
            Example2User::Client { name: _, id: _, orders: _ } => { return Err(format!("User is Client")); },
            Example2User::Admin { name: _, id: _, pass: _, actions, } => { actions },
            Example2User::Employee( employee ) => { &employee.actions },
        };

        let mut result: Vec<String> = Vec::new();
        // Create a copy of vec, using references
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


    /// Create some instance to test
    fn example1_create() -> (
        Example1,
        Example1,
        Example1,
        Example1,
        Example1,
        Example1,
    ){
        // Return a tuple with different types available
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
    /// Check for valid return strings
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

    `   
    /// Check that only true is returned for no_value
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

    /// Create 3 instances of Example2User for our tests
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
        // Create three users for testing below
        let [admin, employee, client] = example2_user_create();

        // destructure names into variables
        let (result_admin, result_employee, result_client) = (
            admin.get_name(),
            employee.get_name(),
            client.get_name(),
        );

        // check all names are OK 
        // we are comparing a String against a &str
        // because we implemented the trait partial_eq
        assert_eq!(result_admin, "Lucas");
        assert_eq!(result_employee, "Lucas");
        assert_eq!(result_client, "Lucas");
    }

    #[test]
    fn example2_has_permission(){
        // Create three users for testing below
        let [admin, employee, client] = example2_user_create();

        // Check if users have the log permission
        let (result_admin, result_employee, result_client) = (
            admin.has_permission(String::from("Access client logs")),
            employee.has_permission(String::from("Access client logs")),
            client.has_permission(String::from("Access client logs")),
        );

        assert_eq!(result_admin, true);
        assert_eq!(result_employee, true);
        assert_eq!(result_client, false);
    }

    #[test]
    fn example2_get_actions(){

        // helper function to compare vectors
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

            // vectors must have the same number of elements
            assert_eq!(first.len(), second.len(), "len is different");

            let length: usize = first.len();

            for counter in 0..length {
                assert_eq!(first[counter], second[counter], "Failed comparison between {} and {}", first[counter], second[counter]);
            }

            return true;
        }

        // Create three users for testing below
        let [admin, employee, client] = example2_user_create();

        // get actions for each user
        let (result_admin, result_employee, result_client) = (
            admin.get_actions(),
            employee.get_actions(),
            client.get_actions(),
        );

        // check admin actions
        assert!(
            vec_eq(
                result_admin, 
                Ok(vec![
                    String::from("Signed in 24 dec 2022, 06h33m49.67s"),
                    String::from("Logged off 24 dec 2022, 09h22m01.18s"),
                ])
            )
        );

        // check employee actions
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

        // check client, and make sure it returns an error.
        assert!(
            vec_eq(
                result_client,
                Err(format!("User is Client")),
            )
        )

    }
}