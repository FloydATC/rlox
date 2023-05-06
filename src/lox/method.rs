

use super::value::Value;

// Note: This object represents a Bound Method at runtime

// A Class/Instance method is represented by a Function object
// wrapped in a Closure object right up to the point where the 
// method is fetched from the object instance at runtime.

pub struct Method {
    receiver:	Value, // Must contain a Value::Obj::Instance
    method:	Value, // Must contain a Value::Obj::Closure
}


impl Method {
    // ======== Constructors ========

    pub fn new(receiver: Value, method: Value) -> Self {
        if !receiver.is_instance() {
            panic!("Receiver {} is not an object Instance", receiver);
        }
//        if !receiver.is_class() {
//            panic!("{} is not an object Class", receiver);
//        }
        if !method.is_closure() {
            panic!("Closure {} is not a Closure", method);
        }
        Self {
            receiver:	receiver,
            method:	method,
        }
    }
}


#[allow(dead_code)]
impl Method {
    // ======== Methods ========
    
    pub fn receiver_class_name(&self) -> String {
        return self.receiver.as_instance().class_name();
//        return self.receiver.as_class().name().to_string();
    }
    
    pub fn receiver(&self) -> &Value {
        return &self.receiver;
    }
    
    pub fn method_name(&self) -> String {
        return self.method.as_closure().name();
    }
    
    pub fn method(&self) -> &Value {
        return &self.method;
    }
}


// ======== Traits ========

impl std::fmt::Debug for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Method")
            .field("receiver", &self.receiver)
            .field("method", &self.method)
            .finish()
    }
}


impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Method({}.{})", self.receiver_class_name(), self.method_name())
    }
}


impl Drop for Method {
    fn drop(&mut self) {
        println!("Method.drop() {}.{}", self.receiver_class_name(), self.method_name());
    }
}
