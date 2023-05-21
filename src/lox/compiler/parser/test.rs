

use crate::lox::common::{ByteCode, Function, FunctionKind, Globals};
use crate::lox::compiler::{ChunkWriter, CompileError, Locals, Scanner, Tokenizer};


use super::Parser;
use super::ParserOutput;


mod arrays;
mod base2numbers;
mod base8numbers;
mod base10numbers;
mod base16numbers;
mod for_loops;
mod if_statement;
mod return_statement;
mod trailing_comma;
mod while_loops;
mod literals;
mod strings;


fn test(code: &str) -> Result<ByteCode, CompileError> {
    
    // This code duplicates a lot of what the Compiler does. Hmm.
    let reader = std::io::Cursor::new(code);
    let scanner = Scanner::new(reader);
    let mut input = Tokenizer::new(scanner);

    let function = Function::new("__test__", FunctionKind::Script);
    let mut writer = ChunkWriter::new(function);
    let mut globals = Globals::new();

    let mut output = ParserOutput {
        writer: 	&mut writer,
        globals: 	&mut globals,
        locals:	&mut Locals::new(false),
    };

    // Be verbose to make debugging a little easier
    match Parser::new().parse(&mut input, &mut output) {
        Ok(function) => {
            println!("parse() returned Ok({:?})", function);
            return Ok(ByteCode::new(function, globals));
        }
        Err(compile_error) => {
            println!("parse() returned Err({:?})", compile_error);
            return Err(compile_error);
        }
    }
}

#[test]
fn parser_emptystring() {
    let code = "";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_global_var_one() {
    let code = "var a;";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_global_var_two() {
    let code = "var a; var b;";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_global_var_three() {
    let code = "var a; var b; var c;";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 3);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_global_var_duplicate() {
    let code = "var a; var a;";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Global 'a' already declared");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 12);
}

#[test]
fn parser_global_function() {
    let code = "fun f() {}";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_global_function_duplicate() {
    let code = "fun f() {} fun f() {}";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Global 'f' already declared");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 16);
}

#[test]
fn parser_global_class() {
    let code = "class c {}";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_global_class_malformed_1() {
    let code = "class c() {}";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected '{' after class name, got '('");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 8);
}

#[test]
fn parser_global_class_malformed_2() {
    let code = "class c {";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected '}' after class body, got '\0'");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 10);
}

#[test]
fn parser_global_class_malformed_3() {
    let code = "class {}";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected class name, got '{'");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 7);
}

#[test]
fn parser_global_class_duplicate() {
    let code = "class c {} class c {}";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Global 'c' already declared");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 18);
}

#[test]
fn parser_break_not_in_loop() {
    let code = "break;";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Keyword 'break' is misplaced");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 1);
}

#[test]
fn parser_continue_not_in_loop() {
    let code = "continue;";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Keyword 'continue' is misplaced");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 1);
}

#[test]
fn parser_function_255_params_ok() {
    let code = format!(
        "fun f({}) {{}}",
        (0 .. 255).map(|byte| format!("arg_{:02X}", byte)).collect::<Vec<String>>().join(", ")
    );
    let res = test(code.as_str());
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_function_256_params_bad() {
    let code = format!(
        "fun f({}) {{}}",
        (0 .. 256).map(|byte| format!("arg_{:02X}", byte)).collect::<Vec<String>>().join(", ")
    );
    let res = test(code.as_str());
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Can not have more than 255 parameters");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 2047);
}

#[test]
fn parser_global_and_local_var_ok() {
    let code = "var v; fun f() { var v; }";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_global_and_arg_ok() {
    let code = "var v; fun f(v) {}";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_duplicate_locals() {
    let code = "fun f() { var v; var v; }";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Variable named 'v' already declared in this scope");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 22);
}

#[test]
fn parser_duplicate_arg_bad() {
    let code = "fun f(v, v) {}";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Variable named 'v' already declared in this scope");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 10);
}

#[test]
fn parser_same_arg_and_local_bad() {
    let code = "fun f(v) { var v; }";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Variable named 'v' already declared in this scope");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 16);
}

#[test]
fn parser_undeclared_local_bad() {
    let code = "fun f() { return v; }";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Undeclared variable 'v'");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 18);
}

#[test]
fn parser_self_initialize_global_ok() {
    let code = "var v=v;"; // This is nonsense but globals are auto-initialized as Value::Null
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_self_initialize_local_bad() {
    let code = "fun f() { var v=v; }";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Can not read local variable in its own initializer");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 17);
}

#[test]
fn parser_initialize_local_using_same_global_impossible() {
    let code = "var v; fun f() { var v=v; }"; // 'v' is ambiguous, compiler assumes local 'v'
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Can not read local variable in its own initializer");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 24);
}

#[test]
fn parser_initialize_local_using_different_global_ok() {
    let code = "var v; fun f() { var v2=v; }";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_class_inherit_other_ok() {
    let code = "class c1 {} class c2 of c1 {}";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_class_inherit_self_bad() {
    let code = "class c1 {} class c2 of c2 {}";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Class 'c2' can not inherit from itself");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 25);
}

#[test]
fn parser_class_inherit_instance_is_runtime_error() {
    let code = "class c1 {} var i1=c1(); class c2 of i1 {}"; // Runtime error
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 3);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_super_with_superclass() {
    let code = "class c1 { m() {} } class c2 of c1 { m() { return super.m(); } }";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_super_without_superclass() {
    let code = "class c1 { m() { return super.m(); } }";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Can not use 'super' in a class with no superclass");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 25);
}

#[test]
fn parser_super_with_missing_superclass_method() {
    let code = "class c1 {} class c2 of c1 { m() { return super.m(); } }"; // Runtime error
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_super_is_accessor_only() {
    let code = "class c1 {} class c2 of c1 { m() { return super; } }";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected '.' after 'super', got ';'");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 48);
}

