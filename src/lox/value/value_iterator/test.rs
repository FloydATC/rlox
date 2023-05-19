

use crate::lox::Array;
use super::Value;
use super::ValueIterator;


#[test]
fn string_iterator_empty() {
    let str = "";
    let s = Value::String(str.into());
    let mut iter = ValueIterator::new(s.clone()).unwrap();
    let (val, next) = iter.next();
    assert_eq!((val, next), (&s, None));
}

#[test]
fn string_iterator_ascii() {
    let str = "abc";
    let s = Value::String(str.into());
    let mut iter = ValueIterator::new(s.clone()).unwrap();
    let (val, next) = iter.next();
    assert_eq!((val, next.unwrap().as_string().as_str()), (&s, "a"));
    let (val, next) = iter.next();
    assert_eq!((val, next.unwrap().as_string().as_str()), (&s, "b"));
    let (val, next) = iter.next();
    assert_eq!((val, next.unwrap().as_string().as_str()), (&s, "c"));
    let (val, next) = iter.next();
    assert_eq!((val, next), (&s, None));
}

#[test]
fn string_iterator_utf8() {
    let str = "\u{0201}\u{0202}\u{0203}";
    let s = Value::String(str.into());
    let mut iter = ValueIterator::new(s.clone()).unwrap();
    let (val, next) = iter.next();
    assert_eq!((val, next.unwrap().as_string().as_str()), (&s, "\u{0201}"));
    let (val, next) = iter.next();
    assert_eq!((val, next.unwrap().as_string().as_str()), (&s, "\u{0202}"));
    let (val, next) = iter.next();
    assert_eq!((val, next.unwrap().as_string().as_str()), (&s, "\u{0203}"));
    let (val, next) = iter.next();
    assert_eq!((val, next), (&s, None));
}

#[test]
fn array_iterator_empty() {
    let array = Value::array(Array::from(vec![].as_slice()));
    let mut iter = ValueIterator::new(array.clone()).unwrap();
    let (val, next) = iter.next();
    assert_eq!((val, next), (&array, None));
}

#[test]
fn array_iterator_three() {
    let a = Value::Number(1.0);
    let b = Value::Number(2.0);
    let c = Value::Number(3.0);
    let array = Value::array(Array::from(vec![a,b,c].as_slice()));
    let mut iter = ValueIterator::new(array.clone()).unwrap();
    let (val, next) = iter.next();
    assert_eq!((val, next.unwrap()), (&array, &Value::Number(1.0)));
    let (val, next) = iter.next();
    assert_eq!((val, next.unwrap()), (&array, &Value::Number(2.0)));
    let (val, next) = iter.next();
    assert_eq!((val, next.unwrap()), (&array, &Value::Number(3.0)));
    let (val, next) = iter.next();
    assert_eq!((val, next), (&array, None));
}
