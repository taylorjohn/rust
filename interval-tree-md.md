# Interval Tree

An Interval Tree is a data structure that stores intervals and allows for efficient querying of overlaps for a given interval or point.

## Implementation

```rust
use std::cmp::{max, min};

#[derive(Debug)]
struct Interval {
    low: i32,
    high: i32,
}

struct IntervalTreeNode {
    interval: Interval,
    max: i32,
    left: Option<Box<IntervalTreeNode>>,
    right: Option<Box<IntervalTreeNode>>,
}

struct IntervalTree {
    root: Option<Box<IntervalTreeNode>>,
}

impl IntervalTree {
    fn new() -> Self {
        IntervalTree { root: None }
    }

    fn insert(&mut self, interval: Interval) {
        self.root = Self::insert_recursive(self.root.take(), interval);
    }

    fn insert_recursive(node: Option<Box<IntervalTreeNode>>, interval: Interval) -> Option<Box<IntervalTreeNode>> {
        match node {
            None => Some(Box::new(IntervalTreeNode {
                interval: interval.clone(),
                max: interval.high,
                left: None,
                right: None,
            })),
            Some(mut node) => {
                if interval.low < node.interval.low {
                    node.left = Self::insert_recursive(node.left.take(), interval);
                } else {
                    node.right = Self::insert_recursive(node.right.take(), interval);
                }
                node.max = max(node.max, interval.high);
                Some(node)
            }
        }
    }

    fn search_overlapping(&self, interval: &Interval) -> Vec<Interval> {
        let mut result = Vec::new();
        Self::search_overlapping_recursive(&self.root, interval, &mut result);
        result
    }

    fn search_overlapping_recursive(node: &Option<Box<IntervalTreeNode>>, interval: &Interval, result: &mut Vec<Interval>) {
        if let Some(node) = node {
            if Self::do_overlap(&node.interval, interval) {
                result.push(node.interval.clone());
            }
            if let Some(left) = &node.left {
                if left.max >= interval.low {
                    Self::search_overlapping_recursive(&node.left, interval, result);
                }
            }
            Self::search_overlapping_recursive(&node.right, interval, result);
        }
    }

    fn do_overlap(a: &Interval, b: &Interval) -> bool {
        a.low <= b.high && b.low <= a.high
    }
}

fn main() {
    let mut tree = IntervalTree::new();
    tree.insert(Interval { low: 15, high: 20 });
    tree.insert(Interval { low: 10, high: 30 });
    tree.insert(Interval { low: 17, high: 19 });
    tree.insert(Interval { low: 5, high: 20 });
    tree.insert(Interval { low: 12, high: 15 });

    let query = Interval { low: 14, high: 16 };
    let overlapping = tree.search_overlapping(&query);
    println!("Intervals overlapping with {:?}:", query);
    for interval in overlapping {
        println!("{:?}", interval);
    }
}
```

## Explanation

1. Each node in the Interval Tree stores an interval and the maximum end point in its subtree.
2. The tree is organized similar to a binary search tree, but based on the low end of the intervals.
3. The `insert` operation maintains the max value at each node.
4. The `search_overlapping` operation uses the max values to efficiently prune the search space.

## Time Complexity

- Insertion: O(log n) on average for a balanced tree
- Search for overlapping intervals: O(min(n, k log n)), where k is the number of reported intervals

## Use Cases

- Range queries in databases
- Scheduling algorithms
- Computational geometry

Interval Trees are particularly useful when you need to efficiently find all intervals that overlap with a given interval or point. They provide a good balance between query time and update time for interval-based problems.
