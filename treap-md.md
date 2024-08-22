# Treap (Randomized Binary Search Tree)

A Treap is a randomized binary search tree. It combines the properties of a binary search tree and a heap. Each node has a key (following BST property) and a randomly assigned priority (following max-heap property).

## Implementation

```rust
use std::cmp::Ordering;
use rand::Rng;

struct TreapNode<T: Ord> {
    key: T,
    priority: u32,
    left: Option<Box<TreapNode<T>>>,
    right: Option<Box<TreapNode<T>>>,
}

impl<T: Ord> TreapNode<T> {
    fn new(key: T) -> Self {
        let mut rng = rand::thread_rng();
        TreapNode {
            key,
            priority: rng.gen(),
            left: None,
            right: None,
        }
    }
}

struct Treap<T: Ord> {
    root: Option<Box<TreapNode<T>>>,
}

impl<T: Ord> Treap<T> {
    fn new() -> Self {
        Treap { root: None }
    }

    fn insert(&mut self, key: T) {
        self.root = Self::insert_rec(self.root.take(), key);
    }

    fn insert_rec(mut node: Option<Box<TreapNode<T>>>, key: T) -> Option<Box<TreapNode<T>>> {
        if let Some(mut n) = node {
            match key.cmp(&n.key) {
                Ordering::Less => {
                    n.left = Self::insert_rec(n.left.take(), key);
                    if n.left.as_ref().unwrap().priority > n.priority {
                        return Self::rotate_right(n);
                    }
                }
                Ordering::Greater => {
                    n.right = Self::insert_rec(n.right.take(), key);
                    if n.right.as_ref().unwrap().priority > n.priority {
                        return Self::rotate_left(n);
                    }
                }
                Ordering::Equal => {} // Key already exists, do nothing
            }
            Some(n)
        } else {
            Some(Box::new(TreapNode::new(key)))
        }
    }

    fn rotate_left(mut node: Box<TreapNode<T>>) -> Option<Box<TreapNode<T>>> {
        let mut right = node.right.take().unwrap();
        node.right = right.left.take();
        right.left = Some(node);
        Some(right)
    }

    fn rotate_right(mut node: Box<TreapNode<T>>) -> Option<Box<TreapNode<T>>> {
        let mut left = node.left.take().unwrap();
        node.left = left.right.take();
        left.right = Some(node);
        Some(left)
    }

    fn search(&self, key: &T) -> bool {
        Self::search_rec(&self.root, key)
    }

    fn search_rec(node: &Option<Box<TreapNode<T>>>, key: &T) -> bool {
        match node {
            Some(n) => match key.cmp(&n.key) {
                Ordering::Less => Self::search_rec(&n.left, key),
                Ordering::Greater => Self::search_rec(&n.right, key),
                Ordering::Equal => true,
            },
            None => false,
        }
    }
}

// Usage
fn main() {
    let mut treap = Treap::new();
    treap.insert(5);
    treap.insert(2);
    treap.insert(7);
    treap.insert(1);
    treap.insert(9);

    println!("Search for 7: {}", treap.search(&7));
    println!("Search for 3: {}", treap.search(&3));
}
```

## Key Concepts

1. **Binary Search Tree Property**: Keys are ordered according to the binary search tree invariant.
2. **Heap Property**: Each node has a randomly assigned priority, and the tree maintains a max-heap property based on these priorities.
3. **Randomization**: The random priorities ensure that the tree remains balanced with high probability.
4. **Rotations**: Tree rotations are used to maintain the heap property after insertions.

## When to Use

Use a Treap when:

- You need a balanced binary search tree with simple implementation.
- You want the benefits of randomization in your data structure.
- You need efficient average-case performance for insertions, deletions, and searches.
- You're working on problems that benefit from a dynamically balanced tree structure.

Treaps are particularly useful in:

- Implementing dynamic sets or associative arrays.
- Solving problems in computational geometry.
- Maintaining sorted sequences with frequent modifications.
- Scenarios where you need to avoid worst-case behaviors of deterministic data structures.

The main advantage of Treaps is their simplicity compared to other self-balancing binary search trees (like AVL or Red-Black trees) while still providing good average-case performance. The expected height of a Treap is O(log n), which leads to O(log n) expected time for operations like insertion, deletion, and search.
