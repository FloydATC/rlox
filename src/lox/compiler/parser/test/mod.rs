

use crate::lox::common::{ByteCode, Function, FunctionKind, Globals};
use crate::lox::compiler::{ChunkWriter, CompileError, Locals, Scanner, Tokenizer};


use super::Parser;
use super::ParserOutput;


mod arrays;
mod base2numbers;
mod base8numbers;
mod base10numbers;
mod base16numbers;
mod for_loops;
mod if_statement;
mod return_statement;
mod trailing_comma;
mod variables;
mod constants;
mod while_loops;
mod literals;
mod strings;
mod misc;


fn test(code: &str) -> Result<ByteCode, CompileError> {
    
    // This code duplicates a lot of what the Compiler does. Hmm.
    let reader = std::io::Cursor::new(code);
    let scanner = Scanner::new("test", reader);
    let mut input = Tokenizer::new(scanner);

    let function = Function::new("__test__", FunctionKind::Script, None);
    let mut writer = ChunkWriter::new(function);
    let mut globals = Globals::new();

    let mut output = ParserOutput {
        writer: 	&mut writer,
        globals: 	&mut globals,
        locals:	&mut Locals::new(false),
    };

    // Be verbose to make debugging a little easier
    match Parser::new().parse(&mut input, &mut output) {
        Ok(function) => {
            println!("parse() returned Ok({:?})", function);
            return Ok(ByteCode::new(function, globals));
        }
        Err(compile_error) => {
            println!("parse() returned Err({:?})", compile_error);
            return Err(compile_error);
        }
    }
}

