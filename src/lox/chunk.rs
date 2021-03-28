
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
}