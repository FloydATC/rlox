

use crate::lox::compiler::{ChunkWriter, Locals};
use crate::lox::common::{Globals, Value};


pub struct ParserOutput<'a> {
    pub writer: 	&'a mut ChunkWriter,
    pub globals: 	&'a mut Globals<Value>,
    pub locals: 	&'a mut Locals,
}
