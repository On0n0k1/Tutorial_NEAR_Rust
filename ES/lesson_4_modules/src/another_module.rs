pub fn hello() -> String {
    String::from("Hello from another_module")
}


// A função abaixo é privada.
// #[allow(unused)] é uma forma de dizer ao compilador/linter para ignorar o aviso. Mesmo em um situação como essa, em que é uma má ideia. Pois esta função é inutil.
// Comente o #[allow(unused)] para ver o aviso
#[allow(unused)]
fn this_is_private() -> String {
    // Tradução: Esta função nunca será usada pois é privada.
    String::from("This will never be used because it's private")
}

