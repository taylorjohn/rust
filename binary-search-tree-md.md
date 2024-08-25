# Binary Search Tree (BST)

A Binary Search Tree is a binary tree data structure where each node has at most two children, referred to as the left child and the right child. For each node, all elements in the left subtree are less than the node, and all elements in the right subtree are greater than the node.

## Basic Implementation

```rust
use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct BinarySearchTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, value: T) {
        self.root = self.insert_rec(self.root.take(), value);
    }

    fn insert_rec(&mut self, node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>> {
        match node {
            None => Some(Box::new(Node::new(value))),
            Some(mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Less => node.left = self.insert_rec(node.left.take(), value),
                    Ordering::Greater => node.right = self.insert_rec(node.right.take(), value),
                    Ordering::Equal => {} // Do nothing if value already exists
                }
                Some(node)
            }
        }
    }

    fn contains(&self, value: &T) -> bool {
        self.contains_rec(&self.root, value)
    }

    fn contains_rec(&self, node: &Option<Box<Node<T>>>, value: &T) -> bool {
        match node {
            None => false,
            Some(node) => {
                match value.cmp(&node.value) {
                    Ordering::Less => self.contains_rec(&node.left, value),
                    Ordering::Greater => self.contains_rec(&node.right, value),
                    Ordering::Equal => true,
                }
            }
        }
    }
}

// Usage
fn main() {
    let mut bst = BinarySearchTree::new();
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(1);
    bst.insert(9);

    println!("BST contains 7: {}", bst.contains(&7));
    println!("BST contains 4: {}", bst.contains(&4));
}
```

## Variations

### 1. Self-Balancing BST (AVL Tree)

AVL Trees maintain balance by ensuring that the height difference between left and right subtrees is at most 1.

```rust
use std::cmp::{max, Ordering};

struct AVLNode<T: Ord> {
    value: T,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
    height: i32,
}

impl<T: Ord> AVLNode<T> {
    fn new(value: T) -> Self {
        AVLNode {
            value,
            left: None,
            right: None,
            height: 1,
        }
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn balance_factor(&self) -> i32 {
        let left_height = self.left.as_ref().map_or(0, |n| n.height());
        let right_height = self.right.as_ref().map_or(0, |n| n.height());
        left_height - right_height
    }

    fn update_height(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |n| n.height());
        let right_height = self.right.as_ref().map_or(0, |n| n.height());
        self.height = 1 + max(left_height, right_height);
    }
}

struct AVLTree<T: Ord> {
    root: Option<Box<AVLNode<T>>>,
}

impl<T: Ord> AVLTree<T> {
    fn new() -> Self {
        AVLTree { root: None }
    }

    fn insert(&mut self, value: T) {
        self.root = self.insert_rec(self.root.take(), value);
    }

    fn insert_rec(&mut self, node: Option<Box<AVLNode<T>>>, value: T) -> Option<Box<AVLNode<T>>> {
        let mut node = match node {
            None => return Some(Box::new(AVLNode::new(value))),
            Some(mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Less => node.left = self.insert_rec(node.left.take(), value),
                    Ordering::Greater => node.right = self.insert_rec(node.right.take(), value),
                    Ordering::Equal => return Some(node), // Value already exists
                }
                node
            }
        };

        node.update_height();
        self.balance(node)
    }

    fn balance(&mut self, node: Box<AVLNode<T>>) -> Option<Box<AVLNode<T>>> {
        let balance_factor = node.balance_factor();

        if balance_factor > 1 {
            if node.left.as_ref().unwrap().balance_factor() < 0 {
                let mut node = node;
                node.left = self.rotate_left(node.left.take().unwrap());
                self.rotate_right(node)
            } else {
                self.rotate_right(node)
            }
        } else if balance_factor < -1 {
            if node.right.as_ref().unwrap().balance_factor() > 0 {
                let mut node = node;
                node.right = self.rotate_right(node.right.take().unwrap());
                self.rotate_left(node)
            } else {
                self.rotate_left(node)
            }
        } else {
            Some(node)
        }
    }

    fn rotate_left(&mut self, mut node: Box<AVLNode<T>>) -> Option<Box<AVLNode<T>>> {
        let mut right_child = node.right.take().unwrap();
        node.right = right_child.left.take();
        node.update_height();

        right_child.left = Some(node);
        right_child.update_height();

        Some(right_child)
    }

    fn rotate_right(&mut self, mut node: Box<AVLNode<T>>) -> Option<Box<AVLNode<T>>> {
        let mut left_child = node.left.take().unwrap();
        node.left = left_child.right.take();
        node.update_height();

        left_child.right = Some(node);
        left_child.update_height();

        Some(left_child)
    }
}

// Usage remains similar to the basic BST
```

