

use crate::lox::common::IdentifierKind;


#[allow(dead_code)]
#[derive(Clone)]
pub struct Global {
    index: usize,
    kind: IdentifierKind,
    defined: bool,
}


#[allow(dead_code)]
impl Global {

    pub fn new(index: usize, kind: IdentifierKind) -> Self {
        Global {
            index,
            kind,
            defined: false,
        }
    }


    pub fn define(&mut self) {
        self.defined = true;
    }


    pub fn is_defined(&self) -> bool {
        self.defined
    }


    pub fn is_mutable(&self) -> bool {
        self.kind.is_mutable()
    }


    pub fn kind(&self) -> &IdentifierKind {
        return &self.kind;
    }


    pub fn index(&self) -> usize {
        self.index
    }

}
