

use super::Value;
use crate::lox::function::Function;
use crate::lox::function_kind::FunctionKind;
use crate::lox::vm::Class;
use crate::lox::closure::Closure;

#[test]
fn value_null() {
    let value = Value::null();
    assert_eq!(value.is_null(), 	true);
    assert_eq!(value.is_boolean(), 	false);
    assert_eq!(value.is_number(), 	false);
    assert_eq!(value.is_string(), 	false);
    assert_eq!(value.is_function(), 	false);
    assert_eq!(value.is_class(), 	false);
    assert_eq!(value.is_closure(), 	false);

    assert_eq!(value.is_truthy(), 	false);
}

#[test]
fn value_boolean_1() {
    let value = Value::boolean(true);
    assert_eq!(value.is_null(), 	false);
    assert_eq!(value.is_boolean(), 	true);
    assert_eq!(value.is_number(), 	false);
    assert_eq!(value.is_string(), 	false);
    assert_eq!(value.is_function(), 	false);
    assert_eq!(value.is_class(), 	false);
    assert_eq!(value.is_closure(), 	false);

    assert_eq!(value.is_truthy(), 	true);
    assert_eq!(value.as_boolean(), 	true);
}

#[test]
fn value_boolean_2() {
    let value = Value::boolean(false);
    assert_eq!(value.is_null(), 	false);
    assert_eq!(value.is_boolean(), 	true);
    assert_eq!(value.is_number(), 	false);
    assert_eq!(value.is_string(), 	false);
    assert_eq!(value.is_function(), 	false);
    assert_eq!(value.is_class(), 	false);
    assert_eq!(value.is_closure(), 	false);

    assert_eq!(value.is_truthy(), 	false);
    assert_eq!(value.as_boolean(), 	false);
}

#[test]
fn value_number_1() {
    let value = Value::number(0.0);
    assert_eq!(value.is_null(), 	false);
    assert_eq!(value.is_boolean(), 	false);
    assert_eq!(value.is_number(), 	true);
    assert_eq!(value.is_string(), 	false);
    assert_eq!(value.is_function(), 	false);
    assert_eq!(value.is_class(), 	false);
    assert_eq!(value.is_closure(), 	false);

    assert_eq!(value.is_truthy(), 	false);
    assert_eq!(value.as_number(), 	0.0);
}

#[test]
fn value_number_2() {
    let value = Value::number(-1230.0);
    assert_eq!(value.is_null(), 	false);
    assert_eq!(value.is_boolean(), 	false);
    assert_eq!(value.is_number(), 	true);
    assert_eq!(value.is_string(), 	false);
    assert_eq!(value.is_function(), 	false);
    assert_eq!(value.is_class(), 	false);
    assert_eq!(value.is_closure(), 	false);

    assert_eq!(value.is_truthy(), 	true);
    assert_eq!(value.as_number(), 	-1230.0);
}

#[test]
fn value_number_3() {
    let value = Value::number(1230.0);
    assert_eq!(value.is_null(), 	false);
    assert_eq!(value.is_boolean(), 	false);
    assert_eq!(value.is_number(), 	true);
    assert_eq!(value.is_string(), 	false);
    assert_eq!(value.is_function(), 	false);
    assert_eq!(value.is_class(), 	false);
    assert_eq!(value.is_closure(), 	false);

    assert_eq!(value.is_truthy(), 	true);
    assert_eq!(value.as_number(), 	1230.0);
}

#[test]
fn value_string_1() {
    let value = Value::string("");
    assert_eq!(value.is_null(), 	false);
    assert_eq!(value.is_boolean(), 	false);
    assert_eq!(value.is_number(), 	false);
    assert_eq!(value.is_string(), 	true);
    assert_eq!(value.is_function(), 	false);
    assert_eq!(value.is_class(), 	false);
    assert_eq!(value.is_closure(), 	false);

    assert_eq!(value.is_truthy(), 	false);
    assert_eq!(value.as_string(), 	"");
}

#[test]
fn value_string_2() {
    let value = Value::string("foo");
    assert_eq!(value.is_null(), 	false);
    assert_eq!(value.is_boolean(), 	false);
    assert_eq!(value.is_number(), 	false);
    assert_eq!(value.is_string(), 	true);
    assert_eq!(value.is_function(), 	false);
    assert_eq!(value.is_class(), 	false);
    assert_eq!(value.is_closure(), 	false);

    assert_eq!(value.is_truthy(), 	true);
    assert_eq!(value.as_string(), 	"foo");
}

#[test]
fn value_function() {
    let f = Function::new("", FunctionKind::Script);
    let value = Value::function(f);
    assert_eq!(value.is_null(), 	false);
    assert_eq!(value.is_boolean(), 	false);
    assert_eq!(value.is_number(), 	false);
    assert_eq!(value.is_string(), 	false);
    assert_eq!(value.is_function(), 	true);
    assert_eq!(value.is_class(), 	false);
    assert_eq!(value.is_closure(), 	false);

    assert_eq!(value.is_truthy(), 	true);
}

#[test]
fn value_class() {
    let c = Class::new("");
    let value = Value::class(c);
    assert_eq!(value.is_null(), 	false);
    assert_eq!(value.is_boolean(), 	false);
    assert_eq!(value.is_number(), 	false);
    assert_eq!(value.is_string(), 	false);
    assert_eq!(value.is_function(), 	false);
    assert_eq!(value.is_class(), 	true);
    assert_eq!(value.is_closure(), 	false);

    assert_eq!(value.is_truthy(), 	true);
}

#[test]
fn value_closure() {
    let f = Function::new("", FunctionKind::Script);
    let f_value = Value::function(f);
    let c = Closure::new(f_value);
    let value = Value::closure(c);
    assert_eq!(value.is_null(), 	false);
    assert_eq!(value.is_boolean(), 	false);
    assert_eq!(value.is_number(), 	false);
    assert_eq!(value.is_string(), 	false);
    assert_eq!(value.is_function(), 	false);
    assert_eq!(value.is_class(), 	false);
    assert_eq!(value.is_closure(), 	true);

    assert_eq!(value.is_truthy(), 	true);
}


