

#[cfg(test)]
mod test;


use super::Value;
use crate::lox::common::keyword::*;
pub use super::Array;


#[derive(Debug, Clone)]
pub enum ValueIterator {
    String(Value, usize, Value), // String value, byte index of next character, last value (or Value::Null)
    Array(Value, usize, Value), // Array value, index of next element, last value (or Value::Null)
    Instance(Value, Value), // Instance value, last value (or Value::Null)
}


impl ValueIterator {

    pub fn new(value: Value) -> Result<Self, String> {
        if value.is_string() { return ValueIterator::new_string(value) }
        if value.is_array() { return ValueIterator::new_array(value) }
        if value.is_instance() { return ValueIterator::new_instance(value) }
        Err(format!("Can not iterate over {}", value))
    }


    fn new_string(s: Value) -> Result<Self, String> {
        Ok(ValueIterator::String(s, 0, Value::Null))
    }


    fn new_array(a: Value) -> Result<Self, String> {
        Ok(ValueIterator::Array(a, 0, Value::Null))
    }


    fn new_instance(ivalue: Value) -> Result<Self, String> {
        //println!("new_instance() iterator from {}", ivalue);
        match ivalue.as_instance().class().get(&Value::string(KEYWORD_NEXT)) {
            None => return Err(format!("No method named '{}' in {}", KEYWORD_NEXT, ivalue)),
            Some(m) => {
                //println!("new_instance() verifying method={}", m);
                let arity = m.as_closure().function_ref().arity();
                if arity != 1 { return Err(format!("Method '{}' of {} takes {} arguments, must be 1", KEYWORD_NEXT, m, arity)); }
            }
        }
        Ok(ValueIterator::Instance(ivalue, Value::Null))
    }


    pub fn next(&mut self) -> (&Value, Option<&Value>) {
        match self {
            ValueIterator::String(..) => return self.next_in_string(),
            ValueIterator::Array(..) => return self.next_in_array(),
            ValueIterator::Instance(..) => return self.next_in_instance(),
        }
    }


    fn next_in_string(&mut self) -> (&Value, Option<&Value>) {
        if let ValueIterator::String(s, byte, last) = self {
            let input = s.as_string();
            let (_head, tail) = input.split_at(*byte);
            match tail.chars().nth(0) {
                None => { 
                    // Reached end of string
                    return (s, None); 
                }
                Some(ch) => {
                    *byte = *byte + ch.len_utf8();
                    *last = Value::string(String::from(ch).as_str());
                    return (s, Some(last));
                }
            }
        } else {
            panic!("Internal error");
        }
    }


    fn next_in_array(&mut self) -> (&Value, Option<&Value>) {
        if let ValueIterator::Array(a, index, last) = self {
            if *index >= a.as_array().len() {
                // Reached end of array
                return (a, None);
            } else {
                *last = a.as_array().get(*index).unwrap().clone();
                *index = *index + 1;
                return (a, Some(last));
            }
        } else {
            panic!("Internal error");
        }
    }


    fn next_in_instance(&self) -> (&Value, Option<&Value>) {
        // This one is a little different; rather than provide the next value we provide the means to get it.
        // The VM will know what to do when it sees a Method value.
        if let ValueIterator::Instance(ivalue, last) = self {
            return (ivalue, Some(last));
        } else {
            panic!("Internal error");
        }
    }


    pub fn last(&mut self) -> &mut Value {
        match self {
            ValueIterator::String(_, _, last) => return last,
            ValueIterator::Array(_, _, last) => return last,
            ValueIterator::Instance(_, last) => return last,
        }
    }

}


impl std::fmt::Display for ValueIterator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValueIterator::String(svalue, ..) => {
                write!(f, "ValueIterator::String({:?})", svalue.as_string())
            }
            ValueIterator::Array(avalue, ..) => {
                write!(f, "ValueIterator::Array({})", avalue.as_array())
            }
            ValueIterator::Instance(ivalue, ..) => {
                write!(f, "ValueIterator::Instance({})", ivalue.as_instance())
            }
        }
    }
}
