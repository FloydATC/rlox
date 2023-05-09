

// Compile-time representation of a class
// "parent" in this context is the superclass, if any


use super::token::Token;


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Class {
    token: Token,
    parent: Option<Token>,
}


#[allow(dead_code)]
impl Class {

    pub fn new(token: &Token) -> Self {
        Self {
            token: token.clone(),
            parent: None,
        }
    }


    pub fn set_parent(&mut self, token: &Token) {
        self.parent = Some(token.clone());
    }


    pub fn get_parent(&self) -> Option<&Token> {
        return self.parent.as_ref();
    }


    pub fn has_parent(&self) -> bool {
        return self.parent.is_some();
    }

}


