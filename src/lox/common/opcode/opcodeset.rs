
use super::OpCode;

// Note: compiler emit_op_variant() now verifies the .len() of each OpCode
// so testing this module seems like a lot of extra work with little benefit

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


    pub fn defarray() -> OpCodeSet {
        OpCodeSet {
            byte: 	OpCode::DefArray8,
            word:	OpCode::DefArray16,
            dword:	OpCode::DefArray32,
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
