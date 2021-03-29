


#[allow(dead_code)]
#[repr(u8)]
pub enum OpCode {
    RETURN = 0,
    BAD = 255,	// Unknown/bad opcodes resolve to this
}


impl OpCode {
    pub fn name(byte: u8) -> &'static str {
        if byte == OpCode::RETURN as u8 { return "RETURN"; }
        return "**BAD**";
    }
    pub fn code(byte: u8) -> OpCode {
        if byte == OpCode::RETURN as u8 { return OpCode::RETURN; }
        return OpCode::BAD;	// Do not use
    }
}
