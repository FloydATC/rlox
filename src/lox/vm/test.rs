

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

