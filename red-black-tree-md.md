# Red-Black Tree

A Red-Black tree is a type of self-balancing binary search tree where each node has an extra bit for denoting the color of the node, either red or black.

## Implementation

```rust
use std::cmp::Ordering;
use std::mem;

#[derive(Clone, Copy, PartialEq)]
enum Color {
    Red,
    Black,
}

struct Node<T: Ord> {
    value: T,
    color: Color,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            color: Color::Red,
            left: None,
            right: None,
        }
    }
}

pub struct RedBlackTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        let mut root = mem::replace(&mut self.root, None);
        root = Self::insert_rec(root, value);
        if let Some(ref mut node) = root {
            node.color = Color::Black;
        }
        self.root = root;
    }

    fn insert_rec(mut node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>> {
        match node {
            None => Some(Box::new(Node::new(value))),
            Some(mut n) => {
                match value.cmp(&n.value) {
                    Ordering::Less => {
                        n.left = Self::insert_rec(n.left.take(), value);
                        Self::balance_after_insert(n)
                    }
                    Ordering::Greater => {
                        n.right = Self::insert_rec(n.right.take(), value);
                        Self::balance_after_insert(n)
                    }
                    Ordering::Equal => Some(n),
                }
            }
        }
    }

    fn balance_after_insert(mut node: Box<Node<T>>) -> Option<Box<Node<T>>> {
        if Self::is_red(&node.right) && !Self::is_red(&node.left) {
            node = Self::rotate_left(node);
        }
        if Self::is_red(&node.left) && Self::is_red(&node.left.as_ref().unwrap().left) {
            node = Self::rotate_right(node);
        }
        if Self::is_red(&node.left) && Self::is_red(&node.right) {
            Self::flip_colors(&mut node);
        }
        Some(node)
    }

    fn is_red(node: &Option<Box<Node<T>>>) -> bool {
        node.as_ref().map_or(false, |n| n.color == Color::Red)
    }

    fn rotate_left(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut new_root = node.right.take().unwrap();
        node.right = new_root.left.take();
        new_root.left = Some(node);
        new_root.color = new_root.left.as_ref().unwrap().color;
        new_root.left.as_mut().unwrap().color = Color::Red;
        new_root
    }

    fn rotate_right(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut new_root = node.left.take().unwrap();
        node.left = new_root.right.take();
        new_root.right = Some(node);
        new_root.color = new_root.right.as_ref().unwrap().color;
        new_root.right.as_mut().unwrap().color = Color::Red;
        new_root
    }

    fn flip_colors(node: &mut Box<Node<T>>) {
        node.color = Color::Red;
        if let Some(ref mut left) = node.left {
            left.color = Color::Black;
        }
        if let Some(ref mut right) = node.right {
            right.color = Color::Black;
        }
    }

    // Additional methods (delete, search, etc.) would be implemented here
}
```

## Key Concepts

1. **Color Property**: Each node is either red or black.
2. **Root Property**: The root is always black.
3. **Red Property**: Red nodes cannot have red children.
4. **Depth Property**: Every path from a node to its descendant leaves contains the same number of black nodes.
5. **Balancing Operations**: Includes left rotation, right rotation, and color flipping.

## When to Use

Use a Red-Black tree when:

- You need guaranteed O(log n) time complexity for insertions, deletions, and searches.
- You expect frequent insertions and deletions.
- You can tolerate slightly slower lookups compared to an AVL tree.
- You need a balanced tree structure, but not necessarily as strictly balanced as an AVL tree.

Red-Black trees are often used in the implementation of associative arrays, such as the `std::map` in C++ and `TreeMap` in Java. They're also used in process scheduling in Linux and in the implementation of some database index structures.
