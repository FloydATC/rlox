

use crate::lox::common::{Value, Array};
use crate::lox::vm::{RuntimeError, r_error};


pub fn shift(args: &mut [Value]) -> Result<Value, RuntimeError> {
    let receiver = &mut args[0];

    if receiver.is_string() {
        let mut input = receiver.as_string_mut();
        match input.chars().nth(0) {
            Some(ch) => {
                let (head, tail) = input.split_at(ch.len_utf8());
                let result = Value::string(head);
                *input = String::from(tail);
                return Ok(result);
            }
            None => r_error!(format!("Tried to shift() from empty string")),
        }
    }

    if receiver.is_array() {
        let mut input = receiver.as_array_mut();
        if input.len() > 0 {
            let result = input.as_slice()[0].clone();
            *input = Array::from(&input.as_slice()[1..]);
            return Ok(result);
        } else {
            r_error!(format!("Tried to shift() from empty array"))
        }
    }

    r_error!(format!("{} does not have a method 'shift'", receiver));
}
