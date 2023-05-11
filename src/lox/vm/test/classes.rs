

use super::compile_and_execute;


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

