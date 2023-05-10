
use std::io;
use std::fs;
use std::process;
use std::error::Error;

mod lox;
use lox::{Builder, VM};

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



// Called from main() after parsing command line
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut vm = VM::new();

    match config.mode {
        Mode::Repl => {
            loop {
                println!("Interactive mode (Enter 'exit' or hit Ctrl+C when done)");
                let line = read_stdin();
                if line == "exit" { break; }
                let reader = std::io::Cursor::new(&line);
                compile_and_execute(reader, &mut vm, |rc| println!("rc={}", rc));
            }
        }
        Mode::Line => {
            let line = config.line.unwrap();
            let reader = std::io::Cursor::new(&line);
            compile_and_execute(reader, &mut vm, |rc| std::process::exit(rc));
    }
        Mode::File => {
            let filename = config.filename.unwrap();
            let code = read_file(&filename);
            let reader = std::io::Cursor::new(&code);
            compile_and_execute(reader, &mut vm, |rc| std::process::exit(rc));
        }
    }
    
    Ok(())
}


fn compile_and_execute<R, F>(input: R, vm: &mut lox::VM, action: F) 
where
    R: std::io::BufRead + std::io::Read, 
    F: FnOnce(i32),
{
    let builder = Builder::new();
    match builder.compile(input) {
        Ok(bytecode) => {
            match vm.execute(&bytecode) {
                Ok(rc) => action(rc),
                Err(runtime_error) => eprint!("{}", runtime_error),
            }
        }
        Err(compile_error) => eprint!("{}", compile_error),
    }
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

