
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
    pub fn peek(&self, depth: usize) -> &T {
        let index = self.elements.len() - 1 - depth;
        &self.elements[index]
    }
    pub fn poke(&mut self, element: T, depth: usize) 
        where T: Clone
    {
        let index = self.elements.len() - 1 - depth;
        self.elements[index] = element.clone();
    }
}

