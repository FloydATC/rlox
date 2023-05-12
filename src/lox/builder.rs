

use std::marker::PhantomData;

use super::ByteCode;
use super::compile_error::CompileError;
use super::compiler::Compiler;
use super::function::Function;
use super::function_kind::FunctionKind;
use super::globals::Globals;
use super::locals::Locals;
use super::parser::Parser;
use super::parser_output::ParserOutput;
use super::scanner::Scanner;
use super::tokenizer::Tokenizer;


pub struct Builder<R> {
    reader: PhantomData<R>,
}


impl<R: std::io::BufRead+std::io::Read> Builder<R> {

    pub fn new() -> Self {
        Builder {
            reader: PhantomData,
        }
    }

    pub fn compile(&self, reader: R) -> Result<ByteCode, CompileError> {

        let scanner = Scanner::new(reader);
        let mut input = Tokenizer::new(scanner);

        let function = Function::new("__main__", FunctionKind::Script);    
        let mut compiler = Compiler::new(function);

        let mut parser = Parser::new();
        let mut globals = Globals::new();

        let mut output = ParserOutput {
            compiler: 	&mut compiler,
            globals: 	&mut globals,
            locals:	&mut Locals::new(false),
        };

        let function = parser.parse(&mut input, &mut output)?;
        println!("{:#?}", function);
        return Ok(ByteCode::new(function, globals));

    }

}
