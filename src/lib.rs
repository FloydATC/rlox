
use std::io;
//use std::env;
use std::fs;
use std::process;
use std::error::Error;



pub enum Mode {
    Repl,
    Line,
    File,
}


pub struct Config {
    pub mode: Mode,
    pub line: Option<String>,
    pub filename: Option<String>,
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {

        let mut mode = Mode::Repl;
        let mut line = None;
        let mut filename = None;
        
        //println!("{:?}", args);

        if args.len() == 2 {
            mode = Mode::File;
            filename = Some(args[1].clone());
        }
        
        if args.len() == 3 && args[1] == "-e" {
            mode = Mode::Line;
            line = Some(args[2].clone());
        }

        Ok(Config { mode, line, filename })
        
    }
}


pub struct Interpreter {
}


impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }
}


impl Interpreter {
    pub fn compile(&mut self, _code: &str) {
        //println!("Interpreter.compile() code={}", code);
    }
}


impl Interpreter {
    pub fn execute(&mut self) {
        //println!("Interpreter.execute()");
    }
}


impl Drop for Interpreter {
    fn drop(&mut self) {
        //println!("Interpreter.drop()");
    }
}



// Called from main() after parsing command line
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut interpreter = Interpreter::new();

    match config.mode {
        Mode::Repl => {
            println!("Interactive mode (Enter 'exit' or hit Ctrl+C when done)");
            loop {
                let line = read_stdin();
                if line == "exit" { break; }
                interpreter.compile(&line);
                interpreter.execute();
            }
        }
        Mode::Line => {
            let code = config.line.unwrap();
            interpreter.compile(&code);
            interpreter.execute();
        }
        Mode::File => {
            let filename = config.filename.unwrap();
            let code = read_file(&filename);
            interpreter.compile(&code);
            interpreter.execute();
        }
    }
    
    Ok(())
}


// Called by run() if config.mode == Mode::File
fn read_file(filename: &str) -> String {

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|err| { 
            eprintln!("{}: {}", filename, err);
            process::exit(2);
        });

    return contents;
}


// Called by run() if config.mode == Mode::Repl
fn read_stdin() -> String {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
        
    return line.trim().to_string();
}

