

use super::at::At;
use crate::lox::Token;


#[derive(Debug)]
pub struct CompileError {
    message: String,
    at: Option<At>,
}


#[allow(dead_code)]
impl CompileError {

    pub fn new(message: String) -> Self {
        CompileError { 
            message,
            at: None,
        }
    }


    pub fn new_at(message: String, token: &Token) -> Self {
        CompileError { 
            message,
            at: token.get_at().cloned(),
        }
    }


    pub fn get_message(&self) -> &String {
        return &self.message;
    }


    pub fn get_at(&self) -> Option<&At> {
        return self.at.as_ref();
    }


    pub fn set_at(&mut self, at: Option<&At>) {
        self.at = at.cloned();
    }

}


impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.at {
            None => writeln!(f, "ERROR> {}", self.message),
            Some(at) => writeln!(f, "ERROR> {} at {}", self.message, at),
        }
    }
}

