

use super::test;


#[test]
fn parser_function_definition_no_trailing_comma_ok() {
    let code = "fun f(a, b, c) {}";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_function_definition_trailing_comma_ok() {
    let code = "fun f(a, b, c, ) {}";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_function_call_no_trailing_comma_ok() {
    let code = "fun f(a, b, c) {} f(1, 2, 3);";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_function_call_trailing_comma_ok() {
    let code = "fun f(a, b, c) {} f(1, 2, 3, );";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_function_call_trailing_comma_with_formatting_ok() {
    let code = "fun f(a, b, c) {} f(\r\n\t1,\r\n\t2,\r\n\t3,\r\n);";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_method_definition_no_trailing_comma_ok() {
    let code = "class c { m(a, b, c) {} }";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_method_definition_trailing_comma_ok() {
    let code = "class c { m(a, b, c, ) {} }";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_method_call_no_trailing_comma_ok() {
    let code = "class c { m(a, b, c) {} } var i=c(); i.m(1, 2, 3);";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_method_call_trailing_comma_ok() {
    let code = "class c { m(a, b, c) {} } var i=c(); i.m(1, 2, 3, );";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_initializer_call_no_trailing_comma_ok() {
    let code = "class c { init(a, b, c) {} } var i=c(1, 2, 3);";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_initializer_call_trailing_comma_ok() {
    let code = "class c { init(a, b, c) {} } var i=c(1, 2, 3, );";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

