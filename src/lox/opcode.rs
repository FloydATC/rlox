


#[allow(dead_code)]
#[repr(u8)]
pub enum OpCode {
    Return 	= 0,
    
    // Push constant value onto stack
    Const8,		// Followed by BYTE indexing table of constants
    Const16,		// Followed by WORD indexing table of constants
    Const32,		// Followed by DWORD indexing table of constants
    False,
    Null,
    True,    
    
    // Pop two values, perform operation, push result
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    
    // Pop (and discard) one or more values from the stack
    Pop,
    PopN,		// Followed by BYTE indicating number of values
    
    BAD 	= 255,	// Unknown/bad opcodes resolve to this
}


impl OpCode {
    pub fn name(byte: u8) -> &'static str {
        if byte == OpCode::Return as u8 { return "RETURN"; }

        if byte == OpCode::Const8 as u8 { return "CONST"; }
        if byte == OpCode::Const16 as u8 { return "CONST"; }
        if byte == OpCode::Const32 as u8 { return "CONST"; }
        if byte == OpCode::False as u8 	{ return "FALSE"; }
        if byte == OpCode::Null as u8 	{ return "NULL"; }
        if byte == OpCode::True as u8 	{ return "TRUE"; }
        
        if byte == OpCode::Add as u8 	{ return "ADD"; }
        if byte == OpCode::Sub as u8 	{ return "SUB"; }
        if byte == OpCode::Mul as u8 	{ return "MUL"; }
        if byte == OpCode::Div as u8 	{ return "DIV"; }
        if byte == OpCode::Mod as u8 	{ return "MOD"; }
        
        //if byte == OpCode::Push as u8 	{ return "PUSH"; }
        if byte == OpCode::Pop as u8 	{ return "POP"; }
        if byte == OpCode::PopN as u8 	{ return "POP"; }
        return "**BAD**";
    }
    pub fn code(byte: u8) -> OpCode {
        if byte == OpCode::Return as u8 { return OpCode::Return; }

        if byte == OpCode::Const8 as u8 { return OpCode::Const8; }
        if byte == OpCode::Const16 as u8 { return OpCode::Const16; }
        if byte == OpCode::Const32 as u8 { return OpCode::Const32; }
        if byte == OpCode::False as u8 { return OpCode::False; }
        if byte == OpCode::Null as u8 { return OpCode::Null; }
        if byte == OpCode::True as u8 { return OpCode::True; }
        
        if byte == OpCode::Add as u8 	{ return OpCode::Add; }
        if byte == OpCode::Sub as u8 	{ return OpCode::Sub; }
        if byte == OpCode::Mul as u8 	{ return OpCode::Mul; }
        if byte == OpCode::Div as u8 	{ return OpCode::Div; }
        if byte == OpCode::Mod as u8 	{ return OpCode::Mod; }

        //if byte == OpCode::Push as u8 	{ return OpCode::Push; }
        if byte == OpCode::Pop as u8 	{ return OpCode::Pop; }
        if byte == OpCode::PopN as u8 	{ return OpCode::PopN; }
        return OpCode::BAD;	// Do not use
    }
}
