
#[test]
fn interpreter_new() {
    let _interpreter = super::Interpreter::new();
}

#[test]
fn interpreter_compile_emptystring() {
    let mut interpreter = super::Interpreter::new();
    interpreter.compile("");
}

#[test]
fn interpreter_execute_emptystring() {
    let mut interpreter = super::Interpreter::new();
    interpreter.compile("");
    interpreter.execute();
}

