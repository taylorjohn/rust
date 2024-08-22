# Heap (Priority Queue)

A heap is a specialized tree-based data structure that satisfies the heap property. In a max heap, for any given node I, the value of I is greater than or equal to the values of its children. A min heap is the opposite.

## Implementation (Min Heap)

```rust
pub struct MinHeap<T> {
    heap: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap { heap: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(item);
        self.heapify_up(self.heap.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
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

    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
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
```

## Key Concepts

1. **Heap Property**: In a min heap, the parent is always smaller than or equal to its children.
2. **Complete Binary Tree**: The heap is represented as a complete binary tree, which allows for efficient storage in an array.
3. **Efficient Operations**: Insertion and extraction of the minimum element are O(log n) operations.
4. **Heapify**: The process of maintaining the heap property after insertion or removal.

## When to Use

Use a Heap (Priority Queue) when:

- You need to efficiently keep track of the minimum (or maximum) element in a collection.
- Implementing algorithms like Dijkstra's shortest path, Prim's minimum spanning tree, or Huffman coding.
- You need to perform repeated extraction of the minimum (or maximum) element.
- Implementing a scheduler where tasks have priorities.

Heaps are particularly useful in:

- Operating system process schedulers
- Bandwidth management in network routers
- Event-driven simulations
- Algorithms for graph problems

The main advantage of a heap is its ability to provide O(1) access to the minimum (or maximum) element and O(log n) insertion and deletion operations.
