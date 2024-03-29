

use log::trace;


use crate::lox::common::IdentifierKind;


// Compile-time representation of a local variable


#[derive(Debug)]
pub struct Local {
    name: String,
    depth: usize,	// Scope depth within function
    defined: bool,
    captured: bool,
    kind: IdentifierKind,
}


#[allow(dead_code)]
impl Local {
    pub fn new(name: &str, depth: usize, kind: IdentifierKind) -> Local {
        Local {
            name:	name.to_string(),
            depth,
            defined: 	false,
            captured:	false,
            kind,
        }
    }
    
    pub fn define(&mut self) {
        self.defined = true;
    }

    pub fn capture(&mut self) {
        self.captured = true;
    }

    pub fn name(&self) -> &str {
        return self.name.as_str();
    }
    
    pub fn depth(&self) -> usize {
        return self.depth;
    }
    
    pub fn is_defined(&self) -> bool {
        trace!("local {} is_defined={}", self.name, self.name == "this" || self.defined);
        return self.name == "this" || self.defined;
    }

    pub fn is_captured(&self) -> bool {
        return self.captured;
    }


    pub fn is_mutable(&self) -> bool {
        trace!("local {} is_mutable={}", self.name, self.kind.is_mutable());
        return self.kind.is_mutable();
    }

    pub fn kind(&self) -> &IdentifierKind {
        return &self.kind;
    }

}
