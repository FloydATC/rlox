

use log::{info};

mod lox;
mod native;

use lox::{Compiler, VM};

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


fn register_natives(vm: &mut VM) {
    vm.native_callables().insert_method("len", native::len, 0);
    vm.native_callables().insert_method("pop", native::pop, 0);
    vm.native_callables().insert_method("push", native::push, 1);
    vm.native_callables().insert_method("shift", native::shift, 0);
    vm.native_callables().insert_method("unshift", native::unshift, 1);
}


// Called from main() after parsing command line
pub fn run(config: Config) -> Result<(), std::io::Error> {
    let mut vm = VM::new();
    register_natives(&mut vm);

    match config.mode {
        Mode::Repl => {
            loop {
                println!("Interactive mode (Enter 'exit' or hit Ctrl+C when done)");
                let line = read_stdin()?;
                if line == "exit" { break; }
                let reader = std::io::Cursor::new(&line);
                compile_and_execute("INPUT", reader, &mut vm, |rc| info!("rc={}", rc));
            }
        }
        Mode::Line => {
            let line = config.line.unwrap();
            let reader = std::io::Cursor::new(&line);
            compile_and_execute("INPUT", reader, &mut vm, |rc| std::process::exit(rc));
        }
        Mode::File => {
            let filename = config.filename.unwrap();
            let file = std::fs::File::open(&filename)?;
            let reader = std::io::BufReader::new(file);
            compile_and_execute(&filename, reader, &mut vm, |rc| std::process::exit(rc));
        }
    }
    
    Ok(())
}


fn compile_and_execute<R, F>(filename: &str, input: R, vm: &mut lox::VM, action: F) 
where
    R: std::io::BufRead + std::io::Read, 
    F: FnOnce(i32),
{
    let builder = Compiler::new();
    match builder.compile(filename, input) {
        Ok(bytecode) => {
            match vm.execute(&bytecode) {
                Ok(rc) => action(rc),
                Err(runtime_error) => { 
                    eprintln!("{}\n{}", runtime_error, runtime_error.get_stack_trace().join("\n"));
                }
            }
        }
        Err(compile_error) => {
            eprintln!("{}", compile_error);
        }
    }
}


// Called by run() if config.mode == Mode::Repl
fn read_stdin() -> Result<String, std::io::Error> {
    let mut line = String::new();

    std::io::stdin().read_line(&mut line)?;
        
    return Ok(line.trim().to_string());
}

