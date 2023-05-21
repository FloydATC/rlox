

use super::test;


#[test]
fn parser_while_true() {
    let code = "while (true) {}";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_while_true_single_stmt() {
    let code = "while (true) exit;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_while_missing_left_paren() {
    let code = "while true) {}";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected '(' after 'while', got 'true'");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 7);
}

#[test]
fn parser_while_naked_expression() {
    let code = "while true {}"; // Maybe someday we'll support this? Not today.
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected '(' after 'while', got 'true'");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 7);
}

#[test]
fn parser_while_missing_right_paren() {
    let code = "while (true {}";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected ')' after 'while'-condition, got '{'");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 13);
}

#[test]
fn parser_while_missing_condition() {
    let code = "while () {}";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected conditional expression, got ')'");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 8);
}

#[test]
fn parser_while_missing_stmt() {
    let code = "while (true)";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Expected expression");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 13);
}

#[test]
fn parser_while_else() {
    let code = "while (true) {} else {}";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Keyword 'else' is misplaced");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 17);
}

#[test]
fn parser_while_true_break() {
    let code = "while (true) { break; }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn parser_naked_break() {
    let code = "break;";
    println!("code={}", code);
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
fn parser_naked_continue() {
    let code = "continue;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Keyword 'continue' is misplaced");
    assert_eq!(error.get_at().is_some(), true);
    let at = error.get_at().unwrap();
    assert_eq!(at.lineno, 1);
    assert_eq!(at.charno, 1);
}

