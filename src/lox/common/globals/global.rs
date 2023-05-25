

#[derive(Clone)]
pub struct Global {
    defined: bool,
    index: usize,
}


impl Global {

    pub fn new(index: usize) -> Self {
        Global {
            defined: false,
            index,
        }
    }


    pub fn define(&mut self) {
        self.defined = true;
    }


    pub fn is_defined(&self) -> bool {
        self.defined
    }


    pub fn index(&self) -> usize {
        self.index
    }

}
