

use super::at::At;


#[derive(Debug)]
pub struct RuntimeError {
    message: String,
    at: Option<At>,
    stack_trace: Vec<String>,   
}


#[allow(dead_code)]
impl RuntimeError {

    pub fn new(message: String, at: Option<At>) -> Self {
        RuntimeError { 
            message,
            at,
            stack_trace: vec![],
        }
    }


    pub fn get_message(&self) -> &String {
        return &self.message;
    }


    pub fn get_at(&self) -> Option<&At> {
        return self.at.as_ref();
    }


    pub fn get_stack_trace(&self) -> &Vec<String> {
        return &self.stack_trace;
    }


    pub fn set_stack_trace(&mut self, mut stack_trace: Vec<String>) {
        self.stack_trace.extend(stack_trace.drain(..));
    }

}


impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.at {
            None => writeln!(f, "RUNTIME ERROR> {}", self.message),
            Some(at) => writeln!(f, "RUNTIME ERROR> {} at {}", self.message, at),
        }
    }
}

