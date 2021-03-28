
use super::Compiler;


#[test]
fn compiler_new() {
    let _compiler = Compiler::new(None);
}

#[test]
fn compiler_compile_emptystring() {
    let mut compiler = Compiler::new(None);
    let result = compiler.compile("");
    assert!(result.is_ok(), "compiler should return Ok");
}


