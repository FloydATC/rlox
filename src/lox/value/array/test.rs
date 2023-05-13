

use super::{Array, Value};


#[test]
fn new() {
    let _array: Array = Array::new();
}

#[test]
fn display() {
    let array: Array = Array::new();
    let string = format!("{}", array);
    assert_eq!(string.as_str(), "[]");
}

#[test]
fn len_zero() {
    let array: Array = Array::new();
    let len: usize = array.len();
    assert_eq!(len, 0);
}

#[test]
fn push() {
    let mut array: Array = Array::new();
    array.push(Value::Number(123.0));
    array.push(Value::Number(456.0));
    array.push(Value::Number(789.0));
    let len: usize = array.len();
    assert_eq!(len, 3);
    let string = format!("{}", array);
    assert_eq!(string.as_str(), "[123, 456, 789]");
}

#[test]
fn pop() {
    let mut array: Array = Array::new();
    array.push(Value::Number(123.0));
    array.push(Value::Number(456.0));
    array.push(Value::Number(789.0));
    let opt_value: Option<Value> = array.pop();
    let len: usize = array.len();
    assert_eq!(len, 2);
    let string = format!("{}", array);
    assert_eq!(string.as_str(), "[123, 456]");
    assert_eq!(opt_value, Some(Value::Number(789.0)));
}

#[test]
fn eq_true() {
    let mut a: Array = Array::new();
    a.push(Value::Number(123.0));
    a.push(Value::Number(456.0));
    a.push(Value::Number(789.0));
    let mut b: Array = Array::new();
    b.push(Value::Number(123.0));
    b.push(Value::Number(456.0));
    b.push(Value::Number(789.0));
    assert_eq!(a.eq(&b), true);
}

#[test]
fn eq_false() {
    let mut a: Array = Array::new();
    a.push(Value::Number(123.0));
    a.push(Value::Number(456.0));
    a.push(Value::Number(789.0));
    let mut b: Array = Array::new();
    b.push(Value::Number(789.0));
    b.push(Value::Number(456.0));
    b.push(Value::Number(123.0));
    assert_eq!(a.eq(&b), false);
}

#[test]
fn not_same_not_eq() {
    let mut a: Array = Array::new();
    a.push(Value::Number(123.0));
    a.push(Value::Number(456.0));
    a.push(Value::Number(f64::NAN));
    let mut b: Array = Array::new();
    b.push(Value::Number(123.0));
    b.push(Value::Number(456.0));
    b.push(Value::Number(f64::NAN));
    assert_eq!(a.eq(&b), false);
}

#[test]
fn array_value() {
    let value: Value = Value::array(Array::new());
    assert_eq!(value.is_array(), true);
}

#[test]
fn array_value_eq_array_value_true() {
    let mut a: Array = Array::new();
    a.push(Value::Number(123.0));
    a.push(Value::Number(456.0));
    a.push(Value::Number(789.0));
    let mut b: Array = Array::new();
    b.push(Value::Number(123.0));
    b.push(Value::Number(456.0));
    b.push(Value::Number(789.0));
    let value_a: Value = Value::array(a);
    let value_b: Value = Value::array(b);
    assert_eq!(value_a.eq(&value_b), true);
}

#[test]
fn array_value_eq_array_value_false() {
    let mut a: Array = Array::new();
    a.push(Value::Number(123.0));
    a.push(Value::Number(456.0));
    a.push(Value::Number(f64::NAN));
    let mut b: Array = Array::new();
    b.push(Value::Number(123.0));
    b.push(Value::Number(456.0));
    b.push(Value::Number(f64::NAN));
    let value_a: Value = Value::array(a);
    let value_b: Value = Value::array(b);
    assert_eq!(value_a.eq(&value_b), false);
}

#[test]
fn array_value_is_array_value_true() {
    let mut a: Array = Array::new();
    a.push(Value::Number(123.0));
    a.push(Value::Number(456.0));
    a.push(Value::Number(789.0));
    let value_a: Value = Value::array(a);
    assert_eq!(value_a.is(&value_a), true);
}

