use std::fmt::Display;

// Define the Node structure
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Define the LinkedList structure
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    // Create a new empty linked list
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Insert a new element at the beginning of the list
    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Remove the first element and return it, or None if the list is empty
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    // Get a reference to the first element, or None if the list is empty
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    // Get a mutable reference to the first element, or None if the list is empty
    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    // Insert a new element at the end of the list
    pub fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: None,
        });
        
        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut current) => {
                while let Some(ref mut next) = current.next {
                    current = next;
                }
                current.next = Some(new_node);
            }
        }
    }

    // Get the length of the list
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }

    // Create an iterator over the list
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }

    // Reverse the list in-place
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();
        
        while let Some(mut boxed_node) = current {
            let next = boxed_node.next.take();
            boxed_node.next = prev;
            prev = Some(boxed_node);
            current = next;
        }
        
        self.head = prev;
    }

    // Remove all elements from the list
    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

// Define an iterator for the LinkedList
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

// Implement Display trait for LinkedList
impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements: Vec<String> = self.iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", elements.join(" -> "))
    }
}

fn main() {
    // Create a new linked list
    let mut list: LinkedList<i32> = LinkedList::new();

    // Test push_front and push_back
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.push_back(4);
    println!("List after pushes: {}", list);

    // Test pop_front
    if let Some(value) = list.pop_front() {
        println!("Popped value: {}", value);
    }
    println!("List after pop: {}", list);

    // Test peek_front and peek_front_mut
    if let Some(value) = list.peek_front() {
        println!("Front value: {}", value);
    }
    if let Some(value) = list.peek_front_mut() {
        *value += 10;
    }
    println!("List after modifying front: {}", list);

    // Test len
    println!("List length: {}", list.len());

    // Test reverse
    list.reverse();
    println!("Reversed list: {}", list);

    // Test clear
    list.clear();
    println!("List after clear: {}", list);
    println!("Is list empty? {}", list.is_empty());
}
