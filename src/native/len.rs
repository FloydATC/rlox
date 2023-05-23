

use crate::lox::common::Value;
use crate::lox::vm::RuntimeError;



pub fn len(args: &mut [Value]) -> Result<Value, RuntimeError> {
    match args[0].len() {
        Some(length) => return Ok(Value::Number(length as f64)),
        _ => return Ok(Value::Null),
    }
}
