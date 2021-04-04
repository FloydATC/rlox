

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
fn vm_execute_emptystring() {
    let mut vm = VM::new();
    let _res = vm.compile("");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

// 'exit' statement
#[test]
fn vm_execute_exit_null() {
    let mut vm = VM::new();
    let _res = vm.compile("exit;");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_execute_exit_zero() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 0;");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_execute_exit_one() {
    let mut vm = VM::new();
    let _res = vm.compile("exit 1;");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

// 'if' statement
#[test]
fn vm_execute_if_true() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_execute_if_false() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false) { exit 1; }");
    let rc = vm.execute();
    assert_eq!(rc, 0);
}

#[test]
fn vm_execute_if_true_else() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_execute_if_false_else() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

// boolean &&
#[test]
fn vm_execute_boolean_false_and_false() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false && false) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
fn vm_execute_boolean_false_and_true() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false && true) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
fn vm_execute_boolean_true_and_false() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true && false) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
fn vm_execute_boolean_true_and_true() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true && true) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

// boolean ||
#[test]
fn vm_execute_boolean_false_or_false() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false || false) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 2);
}

#[test]
fn vm_execute_boolean_false_or_true() {
    let mut vm = VM::new();
    let _res = vm.compile("if (false || true) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_execute_boolean_true_or_false() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true || false) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

#[test]
fn vm_execute_boolean_true_or_true() {
    let mut vm = VM::new();
    let _res = vm.compile("if (true || true) { exit 1; } else { exit 2; }");
    let rc = vm.execute();
    assert_eq!(rc, 1);
}

