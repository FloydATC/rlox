
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

    pub fn length(&self) -> usize {
        return self.code.len();
    }
    
    pub fn read_byte(&self, index: usize) -> u8 {
        return self.code[index];
    }
}


impl Chunk {
    fn disassemble(&self) -> String {
        let mut result = String::new();
        let mut ip: usize = 0;
        while ip < self.code.len() {
            result += &format!("0x{:08x}  ", ip);
            result += &format!("{}\n", self.opcode(&mut ip));
        }
        return result;
    }
    fn opcode(&self, ip: &mut usize) -> String {
        let mut result = String::new();
        result += &format!("0x{:02x} ", self.code[*ip]);
        let instruction = match OpCode::code(self.code[*ip]) {
            OpCode::Return => self.opcode_immediate(ip),

            OpCode::Const8 => self.opcode_byte(ip),
            OpCode::Const16 => self.opcode_word(ip),
            OpCode::Const32 => self.opcode_dword(ip),

            OpCode::Add => self.opcode_immediate(ip),
            OpCode::Sub => self.opcode_immediate(ip),
            OpCode::Mul => self.opcode_immediate(ip),
            OpCode::Div => self.opcode_immediate(ip),
            OpCode::Mod => self.opcode_immediate(ip),

            OpCode::Pop => self.opcode_immediate(ip),
            OpCode::PopN => self.opcode_byte(ip),

            //OpCode::Push => self.opcode_byte(ip),
            _ => self.opcode_immediate(ip), // "**BAD**"
        };
        result += &instruction;
        return result;
    }
    
    // OpCode has no argument
    fn opcode_immediate(&self, ip: &mut usize) -> String {
        let mut result = String::new();
        result.push_str(OpCode::name(self.code[*ip]));
        *ip = *ip + 1;
        return result;
    }
    
    // OpCode has one byte argument
    fn opcode_byte(&self, ip: &mut usize) -> String {
        let mut result = String::new();
        result.push_str(OpCode::name(self.code[*ip]));
        let byte = self.code[*ip+1];
        result = result + &format!(" 0x{:02x}", byte);
        *ip = *ip + 2;
        return result;
    }

    // OpCode has one word argument
    fn opcode_word(&self, ip: &mut usize) -> String {
        let mut result = String::new();
        result.push_str(OpCode::name(self.code[*ip]));
        let mut word = self.code[*ip+1] as u16;
        word = (word << 8) + (self.code[*ip+2] as u16);
        result = result + &format!(" 0x{:04x}", word);
        *ip = *ip + 3;
        return result;
    }

    // OpCode has one dword argument
    fn opcode_dword(&self, ip: &mut usize) -> String {
        let mut result = String::new();
        result.push_str(OpCode::name(self.code[*ip]));
        let mut dword = self.code[*ip+1] as u32;
        dword = (dword << 8) + (self.code[*ip+2] as u32);
        dword = (dword << 8) + (self.code[*ip+3] as u32);
        dword = (dword << 8) + (self.code[*ip+4] as u32);
        result = result + &format!(" 0x{:08x}", dword);
        *ip = *ip + 5;
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
