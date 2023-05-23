

use crate::lox::common::{Value, Array};
use crate::lox::vm::{RuntimeError, r_error};


pub fn unshift(args: &mut [Value]) -> Result<Value, RuntimeError> {
    let element = args[1].clone();
    let receiver = &mut args[0];

    if receiver.is_string() && element.is_number() {
        match char::from_u32(element.as_number() as u32) {
            Some(ch) => {
                let mut input = receiver.as_string_mut();
                *input =  format!("{}{}", ch, input);
                return Ok(Value::Null);
            }
            None => return Ok(Value::Null),
        }
    }

    if receiver.is_string() {
        let mut input = receiver.as_string_mut();
        *input =  format!("{}{}", element, input);
        return Ok(Value::Null);
}

    if receiver.is_array() {
        let mut input = receiver.as_array_mut();
        let mut array = Array::new();
        array.push(element.clone());
        array.extend_from_slice(input.as_slice());
        *input = array;
        return Ok(Value::Null);
    }

    r_error!(format!("{} does not have a method 'push'", receiver));
}
