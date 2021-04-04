

#[cfg(test)]
mod test;


pub struct Constants<T> {
    values: Vec<T>,
}


impl<T: std::cmp::PartialEq> Constants<T> {

    pub fn new() -> Self {
        Self {
            values:	vec![],
        }
    }

    // O(n) - used at compile time
    pub fn make(&mut self, value: T) -> usize {
        let id = self.id_by_value(&value);
        match id {
            Some(id) => {
                return id;
            }
            None => {
                let id = self.values.len();
                self.values.push(value);
                return id; 
            }
        }
    }

    // O(n) - used at compile time
    pub fn id_by_value(&self, value: &T) -> Option<usize> {
        for (id, v) in self.values.iter().enumerate() {
            if v == value { return Some(id); }
        }
        return None;
    }

    // O(1) - used at runtime
    pub fn value_by_id(&self, id: usize) -> &T {
        return &self.values[id];
    }

}


impl<T> std::fmt::Debug for Constants<T>
    where T: std::fmt::Display {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        for (id, v) in self.values.iter().enumerate() {
            write!(f, "  0x{:04x} {}\n", id, v)?;
        }
        Ok(())
    }
}
