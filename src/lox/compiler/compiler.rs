

use std::marker::PhantomData;
use log::{debug};


use crate::lox::{common::{ByteCode, Function, FunctionKind, Globals}, compiler::Tokenize};


use super::{ChunkWriter, CompileError, Locals, Parser, ParserOutput, Scanner, Tokenizer};


pub struct Compiler<R> {
    reader: PhantomData<R>, // 0-byte marker needed for rustc to accept the <R>
}


impl<R: std::io::BufRead> Compiler<R> {

    pub fn new() -> Self {
        Compiler {
            reader: PhantomData, // 0-byte marker, ignore
        }
    }

    pub fn compile(&self, filename: &str, reader: R) -> Result<ByteCode, CompileError> {

        let scanner = Scanner::new(filename, reader);
        let mut input = Tokenizer::new(scanner);

        let at = input.current().get_at().cloned();
        let function = Function::new("__main__", FunctionKind::Script, at);    
        let mut writer = ChunkWriter::new(function);

        let mut parser = Parser::new();
        let mut globals = Globals::new();

        let mut output = ParserOutput {
            writer: 	&mut writer,
            globals: 	&mut globals,
            locals:	&mut Locals::new(false),
        };

        let function = parser.parse(&mut input, &mut output)?;
        debug!("{:#?}", function);
        return Ok(ByteCode::new(function, globals));

    }

}
