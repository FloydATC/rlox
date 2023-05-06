

use super::token::Token;


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ClassDescriptor {
    token: Token,
    parent: Option<Token>,
}


#[allow(dead_code)]
impl ClassDescriptor {

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


