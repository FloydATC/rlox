use crate::lox::common::Value;

// Note: This object represents a Bound Native Method at runtime

// A NativeCallable is really just a function pointer decorated with arity and a name. 
// Just like compiled functions and methods, native ones can be reassigned and moved around
// so before they can be called, we need to bind them to the receiver Value


#[derive(Clone, Debug)]
pub struct NativeMethod {
    receiver: Value, // Can contain any type of Value but native function may return Value::Null for unexpected types
    method:	Value, // Must contain a Value::Native
}


impl NativeMethod {
    // ======== Constructors ========

    pub fn new(receiver: Value, method: Value) -> Self {
        if !method.is_native() {
            panic!("{} is not a NativeCallable", method);
        }
        Self {
            receiver: receiver,
            method:	method,
        }
    }
}


impl NativeMethod {

    pub fn receiver(&self) -> &Value {
        &self.receiver
    }


    pub fn method(&self) -> &Value {
        &self.method
    }

}