#[test]
fn array_value_is_array_value_false() {
    let mut a: Array = Array::new();
    a.push(Value::Number(123.0));
    a.push(Value::Number(456.0));
    a.push(Value::Number(789.0));
    let mut b: Array = Array::new();
    b.push(Value::Number(123.0));
    b.push(Value::Number(456.0));
    b.push(Value::Number(789.0));
    let value_a: Value = Value::array(a);
    let value_b: Value = Value::array(b);
    assert_eq!(value_a.is(&value_b), false);
}

#[test]
fn value_len_zero() {
    let array: Array = Array::new();
    let value: Value = Value::array(array);
    let len: usize = value.as_array().len();
    assert_eq!(len, 0);
}


#[test]
fn value_push() {
    let array: Array = Array::new();
    let value: Value = Value::array(array);
    value.as_array_mut().push(Value::Number(123.0));
    value.as_array_mut().push(Value::Number(456.0));
    value.as_array_mut().push(Value::Number(789.0));
    let len: usize = value.as_array().len();
    assert_eq!(len, 3);
    let string = format!("{}", value.as_array());
    assert_eq!(string.as_str(), "[123, 456, 789]");
}


#[test]
fn value_pop() {
    let array: Array = Array::new();
    let value: Value = Value::array(array);
    value.as_array_mut().push(Value::Number(123.0));
    value.as_array_mut().push(Value::Number(456.0));
    value.as_array_mut().push(Value::Number(789.0));
    let opt_value: Option<Value> = value.as_array_mut().pop();
    let len: usize = value.as_array().len();
    assert_eq!(len, 2);
    let string = format!("{}", value.as_array());
    assert_eq!(string.as_str(), "[123, 456]");
    assert_eq!(opt_value, Some(Value::Number(789.0)));
}

#[test]
fn from_slice() {
    let v = vec![
        Value::Number(123.0),
        Value::Number(456.0),
        Value::Number(789.0),
    ];
    let array: Array = Array::from(v.as_slice());
    let len: usize = array.len();
    assert_eq!(len, 3);
    let string = format!("{}", array);
    assert_eq!(string.as_str(), "[123, 456, 789]");
}

#[test]
fn extend() {
    let vec_a = vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ];
    let vec_b = vec![
        Value::Number(4.0),
        Value::Number(5.0),
        Value::Number(6.0),
    ];
    let mut array: Array = Array::from(vec_a.as_slice());
    array.extend_from_slice(vec_b.as_slice());
    let len: usize = array.len();
    assert_eq!(len, 6);
    let string = format!("{}", array);
    assert_eq!(string.as_str(), "[1, 2, 3, 4, 5, 6]");

}

#[test]
fn truncate_zero() {
    let v = vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
        Value::Number(6.0),
    ];
    let mut array: Array = Array::from(v.as_slice());
    array.truncate(0);
    let len: usize = array.len();
    assert_eq!(len, 0);
    let string = format!("{}", array);
    assert_eq!(string.as_str(), "[]");
}

#[test]
fn truncate_two() {
    let v = vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
        Value::Number(6.0),
    ];
    let mut array: Array = Array::from(v.as_slice());
    array.truncate(2);
    let len: usize = array.len();
    assert_eq!(len, 2);
    let string = format!("{}", array);
    assert_eq!(string.as_str(), "[1, 2]");
}


#[test]
fn truncate_six() {
    let v = vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
        Value::Number(6.0),
    ];
    let mut array: Array = Array::from(v.as_slice());
    array.truncate(6);
    let len: usize = array.len();
    assert_eq!(len, 6);
    let string = format!("{}", array);
    assert_eq!(string.as_str(), "[1, 2, 3, 4, 5, 6]");
}


#[test]
fn truncate_oversize() {
    let v = vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
        Value::Number(6.0),
    ];
    let mut array: Array = Array::from(v.as_slice());
    array.truncate(8);
    let len: usize = array.len();
    assert_eq!(len, 6);
    let string = format!("{}", array);
    assert_eq!(string.as_str(), "[1, 2, 3, 4, 5, 6]");
}

