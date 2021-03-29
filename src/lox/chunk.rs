
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
            OpCode::RETURN => self.opcode_immediate(ip),
            _ => self.opcode_immediate(ip), // "**BAD**"
        };
        result += &instruction;
        return result;
    }
    fn opcode_immediate(&self, ip: &mut usize) -> String {
        let mut result = String::new();
        result.push_str(OpCode::name(self.code[*ip]));
        *ip = *ip + 1;
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
