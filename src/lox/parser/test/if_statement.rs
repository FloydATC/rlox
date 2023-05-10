
use super::test;


#[test]
fn parser_if_statement() {
    let code = "if (true) {}";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_if_statement_then_single_stmt() {
    let code = "if (true) exit;";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_if_statement_then_single_stmt_else_single_stmt() {
    let code = "if (true) exit; else exit;";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_if_statement_with_else() {
    let code = "if (true) {} else {}";
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_if_statement_missing_paren_1() {
    let code = "if true) {}";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected '(' after 'if', got 'true'");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 4);
}

#[test]
fn parser_if_statement_missing_paren_2() {
    let code = "if (true {}";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected ')' after 'if'-condition, got '{'");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 10);
}

#[test]
fn parser_if_statement_missing_then() {
    let code = "if (true)";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected expression");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 10);
}

#[test]
fn parser_if_statement_open_then() {
    let code = "if (true) {";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected '}' after block, got '\0'");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 12);
}

#[test]
fn parser_if_statement_with_else_nothing() {
    let code = "if (true) {} else";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected expression");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 18);
}

#[test]
fn parser_if_statement_with_open_else() {
    let code = "if (true) {} else {";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected '}' after block, got '\0'");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 20);
}

#[test]
fn parser_else_not_after_if() {
    let code = "else exit;";
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Keyword 'else' is misplaced");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 1);
}

