

#[cfg(test)]
mod test;


//mod array;
mod obj;
mod value;
mod value_iterator;

//pub use array::Array;
pub use obj::{Array, Closure, Function, FunctionKind, NativeFn, NativeCallable, NativeCallables, Obj};
pub use value::Value;
pub use value_iterator::ValueIterator;
