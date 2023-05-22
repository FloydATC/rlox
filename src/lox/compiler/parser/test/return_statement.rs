

use super::test;


#[test]
fn parser_return_at_toplevel() {
    let code = "return;";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Can not 'return' from top level code");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno(), 1);
    assert_eq!(at.charno(), 1);
}

#[test]
fn parser_return_from_function() {
    let code = "fun fn() { return; }";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_return_value_from_function() {
    let code = "fun fn() { return 1; }";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_return_super_from_function() {
    let code = "fun fn() { return super; }";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Can not use 'super' outside of a class");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno(), 1);
    assert_eq!(at.charno(), 19);
}

#[test]
fn parser_return_this_from_function() {
    let code = "fun fn() { return this; }";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Can not use 'this' outside of a class");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno(), 1);
    assert_eq!(at.charno(), 19);
}

#[test]
fn parser_return_from_initializer_ok() {
    let code = "class c { init() { return; } }";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_return_value_from_initializer_bad() {
    let code = "class c { init() { return 1; } }";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Can not 'return' a value from initializer");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno(), 1);
    assert_eq!(at.charno(), 20);
}

#[test]
fn parser_return_from_method_ok() {
    let code = "class c { m() { return; } }";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_return_value_from_method_ok() {
    let code = "class c { m() { return 1; } }";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}
