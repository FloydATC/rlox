

#[derive(Clone)]
pub struct Hierarchy<T: Clone> {
    hierarchy: Vec<(String, T)>,
}


impl<T: Clone> Hierarchy<T> {

    pub fn new() -> Self {
        Hierarchy { 
            hierarchy: vec![],
        }
    }


    pub fn push(&mut self, name: &str, object: T) {
        self.hierarchy.push((name.to_string(), object));
    }


    pub fn pop(&mut self) -> Option<T> {
        return match self.hierarchy.pop() {
            None => None,
            Some((_, object)) => Some(object),
        }
    }


    pub fn current(&mut self) -> Option<&T> {
        return match self.hierarchy.last() {
            None => None,
            Some((_, object_ref)) => Some(object_ref),
        }
    }


    pub fn current_name(&self) -> Option<&str> {
        return match self.hierarchy.last() {
            None => None,
            Some((name_ref, _)) => Some(name_ref),
        }
    }


    pub fn current_path(&self) -> Option<String> {
        if self.hierarchy.is_empty() {
            return None;
        } else {
            let names: Vec<String> = self.hierarchy.iter().map(|(name,_)| name.clone()).collect();
            return Some(names.join("::"));
        }
    }


    pub fn current_depth(&self) -> usize {
        return self.hierarchy.len();
    }

}


impl<T: Clone> std::fmt::Display for Hierarchy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.current_path() {
            None => write!(f, "None"),
            Some(path) => write!(f, "Some({:?})", path)
        }
    }
}

