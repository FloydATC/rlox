

use super::at::At;


#[derive(Debug)]
pub struct CompileError {
    message: String,
    at: Option<At>,
}


#[allow(dead_code)]
impl CompileError {

    pub fn new(message: String, at: Option<At>) -> Self {
        CompileError { 
            message,
            at,
        }
    }


    pub fn get_message(&self) -> &String {
        return &self.message;
    }


    pub fn get_at(&self) -> Option<&At> {
        return self.at.as_ref();
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

