

use super::{ChunkWriter, Locals};
use crate::lox::common::Globals;
use crate::lox::common::Value;


pub struct ParserOutput<'a> {
    pub writer: 	&'a mut ChunkWriter,
    pub globals: 	&'a mut Globals<Value>,
    pub locals: 	&'a mut Locals,
}
