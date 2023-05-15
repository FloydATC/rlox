

use num_enum::FromPrimitive; // let opcode: OpCode = byte.into();


#[cfg(test)]
mod test;

mod opcodeset;

pub use opcodeset::OpCodeSet;


#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug, Default, FromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    Debug	        = 0x00,
    Exit	        = 0x01,
    Print	        = 0x02,
    Return 	        = 0x03,
    
    // Push constant value onto stack
    GetConst8	    = 0x10,	// Followed by BYTE indexing table of constants
    GetConst16	    = 0x11,	// Followed by WORD indexing table of constants
    GetConst32 	    = 0x12,	// Followed by DWORD indexing table of constants
    False	        = 0x13,
    Null	        = 0x14,
    True	        = 0x15,
    NaN             = 0x16,
    Inf             = 0x17,    

    // Push variable value onto stack
    GetLocal8	    = 0x20,
    GetLocal16	    = 0x21,
    GetLocal32	    = 0x22,
    GetUpvalue8	    = 0x23,
    GetUpvalue16    = 0x24,
    GetUpvalue32    = 0x25,
    GetGlobal8	    = 0x26,
    GetGlobal16	    = 0x27,
    GetGlobal32	    = 0x28,
    GetProperty8	= 0x29,
    GetProperty16	= 0x2a,
    GetProperty32	= 0x2b,
    GetSuper8	    = 0x2c,
    GetSuper16	    = 0x2d,
    GetSuper32	    = 0x2f,
    
    // Pop value and put in new variable
    DefGlobal8	    = 0x30,	// Followed by BYTE indexing table of globals
    DefGlobal16     = 0x31,	// Followed by WORD indexing table of globals
    DefGlobal32     = 0x32,	// Followed by DWORD indexing table of globals
    // Pop values and put in new variable
    DefArray8,              // Followed by BYTE with element count
    DefArray16,             // Followed by WORD with element count
    DefArray32,	            // Followed by DWORD with element count

    // Pop value and put in existing variable
    SetLocal8	    = 0x40,
    SetLocal16	    = 0x41,
    SetLocal32	    = 0x42,
    SetUpvalue8	    = 0x43,
    SetUpvalue16    = 0x44,
    SetUpvalue32    = 0x45,
    SetGlobal8	    = 0x46,
    SetGlobal16	    = 0x47,
    SetGlobal32	    = 0x48,        
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
    Same,

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
    GetSubscript,
    SetSubscript,
    
    #[default]
    BAD 	        = 0xff,	// Unknown/bad opcodes resolve to this
}


impl OpCode {

    pub fn mnemonic(&self) -> &str {
        match self {        
            OpCode::Debug           => "DEBUG",
            OpCode::Exit            => "EXIT",
            OpCode::Print           => "PRINT",
            OpCode::Return          => "RET",

            OpCode::GetConst8       => "GETC",
            OpCode::GetConst16      => "GETC",
            OpCode::GetConst32 		=> "GETC",
            OpCode::False 		    => "FALSE",
            OpCode::Null 		    => "NULL",
            OpCode::True 		    => "TRUE",
            OpCode::NaN             => "NAN",
            OpCode::Inf             => "INF",

            OpCode::GetLocal8 		=> "GETL",
            OpCode::GetLocal16 		=> "GETL",
            OpCode::GetLocal32 		=> "GETL",
            OpCode::GetUpvalue8 	=> "GETU",
            OpCode::GetUpvalue16 	=> "GETU",
            OpCode::GetUpvalue32 	=> "GETU",
            OpCode::GetGlobal8 		=> "GETG",
            OpCode::GetGlobal16 	=> "GETG",
            OpCode::GetGlobal32 	=> "GETG",
            OpCode::GetProperty8 	=> "GETP",
            OpCode::GetProperty16 	=> "GETP",
            OpCode::GetProperty32 	=> "GETP",
            OpCode::GetSuper8 	    => "GETS",
            OpCode::GetSuper16 	    => "GETS",
            OpCode::GetSuper32 	    => "GETS",
        
            OpCode::DefGlobal8 		=> "DEFG",
            OpCode::DefGlobal16 	=> "DEFG",
            OpCode::DefGlobal32 	=> "DEFG",
            OpCode::DefArray8 		=> "DEFA",
            OpCode::DefArray16 	    => "DEFA",
            OpCode::DefArray32 	    => "DEFA",

            OpCode::SetLocal8 		=> "SETL",
            OpCode::SetLocal16 		=> "SETL",
            OpCode::SetLocal32 		=> "SETL",
            OpCode::SetUpvalue8 	=> "SETU",
            OpCode::SetUpvalue16 	=> "SETU",
            OpCode::SetUpvalue32 	=> "SETU",
            OpCode::SetGlobal8 		=> "SETG",
            OpCode::SetGlobal16 	=> "SETG",
            OpCode::SetGlobal32 	=> "SETG",
            OpCode::SetProperty8	=> "SETP",
            OpCode::SetProperty16 	=> "SETP",
            OpCode::SetProperty32 	=> "SETP",

            OpCode::Capture8	 	=> "CAP",
            OpCode::Capture16		=> "CAP",
            OpCode::Capture32		=> "CAP",

            OpCode::Class8	 	    => "CLASS",
            OpCode::Class16		    => "CLASS",
            OpCode::Class32		    => "CLASS",
            OpCode::Method8	 	    => "MTHD",
            OpCode::Method16		=> "MTHD",
            OpCode::Method32		=> "MTHD",

            OpCode::Not			    => "NOT",
            OpCode::Negate		    => "NEG",

            OpCode::Add 		    => "ADD",
            OpCode::Sub 		    => "SUB",
            OpCode::Mul 		    => "MUL",
            OpCode::Div 		    => "DIV",
            OpCode::Mod 		    => "MOD",
            OpCode::Equal 		    => "EQ",
            OpCode::NotEqual 		=> "NEQ",
            OpCode::Less 		    => "LT",
            OpCode::Greater 		=> "GT",
            OpCode::LessEqual 		=> "LEQ",
            OpCode::GreaterEqual 	=> "GEQ",
            OpCode::Same 		    => "SAME",
        
            OpCode::Jmp 		    => "JMP",
            OpCode::JmpFalseP 		=> "JFP",
            OpCode::JmpFalseQ 		=> "JFQ",
            OpCode::Call 		    => "CALL",

            OpCode::Pop 		    => "POP",
            OpCode::PopN 		    => "POP",
            OpCode::CloseUpvalue	=> "CLOSE",
            OpCode::Inherit         => "INHRT",
            OpCode::GetSubscript    => "GSUB",
            OpCode::SetSubscript    => "SSUB",
            
            OpCode::BAD 		    => "???",
        }
    }


