

use crate::lox::common::IdentifierKind;


// This is the compile time representation of an upvalue;
// it contains information about what outer upvalue or local
// this upvalue refers to.
// The runtime representation of an upvalue is nested in "vm.rs"
#[allow(dead_code)]
#[derive(Debug)]
pub struct Upvalue {
    id:		usize,
    is_local:	bool,
    kind: IdentifierKind,
}


#[allow(dead_code)]
impl Upvalue {
    pub fn new(id: usize, is_local: bool, kind: IdentifierKind) -> Self {
        Self {
            id,
            is_local,
            kind,
        }
    }
    

    pub fn id(&self) -> usize {
        return self.id;
    }
    

    pub fn is_local(&self) -> bool {
        return self.is_local;
    }


    pub fn kind(&self) -> &IdentifierKind {
        return &self.kind;
    }

}


