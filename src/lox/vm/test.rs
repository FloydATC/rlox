

use super::RuntimeError;
use crate::lox::Builder;
use super::VM;


fn compile_and_execute(code: &str) -> Result<i32, RuntimeError> {
    let builder = Builder::new();
    let reader = std::io::Cursor::new(code);
    match builder.compile(reader) {
        Ok(bytecode) => return VM::new().execute(&bytecode),
        Err(error) => panic!("Compile failed: {}", error),
    }
}


#[test]
fn vm_new() {
    let _vm = VM::new();
}

#[test]
fn vm_emptystring() {
    let code = "";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
}

// 'exit' statement
#[test]
fn vm_exit_null() {
    let code = "exit;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_exit_zero() {
    let code = "exit 0;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_exit_one() {
    let code = "exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

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

// Global vars
#[test]
fn vm_global_undefined() {
    let code = "var a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_global_defined() {
    let code = "var a=1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_global_0() {
    let code = "var a; exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_global_1() {
    let code = "var a=1; exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_global_2() {
    let code = "var a=1; a=a*2; exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_global_3() {
    let code = "var a=1; var b=4; a=a*2; exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_global_4() {
    let code = "var a=1; var b=4; a=a*2; exit b;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

#[test]
fn vm_global_5() {
    let code = "var a=1; var b=4; b=b*2; exit b;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 8);
}

#[test]
#[should_panic]
fn vm_global_redefine() {
    let code = "var a=1; var a=2;";
    let _res = compile_and_execute(code);
}

// Local vars shadowing global ones
#[test]
fn vm_local_shadow_global_1() {
    let code = "var a=1; var b=2; { var a=3; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_local_shadow_global_2() {
    let code = "var a=1; var b=2; { var b=4; exit b; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

#[test]
fn vm_local_shadow_global_3() {
    let code = "var a=1; var b=2; { var a=3; var b=4; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_local_shadow_global_4() {
    let code = "var a=1; var b=2; { var a=3; var b=4; exit b; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

// Re-use of local var names in different scopes
#[test]
fn vm_reuse_local_1() {
    let code = "{ var a=1; } { var a=4; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

#[test]
fn vm_reuse_local_2() {
    let code = "{ var a=1; } { var b=1; var a=4; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

#[test]
fn vm_reuse_local_3() {
    let code = "{ var a=1; var b=1; } { var b=2; var a=4; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

#[test]
fn vm_reuse_local_4() {
    let code = "{ var a=1; var b=1; } { var a=4; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

// Disallow redefine in same scope
#[test]
#[should_panic]
fn vm_redefine_local() {
    let code = "{ var a=1; var a=2; }";
    let _res = compile_and_execute(code);
}

// Disallow self definition
#[test]
#[should_panic]
fn vm_local_self_define() {
    let code = "{ var a=a; }";
    let _res = compile_and_execute(code);
}


// Local var inaccessible in global scope
#[test]
#[should_panic]
fn vm_no_local_in_global_1() {
    let code = "{ var a=1; a=a*2; } exit a;";
    let _res = compile_and_execute(code);
}

// Local var inaccessible in different scope
#[test]
#[should_panic]
fn vm_no_local_in_other_local_1() {
    let code = "{ var a=1; a=a*2; } { exit a; }";
    let _res = compile_and_execute(code);
}


// Local vars shadowing local ones
#[test]
fn vm_local_shadow_local_1() {
    let code = "{ var a=1; var b=2; { var a=3; exit a; } }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_local_shadow_local_2() {
    let code = "{ var a=1; var b=2; { var b=4; exit b; } }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

#[test]
fn vm_local_shadow_local_3() {
    let code = "{ var a=1; var b=2; { var a=3; var b=4; exit a; } }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_local_shadow_local_4() {
    let code = "{ var a=1; var b=2; { var a=3; var b=4; exit b; } }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

// 'if' statement
#[test]
fn vm_if_true() {
    let code = "if (true) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_if_false() {
    let code = "if (false) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_if_true_else() {
    let code = "if (true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_if_false_else() {
    let code = "if (false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

// 'then' and 'else' blocks are different scopes if braced
#[test]
fn vm_if_scopes_1() {
    let code = "if (true) { var a=1; exit a; } else { var a=2; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
#[should_panic]
fn vm_if_scopes_2() {
    let code = "if (true) { var a=1; exit a; } else { exit a; }";
    let _res = compile_and_execute(code);
}

#[test]
fn vm_if_scopes_3() {
    let code = "if (false) { var a=1; exit a; } else { var a=2; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
#[should_panic]
fn vm_if_scopes_4() {
    let code = "if (false) { var a=1; exit a; } else { exit a; }";
    let _res = compile_and_execute(code);
}

// 'then' and 'else' blocks are same scope if not braced
#[test]
fn vm_if_noscopes_1() {
    let code = "var a; if (true) a=1; else a=2; exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_if_noscopes_2() {
    let code = "var a; if (false) a=1; else a=2; exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
#[should_panic]
fn vm_if_noscopes_3() {
    let code = "var a; if (true) var a=1; else a=2; exit a;";
    let _res = compile_and_execute(code);
}

#[test]
#[should_panic]
fn vm_if_noscopes_4() {
    let code = "var a; if (true) a=1; else var a=2; exit a;";
    let _res = compile_and_execute(code);
}

// boolean &&
#[test]
fn vm_boolean_false_and_false() {
    let code = "if (false && false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_boolean_false_and_true() {
    let code = "if (false && true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_boolean_true_and_false() {
    let code = "if (true && false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_boolean_true_and_true() {
    let code = "if (true && true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// boolean ||
#[test]
fn vm_boolean_false_or_false() {
    let code = "if (false || false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_boolean_false_or_true() {
    let code = "if (false || true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_boolean_true_or_false() {
    let code = "if (true || false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_boolean_true_or_true() {
    let code = "if (true || true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// Equality
#[test]
fn vm_equal_null() {
    let code = "if (null == null) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_not_equal_null() {
    let code = "if (null != null) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_equal_true() {
    let code = "if (true == true) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_not_equal_true() {
    let code = "if (true != true) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_equal_false() {
    let code = "if (false == false) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_not_equal_false() {
    let code = "if (false != false) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_equal_number() {
    let code = "if (123 == 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_not_equal_number() {
    let code = "if (123 != 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_equal_string() {
    let code = "if ('foo' == 'foo') { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_not_equal_string() {
    let code = "if ('foo' != 'foo') { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_equal_difftypes() {
    let code = "if ('123' == 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_not_equal_difftypes() {
    let code = "if ('123' != 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// Single or double quotes behave the same
#[test]
fn vm_quotes() {
    let code = "if ('123\r\n' == \"123\r\n\") { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// Less
#[test]
fn vm_less_1() {
    let code = "if (123 < 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_less_2() {
    let code = "if (123 < 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_less_3() {
    let code = "if (234 < 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_less_4() {
    let code = "if (234 < 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

// Less or equal
#[test]
fn vm_less_or_equal_1() {
    let code = "if (123 <= 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_less_or_equal_2() {
    let code = "if (123 <= 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_less_or_equal_3() {
    let code = "if (234 <= 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_less_or_equal_4() {
    let code = "if (234 <= 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// Greater
#[test]
fn vm_greater_1() {
    let code = "if (123 > 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_greater_2() {
    let code = "if (123 > 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_greater_3() {
    let code = "if (234 > 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_greater_4() {
    let code = "if (234 > 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

// Greater or equal
#[test]
fn vm_greater_or_equal_1() {
    let code = "if (123 >= 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_greater_or_equal_2() {
    let code = "if (123 >= 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_greater_or_equal_3() {
    let code = "if (234 >= 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_greater_or_equal_4() {
    let code = "if (234 >= 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// 'while' loops with 'break'/'continue'
#[test]
fn vm_while_naked() {
    let code = "var i=0; while (i<5) i=i+1; exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 5);
}

#[test]
fn vm_while_scoped() {
    let code = "var i=0; while (i<5) { i=i+1; } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 5);
}

#[test]
fn vm_while_scoped_var_before() {
    let code = "var i=0; while (i<5) { var j=10; i=i+1; } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 5);
}

#[test]
fn vm_while_scoped_var_after() {
    let code = "var i=0; while (i<5) { i=i+1; var j=10; } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 5);
}

#[test]
fn vm_while_immediate_break() {
    let code = "var i=0; while (i<5) { i=i+1; break; } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_while_nested_break() {
    let code = "var i=0; while (i<5) { i=i+1; { break; } } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_while_if_break() {
    let code = "var i=0; while (i<5) { i=i+1; if (i==3) break; } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_while_if_nested_break() {
    let code = "var i=0; while (i<5) { i=i+1; if (i==3) { break; } } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_while_nested_if_nested_break() {
    let code = "var i=0; while (i<5) { i=i+1; { if (i==3) { break; } } } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_while_if_continue() {
    let code = "var a=0; var i=0; while (i<10) { i=i+1; if (i>4) continue; a=a+2; } exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 8);
}

#[test]
fn vm_while_if_nested_continue() {
    let code = "var a=0; var i=0; while (i<10) { i=i+1; if (i>4) { continue; } a=a+2; } exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 8);
}

#[test]
fn vm_while_nested_if_nested_continue() {
    let code = "var a=0; var i=0; while (i<10) { i=i+1; { if (i>4) { continue; } } a=a+2; } exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 8);
}

// Functions
#[test]
fn vm_fun_empty() {
    let code = "fun f() {} exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_empty_with_var() {
    let code = "fun f() { var a; } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_empty_with_var_defined_1() {
    let code = "fun f() { var a=123; } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_empty_with_var_defined_2() {
    let code = "var a=1; fun f() { var a=123; } exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_args_1() {
    let code = "fun f(a) { } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_args_2() {
    let code = "fun f(a,b) { } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_args_3() {
    let code = "fun f(a,b,c) { } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_args_4() {
    let code = "fun f(a,b,c) { exit a+b+c; } f(1,2,4);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn vm_fun_args_5() {
    let code = "var a=10; fun f(a,b,c) { exit a+b+c; } f(1,2,4);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn vm_fun_args_6() {
    let code = "fun f(a,b,c) { exit a+b+c; } var a=10; f(1,2,4);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

// Return values
#[test]
fn vm_fun_return_implicit() {
    let code = "fun f(a,b,c) { var t=a+b+c; } exit f(1,2,4);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_fun_return_null() {
    let code = "fun f(a,b,c) { var t=a+b+c; return; } exit f(1,2,4);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_fun_return_value() {
    let code = "fun f(a,b,c) { var t=a+b+c; return t; } exit f(1,2,4);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

// Closures
#[test]
fn vm_closure_getupvalue_1() {
    let code = "fun mk() { var a = 123; fun c() { return a; } return c; } var c=mk(); exit c();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_closure_getupvalue_2() {
    let code = "fun mk(v) { fun c() { return v; } return c; } var a = mk(1); var b = mk(2); exit a();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_closure_getupvalue_3() {
    let code = "fun mk(v) { fun c() { return v; } return c; } var a = mk(1); var b = mk(2); exit b();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_closure_setupvalue_1() {
    let code = "fun a() { var x = 123; fun b() { x = 234; } b(); exit x; } a();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 234);
}

#[test]
fn vm_closure_setupvalue_2() {
    let code = "fun a() { var x = 123; fun b() { x = 234; x=x*2; } b(); exit x; } a();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 468);
}

#[test]
fn vm_closure_setupvalue_3() {
    let code = "fun c(y) { exit y; } fun a() { var x = 123; fun b() { x = 234; x=x*2; c(x); } b(); } a();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 468);
}

// Classes
#[test]
fn vm_class_empty() {
    let code = "class c {} exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_class_setproperty() {
    let code = "class cx {} var ix=cx(); ix.field=123; exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_class_getproperty() {
    let code = "class cx {} var ix=cx(); ix.field=123; exit ix.field;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_class_method_no_args_1() {
    let code = "class cx { m1() {} } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_class_method_no_args_2() {
    let code = "class cx { m1() {} m2() {} } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_class_method_no_args_3() {
    let code = "class cx { m1() {} m2() {} m3() {} } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_class_call_method_no_args() {
    let code = "class cx { m1() { exit 1; } } var ix=cx(); ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_class_call_method_with_args_1() {
    let code = "class cx { m1(rc) { exit rc; } } var ix=cx(); ix.m1(123);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_class_call_method_with_args_2() {
    let code = "class cx { m1(rc,x) { exit rc; } } var ix=cx(); ix.m1(123);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_err(), true);
}

#[test]
fn vm_class_call_method_with_args_3() {
    let code = "class cx { m1(rc) { exit rc; } } var ix=cx(); ix.m1(123,234);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_err(), true);
}

#[test]
#[should_panic]
fn vm_class_method_declare_this() {
    let code = "class cx { m1() { var this; } }";
    let _res = compile_and_execute(code);
}

#[test]
#[should_panic]
fn vm_class_method_define_this() {
    let code = "class cx { m1() { var this=123; } }";
    let _res = compile_and_execute(code);
}

#[test]
#[should_panic]
fn vm_function_declare_this() {
    let code = "fun f1() { var this; }";
    let _res = compile_and_execute(code);
}

#[test]
#[should_panic]
fn vm_function_define_this() {
    let code = "fun f1() { var this=123; }";
    let _res = compile_and_execute(code);
}

#[test]
#[should_panic]
fn vm_function_copy_this() {
    let code = "fun f1() { var t=this; }";
    let _res = compile_and_execute(code);
}

#[test]
#[should_panic]
fn vm_function_return_this() {
    let code = "fun f1() { return this; }";
    let _res = compile_and_execute(code);
}

#[test]
#[should_panic]
fn vm_root_declare_this() {
    let code = "var this;";
    let _res = compile_and_execute(code);
}

#[test]
#[should_panic]
fn vm_root_define_this() {
    let code = "var this=123;";
    let _res = compile_and_execute(code);
}

#[test]
#[should_panic]
fn vm_root_get_this() {
    let code = "exit this;";
    let _res = compile_and_execute(code);
}

#[test]
#[should_panic]
fn vm_root_copy_this() {
    let code = "var t=this;";
    let _res = compile_and_execute(code);
}

#[test]
#[should_panic]
fn vm_root_exit_this() {
    let code = "exit this;";
    let _res = compile_and_execute(code);
}

#[test]
fn vm_class_method_return_this_v1() {
    let code = "class cx { m1() { this.v1=123; this.v2=234; return this.v1; } } var ix=cx(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_class_method_return_this_v2() {
    let code = "class cx { m1() { this.v1=123; this.v2=234; return this.v2; } } var ix=cx(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 234);
}

#[test]
fn vm_class_method_return_nested_this_v1() {
    let code = "class cx { m1() { this.v1=123; this.v2=234; fun f1() { return this.v1; } return f1(); } } var ix=cx(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_class_method_return_nested_this_v2() {
    let code = "class cx { m1() { this.v1=123; this.v2=234; fun f1() { return this.v2; } return f1(); } } var ix=cx(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 234);
}

#[test]
fn vm_class_method_return_double_nested_this_v1() {
    let code = "class cx { m1() { this.v1=123; this.v2=234; fun f1() { fun f2() { return this.v1; } return f2(); } return f1(); } } var ix=cx(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_class_method_return_double_nested_this_v2() {
    let code = "class cx { m1() { this.v1=123; this.v2=234; fun f1() { fun f2() { return this.v2; } return f2(); } return f1(); } } var ix=cx(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 234);
}

#[test]
fn vm_class_instance_state() {
    let code = "class cx { set(v) { this.v=v; } dbl() { this.v=this.v*2; } get() { return this.v; } } var ix=cx(); ix.set(123); ix.dbl(); exit ix.get();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 246);
}

#[test]
fn vm_nested_classes_1() {
    let code = "class c1 { m1() { class c2 { m1() { return 234; } } return 123; } } var ix=c1(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_nested_classes_2() {
    let code = "class c1 { m1() { class c2 { m1() { return 234; } } var ix=c2(); return ix.m1(); } } var ix=c1(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 234);
}

#[test]
fn vm_instance_init_no_args_expected() {
    let code = "class cx { init() { this.v1=123; } m1() { return this.v1; } } var ix=cx(1); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_err(), true);
}

#[test]
fn vm_instance_init_args_missing() {
    let code = "class cx { init(v) { this.v1=v; } m1() { return this.v1; } } var ix=cx(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_err(), true);
}

#[test]
fn vm_instance_init_no_args_ok() {
    let code = "class cx { init() { this.v1=123; } m1() { return this.v1; } } var ix=cx(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_instance_init_args_one_ok() {
    let code = "class cx { init(v) { this.v1=v; } m1() { return this.v1; } } var ix=cx(123); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_instance_init_args_two_ok_1() {
    let code = "class cx { init(v1,v2) { this.v1=v1; } m1() { return this.v1; } } var ix=cx(123,234); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_instance_init_args_two_ok_2() {
    let code = "class cx { init(v1,v2) { this.v1=v2; } m1() { return this.v1; } } var ix=cx(123,234); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 234);
}

#[test]
fn vm_instance_init_return() {
    let code = "class cx { init() { this.v1=123; return; } m1() { return this.v1; } } var ix=cx(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
#[should_panic]
fn vm_instance_init_return_value() {
    let code = "class cx { init() { this.v1=123; return 234; } m1() { return this.v1; } } var ix=cx(); exit ix.m1();";
    let _res = compile_and_execute(code);
}

#[test]
fn vm_class_inherit_syntax_ok() {
    let code = "class c1 {} class c2 of c1 {} exit 123;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_class_inherit_method() {
    let code = "class c1 { m1() { return 123; } } class c2 of c1 {} var ix=c2(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_class_inherit_from_nonclass() {
    let code = "var v1=123; class c2 of v1 {}";
    let res = compile_and_execute(code);
    assert_eq!(res.is_err(), true);
}

#[test]
fn vm_class_inherit_wrong_way() {
    let code = "class c1 {} class c2 of c1 { m1() { return 123; } } var ix=c1(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_err(), true);
}

#[test]
fn vm_class_overload_child() {
    let code = "class c1 { m1() { return 123; } } class c2 of c1 { m1() { return 234; } } var ix=c2(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 234);
}

#[test]
fn vm_class_overload_parent() {
    let code = "class c1 { m1() { return 123; } } class c2 of c1 { m1() { return 234; } } var ix=c1(); exit ix.m1();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_class_super() {
    let code = "class c1 { m1() { return 123; } } class c2 of c1 { m1() { return 234; } m2() { return super.m1(); } } var ix=c2(); exit ix.m2();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

