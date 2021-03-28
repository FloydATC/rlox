
use std::io;
use std::fs;
use std::process;
use std::error::Error;

mod lox;


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
    let mut vm = lox::VM::new();

    match config.mode {
        Mode::Repl => {
            println!("Interactive mode (Enter 'exit' or hit Ctrl+C when done)");
            loop {
                let line = read_stdin();
                if line == "exit" { break; }
                vm.compile(&line)?;
                vm.execute();
            }
        }
        Mode::Line => {
            let code = config.line.unwrap();
            vm.compile(&code)?;
            vm.execute();
        }
        Mode::File => {
            let filename = config.filename.unwrap();
            let code = read_file(&filename);
            vm.compile(&code)?;
            vm.execute();
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

