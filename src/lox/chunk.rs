

#[cfg(test)]
mod test;


use super::opcode::OpCode;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Chunk {
    code: Vec<u8>,    
}


impl Chunk {

    pub fn new() -> Chunk {
        Chunk {
            code:	vec![],
        }
    }

    pub fn append_bytes(&mut self, dword: u32, len: usize) {
        if len == 4 {
            self.code.push(((dword >> 24) & 0xff) as u8);
            self.code.push(((dword >> 16) & 0xff) as u8);
        }
        if len == 4 || len == 2 {
            self.code.push(((dword >> 8) & 0xff) as u8);
        }
        if len == 4 || len == 2 || len == 1 {
            self.code.push((dword & 0xff) as u8);
            return;
        }
        panic!("Length must be 1, 2 or 4 bytes; got {}", len);
    }


    pub fn length(&self) -> u32 {
        return self.code.len() as u32;
    }
    
    pub fn read_bytes(&self, index: u32, len: usize) -> u32 {
        //println!("read_bytes(index={}, len={}) from code={:?}", index, len, self.code);
        let mut dword = self.code[(index+0) as usize] as u32;
        if len == 1 { return dword }
        dword = (dword << 8) + self.code[(index+1) as usize] as u32;
        if len == 2 { return dword }
        dword = (dword << 8) + self.code[(index+2) as usize] as u32;
        dword = (dword << 8) + self.code[(index+3) as usize] as u32;
        if len == 4 { return dword }
        panic!("Length must be 1, 2 or 4 bytes; got {}", len);
    }


pub fn write_bytes(&mut self, dword: u32, mut index: u32, len: usize) {
    if len == 4 {
        self.code[index as usize] = ((dword >> 24) & 0xff) as u8;
        index = index + 1;
        self.code[index as usize] = ((dword >> 16) & 0xff) as u8;
        index = index + 1;
    }
    if len == 4 || len == 2 {
        self.code[index as usize] = ((dword >>  8) & 0xff) as u8;
        index = index + 1;
    }
    if len == 4 || len == 2 || len == 1 {
        self.code[index as usize] = ((dword >>  0) & 0xff) as u8;
        //index = index + 1; // Pointless here
        return;
    }
    panic!("Length must be 1, 2 or 4 bytes; got {}", len);
}


}


