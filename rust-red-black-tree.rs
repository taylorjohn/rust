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

    pub fn contains(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return true,
            }
        }
        false
    }

    pub fn inorder_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::inorder_rec(&self.root, &mut result);
        result
    }

    fn inorder_rec(node: &Option<Box<Node<T>>>, result: &mut Vec<&T>) {
        if let Some(n) = node {
            Self::inorder_rec(&n.left, result);
            result.push(&n.value);
            Self::inorder_rec(&n.right, result);
        }
    }

    // Note: Deletion for Red-Black Trees is complex and omitted for brevity
    // A full implementation would include deletion and rebalancing after deletion
}

fn main() {
    let mut tree = RedBlackTree::new();
    
    // Insert some values
    for &value in &[7, 3, 18, 10, 22, 8, 11, 26, 2, 6, 13] {
        tree.insert(value);
    }

    // Check if values exist
    println!("Contains 18: {}", tree.contains(&18));
    println!("Contains 12: {}", tree.contains(&12));

    // Print inorder traversal
    println!("Inorder traversal: {:?}", tree.inorder_traversal());
}
