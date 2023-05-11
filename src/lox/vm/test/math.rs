

use super::compile_and_execute;

// Arithmetics

#[test]
fn vm_plus() {
    let code = "exit 10 + 3;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 13);
}

#[test]
fn vm_minus() {
    let code = "exit 10 - 3;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn vm_multiply() {
    let code = "exit 10 * 3;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 30);
}

#[test]
fn vm_divide() {
    let code = "exit 10 / 3;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_modulo() {
    let code = "exit 10 % 3;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// Simple operator precedence
#[test]
fn vm_add_mul() {
    let code = "exit 2 + 10 * 3;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 32);
}

#[test]
fn vm_mul_add() {
    let code = "exit 10 * 3 + 2;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 32);
}

#[test]
fn vm_sub_div() {
    let code = "exit 2 - 10 / 3;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), -1);
}

#[test]
fn vm_div_sub() {
    let code = "exit 10 / 3 - 2;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// Grouping
#[test]
fn vm_grouped_add_mul() {
    let code = "exit (2 + 10) * 3;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 36);
}

#[test]
fn vm_grouped_mul_add() {
    let code = "exit 10 * (3 + 2);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 50);
}

#[test]
fn vm_grouped_sub_div() {
    let code = "exit (2 - 10) / 3;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), -2);
}

#[test]
fn vm_grouped_div_sub() {
    let code = "exit 10 / (3 - 2);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 10);
}
