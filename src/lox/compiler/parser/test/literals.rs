

use super::test;


#[test]
fn parser_null() {
    let code = "null;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_null_as_initializer() {
    let code = "var v=null;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_cannot_assign_to_null() {
    let code = "null=null;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Invalid assignment target");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno(), 1);
    assert_eq!(at.charno(), 5);
}

#[test]
fn parser_true() {
    let code = "true;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_true_as_initializer() {
    let code = "var v=true;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_cannot_assign_to_true() {
    let code = "true=true;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Invalid assignment target");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno(), 1);
    assert_eq!(at.charno(), 5);
}

#[test]
fn parser_false() {
    let code = "false;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_false_as_initializer() {
    let code = "var v=false;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_cannot_assign_to_false() {
    let code = "false=false;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Invalid assignment target");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno(), 1);
    assert_eq!(at.charno(), 6);
}

