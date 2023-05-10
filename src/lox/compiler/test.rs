
use crate::lox::function::Function;
use crate::lox::function_kind::FunctionKind;
use super::Compiler;


#[test]
fn compiler_new() {
    let function = Function::new("__test__", FunctionKind::Script);
    let _compiler = Compiler::new(function);
}




