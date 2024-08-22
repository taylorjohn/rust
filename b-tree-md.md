# B-Tree

A B-tree is a self-balancing tree data structure that maintains sorted data and allows searches, sequential access, insertions, and deletions in logarithmic time.

## Implementation

```rust
use std::fmt::Debug;
use std::cmp::Ordering;

const B: usize = 6; // B is the minimum degree of the B-tree
const MAX_KEYS: usize = 2 * B - 1;
const MIN_KEYS: usize = B - 1;

#[derive(Debug)]
struct Node<T: Ord + Debug> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
    leaf: bool,
}

impl<T: Ord + Debug> Node<T> {
    fn new(leaf: bool) -> Self {
        Node {
            keys: Vec::with_capacity(MAX_KEYS),
            children: if leaf { Vec::new() } else { Vec::with_capacity(MAX_KEYS + 1) },
            leaf,
        }
    }
}

pub struct BTree<T: Ord + Debug> {
    root: Node<T>,
}

impl<T: Ord + Debug> BTree<T> {
    pub fn new() -> Self {
        BTree {
            root: Node::new(true),
        }
    }

    pub fn insert(&mut self, key: T) {
        let root = &mut self.root;
        if root.keys.len() == MAX_KEYS {
            let mut new_root = Node::new(false);
            new_root.children.push(std::mem::replace(root, Node::new(true)));
            Self::split_child(&mut new_root, 0);
            self.root = new_root;
            Self::insert_non_full(&mut self.root, key);
        } else {
            Self::insert_non_full(root, key);
        }
    }

    fn insert_non_full(node: &mut Node<T>, key: T) {
        let mut i = node.keys.len();
        if node.leaf {
            node.keys.push(key);
            while i > 0 && key < node.keys[i - 1] {
                node.keys.swap(i, i - 1);
                i -= 1;
            }
        } else {
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }
            if node.children[i].keys.len() == MAX_KEYS {
                Self::split_child(node, i);
                if key > node.keys[i] {
                    i += 1;
                }
            }
            Self::insert_non_full(&mut node.children[i], key);
        }
    }

    fn split_child(parent: &mut Node<T>, index: usize) {
        let child = &mut parent.children[index];
        let mut new_child = Node::new(child.leaf);
        
        new_child.keys = child.keys.split_off(MIN_KEYS);
        
        if !child.leaf {
            new_child.children = child.children.split_off(MIN_KEYS + 1);
        }
        
        parent.keys.insert(index, child.keys.pop().unwrap());