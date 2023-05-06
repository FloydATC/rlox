

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Debug	= 0x00,
    Exit	= 0x01,
    Print	= 0x02,
    Return 	= 0x03,
    
    // Push constant value onto stack
    GetConst8	= 0x10,	// Followed by BYTE indexing table of constants
    GetConst16	= 0x11,	// Followed by WORD indexing table of constants
    GetConst32 	= 0x12,	// Followed by DWORD indexing table of constants
    False	= 0x13,
    Null	= 0x14,
    True	= 0x15,    

    // Push variable value onto stack
    GetLocal8	= 0x20,
    GetLocal16	= 0x21,
    GetLocal32	= 0x22,
    GetUpvalue8	= 0x23,
    GetUpvalue16= 0x24,
    GetUpvalue32= 0x25,
    GetGlobal8	= 0x26,
    GetGlobal16	= 0x27,
    GetGlobal32	= 0x28,
    GetProperty8	= 0x29,
    GetProperty16	= 0x2a,
    GetProperty32	= 0x2b,
    GetSuper8	= 0x2c,
    GetSuper16	= 0x2d,
    GetSuper32	= 0x2f,
    
    // Pop value and put in new variable
    DefGlobal8	= 0x30,	// Followed by BYTE indexing table of globals
    DefGlobal16 = 0x31,	// Followed by WORD indexing table of globals
    DefGlobal32 = 0x32,	// Followed by DWORD indexing table of globals

    // Pop value and put in existing variable
    SetLocal8	= 0x40,
    SetLocal16	= 0x41,
    SetLocal32	= 0x42,
    SetUpvalue8	= 0x43,
    SetUpvalue16= 0x44,
    SetUpvalue32= 0x45,
    SetGlobal8	= 0x46,
    SetGlobal16	= 0x47,
    SetGlobal32	= 0x48,        
    SetProperty8	= 0x49,
    SetProperty16	= 0x4a,
    SetProperty32	= 0x4b,        
    
    // Get constant value (should be function) and push a closure
    Capture8,
    Capture16,
    Capture32,

    // Get constant value (should be a name) and ...
    // ... create a named class
    Class8,	// Followed by BYTE indexing table of constants
    Class16,	// Followed by WORD indexing table of constants
    Class32,	// Followed by DWORD indexing table of constants
    // ... add named method to class
    Method8,	// Followed by BYTE indexing table of constants
    Method16,	// Followed by WORD indexing table of constants
    Method32,	// Followed by DWORD indexing table of constants
    
    // Pop one value, perform operation, push result
    Not,
    Negate,
    
    // Pop two values, perform operation, push result
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // Change instruction pointer
    Jmp,
    JmpFalseP,	// POP, then if false JUMP
    JmpFalseQ,  // PEEK, then if false JUMP
    Call,
        
    // Pop (and discard) one or more values from the stack
    Pop,
    PopN,		// Followed by BYTE indicating number of values
    CloseUpvalue,
    Inherit,
    
    BAD 	= 0xff,	// Unknown/bad opcodes resolve to this
}


impl OpCode {
    pub fn name(byte: u8) -> &'static str {
        let opcode = OpCode::code(byte);

