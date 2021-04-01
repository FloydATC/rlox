

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Return 	= 0,
    
    // Push constant value onto stack
    GetConst8,		// Followed by BYTE indexing table of constants
    GetConst16,		// Followed by WORD indexing table of constants
    GetConst32,		// Followed by DWORD indexing table of constants
    False,
    Null,
    True,    

    // Push variable value onto stack
    GetLocal8,
    GetLocal16,
    GetLocal32,
    GetUpvalue8,
    GetUpvalue16,
    GetUpvalue32,
    GetGlobal8,
    GetGlobal16,
    GetGlobal32,
    
    // Pop value and put in new variable
    DefGlobal8,		// Followed by BYTE indexing table of globals
    DefGlobal16,	// Followed by WORD indexing table of globals
    DefGlobal32,	// Followed by DWORD indexing table of globals

    // Pop value and put in existing variable
    SetLocal8,
    SetLocal16,
    SetLocal32,
    SetUpvalue8,
    SetUpvalue16,
    SetUpvalue32,
    SetGlobal8,
    SetGlobal16,
    SetGlobal32,        
    
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

        if byte == OpCode::GetConst8 as u8 { return "GETCONST"; }
        if byte == OpCode::GetConst16 as u8 { return "GETCONST"; }
        if byte == OpCode::GetConst32 as u8 { return "GETCONST"; }
        if byte == OpCode::False as u8 	{ return "FALSE"; }
        if byte == OpCode::Null as u8 	{ return "NULL"; }
        if byte == OpCode::True as u8 	{ return "TRUE"; }

        if byte == OpCode::GetLocal8 as u8  { return "GETLOCAL"; }
        if byte == OpCode::GetLocal16 as u8  { return "GETLOCAL"; }
        if byte == OpCode::GetLocal32 as u8  { return "GETLOCAL"; }
        if byte == OpCode::GetUpvalue8 as u8  { return "GETUPVALUE"; }
        if byte == OpCode::GetUpvalue16 as u8  { return "GETUPVALUE"; }
        if byte == OpCode::GetUpvalue32 as u8  { return "GETUPVALUE"; }
        if byte == OpCode::GetGlobal8 as u8  { return "GETGLOBAL"; }
        if byte == OpCode::GetGlobal16 as u8  { return "GETGLOBAL"; }
        if byte == OpCode::GetGlobal32 as u8  { return "GETGLOBAL"; }
        
        if byte == OpCode::DefGlobal8 as u8 { return "DEFGLOBAL"; }
        if byte == OpCode::DefGlobal16 as u8 { return "DEFGLOBAL"; }
        if byte == OpCode::DefGlobal32 as u8 { return "DEFGLOBAL"; }

        if byte == OpCode::SetLocal8 as u8  { return "SETLOCAL"; }
        if byte == OpCode::SetLocal16 as u8  { return "SETLOCAL"; }
        if byte == OpCode::SetLocal32 as u8  { return "SETLOCAL"; }
        if byte == OpCode::SetUpvalue8 as u8  { return "SETUPVALUE"; }
        if byte == OpCode::SetUpvalue16 as u8  { return "SETUPVALUE"; }
        if byte == OpCode::SetUpvalue32 as u8  { return "SETUPVALUE"; }
        if byte == OpCode::SetGlobal8 as u8  { return "SETGLOBAL"; }
        if byte == OpCode::SetGlobal16 as u8  { return "SETGLOBAL"; }
        if byte == OpCode::SetGlobal32 as u8  { return "SETGLOBAL"; }

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

        if byte == OpCode::GetConst8 as u8 { return OpCode::GetConst8; }
        if byte == OpCode::GetConst16 as u8 { return OpCode::GetConst16; }
        if byte == OpCode::GetConst32 as u8 { return OpCode::GetConst32; }
        if byte == OpCode::False as u8 { return OpCode::False; }
        if byte == OpCode::Null as u8 { return OpCode::Null; }
        if byte == OpCode::True as u8 { return OpCode::True; }
        
        if byte == OpCode::DefGlobal8 as u8 { return OpCode::DefGlobal8; }
        if byte == OpCode::DefGlobal16 as u8 { return OpCode::DefGlobal16; }
        if byte == OpCode::DefGlobal32 as u8 { return OpCode::DefGlobal32; }

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


pub struct OpCodeSet {
    pub byte:	OpCode,
    pub word:	OpCode,
    pub dword:	OpCode,
}


impl OpCodeSet {
    pub fn getconst() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::GetConst8,
            word:	OpCode::GetConst16,
            dword:	OpCode::GetConst32,
        }
    }
    pub fn getlocal() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::GetLocal8,
            word:	OpCode::GetLocal16,
            dword:	OpCode::GetLocal32,
        }
    }
    pub fn setlocal() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::SetLocal8,
            word:	OpCode::SetLocal16,
            dword:	OpCode::SetLocal32,
        }
    }
    pub fn getupvalue() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::GetUpvalue8,
            word:	OpCode::GetUpvalue16,
            dword:	OpCode::GetUpvalue32,
        }
    }
    pub fn setupvalue() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::SetUpvalue8,
            word:	OpCode::SetUpvalue16,
            dword:	OpCode::SetUpvalue32,
        }
    }
    pub fn getglobal() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::GetGlobal8,
            word:	OpCode::GetGlobal16,
            dword:	OpCode::GetGlobal32,
        }
    }
    pub fn setglobal() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::SetGlobal8,
            word:	OpCode::SetGlobal16,
            dword:	OpCode::SetGlobal32,
        }
    }
}


