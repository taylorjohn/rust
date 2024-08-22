use std::collections::VecDeque;

// Stack Implementation
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

// Queue Implementation
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { items: VecDeque::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn front(&self) -> Option<&T> {
        self.items.front()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

// Implementing Display for Stack and Queue
use std::fmt;

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stack: [")?;
        for (i, item) in self.items.iter().rev().enumerate() {
            if i > 0 { write!(f, ", ")? }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

impl<T: fmt::Display> fmt::Display for Queue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Queue: [")?;
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 { write!(f, ", ")? }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

// Example usage and testing
fn main() {
    // Stack example
    println!("Stack Operations:");
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("{}", stack);
    println!("Pop: {:?}", stack.pop());
    println!("Peek: {:?}", stack.peek());
    println!("Size: {}", stack.size());
    println!("Is empty: {}", stack.is_empty());
    println!("{}", stack);

    println!("\nQueue Operations:");
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    println!("{}", queue);
    println!("Dequeue: {:?}", queue.dequeue());
    println!("Front: {:?}", queue.front());
    println!("Size: {}", queue.size());
    println!("Is empty: {}", queue.is_empty());
    println!("{}", queue);

    // Additional operations to demonstrate more features
    println!("\nAdditional Stack Operations:");
    while !stack.is_empty() {
        println!("Popped: {:?}", stack.pop());
    }
    println!("Is stack empty now? {}", stack.is_empty());

    println!("\nAdditional Queue Operations:");
    queue.enqueue(4);
    queue.enqueue(5);
    println!("After enqueuing 4 and 5: {}", queue);
    while !queue.is_empty() {
        println!("Dequeued: {:?}", queue.dequeue());
    }
    println!("Is queue empty now? {}", queue.is_empty());
}
