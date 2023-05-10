

#[cfg(test)]
mod test;


#[allow(dead_code)]
pub struct Stack<T> {
    elements: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { elements: vec![] }
    }
}

#[allow(dead_code)]
impl<T> Stack<T> {

    pub fn push(&mut self, element: T) {
        self.elements.push(element);
    }

    pub fn pop(&mut self) -> T {
        self.elements.pop()
            .expect("Stack underflow; tried to pop() from empty stack")
    }

    // Index from the TOP of the stack
    pub fn peek(&self, depth: usize) -> &T {
        let index = self.elements.len() - 1 - depth;
        //println!("Stack.peek() len{}-1-depth{}=index{}", self.elements.len(), depth, index);
        &self.elements[index]
    }

    // Index from the TOP of the stack
    pub fn poke(&mut self, element: T, depth: usize) 
    {
        let index = self.elements.len() - 1 - depth;
        //println!("Stack.poke() len{}-1-depth{}=index{}", self.elements.len(), depth, index);
        self.elements[index] = element;
    }
    
    // Index from the BOTTOM of the stack
    pub fn peek_addr(&self, addr: usize) -> &T {
        &self.elements[addr]
    }

    // Index from the BOTTOM of the stack
    pub fn poke_addr(&mut self, element: T, addr: usize) 
    {
        self.elements[addr] = element;
    }
    
    pub fn size(&self) -> usize {
        return self.elements.len();
    }
    
    pub fn top(&self) -> usize {
        // Panic if the stack is empty
        return self.elements.len() - 1;
    }


    pub fn clear(&mut self) {
        self.elements.clear();
    }

}


impl<T> std::fmt::Debug for Stack<T>
    where T: std::fmt::Display {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        for (id, v) in self.elements.iter().enumerate() {
            write!(f, "  0x{:04x} {}\n", id, v)?;
        }
        Ok(())
    }
}
