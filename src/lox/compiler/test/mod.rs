

// Most of the tests that exercise the compiler is found under "parser"
// The compiler module is just a thin wrapper around the parser anyway


use crate::lox::common::{Function, FunctionKind};
use super::ChunkWriter;


#[test]
fn chunk_writer_new() {
    let function = Function::new("__test__", FunctionKind::Script, None);
    let _writer = ChunkWriter::new(function);
}

