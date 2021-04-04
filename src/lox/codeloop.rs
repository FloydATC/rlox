

pub struct CodeLoop {
    continue_addr:	u32,
    scope_depth:	usize,
    break_addrs:	Vec<u32>,
}


impl CodeLoop {
    pub fn new(continue_addr: u32, scope_depth: usize) -> Self {
        Self {
            continue_addr,
            scope_depth,
            break_addrs:	vec![],
        }
    }
    
    pub fn continue_addr(&self) -> u32 {
        return self.continue_addr;
    }
    
    pub fn scope_depth(&self) -> usize {
        return self.scope_depth;
    } 
    
    pub fn add_break(&mut self, break_addr: u32) {
        self.break_addrs.push(break_addr);
    } 
    
    pub fn breaks(&self) -> &Vec<u32> {
        return &self.break_addrs;
    } 
}
