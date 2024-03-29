

use crate::lox::common::Value;


use super::Stack;


#[test]
fn stack_new() {
    let _stack = Stack::<Value>::new();
}

#[test]
fn stack_push_one() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    stack.push(value1);
}

#[test]
fn stack_push_two() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    stack.push(value1);
    stack.push(value2);
}

#[test]
fn stack_size_zero() {
    let stack = Stack::<Value>::new();
    let size = stack.size();
    assert_eq!(size, 0);
}

#[test]
fn stack_size_one() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    stack.push(value1);
    let size = stack.size();
    assert_eq!(size, 1);
}

#[test]
fn stack_size_two() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    stack.push(value1);
    stack.push(value2);
    let size = stack.size();
    assert_eq!(size, 2);
}

#[test]
fn stack_pop_one() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    stack.push(value1.clone());
    stack.push(value2.clone());
    let value = stack.pop();
    let size = stack.size();
    assert_eq!(size, 1);
    assert_eq!(value, value2);
}

#[test]
fn stack_pop_two() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    stack.push(value1.clone());
    stack.push(value2.clone());
    let _discard = stack.pop();
    let value = stack.pop();
    let size = stack.size();
    assert_eq!(size, 0);
    assert_eq!(value, value1);
}

#[test]
fn stack_peek_one() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    stack.push(value1.clone());
    stack.push(value2.clone());
    let value = stack.peek(0);
    assert_eq!(value, &value2);
}

#[test]
fn stack_peek_two() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    stack.push(value1.clone());
    stack.push(value2.clone());
    let value = stack.peek(1);
    assert_eq!(value, &value1);
}

#[test]
fn stack_poke_one() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    stack.push(value1.clone());
    stack.push(value2.clone());

    let size = stack.size();
    assert_eq!(size, 2);
    stack.poke(Value::string("BAR"),0);
    let value = stack.peek(0);
    assert_eq!(value, &Value::string("BAR"));
    let value = stack.peek(1);
    assert_eq!(value, &Value::string("foo"));
}

#[test]
fn stack_poke_two() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    stack.push(value1.clone());
    stack.push(value2.clone());
    stack.poke(Value::string("FOO"),1);

    let size = stack.size();
    assert_eq!(size, 2);
    let value = stack.peek(0);
    assert_eq!(value, &Value::string("bar"));
    let value = stack.peek(1);
    assert_eq!(value, &Value::string("FOO"));
}




#[test]
fn stack_debug() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    let expect = format!("\n  0x0000 {}\n  0x0001 {}\n", value1, value2);
    stack.push(value1);
    stack.push(value2);
    let result = format!("{:?}", stack);
    assert_eq!(result, expect);
}


#[test]
fn stack_as_slice() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    stack.push(value1);
    stack.push(value2);
    assert_eq!(stack.as_slice(), &[Value::string("foo"), Value::string("bar")]);
}

#[test]
fn stack_len() {
    let mut stack = Stack::<Value>::new();
    let value1 = Value::string("foo");
    let value2 = Value::string("bar");
    stack.push(value1);
    stack.push(value2);
    assert_eq!(stack.len(), 2);
}

#[test]
fn stack_truncate() {
    let mut stack = Stack::<Value>::new();
    for i in 1..=10 { stack.push(Value::Number(i as f64)); }
    assert_eq!(stack.len(), 10);
    stack.truncate(4);
    let result = format!("{:?}", stack);
    assert_eq!(result.as_str(), "\n  0x0000 1\n  0x0001 2\n  0x0002 3\n  0x0003 4\n");
}

