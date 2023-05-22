

use super::test;


#[test]
fn parser_emptystring() {
    let code = "\"\";";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_anystring() {
    let code = "\"any\";";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_string_as_initializer() {
    let code = "var v=\"any\";";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_reassign_string_variable() {
    let code = "var v=\"\"; v=\"any\";";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_string_concatenation_is_expr() {
    let code = "exit \"\"+\"any\";";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_cannot_assign_to_literal_string() {
    let code = "\"\"=\"any\";";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Invalid assignment target");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno(), 1);
    assert_eq!(at.charno(), 3);
}

#[test]
fn parser_unterminated_emptystring() {
    let code = "\";";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected expression");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno(), 1);
    assert_eq!(at.charno(), 3);
}

#[test]
fn parser_unterminated_anystring() {
    let code = "\"any;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected expression");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno(), 1);
    assert_eq!(at.charno(), 6);
}

