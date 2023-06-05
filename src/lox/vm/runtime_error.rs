

use at::At;


#[macro_export]
macro_rules! r_error {
    ( $msg:expr ) => {
        {
            return Err(RuntimeError::new($msg));
        }
    };
    ( $msg:expr, $token:expr ) => {
        {
            return Err(RuntimeError::new_at($msg, $token));
        }
    };
}
pub use r_error;


#[derive(Debug)]
pub struct RuntimeError {
    message: String,
    at: Option<At>,
    stack_trace: Vec<String>,   
}


#[allow(dead_code)]
impl RuntimeError {

    pub fn new(message: String) -> Self {
        RuntimeError { 
            message,
            at: None,
            stack_trace: vec![],
        }
    }


    pub fn new_at(message: String, at: &At) -> Self {
        RuntimeError { 
            message,
            at: Some(at.clone()),
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

