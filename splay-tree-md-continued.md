```rust
                            = root.right;
                            root = tmp.unwrap();
                        } else {
                            *right_min = Some(root);
                            right_min = &mut right_min.as_mut().unwrap().left;
                            root = left_child;
                            break;
                        }
                    } else {
                        *right_min = Some(root);
                        break;
                    }
                }
                Ordering::Greater => {
                    if let Some(mut right_child) = root.right.take() {
                        if key < &right_child.key {
                            let tmp = right_child.left.take();
                            right_child.left = Some(Box::new(Node::new(root.key)));
                            right_child.left.as_mut().unwrap().left = root.left;
                            root = tmp.unwrap();
                        } else if key > &right_child.key {
                            let tmp = right_child.right.take();
                            right_child.right = Some(Box::new(Node::new(root.key)));
                            right_child.right.as_mut().unwrap().left = root.left;
                            root = tmp.unwrap();
                            right_child.right.as_mut().unwrap().right = Some(right_child);
                        } else {
                            *left_max = Some(root);
                            left_max = &mut left_max.as_mut().unwrap().right;
                            root = right_child;
                            break;
                        }
                    } else {
                        *left_max = Some(root);
                        break;
                    }
                }
            }
        }
        *left_max = root.left.take();
        *right_min = root.right.take();
        root.left = left;
        root.right = right;
        self.root = Some(root);
    }

    fn remove(&mut self, key: &T) -> Option<T> {
        if self.root.is_none() {
            return None;
        }
        self.splay(key);
        if self.root.as_ref().unwrap().key != *key {
            return None;
        }
        let mut root = self.root.take().unwrap();
        let result = Some(root.key);
        if root.left.is_none() {
            self.root = root.right;
        } else {
            let mut new_root = root.left.take().unwrap();
            self.root = Some(new_root);
            self.splay(key);
            self.root.as_mut().unwrap().right = root.right;
        }
        result
    }
}

// Usage
fn main() {
    let mut tree = SplayTree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);

    println!("Contains 7: {}", tree.contains(&7));
    println!("Contains 4: {}", tree.contains(&4));

    tree.remove(&5);
    println!("Contains 5 after removal: {}", tree.contains(&5));
}
```

## Key Concepts

1. **Self-adjusting**: Recently accessed elements are moved to the root, making subsequent accesses faster.
2. **Splaying**: The core operation that rearranges the tree to bring a specific element to the root.
3. **Amortized Time Complexity**: While individual operations can take O(n) time in the worst case, the amortized time for a sequence of operations is O(log n) per operation.
4. **No Balance Factor**: Unlike AVL or Red-Black trees, Splay trees don't explicitly maintain balance information.

## When to Use

Use a Splay Tree when:

- You have a scenario where recently accessed items are likely to be accessed again soon.
- You need a self-adjusting data structure that can adapt to access patterns.
- You want good amortized performance without the overhead of maintaining explicit balance information.

Splay Trees are particularly useful in:

- Implementing caches, where recently used items should be quickly accessible.
- Implementing undo operations in text editors.
- Implementing network protocols that exhibit temporal locality.
- Solving problems where access patterns have high temporal locality.

The main advantages of Splay Trees are:
- They adapt to access patterns automatically.
- They have good amortized performance for a sequence of operations.
- They are simpler to implement than some other self-balancing trees.

However, they may not be suitable when:
- You need guaranteed worst-case performance for individual operations.
- The data doesn't exhibit temporal locality in access patterns.
