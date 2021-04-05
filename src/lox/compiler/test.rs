
use crate::lox::function::{Function, FunctionKind};
use super::Compiler;


#[test]
fn compiler_new() {
    let function = Function::new("__test__", FunctionKind::Script, 0);
    let _compiler = Compiler::new(function);
}




