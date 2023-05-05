

use super::VM;


#[test]
fn vm_new() {
    let _vm = VM::new();
}

#[test]
fn vm_compile_emptystring() {
    let mut vm = VM::new();
    let res = vm.compile("");
    assert_eq!(res, Ok(()));
}

#[test]
fn vm_emptystring() {
    let mut vm = VM::new();
    let _res = vm.compile("");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

// 'exit' statement
#[test]
fn vm_exit_null() {
    let mut vm = VM::new();
    let _res = vm.compile("exit;");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_exit_zero() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 0;");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_exit_one() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

// Arithmetics
#[test]
fn vm_plus() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 10 + 3;");
    let rc = vm.execute();
    assert_eq!(rc, 13);
}

#[test]
fn vm_minus() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 10 - 3;");
    let rc = vm.execute();
    assert_eq!(rc, 7);
}

#[test]
fn vm_multiply() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 10 * 3;");
    let rc = vm.execute();
    assert_eq!(rc, 30);
}

#[test]
fn vm_divide() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 10 / 3;");
    let rc = vm.execute();
    assert_eq!(rc, 3);
}

#[test]
fn vm_modulo() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 10 % 3;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

// Simple operator precedence
#[test]
fn vm_add_mul() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 2 + 10 * 3;");
    let rc = vm.execute();
    assert_eq!(rc, 32);
}

#[test]
fn vm_mul_add() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 10 * 3 + 2;");
    let rc = vm.execute();
    assert_eq!(rc, 32);
}

#[test]
fn vm_sub_div() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 2 - 10 / 3;");
    let rc = vm.execute();
    assert_eq!(rc, -1);
}

#[test]
fn vm_div_sub() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 10 / 3 - 2;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

// Grouping
#[test]
fn vm_grouped_add_mul() {
    let mut vm = VM::new();
    let _res = vm.compile("exit (2 + 10) * 3;");
    let rc = vm.execute();
    assert_eq!(rc, 36);
}

#[test]
fn vm_grouped_mul_add() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 10 * (3 + 2);");
    let rc = vm.execute();
    assert_eq!(rc, 50);
}

#[test]
fn vm_grouped_sub_div() {
    let mut vm = VM::new();
    let _res = vm.compile("exit (2 - 10) / 3;");
    let rc = vm.execute();
    assert_eq!(rc, -2);
}

#[test]
fn vm_grouped_div_sub() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 10 / (3 - 2);");
    let rc = vm.execute();
    assert_eq!(rc, 10);
}

// Global vars
#[test]
fn vm_global_undefined() {
    let mut vm = VM::new();
    let _res = vm.compile("var a;");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_global_defined() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=1;");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_global_0() {
    let mut vm = VM::new();
    let _res = vm.compile("var a; exit a;");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_global_1() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=1; exit a;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_global_2() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=1; a=a*2; exit a;");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
fn vm_global_3() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=1; var b=4; a=a*2; exit a;");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
fn vm_global_4() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=1; var b=4; a=a*2; exit b;");
    let rc = vm.execute();
    assert_eq!(rc, 4);
}

#[test]
fn vm_global_5() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=1; var b=4; b=b*2; exit b;");
    let rc = vm.execute();
    assert_eq!(rc, 8);
}

#[test]
#[should_panic]
fn vm_global_redefine() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=1; var a=2;");
}

// Local vars shadowing global ones
#[test]
fn vm_local_shadow_global_1() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=1; var b=2; { var a=3; exit a; }");
    let rc = vm.execute();
    assert_eq!(rc, 3);
}

#[test]
fn vm_local_shadow_global_2() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=1; var b=2; { var b=4; exit b; }");
    let rc = vm.execute();
    assert_eq!(rc, 4);
}

#[test]
fn vm_local_shadow_global_3() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=1; var b=2; { var a=3; var b=4; exit a; }");
    let rc = vm.execute();
    assert_eq!(rc, 3);
}

#[test]
fn vm_local_shadow_global_4() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=1; var b=2; { var a=3; var b=4; exit b; }");
    let rc = vm.execute();
    assert_eq!(rc, 4);
}

// Re-use of local var names in different scopes
#[test]
fn vm_reuse_local_1() {
    let mut vm = VM::new();
    let _res = vm.compile("{ var a=1; } { var a=4; exit a; }");
    let rc = vm.execute();
    assert_eq!(rc, 4);
}

#[test]
fn vm_reuse_local_2() {
    let mut vm = VM::new();
    let _res = vm.compile("{ var a=1; } { var b=1; var a=4; exit a; }");
    let rc = vm.execute();
    assert_eq!(rc, 4);
}

