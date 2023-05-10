

use super::compiler::Compiler;
use super::globals::Globals;
use super::locals::Locals;
use super::value::Value;


pub struct ParserOutput<'a> {
    pub compiler: 	&'a mut Compiler,
    pub globals: 	&'a mut Globals<Value>,
    pub locals: 	&'a mut Locals,
}
