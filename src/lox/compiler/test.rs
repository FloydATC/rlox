
use crate::lox::function::Function;
use super::Compiler;


#[test]
fn compiler_new() {
    let function = Function::new("__test__", 0);
    let _compiler = Compiler::new(function);
}



