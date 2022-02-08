//! Cada arquivo rusts no diretório tests é testado individualmente.
//! Não há necessidade de incluí-los usando mod.

// Descomente a linha abaixo e verá um aviso de erro "privado".
// Isso é porque é público apenas dentro da crate. E "tests" age como a própria crate.
// use lesson_4_modules::a_module::specific_module::hello;

#[test]
fn oneplusone() {
    assert_eq!(1 + 1, 2);
}
