# Splay Tree

A Splay Tree is a self-adjusting binary search tree with the additional property that recently accessed elements are quick to access again. It performs basic operations such as insertion, look-up and removal in O(log n) amortized time.

## Implementation

```rust
use std::cmp::Ordering;

struct Node<T: Ord> {
    key: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(key: T) -> Self {
        Node { key, left: None, right: None }
    }
}

struct SplayTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> SplayTree<T> {
    fn new() -> Self {
        SplayTree { root: None }
    }

    fn insert(&mut self, key: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(key)));
            return;
        }
        self.splay(&key);
        let root = self.root.take().unwrap();
        match key.cmp(&root.key) {
            Ordering::Equal => self.root = Some(root),
            Ordering::Less => {
                let mut new_root = Box::new(Node::new(key));
                new_root.right = Some(root);
                new_root.left = root.left.take();
                self.root = Some(new_root);
            }
            Ordering::Greater => {
                let mut new_root = Box::new(Node::new(key));
                new_root.left = Some(root);
                new_root.right = root.right.take();
                self.root = Some(new_root);
            }
        }
    }

    fn contains(&mut self, key: &T) -> bool {
        self.splay(key);
        self.root.as_ref().map_or(false, |node| node.key == *key)
    }

    fn splay(&mut self, key: &T) {
        if self.root.is_none() {
            return;
        }
        let (mut left, mut right) = (None, None);
        let (mut left_max, mut right_min) = (&mut left, &mut right);
        let mut root = self.root.take().unwrap();
        loop {
            match key.cmp(&root.key) {
                Ordering::Equal => break,
                Ordering::Less => {
                    if let Some(mut left_child) = root.left.take() {
                        if key < &left_child.key {
                            let tmp = left_child.left.take();
                            left_child.left = Some(Box::new(Node::new(root.key)));
                            left_child.left.as_mut().unwrap().right = root.right;
                            root = tmp.unwrap();
                            left_child.left.as_mut().unwrap().left = Some(left_child);
                        } else if key > &left_child.key {
                            let tmp = left_child.right.take();
                            left_child.right = Some(Box::new(Node::new(root.key)));
                            left_child.right.as_mut().unwrap().right