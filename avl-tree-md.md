# AVL Tree

An AVL tree is a self-balancing binary search tree where the height of the left and right subtrees of any node differ by at most one.

## Implementation

```rust
use std::cmp::max;

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

pub struct AVLTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    fn height(node: &Option<Box<Node<T>>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn update_height(node: &mut Box<Node<T>>) {
        node.height = 1 + max(Self::height(&node.left), Self::height(&node.right));
    }

    fn balance_factor(node: &Box<Node<T>>) -> i32 {
        Self::height(&node.left) - Self::height(&node.right)
    }

    fn rotate_right(mut y: Box<Node<T>>) -> Box<Node<T>> {
        let mut x = y.left.take().unwrap();
        let t2 = x.right.take();
        y.left = t2;
        x.right = Some(y);
        
        Self::update_height(&mut x.right.as_mut().unwrap());
        Self::update_height(&mut x);
        x
    }

    fn rotate_left(mut x: Box<Node<T>>) -> Box<Node<T>> {
        let mut y = x.right.take().unwrap();
        let t2 = y.left.take();
        x.right = t2;
        y.left = Some(x);
        
        Self::update_height(&mut y.left.as_mut().unwrap());
        Self::update_height(&mut y);
        y
    }

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
                    return Some(node);
                }
                node
            }
        };

        Self::update_height(&mut node);

        let balance = Self::balance_factor(&node);

        if balance > 1 && value < node.left.as_ref().unwrap().value {
            return Some(Self::rotate_right(node));
        }

        if balance < -1 && value > node.right.as_ref().unwrap().value {
            return Some(Self::rotate_left(node));
        }

        if balance > 1 && value > node.left.as_ref().unwrap().value {
            node.left = Some(Self::rotate_left(node.left.take().unwrap()));
            return Some(Self::rotate_right(node));
        }

        if balance < -1 && value < node.right.as_ref().unwrap().value {
            node.right = Some(Self::rotate_right(node.right.take().unwrap()));
            return Some(Self::rotate_left(node));
        }

        Some(node)
    }

    // Additional methods (delete, search, etc.) would be implemented here
}
```

## Key Concepts

1. **Self-Balancing**: The tree maintains balance after insertions and deletions, ensuring O(log n) time complexity for these operations.
2. **Balance Factor**: Calculated as the difference between the heights of the left and right subtrees.
3. **Rotations**: Implements both left and right rotations to rebalance the tree.

## When to Use

Use an AVL tree when:

- You need guaranteed O(log n) time complexity for insertions, deletions, and searches.
- The data structure will experience frequent lookups.
- You can afford slightly slower insertions and deletions compared to a Red-Black tree.
- You need a strictly balanced tree structure.

AVL trees are particularly useful in scenarios where lookup speed is critical and the tree won't experience too many modifications, such as in-memory databases or certain types of caches.
