use std::cmp::max;

// Node structure for AVL tree
struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    height: i32,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
            height: 1,
        }
    }
}

// AVL Tree structure
pub struct AVLTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    // Get the height of a node
    fn height(node: &Option<Box<Node<T>>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    // Update the height of a node
    fn update_height(node: &mut Box<Node<T>>) {
        node.height = 1 + max(Self::height(&node.left), Self::height(&node.right));
    }

    // Get the balance factor of a node
    fn balance_factor(node: &Box<Node<T>>) -> i32 {
        Self::height(&node.left) - Self::height(&node.right)
    }

    // Right rotation
    fn rotate_right(mut y: Box<Node<T>>) -> Box<Node<T>> {
        let mut x = y.left.take().unwrap();
        let t2 = x.right.take();
        y.left = t2;
        x.right = Some(y);
        
        Self::update_height(&mut x.right.as_mut().unwrap());
        Self::update_height(&mut x);
        x
    }

    // Left rotation
    fn rotate_left(mut x: Box<Node<T>>) -> Box<Node<T>> {
        let mut y = x.right.take().unwrap();
        let t2 = y.left.take();
        x.right = t2;
        y.left = Some(x);
        
        Self::update_height(&mut y.left.as_mut().unwrap());
        Self::update_height(&mut y);
        y
    }

    // Insert a value into the AVL tree
    pub fn insert(&mut self, value: T) {
        self.root = Self::insert_rec(self.root.take(), value);
    }

    fn insert_rec(node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>> {
        let mut node = match node {
            None => return Some(Box::new(Node::new(value))),
            Some(mut node) => {
                if value < node.value {
                    node.left = Self::insert_rec(node.left.take(), value);
                } else if value > node.value {
                    node.right = Self::insert_rec(node.right.take(), value);
                } else {
                    return Some(node); // Value already exists
                }
                node
            }
        };

        Self::update_height(&mut node);

        let balance = Self::balance_factor(&node);

        // Left Left Case
        if balance > 1 && value < node.left.as_ref().unwrap().value {
            return Some(Self::rotate_right(node));
        }

        // Right Right Case
        if balance < -1 && value > node.right.as_ref().unwrap().value {
            return Some(Self::rotate_left(node));
        }

        // Left Right Case
        if balance > 1 && value > node.left.as_ref().unwrap().value {
            node.left = Some(Self::rotate_left(node.left.take().unwrap()));
            return Some(Self::rotate_right(node));
        }

        // Right Left Case
        if balance < -1 && value < node.right.as_ref().unwrap().value {
            node.right = Some(Self::rotate_right(node.right.take().unwrap()));
            return Some(Self::rotate_left(node));
        }

        Some(node)
    }

    // Helper function to find the node with minimum value
    fn min_value_node(node: &Box<Node<T>>) -> &T {
        let mut current = node;
        while let Some(ref left) = current.left {
            current = left;
        }
        &current.value
    }

    // Delete a value from the AVL tree
    pub fn delete(&mut self, value: &T) {
        self.root = Self::delete_rec(self.root.take(), value);
    }

    fn delete_rec(node: Option<Box<Node<T>>>, value: &T) -> Option<Box<Node<T>>> {
        let mut node = match node {
            None => return None,
            Some(mut node) => {
                if value < &node.value {
                    node.left = Self::delete_rec(node.left.take(), value);
                } else if value > &node.value {
                    node.right = Self::delete_rec(node.right.take(), value);
                } else {
                    // Node with only one child or no child
                    if node.left.is_none() {
                        return node.right;
                    } else if node.right.is_none() {
                        return node.left;
                    }

                    // Node with two children
                    let min_value = Self::min_value_node(node.right.as_ref().unwrap()).clone();
                    node.value = min_value;
                    node.right = Self::delete_rec(node.right.take(), &node.value);
                }
                node
            }
        };

        Self::update_height(&mut node);

        let balance = Self::balance_factor(&node);

        // Left Left Case
        if balance > 1 && Self::balance_factor(node.left.as_ref().unwrap()) >= 0 {
            return Some(Self::rotate_right(node));
        }

        // Left Right Case
        if balance > 1 && Self::balance_factor(node.left.as_ref().unwrap()) < 0 {
            node.left = Some(Self::rotate_left(node.left.take().unwrap()));
            return Some(Self::rotate_right(node));
        }

        // Right Right Case
        if balance < -1 && Self::balance_factor(node.right.as_ref().unwrap()) <= 0 {
            return Some(Self::rotate_left(node));
        }

        // Right Left Case
        if balance < -1 && Self::balance_factor(node.right.as_ref().unwrap()) > 0 {
            node.right = Some(Self::rotate_right(node.right.take().unwrap()));
            return Some(Self::rotate_left(node));
        }

        Some(node)
    }

    // In-order traversal of the AVL tree
    pub fn inorder(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::inorder_rec(&self.root, &mut result);
        result
    }

    fn inorder_rec(node: &Option<Box<Node<T>>>, result: &mut Vec<&T>) {
        if let Some(node) = node {
            Self::inorder_rec(&node.left, result);
            result.push(&node.value);
            Self::inorder_rec(&node.right, result);
        }
    }
}

fn main() {
    let mut avl_tree = AVLTree::new();

    // Insert some values
    for value in &[3, 2, 1, 4, 5, 6, 7, 16, 15, 14, 13, 12, 11, 10, 8, 9] {
        avl_tree.insert(*value);
    }

    println!("Inorder traversal after insertions:");
    println!("{:?}", avl_tree.inorder());

    // Delete some values
    for value in &[3, 7, 11, 15] {
        avl_tree.delete(value);
    }

    println!("Inorder traversal after deletions:");
    println!("{:?}", avl_tree.inorder());
}