#[test]
fn vm_reuse_local_3() {
    let mut vm = VM::new();
    let _res = vm.compile("{ var a=1; var b=1; } { var b=2; var a=4; exit a; }");
    let rc = vm.execute();
    assert_eq!(rc, 4);
}

#[test]
fn vm_reuse_local_4() {
    let mut vm = VM::new();
    let _res = vm.compile("{ var a=1; var b=1; } { var a=4; exit a; }");
    let rc = vm.execute();
    assert_eq!(rc, 4);
}

// Disallow redefine in same scope
#[test]
#[should_panic]
fn vm_redefine_local() {
    let mut vm = VM::new();
    let _res = vm.compile("{ var a=1; var a=2; }");
}

// Disallow self definition
#[test]
#[should_panic]
fn vm_local_self_define() {
    let mut vm = VM::new();
    let _res = vm.compile("{ var a=a; }");
}


// Local var inaccessible in global scope
#[test]
#[should_panic]
fn vm_no_local_in_global_1() {
    let mut vm = VM::new();
    let _res = vm.compile("{ var a=1; a=a*2; } exit a;");
    let _rc = vm.execute();
}

// Local var inaccessible in different scope
#[test]
#[should_panic]
fn vm_no_local_in_other_local_1() {
    let mut vm = VM::new();
    let _res = vm.compile("{ var a=1; a=a*2; } { exit a; }");
    let _rc = vm.execute();
}


// Local vars shadowing local ones
#[test]
fn vm_local_shadow_local_1() {
    let mut vm = VM::new();
    let _res = vm.compile("{ var a=1; var b=2; { var a=3; exit a; } }");
    let rc = vm.execute();
    assert_eq!(rc, 3);
}

#[test]
fn vm_local_shadow_local_2() {
    let mut vm = VM::new();
    let _res = vm.compile("{ var a=1; var b=2; { var b=4; exit b; } }");
    let rc = vm.execute();
    assert_eq!(rc, 4);
}

#[test]
fn vm_local_shadow_local_3() {
    let mut vm = VM::new();
    let _res = vm.compile("{ var a=1; var b=2; { var a=3; var b=4; exit a; } }");
    let rc = vm.execute();
    assert_eq!(rc, 3);
}

#[test]
fn vm_local_shadow_local_4() {
    let mut vm = VM::new();
    let _res = vm.compile("{ var a=1; var b=2; { var a=3; var b=4; exit b; } }");
    let rc = vm.execute();
    assert_eq!(rc, 4);
}

// 'if' statement
#[test]
fn vm_if_true() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_if_false() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_if_true_else() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_if_false_else() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

// 'then' and 'else' blocks are different scopes if braced
#[test]
fn vm_if_scopes_1() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true) { var a=1; exit a; } else { var a=2; exit a; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
#[should_panic]
fn vm_if_scopes_2() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true) { var a=1; exit a; } else { exit a; }");
    let _rc = vm.execute();
}

#[test]
fn vm_if_scopes_3() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false) { var a=1; exit a; } else { var a=2; exit a; }");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
#[should_panic]
fn vm_if_scopes_4() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false) { var a=1; exit a; } else { exit a; }");
    let _rc = vm.execute();
}

// 'then' and 'else' blocks are same scope if not braced
#[test]
fn vm_if_noscopes_1() {
    let mut vm = VM::new();
    let _res = vm.compile("var a; if (true) a=1; else a=2; exit a;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_if_noscopes_2() {
    let mut vm = VM::new();
    let _res = vm.compile("var a; if (false) a=1; else a=2; exit a;");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
#[should_panic]
fn vm_if_noscopes_3() {
    let mut vm = VM::new();
    let _res = vm.compile("var a; if (true) var a=1; else a=2; exit a;");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_if_noscopes_4() {
    let mut vm = VM::new();
    let _res = vm.compile("var a; if (true) a=1; else var a=2; exit a;");
    let _rc = vm.execute();
}

// boolean &&
#[test]
fn vm_boolean_false_and_false() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false && false) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
fn vm_boolean_false_and_true() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false && true) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
fn vm_boolean_true_and_false() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true && false) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
fn vm_boolean_true_and_true() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true && true) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

// boolean ||
#[test]
fn vm_boolean_false_or_false() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false || false) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
fn vm_boolean_false_or_true() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false || true) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_boolean_true_or_false() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true || false) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_boolean_true_or_true() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true || true) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