// Internal methods for chunk disassembly
impl Chunk {
    fn disassemble(&self) -> String {
        let mut result = String::new();
        let mut ip: u32 = 0;
        while ip < self.code.len() as u32 {
            result += &format!("  0x{:04x}  ", ip);
            result += &format!("{}\n", self.opcode(&mut ip));
        }
        return result;
    }
    fn opcode(&self, ip: &mut u32) -> String {
        let mut result = String::new();
        result += &format!("0x{:02x} ", self.code[*ip as usize]);
        let instruction = match self.code[*ip as usize].into() {
            OpCode::Debug 		    => self.opcode_immediate(ip),
            OpCode::Exit 		    => self.opcode_immediate(ip),
            OpCode::Print 		    => self.opcode_immediate(ip),
            OpCode::Return 		    => self.opcode_immediate(ip),
            OpCode::Dup 		    => self.opcode_immediate(ip),

            OpCode::GetConst8 		|
            OpCode::GetConst16 		|
            OpCode::GetConst32 		=> self.opcode_variant(ip),

            OpCode::False 		    => self.opcode_immediate(ip),
            OpCode::Null 		    => self.opcode_immediate(ip),
            OpCode::True 		    => self.opcode_immediate(ip),
            OpCode::NaN 		    => self.opcode_immediate(ip),
            OpCode::Inf 		    => self.opcode_immediate(ip),

            OpCode::GetLocal8 		|
            OpCode::GetLocal16 		|
            OpCode::GetLocal32 		=> self.opcode_variant(ip),
            OpCode::GetUpvalue8 	|
            OpCode::GetUpvalue16 	|
            OpCode::GetUpvalue32 	=> self.opcode_variant(ip),
            OpCode::GetGlobal8 		|
            OpCode::GetGlobal16 	|
            OpCode::GetGlobal32 	=> self.opcode_variant(ip),
            OpCode::GetProperty8 	|
            OpCode::GetProperty16 	|
            OpCode::GetProperty32 	=> self.opcode_variant(ip),
            OpCode::GetSuper8 		|
            OpCode::GetSuper16 	    |
            OpCode::GetSuper32 	    => self.opcode_variant(ip),

            OpCode::DefGlobal8 		|
            OpCode::DefGlobal16 	|
            OpCode::DefGlobal32 	=> self.opcode_variant(ip),
            OpCode::DefArray8 		|
            OpCode::DefArray16 	    |
            OpCode::DefArray32 	    => self.opcode_variant(ip),

            OpCode::SetLocal8 		|
            OpCode::SetLocal16 		|
            OpCode::SetLocal32 		=> self.opcode_variant(ip),
            OpCode::SetUpvalue8 	|
            OpCode::SetUpvalue16 	|
            OpCode::SetUpvalue32 	=> self.opcode_variant(ip),
            OpCode::SetGlobal8 		|
            OpCode::SetGlobal16 	|
            OpCode::SetGlobal32 	=> self.opcode_variant(ip),
            OpCode::SetProperty8 	|
            OpCode::SetProperty16 	|
            OpCode::SetProperty32 	=> self.opcode_variant(ip),

            OpCode::Capture8 		|
            OpCode::Capture16 		|
            OpCode::Capture32 		=> self.opcode_capture(ip), // Note the difference

            OpCode::Class8 		    |
            OpCode::Class16 		|
            OpCode::Class32 		=> self.opcode_variant(ip),
            OpCode::Method8 		|
            OpCode::Method16 		|
            OpCode::Method32 		=> self.opcode_variant(ip),

            OpCode::Not 		    => self.opcode_immediate(ip),
            OpCode::Negate 		    => self.opcode_immediate(ip),

            OpCode::Add 		    => self.opcode_immediate(ip),
            OpCode::Sub 		    => self.opcode_immediate(ip),
            OpCode::Mul 		    => self.opcode_immediate(ip),
            OpCode::Div 		    => self.opcode_immediate(ip),
            OpCode::Mod 		    => self.opcode_immediate(ip),
            OpCode::Equal		    => self.opcode_immediate(ip),
            OpCode::NotEqual		=> self.opcode_immediate(ip),
            OpCode::Less		    => self.opcode_immediate(ip),
            OpCode::Greater		    => self.opcode_immediate(ip),
            OpCode::LessEqual		=> self.opcode_immediate(ip),
            OpCode::GreaterEqual	=> self.opcode_immediate(ip),
            OpCode::Same		    => self.opcode_immediate(ip),

            OpCode::Jmp			    => self.opcode_variant(ip),
            OpCode::JmpFalseP		=> self.opcode_variant(ip),
            OpCode::JmpFalseQ		=> self.opcode_variant(ip),
            OpCode::Call		    => self.opcode_variant(ip),

            OpCode::Pop 		    => self.opcode_immediate(ip),
            OpCode::PopN 		    => self.opcode_variant(ip),
            OpCode::CloseUpvalue	=> self.opcode_immediate(ip),
            OpCode::Inherit	        => self.opcode_immediate(ip),
            OpCode::GetSubscript    => self.opcode_immediate(ip),
            OpCode::SetSubscript    => self.opcode_immediate(ip),

            OpCode::BAD 		    => self.opcode_immediate(ip),
        };
        result += &instruction;
        return result;
    }
    
    // OpCode has no argument
    fn opcode_immediate(&self, ip: &mut u32) -> String {
        let mut result = String::new();
        result.push_str(OpCode::from(self.code[*ip as usize]).mnemonic());
        *ip = *ip + 1;
        return result;
    }

    // Opcode has variable length argument
    fn opcode_variant(&self, ip: &mut u32) -> String {
        let mut result = String::new();
        let opcode = OpCode::from(self.code[*ip as usize]);
        result.push_str(opcode.mnemonic());
        *ip = *ip + 1;
        let arg = self.read_bytes(*ip, opcode.len());
        *ip = *ip + opcode.len() as u32;
        match opcode.len() {
            1 => result = result + &format!(" 0x{:02x}", arg),
            2 => result = result + &format!(" 0x{:04x}", arg),
            4 => result = result + &format!(" 0x{:08x}", arg),
            _ => {}
        }
        return result;
    }


    // I have no idea how to decode these from the viewpoint
    // of a chunk because they require insight into the function
    // that the opcode will operate on.
    fn opcode_capture(&self, ip: &mut u32) -> String {
        let mut result = String::new();
        result = result + self.opcode_variant(ip).as_str();
        result = result + self.opcode_capture_upvalues(ip).as_str();
        return result;
    }
    
    fn opcode_capture_upvalues(&self, _ip: &mut u32) -> String {
        return " // WARNING: Disassembly incomplete".to_string();
    }

}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self.disassemble();
        write!(f, "\n{}", result)
    }
}