        match opcode {        
            OpCode::Debug		=> { return "DEBUG"; }
            OpCode::Exit		=> { return "EXIT"; }
            OpCode::Print		=> { return "PRINT"; }
            OpCode::Return 		=> { return "RET"; }

            OpCode::GetConst8 		=> { return "GETC"; }
            OpCode::GetConst16 		=> { return "GETC"; }
            OpCode::GetConst32 		=> { return "GETC"; }
            OpCode::False 		=> { return "FALSE"; }
            OpCode::Null 		=> { return "NULL"; }
            OpCode::True 		=> { return "TRUE"; }

            OpCode::GetLocal8 		=> { return "GETL"; }
            OpCode::GetLocal16 		=> { return "GETL"; }
            OpCode::GetLocal32 		=> { return "GETL"; }
            OpCode::GetUpvalue8 	=> { return "GETU"; }
            OpCode::GetUpvalue16 	=> { return "GETU"; }
            OpCode::GetUpvalue32 	=> { return "GETU"; }
            OpCode::GetGlobal8 		=> { return "GETG"; }
            OpCode::GetGlobal16 	=> { return "GETG"; }
            OpCode::GetGlobal32 	=> { return "GETG"; }
            OpCode::GetProperty8 	=> { return "GETP"; }
            OpCode::GetProperty16 	=> { return "GETP"; }
            OpCode::GetProperty32 	=> { return "GETP"; }
            OpCode::GetSuper8 	=> { return "GETS"; }
            OpCode::GetSuper16 	=> { return "GETS"; }
            OpCode::GetSuper32 	=> { return "GETS"; }
        
            OpCode::DefGlobal8 		=> { return "DEFG"; }
            OpCode::DefGlobal16 	=> { return "DEFG"; }
            OpCode::DefGlobal32 	=> { return "DEFG"; }

            OpCode::SetLocal8 		=> { return "SETL"; }
            OpCode::SetLocal16 		=> { return "SETL"; }
            OpCode::SetLocal32 		=> { return "SETL"; }
            OpCode::SetUpvalue8 	=> { return "SETU"; }
            OpCode::SetUpvalue16 	=> { return "SETU"; }
            OpCode::SetUpvalue32 	=> { return "SETU"; }
            OpCode::SetGlobal8 		=> { return "SETG"; }
            OpCode::SetGlobal16 	=> { return "SETG"; }
            OpCode::SetGlobal32 	=> { return "SETG"; }
            OpCode::SetProperty8	=> { return "SETP"; }
            OpCode::SetProperty16 	=> { return "SETP"; }
            OpCode::SetProperty32 	=> { return "SETP"; }

            OpCode::Capture8	 	=> { return "CAP"; }
            OpCode::Capture16		=> { return "CAP"; }
            OpCode::Capture32		=> { return "CAP"; }

            OpCode::Class8	 	=> { return "CLASS"; }
            OpCode::Class16		=> { return "CLASS"; }
            OpCode::Class32		=> { return "CLASS"; }
            OpCode::Method8	 	=> { return "MTHD"; }
            OpCode::Method16		=> { return "MTHD"; }
            OpCode::Method32		=> { return "MTHD"; }

            OpCode::Not			=> { return "NOT"; }
            OpCode::Negate		=> { return "NEG"; }

            OpCode::Add 		=> { return "ADD"; }
            OpCode::Sub 		=> { return "SUB"; }
            OpCode::Mul 		=> { return "MUL"; }
            OpCode::Div 		=> { return "DIV"; }
            OpCode::Mod 		=> { return "MOD"; }
            OpCode::Equal 		=> { return "EQ"; }
            OpCode::NotEqual 		=> { return "NEQ"; }
            OpCode::Less 		=> { return "LT"; }
            OpCode::Greater 		=> { return "GT"; }
            OpCode::LessEqual 		=> { return "LEQ"; }
            OpCode::GreaterEqual 	=> { return "GEQ"; }
        
            OpCode::Jmp 		=> { return "JMP"; }
            OpCode::JmpFalseP 		=> { return "JFP"; }
            OpCode::JmpFalseQ 		=> { return "JFQ"; }
            OpCode::Call 		=> { return "CALL"; }

            OpCode::Pop 		=> { return "POP"; }
            OpCode::PopN 		=> { return "POP"; }
            OpCode::CloseUpvalue	=> { return "CLOSE"; }
            OpCode::Inherit     => { return "INHRT"; }
            
            OpCode::BAD 		=> { return "???"; }
        }
    }
    pub fn code(byte: u8) -> OpCode {
        if byte == OpCode::Debug as u8 { return OpCode::Debug; }
        if byte == OpCode::Exit as u8 { return OpCode::Exit; }
        if byte == OpCode::Print as u8 { return OpCode::Print; }
        if byte == OpCode::Return as u8 { return OpCode::Return; }

        if byte == OpCode::GetConst8 as u8 { return OpCode::GetConst8; }
        if byte == OpCode::GetConst16 as u8 { return OpCode::GetConst16; }
        if byte == OpCode::GetConst32 as u8 { return OpCode::GetConst32; }
        if byte == OpCode::False as u8 { return OpCode::False; }
        if byte == OpCode::Null as u8 { return OpCode::Null; }
        if byte == OpCode::True as u8 { return OpCode::True; }
        
        if byte == OpCode::GetLocal8 as u8  { return OpCode::GetLocal8; }
        if byte == OpCode::GetLocal16 as u8  { return OpCode::GetLocal16; }
        if byte == OpCode::GetLocal32 as u8  { return OpCode::GetLocal32; }
        if byte == OpCode::GetUpvalue8 as u8  { return OpCode::GetUpvalue8; }
        if byte == OpCode::GetUpvalue16 as u8  { return OpCode::GetUpvalue16; }
        if byte == OpCode::GetUpvalue32 as u8  { return OpCode::GetUpvalue32; }
        if byte == OpCode::GetGlobal8 as u8  { return OpCode::GetGlobal8; }
        if byte == OpCode::GetGlobal16 as u8  { return OpCode::GetGlobal16; }
        if byte == OpCode::GetGlobal32 as u8  { return OpCode::GetGlobal32; }
        if byte == OpCode::GetProperty8 as u8  { return OpCode::GetProperty8; }
        if byte == OpCode::GetProperty16 as u8  { return OpCode::GetProperty16; }
        if byte == OpCode::GetProperty32 as u8  { return OpCode::GetProperty32; }
        if byte == OpCode::GetSuper8 as u8  { return OpCode::GetSuper8; }
        if byte == OpCode::GetSuper16 as u8  { return OpCode::GetSuper16; }
        if byte == OpCode::GetSuper32 as u8  { return OpCode::GetSuper32; }
        
        if byte == OpCode::DefGlobal8 as u8 { return OpCode::DefGlobal8; }
        if byte == OpCode::DefGlobal16 as u8 { return OpCode::DefGlobal16; }
        if byte == OpCode::DefGlobal32 as u8 { return OpCode::DefGlobal32; }

        if byte == OpCode::SetLocal8 as u8  { return OpCode::SetLocal8; }
        if byte == OpCode::SetLocal16 as u8  { return OpCode::SetLocal16; }
        if byte == OpCode::SetLocal32 as u8  { return OpCode::SetLocal32; }
        if byte == OpCode::SetUpvalue8 as u8  { return OpCode::SetUpvalue8; }
        if byte == OpCode::SetUpvalue16 as u8  { return OpCode::SetUpvalue16; }
        if byte == OpCode::SetUpvalue32 as u8  { return OpCode::SetUpvalue32; }
        if byte == OpCode::SetGlobal8 as u8  { return OpCode::SetGlobal8; }
        if byte == OpCode::SetGlobal16 as u8  { return OpCode::SetGlobal16; }
        if byte == OpCode::SetGlobal32 as u8  { return OpCode::SetGlobal32; }
        if byte == OpCode::SetProperty8 as u8  { return OpCode::SetProperty8; }
        if byte == OpCode::SetProperty16 as u8  { return OpCode::SetProperty16; }
        if byte == OpCode::SetProperty32 as u8  { return OpCode::SetProperty32; }

        if byte == OpCode::Capture8 as u8  { return OpCode::Capture8; }
        if byte == OpCode::Capture16 as u8  { return OpCode::Capture16; }
        if byte == OpCode::Capture32 as u8  { return OpCode::Capture32; }
        
        if byte == OpCode::Class8 as u8  { return OpCode::Class8; }
        if byte == OpCode::Class16 as u8  { return OpCode::Class16; }
        if byte == OpCode::Class32 as u8  { return OpCode::Class32; }
        if byte == OpCode::Method8 as u8  { return OpCode::Method8; }
        if byte == OpCode::Method16 as u8  { return OpCode::Method16; }
        if byte == OpCode::Method32 as u8  { return OpCode::Method32; }
        
        if byte == OpCode::Not as u8 	{ return OpCode::Not; }
        if byte == OpCode::Negate as u8 { return OpCode::Negate; }

        if byte == OpCode::Add as u8 	{ return OpCode::Add; }
        if byte == OpCode::Sub as u8 	{ return OpCode::Sub; }
        if byte == OpCode::Mul as u8 	{ return OpCode::Mul; }
        if byte == OpCode::Div as u8 	{ return OpCode::Div; }
        if byte == OpCode::Mod as u8 	{ return OpCode::Mod; }
        if byte == OpCode::Equal as u8		{ return OpCode::Equal; }
        if byte == OpCode::NotEqual as u8	{ return OpCode::NotEqual; }
        if byte == OpCode::Less as u8    { return OpCode::Less; }
        if byte == OpCode::Greater as u8    { return OpCode::Greater; }
        if byte == OpCode::LessEqual as u8    { return OpCode::LessEqual; }
        if byte == OpCode::GreaterEqual as u8    { return OpCode::GreaterEqual; }

        if byte == OpCode::Jmp as u8 	{ return OpCode::Jmp; }
        if byte == OpCode::JmpFalseP as u8 	{ return OpCode::JmpFalseP; }
        if byte == OpCode::JmpFalseQ as u8 	{ return OpCode::JmpFalseQ; }
        if byte == OpCode::Call as u8 	{ return OpCode::Call; }

        if byte == OpCode::Pop as u8 	{ return OpCode::Pop; }
        if byte == OpCode::PopN as u8 	{ return OpCode::PopN; }
        if byte == OpCode::CloseUpvalue as u8 	{ return OpCode::CloseUpvalue; }
        if byte == OpCode::Inherit as u8 	{ return OpCode::Inherit; }
        return OpCode::BAD;	// Do not use
    }
}


pub struct OpCodeSet {
    pub byte:	OpCode,
    pub word:	OpCode,
    pub dword:	OpCode,
}


impl OpCodeSet {
    pub fn defglobal() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::DefGlobal8,
            word:	OpCode::DefGlobal16,
            dword:	OpCode::DefGlobal32,
        }
    }
    pub fn capture() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::Capture8,
            word:	OpCode::Capture16,
            dword:	OpCode::Capture32,
        }
    }
    pub fn class() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::Class8,
            word:	OpCode::Class16,
            dword:	OpCode::Class32,
        }
    }
    pub fn method() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::Method8,
            word:	OpCode::Method16,
            dword:	OpCode::Method32,
        }
    }
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
    pub fn getproperty() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::GetProperty8,
            word:	OpCode::GetProperty16,
            dword:	OpCode::GetProperty32,
        }
    }
    pub fn setproperty() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::SetProperty8,
            word:	OpCode::SetProperty16,
            dword:	OpCode::SetProperty32,
        }
    }
    pub fn get_super() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::GetSuper8,
            word:	OpCode::GetSuper16,
            dword:	OpCode::GetSuper32,
        }
    }
}


