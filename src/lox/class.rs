

pub struct Class {
    name	: String,
}


#[allow(dead_code)]
impl Class {

    pub fn new(name: &str) -> Self {
        Self {
            name:	name.to_string(),
        }
    }

}


impl Class {
    
    pub fn name(&self) -> &str {
        return &self.name;
    }
    
}


impl std::fmt::Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class")
            .field("name", &self.name)
            .finish()
    }
}