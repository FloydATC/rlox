
use super::opcode::OpCode;

#[allow(dead_code)]
pub struct Chunk {
    code: Vec<u8>,    
}


#[allow(dead_code)]
impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code:	vec![],
        }
    }
    pub fn append_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }
    pub fn append_word(&mut self, word: u16) {
        self.code.push((word >> 8) as u8);
        self.code.push((word & 0xff) as u8);
    }
    pub fn append_dword(&mut self, dword: u32) {
        self.code.push(((dword >> 24) & 0xff) as u8);
        self.code.push(((dword >> 16) & 0xff) as u8);
        self.code.push(((dword >> 8) & 0xff) as u8);
        self.code.push((dword & 0xff) as u8);
    }

    pub fn length(&self) -> u32 {
        return self.code.len() as u32;
    }
    
    pub fn read_byte(&self, index: u32) -> u8 {
        return self.code[index as usize];
    }
    
    pub fn read_word(&self, index: u32) -> u16 {
        let mut word = self.code[(index+0) as usize] as u16;
        word = (word << 8) + (self.code[(index+1) as usize] as u16);
        return word;
    }

    pub fn read_dword(&self, index: u32) -> u32 {
        let mut dword = self.code[(index+0) as usize] as u32;
        dword = (dword << 8) + (self.code[(index+1) as usize] as u32);
        dword = (dword << 8) + (self.code[(index+2) as usize] as u32);
        dword = (dword << 8) + (self.code[(index+3) as usize] as u32);
        return dword;
    }
    
    pub fn write_dword(&mut self, dword: u32, index: u32) {
        self.code[(index+0) as usize] = ((dword >> 24) & 0xff) as u8;
        self.code[(index+1) as usize] = ((dword >> 16) & 0xff) as u8;
        self.code[(index+2) as usize] = ((dword >>  8) & 0xff) as u8;
        self.code[(index+3) as usize] = ((dword >>  0) & 0xff) as u8;
    }
}


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
        let instruction = match OpCode::code(self.code[*ip as usize]) {
            OpCode::Print 		=> self.opcode_immediate(ip),
            OpCode::Return 		=> self.opcode_immediate(ip),

            OpCode::GetConst8 		=> self.opcode_byte(ip),
            OpCode::GetConst16 		=> self.opcode_word(ip),
            OpCode::GetConst32 		=> self.opcode_dword(ip),
            OpCode::False 		=> self.opcode_immediate(ip),
            OpCode::Null 		=> self.opcode_immediate(ip),
            OpCode::True 		=> self.opcode_immediate(ip),
            OpCode::GetLocal8 		=> self.opcode_byte(ip),
            OpCode::GetLocal16 		=> self.opcode_word(ip),
            OpCode::GetLocal32 		=> self.opcode_dword(ip),
            OpCode::GetUpvalue8 	=> self.opcode_byte(ip),
            OpCode::GetUpvalue16 	=> self.opcode_word(ip),
            OpCode::GetUpvalue32 	=> self.opcode_dword(ip),
            OpCode::GetGlobal8 		=> self.opcode_byte(ip),
            OpCode::GetGlobal16 	=> self.opcode_word(ip),
            OpCode::GetGlobal32 	=> self.opcode_dword(ip),

            OpCode::DefGlobal8 		=> self.opcode_byte(ip),
            OpCode::DefGlobal16 	=> self.opcode_word(ip),
            OpCode::DefGlobal32 	=> self.opcode_dword(ip),
            OpCode::SetLocal8 		=> self.opcode_byte(ip),
            OpCode::SetLocal16 		=> self.opcode_word(ip),
            OpCode::SetLocal32 		=> self.opcode_dword(ip),
            OpCode::SetUpvalue8 	=> self.opcode_byte(ip),
            OpCode::SetUpvalue16 	=> self.opcode_word(ip),
            OpCode::SetUpvalue32 	=> self.opcode_dword(ip),
            OpCode::SetGlobal8 		=> self.opcode_byte(ip),
            OpCode::SetGlobal16 	=> self.opcode_word(ip),
            OpCode::SetGlobal32 	=> self.opcode_dword(ip),

            OpCode::Not 		=> self.opcode_immediate(ip),
            OpCode::Negate 		=> self.opcode_immediate(ip),

            OpCode::Add 		=> self.opcode_immediate(ip),
            OpCode::Sub 		=> self.opcode_immediate(ip),
            OpCode::Mul 		=> self.opcode_immediate(ip),
            OpCode::Div 		=> self.opcode_immediate(ip),
            OpCode::Mod 		=> self.opcode_immediate(ip),
            OpCode::Equal		=> self.opcode_immediate(ip),
            OpCode::NotEqual		=> self.opcode_immediate(ip),
            OpCode::Less		=> self.opcode_immediate(ip),
            OpCode::Greater		=> self.opcode_immediate(ip),
            OpCode::LessEqual		=> self.opcode_immediate(ip),
            OpCode::GreaterEqual	=> self.opcode_immediate(ip),

            OpCode::Jmp			=> self.opcode_dword(ip),
            OpCode::JmpFalseP		=> self.opcode_dword(ip),
            OpCode::JmpFalseQ		=> self.opcode_dword(ip),

            OpCode::Pop 		=> self.opcode_immediate(ip),
            OpCode::PopN 		=> self.opcode_byte(ip),

            OpCode::BAD 		=> self.opcode_immediate(ip),
        };
        result += &instruction;
        return result;
    }
    
    // OpCode has no argument
    fn opcode_immediate(&self, ip: &mut u32) -> String {
        let mut result = String::new();
        result.push_str(OpCode::name(self.code[*ip as usize]));
        *ip = *ip + 1;
        return result;
    }
    
    // OpCode has one byte argument
    fn opcode_byte(&self, ip: &mut u32) -> String {
        let mut result = String::new();
        result.push_str(OpCode::name(self.code[*ip as usize]));
        *ip = *ip + 1;
        let byte = self.read_byte(*ip);
        *ip = *ip + 1;
        result = result + &format!(" 0x{:02x}", byte);
        return result;
    }

    // OpCode has one word argument
    fn opcode_word(&self, ip: &mut u32) -> String {
        let mut result = String::new();
        result.push_str(OpCode::name(self.code[*ip as usize]));
        *ip = *ip + 1;
        let word = self.read_word(*ip);
        *ip = *ip + 2;
        result = result + &format!(" 0x{:04x}", word);
        return result;
    }

    // OpCode has one dword argument
    fn opcode_dword(&self, ip: &mut u32) -> String {
        let mut result = String::new();
        result.push_str(OpCode::name(self.code[*ip as usize]));
        *ip = *ip + 1;
        let dword = self.read_dword(*ip);
        *ip = *ip + 4;
        result = result + &format!(" 0x{:08x}", dword);
        return result;
    }

//    fn opcode_bad(&self, ip: &mut usize) -> String {
//        let mut result = String::new();
//        result += "**UNKNOWN**";
//        *ip = *ip + 1;
//        return result;
//    }
}


impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self.disassemble();
        write!(f, "\n{}", result)
    }
}


impl Drop for Chunk {
    fn drop(&mut self) {
        println!("Chunk.drop()");
    }
}
