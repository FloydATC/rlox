
mod native_callable;
mod native_callables;

pub use native_callable::NativeCallable;
pub use native_callables::NativeCallables;

use crate::lox::common::Value;
use crate::lox::vm::RuntimeError;

pub type NativeFn = fn(&[Value]) -> Result<Value, RuntimeError>;

