use crate::lox::common::Value;
use crate::lox::vm::{RuntimeError, r_error};


pub fn pop(args: &mut [Value]) -> Result<Value, RuntimeError> {
    let receiver = &mut args[0];

    if receiver.is_string() {
        let mut input = receiver.as_string_mut();
        match input.chars().rev().nth(0) {
            Some(ch) => {
                let (head, tail) = input.split_at(input.len() - ch.len_utf8());
                let result = Value::string(tail);
                *input = String::from(head);
                return Ok(result);
            }
            None => r_error!(format!("Tried to pop() from empty string")),
        }
    }

    if receiver.is_array() {
        match receiver.as_array_mut().pop() {
            Some(element) => return Ok(element),
            None => r_error!(format!("Tried to pop() from empty array")),
        }
    }

    r_error!(format!("{} does not have a method 'pop'", receiver));
}