    // The "length" of an opcode is how many subsequent bytes should be read from the bytecode chunk
    // Only relevant for opcodes that have 8, 16 or 32 bit variants; others return 0
    // The purpose is to make compiler, chunk disassembly and VM code simpler and less error-prone
    pub fn len(&self) -> usize {
        match self {
            OpCode::GetConst8       => 1,
            OpCode::GetConst16      => 2,
            OpCode::GetConst32 		=> 4,

            OpCode::GetLocal8 		=> 1,
            OpCode::GetLocal16 		=> 2,
            OpCode::GetLocal32 		=> 4,
            OpCode::GetUpvalue8 	=> 1,
            OpCode::GetUpvalue16 	=> 2,
            OpCode::GetUpvalue32 	=> 4,
            OpCode::GetGlobal8 		=> 1,
            OpCode::GetGlobal16 	=> 2,
            OpCode::GetGlobal32 	=> 4,
            OpCode::GetProperty8 	=> 1,
            OpCode::GetProperty16 	=> 2,
            OpCode::GetProperty32 	=> 4,
            OpCode::GetSuper8 	    => 1,
            OpCode::GetSuper16 	    => 2,
            OpCode::GetSuper32 	    => 4,
        
            OpCode::DefGlobal8 		=> 1,
            OpCode::DefGlobal16 	=> 2,
            OpCode::DefGlobal32 	=> 4,
            OpCode::DefArray8 		=> 1,
            OpCode::DefArray16 	    => 2,
            OpCode::DefArray32 	    => 4,

            OpCode::SetLocal8 		=> 1,
            OpCode::SetLocal16 		=> 2,
            OpCode::SetLocal32 		=> 4,
            OpCode::SetUpvalue8 	=> 1,
            OpCode::SetUpvalue16 	=> 2,
            OpCode::SetUpvalue32 	=> 4,
            OpCode::SetGlobal8 		=> 1,
            OpCode::SetGlobal16 	=> 2,
            OpCode::SetGlobal32 	=> 4,
            OpCode::SetProperty8	=> 1,
            OpCode::SetProperty16 	=> 2,
            OpCode::SetProperty32 	=> 4,

            OpCode::Capture8	 	=> 1,
            OpCode::Capture16		=> 2,
            OpCode::Capture32		=> 4,

            OpCode::Class8	 	    => 1,
            OpCode::Class16		    => 2,
            OpCode::Class32		    => 4,
            OpCode::Method8	 	    => 1,
            OpCode::Method16		=> 2,
            OpCode::Method32		=> 4,

            OpCode::Jmp 		    => 4,
            OpCode::JmpFalseP 		=> 4,
            OpCode::JmpFalseQ 		=> 4,
            OpCode::Call 		    => 1, // Number of arguments on the stack

            OpCode::PopN 		    => 1, // Number of values to pop

            _ => 0,
        }
    }


    pub fn as_byte(&self) -> u8 {
        return *self as u8;
    }

}
