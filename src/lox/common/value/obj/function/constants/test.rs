
use super::Constants;

use crate::lox::common::Value;


#[test]
fn constants_new() {
    let _constants = Constants::<Value>::new();
}


#[test]
fn constants_make_one() {
    let mut constants = Constants::<Value>::new();
    let value = Value::string("foo");
    let id = constants.make(value);
    assert_eq!(id, 0);
}

#[test]
fn constants_make_two() {
    let mut constants = Constants::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    let _id1 = constants.make(value1);
    let id2 = constants.make(value2);
    assert_eq!(id2, 1);
}

#[test]
fn constants_make_duplicate() {
    let mut constants = Constants::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("foo");
    let _id1 = constants.make(value1);
    let id2 = constants.make(value2);
    assert_eq!(id2, 0);
}

#[test]
fn constants_id_by_value_1() {
    let mut constants = Constants::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    let _id1 = constants.make(value1);
    let _id2 = constants.make(value2);
    let id1 = constants.id_by_value(&Value::string("foo"));
    assert_eq!(id1, Some(0));
}

#[test]
fn constants_id_by_value_2() {
    let mut constants = Constants::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    let _id1 = constants.make(value1);
    let _id2 = constants.make(value2);
    let id2 = constants.id_by_value(&Value::string("bar"));
    assert_eq!(id2, Some(1));
}

#[test]
fn constants_value_by_id_1() {
    let mut constants = Constants::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    let id1 = constants.make(value1);
    let _id2 = constants.make(value2);
    let value = constants.value_by_id(id1);
    assert_eq!(value, &Value::string("foo"));
}

#[test]
fn constants_value_by_id_2() {
    let mut constants = Constants::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    let _id1 = constants.make(value1);
    let id2 = constants.make(value2);
    let value = constants.value_by_id(id2);
    assert_eq!(value, &Value::string("bar"));
}

#[test]
fn constants_debug() {
    let mut constants = Constants::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    let expect = format!("\n  0x0000 {}\n  0x0001 {}\n", value1, value2);
    let _id1 = constants.make(value1);
    let _id2 = constants.make(value2);
    let result = format!("{:?}", constants);
    assert_eq!(result, expect);
}

