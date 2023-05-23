use crate::lox::common::Value;
use crate::lox::vm::{RuntimeError, r_error};


pub fn push(args: &mut [Value]) -> Result<Value, RuntimeError> {
    let element = args[1].clone();
    let receiver = &mut args[0];

    if receiver.is_string() && element.is_number() {
        match char::from_u32(element.as_number() as u32) {
            Some(ch) => {
                receiver.as_string_mut().push(ch);
                return Ok(Value::Null);
            }
            None => return Ok(Value::Null),
        }
    }

    if receiver.is_string() {
        receiver.as_string_mut().push_str(element.to_string().as_str());
        return Ok(Value::Null);
    }

    if receiver.is_array() {
        receiver.as_array_mut().push(element.clone());
        return Ok(Value::Null);
    }

    r_error!(format!("{} does not have a method 'push'", receiver));
}