### 2. Red-Black Tree

Red-Black Trees are another type of self-balancing BST that maintain balance using color properties.

```rust
use std::cmp::Ordering;

enum Color {
    Red,
    Black,
}

struct RBNode<T: Ord> {
    value: T,
    color: Color,
    left: Option<Box<RBNode<T>>>,
    right: Option<Box<RBNode<T>>>,
}

impl<T: Ord> RBNode<T> {
    fn new(value: T) -> Self {
        RBNode {
            value,
            color: Color::Red,
            left: None,
            right: None,
        }
    }
}

struct RedBlackTree<T: Ord> {
    root: Option<Box<RBNode<T>>>,
}

impl<T: Ord> RedBlackTree<T> {
    fn new() -> Self {
        RedBlackTree { root: None }
    }

    fn insert(&mut self, value: T) {
        self.root = self.insert_rec(self.root.take(), value);
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
    }

    fn insert_rec(&mut self, node: Option<Box<RBNode<T>>>, value: T) -> Option<Box<RBNode<T>>> {
        match node {
            None => Some(Box::new(RBNode::new(value))),
            Some(mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Less => node.left = self.insert_rec(node.left.take(), value),
                    Ordering::Greater => node.right = self.insert_rec(node.right.take(), value),
                    Ordering::Equal => return Some(node), // Value already exists
                }
                self.fix_up(node)
            }
        }
    }

    fn fix_up(&mut self, mut node: Box<RBNode<T>>) -> Option<Box<RBNode<T>>> {
        if self.is_red(&node.right) && !self.is_red(&node.left) {
            node = self.rotate_left(node);
        }
        if self.is_red(&node.left) && self.is_red(&node.left.as_ref().unwrap().left) {
            node = self.rotate_right(node);
        }
        if self.is_red(&node.left) && self.is_red(&node.right) {
            self.flip_colors(&mut node);
        }
        Some(node)
    }

    fn is_red(&self, node: &Option<Box<RBNode<T>>>) -> bool {
        match node {
            Some(n) => matches!(n.color, Color::Red),
            None => false,
        }
    }

    fn rotate_left(&mut self, mut node: Box<RBNode<T>>) -> Box<RBNode<T>> {
        let mut right = node.right.take().unwrap();
        node.right = right.left.take();
        right.left = Some(node);
        right.color = right.left.as_ref().unwrap().color;
        right.left.as_mut().unwrap().color = Color::Red;
        right
    }

    fn rotate_right(&mut self, mut node: Box<RBNode<T>>) -> Box<RBNode<T>> {
        let mut left = node.left.take().unwrap();
        node.left = left.right.take();
        left.right = Some(node);
        left.color = left.right.as_ref().unwrap().color;
        left.right.as_mut().unwrap().color = Color::Red;
        left
    }

    fn flip_colors(&mut self, node: &mut Box<RBNode<T>>) {
        node.color = Color::Red;
        if let Some(ref mut left) = node.left {
            left.color = Color::Black;
        }
        if let Some(ref mut right) = node.right {
            right.color = Color::Black;
        }
    }
}

// Usage remains similar to the basic BST
```

## When to Use

Use Binary Search Trees when:

- You need to maintain a sorted collection of elements.
- You frequently perform insertions, deletions, and searches.
- You want an efficient data structure for in-order traversal.

Use self-balancing variants (AVL or Red-Black) when:

- You need guaranteed O(log n) time complexity for operations.
- The tree may become unbalanced due to insertion/deletion patterns.

BSTs are particularly useful in:

- Implementing associative arrays (dictionaries)
- Database indexing
- Maintaining sorted data for efficient searching and traversal

## Time Complexity

For a balanced BST:
- Search: O(log n)
- Insert: O(log n)
- Delete: O(log n)

For an unbalanced BST, these operations can degrade to O(n) in the worst case.

Self-balancing variants (AVL and Red-Black Trees) guarantee O(log n) time complexity for all operations.

## Space Complexity

O(n) for all variants, where n is the number of nodes in the tree.

The choice between basic BST, AVL Tree, and Red-Black Tree depends on the specific requirements of your application, particularly regarding the frequency of insertions and deletions versus searches.
