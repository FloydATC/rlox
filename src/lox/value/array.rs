

#[cfg(test)]
mod test;


use super::Value;


#[derive(Debug, Clone)]
pub struct Array {
    values: Vec<Value>,
}


#[allow(dead_code)]
impl Array {

    pub fn new() -> Self {
        Array {
            values: vec![],
        }
    }


    pub fn len(&self) -> usize {
        return self.values.len();
    }


    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }


    pub fn pop(&mut self) -> Option<Value> {
        return self.values.pop();
    }


    pub fn as_slice(&self) -> &[Value] {
        return &self.values.as_slice();
    }


    pub fn get(&self, index: usize) -> Option<&Value> {
        return self.values.get(index);
    }


    pub fn set(&mut self, index: usize, value: Value) -> Result<(), String> {
        if index >= self.values.len() { return Err(format!("Bad subscript {} for {:?}", index, self.values)) };
        self.values[index] = value;
        Ok(())
    }


    pub fn extend_from_slice(&mut self, slice: &[Value]) 
    {
        self.values.extend_from_slice(slice);
    }


    pub fn truncate(&mut self, len: usize) {
        self.values.truncate(len);
    }

}


// Traits

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.values.iter().map(|value| format!("{}", value)).collect::<Vec<String>>().join(", "))
    }
}


impl PartialEq for Array {
    fn eq(&self, other: &Array) -> bool {
        self.values.eq(&other.values)
    }
}

impl From<&[Value]> for Array {
    fn from(slice: &[Value]) -> Self {
        Array { 
            values: Vec::from(slice),
        }
    }
}
