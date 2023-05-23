

use super::Value;
use super::Function;
use super::FunctionKind;
use crate::lox::vm::Class;
use super::Closure;

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
    assert_eq!(value.as_string().as_str(), 	"");
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
    assert_eq!(value.as_string().as_str(), 	"foo");
}

#[test]
fn value_function() {
    let f = Function::new("", FunctionKind::Script, None);
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
    let f = Function::new("", FunctionKind::Script, None);
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

#[test]
fn get_from_ascii_string() {
    let s = Value::string("abc");
    assert_eq!(s.can_get(), true);
    assert_eq!(s.get(&Value::number(-1.0)), None);
    assert_eq!(s.get(&Value::number(0.0)), Some(Value::string("a")));
    assert_eq!(s.get(&Value::number(1.0)), Some(Value::string("b")));
    assert_eq!(s.get(&Value::number(2.0)), Some(Value::string("c")));
    assert_eq!(s.get(&Value::number(3.0)), None);
}

#[test]
fn get_from_utf8_string() {
    let s = Value::string("\u{0200}\u{0201}\u{0202}");
    assert_eq!(s.can_get(), true);
    assert_eq!(s.get(&Value::number(-1.0)), None);
    assert_eq!(s.get(&Value::number(0.0)), Some(Value::string("\u{0200}")));
    assert_eq!(s.get(&Value::number(1.0)), Some(Value::string("\u{0201}")));
    assert_eq!(s.get(&Value::number(2.0)), Some(Value::string("\u{0202}")));
    assert_eq!(s.get(&Value::number(3.0)), None);
}

#[test]
fn set_into_ascii_string() {
    let mut s = Value::string("abc");
    assert_eq!(s.can_set(), true);
    s.set(&Value::number(0.0), Value::string("\u{0200}")).expect("failed unexpectedly");
    s.set(&Value::number(1.0), Value::string("\u{0201}")).expect("failed unexpectedly");
    s.set(&Value::number(2.0), Value::string("\u{0202}")).expect("failed unexpectedly");
    assert_eq!(s.set(&Value::number(3.0), Value::string("X")).is_err(), true);
    assert_eq!(s, Value::string("\u{0200}\u{0201}\u{0202}"));
}

#[test]
fn set_into_utf8_string() {
    let mut s = Value::string("\u{0200}\u{0201}\u{0202}");
    assert_eq!(s.can_set(), true);
    s.set(&Value::number(0.0), Value::string("a")).expect("failed unexpectedly");
    s.set(&Value::number(1.0), Value::string("b")).expect("failed unexpectedly");
    s.set(&Value::number(2.0), Value::string("c")).expect("failed unexpectedly");
    assert_eq!(s.set(&Value::number(3.0), Value::string("X")).is_err(), true);
    assert_eq!(s, Value::string("abc"));
}

