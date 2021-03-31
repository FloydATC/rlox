

use super::value::Value;


pub struct Constants {
    values: Vec<Value>,
}


impl Constants {

    pub fn new() -> Constants {
        Constants {
            values:	vec![],
        }
    }

    pub fn make(&mut self, value: Value) -> usize {
        let i = self.index_by_value(&value);
        match i {
            Some(index) => {
                return index;
            }
            None => {
                let index = self.values.len();
                self.values.push(value);
                return index; 
            }
        }
    }
}


impl Constants {

    // Note: O(n)
    pub fn index_by_value(&self, value: &Value) -> Option<usize> {
        for (i, v) in self.values.iter().enumerate() {
            if v == value { return Some(i); }
        }
        return None;
    }

}
