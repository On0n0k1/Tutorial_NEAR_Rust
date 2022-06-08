// Ter (crate) após pub torna a função (ou módulo, trait, ...) disponivel 
// para módulos locais, mas não para crates externos.
pub(crate) fn hello() -> String{
    String::from("Hello from crate::a_module::specific_module")
}

/// panic é quando algo de errado acontece e tudo deve parar imediatamente.
/// Podemos causar panic sob demanda.
pub fn this_will_panic() {
    panic!("A panic has just happened");
}


// cfg(test) diz ao compilador que isto apenas existe em ambientes de teste.
#[cfg(test)]
mod tests{
    // carrega tudo que está neste arquivo, fora deste módulo.
    use super::*;

    #[test]
    fn hello_test(){
        // Hello é público apenas para módulos desta crate.
        // Tentaremos importá-lo no diretório de tests, o que causará um erro.
        // Pois o diretório de testes atua como a própria crate.
        assert_eq!(
            hello(),
            "Hello from crate::a_module::specific_module",
        );
    }


    #[test]
    // Podemos testar em situações que causam panic.
    // expected garante que entraremos em pânico pelo motivo correto.
    #[should_panic(expected = "A panic has just happened")]
    fn this_will_panic_test() {
        this_will_panic();
    }
}