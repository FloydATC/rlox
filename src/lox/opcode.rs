


#[allow(dead_code)]
#[repr(u8)]
pub enum OpCode {
    Return 	= 0,
    Add		= 1,	// TEST
    Push	= 2, 	// TEST
    BAD 	= 255,	// Unknown/bad opcodes resolve to this
}


impl OpCode {
    pub fn name(byte: u8) -> &'static str {
        if byte == OpCode::Return as u8 { return "RETURN"; }
        if byte == OpCode::Add as u8 { return "ADD"; }
        if byte == OpCode::Push as u8 { return "PUSH"; }
        return "**BAD**";
    }
    pub fn code(byte: u8) -> OpCode {
        if byte == OpCode::Return as u8 { return OpCode::Return; }
        if byte == OpCode::Add as u8 { return OpCode::Add; }
        if byte == OpCode::Push as u8 { return OpCode::Push; }
        return OpCode::BAD;	// Do not use
    }
}
