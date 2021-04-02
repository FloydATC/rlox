

//use std::collections::HashMap;

use super::value::Value;


pub struct Constants {
    //index: HashMap<Value,usize>, // Value::Number (f64) doesn't implement Eq
    values: Vec<Value>,
}


impl Constants {

    pub fn new() -> Constants {
        Constants {
            //index:	HashMap::new(),
            values:	vec![],
        }
    }

    pub fn make(&mut self, value: Value) -> usize {
        let id = self.id_by_value(&value);
        match id {
            Some(id) => {
                return id;
            }
            None => {
                let id = self.values.len();
                self.values.push(value);
                //self.index.insert(value, index);
                return id; 
            }
        }
    }
}


impl Constants {

    // O(n) - used at compile time
    pub fn id_by_value(&self, value: &Value) -> Option<usize> {
        for (id, v) in self.values.iter().enumerate() {
            if v == value { return Some(id); }
        }
        return None;
        //return self.index.get(value);
    }

    // O(1) - used at runtime
    pub fn value_by_id(&self, id: usize) -> Value {
        return self.values[id].clone();
    }

}


impl std::fmt::Debug for Constants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        for (id, v) in self.values.iter().enumerate() {
            write!(f, "  0x{:04x} {}\n", id, v)?;
        }
        Ok(())
    }
}
