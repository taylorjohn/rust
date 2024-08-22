use std::collections::BinaryHeap;
use std::cmp::Ordering;

// 1. Using Rust's standard BinaryHeap (max-heap)
fn standard_binary_heap_example() {
    let mut heap = BinaryHeap::new();
    
    // Push elements
    heap.push(3);
    heap.push(5);
    heap.push(1);
    heap.push(4);
    
    println!("Max-heap contents:");
    while let Some(item) = heap.pop() {
        println!("{}", item);
    }
}

// 2. Custom MinHeap implementation
#[derive(Debug)]
struct MinHeap<T> {
    heap: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    fn new() -> Self {
        MinHeap { heap: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.heap.push(item);
        self.heapify_up(self.heap.len() - 1);
    }

    fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            None
        } else {
            let last = self.heap.pop().unwrap();
            if !self.heap.is_empty() {
                let first = std::mem::replace(&mut self.heap[0], last);
                self.heapify_down(0);
                Some(first)
            } else {
                Some(last)
            }
        }
    }

    fn peek(&self) -> Option<&T> {
        self.heap.first()
    }

    fn len(&self) -> usize {
        self.heap.len()
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.heap[index] < self.heap[parent] {
                self.heap.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        let len = self.heap.len();
        loop {
            let left_child = 2 * index + 1;
            let right_child = 2 * index + 2;
            let mut smallest = index;

            if left_child < len && self.heap[left_child] < self.heap[smallest] {
                smallest = left_child;
            }
            if right_child < len && self.heap[right_child] < self.heap[smallest] {
                smallest = right_child;
            }

            if smallest != index {
                self.heap.swap(index, smallest);
                index = smallest;
            } else {
                break;
            }
        }
    }
}

// 3. Using a custom struct with BinaryHeap
#[derive(Eq, PartialEq)]
struct Task {
    priority: i32,
    description: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn custom_struct_priority_queue() {
    let mut pq = BinaryHeap::new();
    
    pq.push(Task { priority: 3, description: "Low priority task".to_string() });
    pq.push(Task { priority: 5, description: "High priority task".to_string() });
    pq.push(Task { priority: 1, description: "Very low priority task".to_string() });
    
    println!("Priority Queue with custom struct:");
    while let Some(Task { priority, description }) = pq.pop() {
        println!("Priority: {}, Task: {}", priority, description);
    }
}

fn main() {
    println!("1. Standard BinaryHeap (Max-Heap) Example:");
    standard_binary_heap_example();

    println!("\n2. Custom MinHeap Example:");
    let mut min_heap = MinHeap::new();
    min_heap.push(3);
    min_heap.push(1);
    min_heap.push(4);
    min_heap.push(1);
    min_heap.push(5);
    
    println!("MinHeap contents:");
    while let Some(item) = min_heap.pop() {
        println!("{}", item);
    }

    println!("\n3. Priority Queue with Custom Struct:");
    custom_struct_priority_queue();
}
