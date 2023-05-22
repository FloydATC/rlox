

use crate::lox::common::Value;
use crate::lox::vm::RuntimeError;



pub fn len(args: &[Value]) -> Result<Value, RuntimeError> {
    println!("Native method len() invoked on {}", args[0]);
    match args[0].len() {
        Some(length) => return Ok(Value::Number(length as f64)),
        _ => return Ok(Value::Null),
    }
}

