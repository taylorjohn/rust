# Persistent Segment Tree

A Persistent Segment Tree is a data structure that allows for efficient range queries and updates while maintaining the history of all previous versions of the tree. This is particularly useful in scenarios where you need to perform operations on historical versions of the data.

## Implementation

```rust
use std::rc::Rc;

#[derive(Clone)]
struct Node {
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
    sum: i64,
}

impl Node {
    fn new(sum: i64) -> Self {
        Node {
            left: None,
            right: None,
            sum,
        }
    }
}

struct PersistentSegmentTree {
    roots: Vec<Rc<Node>>,
}

impl PersistentSegmentTree {
    fn new(arr: &[i64]) -> Self {
        let root = Self::build(arr, 0, arr.len() - 1);
        PersistentSegmentTree { roots: vec![root] }
    }

    fn build(arr: &[i64], start: usize, end: usize) -> Rc<Node> {
        if start == end {
            return Rc::new(Node::new(arr[start]));
        }
        let mid = (start + end) / 2;
        let left = Self::build(arr, start, mid);
        let right = Self::build(arr, mid + 1, end);
        Rc::new(Node {
            left: Some(left.clone()),
            right: Some(right.clone()),
            sum: left.sum + right.sum,
        })
    }

    fn update(&mut self, version: usize, index: usize, value: i64) {
        let new_root = self.update_recursive(self.roots[version].clone(), 0, self.roots[version].sum as usize - 1, index, value);
        self.roots.push(new_root);
    }

    fn update_recursive(&self, node: Rc<Node>, start: usize, end: usize, index: usize, value: i64) -> Rc<Node> {
        if start == end {
            return Rc::new(Node::new(value));
        }
        let mid = (start + end) / 2;
        let mut new_node = (*node).clone();
        if index <= mid {
            new_node.left = Some(self.update_recursive(node.left.as_ref().unwrap().clone(), start, mid, index, value));
        } else {
            new_node.right = Some(self.update_recursive(node.right.as_ref().unwrap().clone(), mid + 1, end, index, value));
        }
        new_node.sum = new_node.left.as_ref().unwrap().sum + new_node.right.as_ref().unwrap().sum;
        Rc::new(new_node)
    }

    fn query(&self, version: usize, left: usize, right: usize) -> i64 {
        self.query_recursive(self.roots[version].clone(), 0, self.roots[version].sum as usize - 1, left, right)
    }

    fn query_recursive(&self, node: Rc<Node>, start: usize, end: usize, left: usize, right: usize) -> i64 {
        if left > end || right < start {
            return 0;
        }
        if left <= start && end <= right {
            return node.sum;
        }
        let mid = (start + end) / 2;
        self.query_recursive(node.left.as_ref().unwrap().clone(), start, mid, left, right) +
        self.query_recursive(node.right.as_ref().unwrap().clone(), mid + 1, end, left, right)
    }
}

// Usage
fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let mut pst = PersistentSegmentTree::new(&arr);

    println!("Sum of range [1, 3] in version 0: {}", pst.query(0, 1, 3));

    pst.update(0, 2, 10);  // Update index 2 to value 10, creates version 1
    println!("Sum of range [1, 3] in version 1: {}", pst.query(1, 1, 3));
    println!("Sum of range [1, 3] in version 0: {}", pst.query(0, 1, 3));  // Original version remains unchanged
}
```

## Key Concepts

1. **Persistence**: Each update creates a new version of the tree while preserving previous versions.
2. **Path Copying**: Only the nodes along the path from root to the updated leaf are copied and modified.
3. **Version Control**: Maintains a history of all versions, allowing queries on any past version.
4. **Efficient Space Usage**: Despite maintaining multiple versions, space complexity is typically O(log n) per update.

## When to Use

Use a Persistent Segment Tree when:

- You need to perform range queries and point updates efficiently.
- You need to maintain and query historical versions of the data structure.
- Working on problems that involve time travel or undo operations.

Persistent Segment Trees are particularly useful in:

- Version control systems for managing different versions of data.
- Computational geometry for handling dynamic sets of intervals or points.
- Problems involving historical data analysis.
- Implementing undo/redo functionality in applications.

The main advantage of Persistent Segment Trees is their ability to maintain historical versions efficiently, allowing for queries on any past state of the data structure. This comes at the cost of increased space complexity compared to regular segment trees.