// Equality
#[test]
fn vm_equal_null() {
    let mut vm = VM::new();
    let _res = vm.compile("if (null == null) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_not_equal_null() {
    let mut vm = VM::new();
    let _res = vm.compile("if (null != null) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_equal_true() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true == true) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_not_equal_true() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true != true) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_equal_false() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false == false) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_not_equal_false() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false != false) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_equal_number() {
    let mut vm = VM::new();
    let _res = vm.compile("if (123 == 123) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_not_equal_number() {
    let mut vm = VM::new();
    let _res = vm.compile("if (123 != 123) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_equal_string() {
    let mut vm = VM::new();
    let _res = vm.compile("if ('foo' == 'foo') { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_not_equal_string() {
    let mut vm = VM::new();
    let _res = vm.compile("if ('foo' != 'foo') { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_equal_difftypes() {
    let mut vm = VM::new();
    let _res = vm.compile("if ('123' == 123) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_not_equal_difftypes() {
    let mut vm = VM::new();
    let _res = vm.compile("if ('123' != 123) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

// Single or double quotes behave the same
#[test]
fn vm_quotes() {
    let mut vm = VM::new();
    let _res = vm.compile("if ('123\r\n' == \"123\r\n\") { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

// Less
#[test]
fn vm_less_1() {
    let mut vm = VM::new();
    let _res = vm.compile("if (123 < 123) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_less_2() {
    let mut vm = VM::new();
    let _res = vm.compile("if (123 < 234) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_less_3() {
    let mut vm = VM::new();
    let _res = vm.compile("if (234 < 123) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_less_4() {
    let mut vm = VM::new();
    let _res = vm.compile("if (234 < 234) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

// Less or equal
#[test]
fn vm_less_or_equal_1() {
    let mut vm = VM::new();
    let _res = vm.compile("if (123 <= 123) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_less_or_equal_2() {
    let mut vm = VM::new();
    let _res = vm.compile("if (123 <= 234) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_less_or_equal_3() {
    let mut vm = VM::new();
    let _res = vm.compile("if (234 <= 123) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_less_or_equal_4() {
    let mut vm = VM::new();
    let _res = vm.compile("if (234 <= 234) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

// Greater
#[test]
fn vm_greater_1() {
    let mut vm = VM::new();
    let _res = vm.compile("if (123 > 123) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_greater_2() {
    let mut vm = VM::new();
    let _res = vm.compile("if (123 > 234) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_greater_3() {
    let mut vm = VM::new();
    let _res = vm.compile("if (234 > 123) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_greater_4() {
    let mut vm = VM::new();
    let _res = vm.compile("if (234 > 234) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

// Greater or equal
#[test]
fn vm_greater_or_equal_1() {
    let mut vm = VM::new();
    let _res = vm.compile("if (123 >= 123) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_greater_or_equal_2() {
    let mut vm = VM::new();
    let _res = vm.compile("if (123 >= 234) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_greater_or_equal_3() {
    let mut vm = VM::new();
    let _res = vm.compile("if (234 >= 123) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_greater_or_equal_4() {
    let mut vm = VM::new();
    let _res = vm.compile("if (234 >= 234) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

// 'while' loops with 'break'/'continue'
#[test]
fn vm_while_naked() {
    let mut vm = VM::new();
    let _res = vm.compile("var i=0; while (i<5) i=i+1; exit i;");
    let rc = vm.execute();
    assert_eq!(rc, 5);
}

#[test]
fn vm_while_scoped() {
    let mut vm = VM::new();
    let _res = vm.compile("var i=0; while (i<5) { i=i+1; } exit i;");
    let rc = vm.execute();
    assert_eq!(rc, 5);
}

#[test]
fn vm_while_scoped_var_before() {
    let mut vm = VM::new();
    let _res = vm.compile("var i=0; while (i<5) { var j=10; i=i+1; } exit i;");
    let rc = vm.execute();
    assert_eq!(rc, 5);
}

#[test]
fn vm_while_scoped_var_after() {
    let mut vm = VM::new();
    let _res = vm.compile("var i=0; while (i<5) { i=i+1; var j=10; } exit i;");
    let rc = vm.execute();
    assert_eq!(rc, 5);
}

#[test]
fn vm_while_immediate_break() {
    let mut vm = VM::new();
    let _res = vm.compile("var i=0; while (i<5) { i=i+1; break; } exit i;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_while_nested_break() {
    let mut vm = VM::new();
    let _res = vm.compile("var i=0; while (i<5) { i=i+1; { break; } } exit i;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_while_if_break() {
    let mut vm = VM::new();
    let _res = vm.compile("var i=0; while (i<5) { i=i+1; if (i==3) break; } exit i;");
    let rc = vm.execute();
    assert_eq!(rc, 3);
}

#[test]
fn vm_while_if_nested_break() {
    let mut vm = VM::new();
    let _res = vm.compile("var i=0; while (i<5) { i=i+1; if (i==3) { break; } } exit i;");
    let rc = vm.execute();
    assert_eq!(rc, 3);
}

#[test]
fn vm_while_nested_if_nested_break() {
    let mut vm = VM::new();
    let _res = vm.compile("var i=0; while (i<5) { i=i+1; { if (i==3) { break; } } } exit i;");
    let rc = vm.execute();
    assert_eq!(rc, 3);
}

#[test]
fn vm_while_if_continue() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=0; var i=0; while (i<10) { i=i+1; if (i>4) continue; a=a+2; } exit a;");
    let rc = vm.execute();
    assert_eq!(rc, 8);
}

#[test]
fn vm_while_if_nested_continue() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=0; var i=0; while (i<10) { i=i+1; if (i>4) { continue; } a=a+2; } exit a;");
    let rc = vm.execute();
    assert_eq!(rc, 8);
}

#[test]
fn vm_while_nested_if_nested_continue() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=0; var i=0; while (i<10) { i=i+1; { if (i>4) { continue; } } a=a+2; } exit a;");
    let rc = vm.execute();
    assert_eq!(rc, 8);
}

// Functions
#[test]
fn vm_fun_empty() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f() {} exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_fun_empty_with_var() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f() { var a; } exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_fun_empty_with_var_defined_1() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f() { var a=123; } exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_fun_empty_with_var_defined_2() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=1; fun f() { var a=123; } exit a;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_fun_args_1() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f(a) { } exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_fun_args_2() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f(a,b) { } exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_fun_args_3() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f(a,b,c) { } exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_fun_args_4() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f(a,b,c) { exit a+b+c; } f(1,2,4);");
    let rc = vm.execute();
    assert_eq!(rc, 7);
}

#[test]
fn vm_fun_args_5() {
    let mut vm = VM::new();
    let _res = vm.compile("var a=10; fun f(a,b,c) { exit a+b+c; } f(1,2,4);");
    let rc = vm.execute();
    assert_eq!(rc, 7);
}

#[test]
fn vm_fun_args_6() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f(a,b,c) { exit a+b+c; } var a=10; f(1,2,4);");
    let rc = vm.execute();
    assert_eq!(rc, 7);
}

// Return values
#[test]
fn vm_fun_return_implicit() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f(a,b,c) { var t=a+b+c; } exit f(1,2,4);");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_fun_return_null() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f(a,b,c) { var t=a+b+c; return; } exit f(1,2,4);");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_fun_return_value() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f(a,b,c) { var t=a+b+c; return t; } exit f(1,2,4);");
    let rc = vm.execute();
    assert_eq!(rc, 7);
}

// Closures
#[test]
fn vm_closure_getupvalue_1() {
    let mut vm = VM::new();
    let _res = vm.compile("fun mk() { var a = 123; fun c() { return a; } return c; } var c=mk(); exit c();");
    let rc = vm.execute();
    assert_eq!(rc, 123);
}

#[test]
fn vm_closure_getupvalue_2() {
    let mut vm = VM::new();
    let _res = vm.compile("fun mk(v) { fun c() { return v; } return c; } var a = mk(1); var b = mk(2); exit a();");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_closure_getupvalue_3() {
    let mut vm = VM::new();
    let _res = vm.compile("fun mk(v) { fun c() { return v; } return c; } var a = mk(1); var b = mk(2); exit b();");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
fn vm_closure_setupvalue_1() {
    let mut vm = VM::new();
    let _res = vm.compile("fun a() { var x = 123; fun b() { x = 234; } b(); exit x; } a();");
    let rc = vm.execute();
    assert_eq!(rc, 234);
}

#[test]
fn vm_closure_setupvalue_2() {
    let mut vm = VM::new();
    let _res = vm.compile("fun a() { var x = 123; fun b() { x = 234; x=x*2; } b(); exit x; } a();");
    let rc = vm.execute();
    assert_eq!(rc, 468);
}

#[test]
fn vm_closure_setupvalue_3() {
    let mut vm = VM::new();
    let _res = vm.compile("fun c(y) { exit y; } fun a() { var x = 123; fun b() { x = 234; x=x*2; c(x); } b(); } a();");
    let rc = vm.execute();
    assert_eq!(rc, 468);
}

// Classes
#[test]
fn vm_class_empty() {
    let mut vm = VM::new();
    let _res = vm.compile("class c {} exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_class_setproperty() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx {} var ix=cx(); ix.field=123; exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_class_getproperty() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx {} var ix=cx(); ix.field=123; exit ix.field;");
    let rc = vm.execute();
    assert_eq!(rc, 123);
}

#[test]
fn vm_class_method_no_args_1() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1() {} } exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_class_method_no_args_2() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1() {} m2() {} } exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_class_method_no_args_3() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1() {} m2() {} m3() {} } exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_class_call_method_no_args() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1() { exit 1; } } var ix=cx(); ix.m1();");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_class_call_method_with_args_1() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1(rc) { exit rc; } } var ix=cx(); ix.m1(123);");
    let rc = vm.execute();
    assert_eq!(rc, 123);
}

#[test]
#[should_panic]
fn vm_class_call_method_with_args_2() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1(rc,x) { exit rc; } } var ix=cx(); ix.m1(123);");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_class_call_method_with_args_3() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1(rc) { exit rc; } } var ix=cx(); ix.m1(123,234);");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_class_method_declare_this() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1() { var this; } }");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_class_method_define_this() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1() { var this=123; } }");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_function_declare_this() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f1() { var this; }");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_function_define_this() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f1() { var this=123; }");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_function_copy_this() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f1() { var t=this; }");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_function_return_this() {
    let mut vm = VM::new();
    let _res = vm.compile("fun f1() { return this; }");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_root_declare_this() {
    let mut vm = VM::new();
    let _res = vm.compile("var this;");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_root_define_this() {
    let mut vm = VM::new();
    let _res = vm.compile("var this=123;");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_root_get_this() {
    let mut vm = VM::new();
    let _res = vm.compile("exit this;");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_root_copy_this() {
    let mut vm = VM::new();
    let _res = vm.compile("var t=this;");
    let _rc = vm.execute();
}

#[test]
#[should_panic]
fn vm_root_exit_this() {
    let mut vm = VM::new();
    let _res = vm.compile("exit this;");
    let _rc = vm.execute();
}

#[test]
fn vm_class_method_return_this_v1() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1() { this.v1=123; this.v2=234; return this.v1; } } var ix=cx(); exit ix.m1();");
    let rc = vm.execute();
    assert_eq!(rc, 123);
}

#[test]
fn vm_class_method_return_this_v2() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1() { this.v1=123; this.v2=234; return this.v2; } } var ix=cx(); exit ix.m1();");
    let rc = vm.execute();
    assert_eq!(rc, 234);
}

#[test]
fn vm_class_method_return_nested_this_v1() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1() { this.v1=123; this.v2=234; fun f1() { return this.v1; } return f1(); } } var ix=cx(); exit ix.m1();");
    let rc = vm.execute();
    assert_eq!(rc, 123);
}

#[test]
fn vm_class_method_return_nested_this_v2() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1() { this.v1=123; this.v2=234; fun f1() { return this.v2; } return f1(); } } var ix=cx(); exit ix.m1();");
    let rc = vm.execute();
    assert_eq!(rc, 234);
}

#[test]
fn vm_class_method_return_double_nested_this_v1() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1() { this.v1=123; this.v2=234; fun f1() { fun f2() { return this.v1; } return f2(); } return f1(); } } var ix=cx(); exit ix.m1();");
    let rc = vm.execute();
    assert_eq!(rc, 123);
}

#[test]
fn vm_class_method_return_double_nested_this_v2() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { m1() { this.v1=123; this.v2=234; fun f1() { fun f2() { return this.v2; } return f2(); } return f1(); } } var ix=cx(); exit ix.m1();");
    let rc = vm.execute();
    assert_eq!(rc, 234);
}

#[test]
fn vm_class_instance_state() {
    let mut vm = VM::new();
    let _res = vm.compile("class cx { set(v) { this.v=v; } dbl() { this.v=this.v*2; } get() { return this.v; } } var ix=cx(); ix.set(123); ix.dbl(); exit ix.get();");
    let rc = vm.execute();
    assert_eq!(rc, 246);
}

#[test]
fn vm_nested_classes_1() {
    let mut vm = VM::new();
    let _res = vm.compile("class c1 { m1() { class c2 { m1() { return 234; } } return 123; } } var ix=c1(); exit ix.m1();");
    let rc = vm.execute();
    assert_eq!(rc, 123);
}

#[test]
fn vm_nested_classes_2() {
    let mut vm = VM::new();
    let _res = vm.compile("class c1 { m1() { class c2 { m1() { return 234; } } var ix=c2(); return ix.m1(); } } var ix=c1(); exit ix.m1();");
    let rc = vm.execute();
    assert_eq!(rc, 234);
}
