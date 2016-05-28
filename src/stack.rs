use std::vec::Vec;


pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            stack: Vec::new(),
        }
    }

    #[inline]
    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

impl<T> Stack<T> where T: Clone {
    #[inline]
    pub fn peek(&self) -> Option<T> {
        let top_ptr = self.stack.last();

        match top_ptr {
            None      => None,
            Some(val) => Some(val.clone()),
        }
    }
}