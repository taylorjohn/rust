# Segment Tree

A Segment Tree is a tree data structure used for storing information about intervals, or segments. It allows querying which of the stored segments contain a given point and is particularly efficient for range queries and updates.

## Implementation

```rust
pub struct SegmentTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegmentTree {
    pub fn new(arr: &[i32]) -> Self {
        let n = arr.len();
        let mut tree = vec![0; 4 * n];
        Self::build_tree(arr, &mut tree, 0, 0, n - 1);
        SegmentTree { tree, n }
    }

    fn build_tree(arr: &[i32], tree: &mut Vec<i32>, node: usize, start: usize, end: usize) {
        if start == end {
            tree[node] = arr[start];
            return;
        }
        let mid = (start + end) / 2;
        Self::build_tree(arr, tree, 2 * node + 1, start, mid);
        Self::build_tree(arr, tree, 2 * node + 2, mid + 1, end);
        tree[node] = tree[2 * node + 1] + tree[2 * node + 2];
    }

    pub fn update(&mut self, index: usize, value: i32) {
        self.update_tree(0, 0, self.n - 1, index, value);
    }

    fn update_tree(&mut self, node: usize, start: usize, end: usize, index: usize, value: i32) {
        if start == end {
            self.tree[node] = value;
            return;
        }
        let mid = (start + end) / 2;
        if index <= mid {
            self.update_tree(2 * node + 1, start, mid, index, value);
        } else {
            self.update_tree(2 * node + 2, mid + 1, end, index, value);
        }
        self.tree[node] = self.tree[2 * node + 1] + self.tree[2 * node + 2];
    }

    pub fn query(&self, left: usize, right: usize) -> i32 {
        self.query_tree(0, 0, self.n - 1, left, right)
    }

    fn query_tree(&self, node: usize, start: usize, end: usize, left: usize, right: usize) -> i32 {
        if left > end || right < start {
            return 0;
        }
        if left <= start && end <= right {
            return self.tree[node];
        }
        let mid = (start + end) / 2;
        let left_sum = self.query_tree(2 * node + 1, start, mid, left, right);
        let right_sum = self.query_tree(2 * node + 2, mid + 1, end, left, right);
        left_sum + right_sum
    }
}

// Usage
fn main() {
    let arr = vec![1, 3, 5, 7, 9, 11];
    let mut seg_tree = SegmentTree::new(&arr);
    
    println!("Sum of range [1, 3]: {}", seg_tree.query(1, 3));
    seg_tree.update(2, 10);
    println!("Sum of range [1, 3] after update: {}", seg_tree.query(1, 3));
}
```

## Key Concepts

1. **Tree Structure**: Represents intervals as a binary tree.
2. **Range Queries**: Efficiently answers queries over a range of indices.
3. **Lazy Propagation**: An optimization technique (not implemented here) for handling range updates efficiently.
4. **Recursive Construction**: The tree is built and queried recursively.

## When to Use

Use a Segment Tree when:

- You need to perform range queries (like sum, minimum, maximum) on an array.
- The array is subject to frequent updates.
- You require efficient (logarithmic time) query and update operations.

Segment Trees are particularly useful in:

- Computational geometry for range searching
- Geographic information systems
- Competitive programming for solving range query problems
- Database systems for range queries

The main advantage of Segment Trees is their ability to perform both range queries and element updates in O(log n) time, making them very efficient for scenarios with frequent queries and updates on ranges.
