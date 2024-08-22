# Segment Tree with Lazy Propagation

A Segment Tree with Lazy Propagation is an advanced version of the Segment Tree that allows for efficient range updates in addition to range queries. Lazy Propagation defers updates to children nodes until they are needed.

## Implementation

```rust
struct SegmentTree {
    tree: Vec<i64>,
    lazy: Vec<i64>,
    n: usize,
}

impl SegmentTree {
    fn new(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut tree = vec![0; 4 * n];
        let lazy = vec![0; 4 * n];
        let mut st = SegmentTree { tree, lazy, n };
        st.build(arr, 1, 0, n - 1);
        st
    }

    fn build(&mut self, arr: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = arr[start];
            return;
        }
        let mid = (start + end) / 2;
        self.build(arr, 2 * node, start, mid);
        self.build(arr, 2 * node + 1, mid + 1, end);
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    fn update_range(&mut self, left: usize, right: usize, value: i64) {
        self.update_range_util(1, 0, self.n - 1, left, right, value);
    }

    fn update_range_util(&mut self, node: usize, start: usize, end: usize, left: usize, right: usize, value: i64) {
        self.propagate(node, start, end);

        if start > right || end < left {
            return;
        }

        if start >= left && end <= right {
            self.tree[node] += (end - start + 1) as i64 * value;
            if start != end {
                self.lazy[2 * node] += value;
                self.lazy[2 * node + 1] += value;
            }
            return;
        }

        let mid = (start + end) / 2;
        self.update_range_util(2 * node, start, mid, left, right, value);
        self.update_range_util(2 * node + 1, mid + 1, end, left, right, value);
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    fn query_range(&mut self, left: usize, right: usize) -> i64 {
        self.query_range_util(1, 0, self.n - 1, left, right)
    }

    fn query_range_util(&mut self, node: usize, start: usize, end: usize, left: usize, right: usize) -> i64 {
        self.propagate(node, start, end);

        if start > right || end < left {
            return 0;
        }

        if start >= left && end <= right {
            return self.tree[node];
        }

        let mid = (start + end) / 2;
        let left_sum = self.query_range_util(2 * node, start, mid, left, right);
        let right_sum = self.query_range_util(2 * node + 1, mid + 1, end, left, right);
        left_sum + right_sum
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] != 0 {
            self.tree[node] += (end - start + 1) as i64 * self.lazy[node];
            if start != end {
                self.lazy[2 * node] += self.lazy[node];
                self.lazy[2 * node + 1] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }
    }
}

// Usage
fn main() {
    let arr = vec![1, 3, 5, 7, 9, 11];
    let mut st = SegmentTree::new(&arr);

    println!("Sum of range [1, 3]: {}", st.query_range(1, 3));
    st.update_range(1, 5, 2);
    println!("Sum of range [1, 3] after update: {}", st.query_range(1, 3));
}
```

## Key Concepts

1. **Lazy Propagation**: Defers updates to children nodes until they are needed.
2. **Range Updates**: Efficiently updates a range of elements in O(log N) time.
3. **Range Queries**: Performs range sum queries in O(log N) time.
4. **Propagation**: Pushes lazy updates down the tree when a node is accessed.

## When to Use

Use a Segment Tree with Lazy Propagation when:

- You need to perform both range updates and range queries efficiently.
- Working with large arrays where both element values and ranges need frequent modifications.
- Solving problems that involve cumulative operations over ranges.

Segment Trees with Lazy Propagation are particularly useful in:

- Competitive programming for solving range update and query problems.
- Game development for efficiently managing and querying game state over ranges.
- Financial applications for managing and querying time series data.
- Geographic Information Systems (GIS) for efficient spatial data management.

The main advantage of this implementation is its ability to handle both range updates and queries in O(log N) time, making it very efficient for scenarios with frequent range operations on large datasets.
