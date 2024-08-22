```rust
        parent.children.insert(index + 1, new_child);
    }

    pub fn search(&self, key: &T) -> bool {
        Self::search_node(&self.root, key)
    }

    fn search_node(node: &Node<T>, key: &T) -> bool {
        let mut i = 0;
        while i < node.keys.len() && key > &node.keys[i] {
            i += 1;
        }
        if i < node.keys.len() && key == &node.keys[i] {
            true
        } else if node.leaf {
            false
        } else {
            Self::search_node(&node.children[i], key)
        }
    }

    pub fn traverse(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::traverse_node(&self.root, &mut result);
        result
    }

    fn traverse_node<'a>(node: &'a Node<T>, result: &mut Vec<&'a T>) {
        let mut i = 0;
        while i < node.keys.len() {
            if !node.leaf {
                Self::traverse_node(&node.children[i], result);
            }
            result.push(&node.keys[i]);
            i += 1;
        }
        if !node.leaf {
            Self::traverse_node(&node.children[i], result);
        }
    }
}
```

## Key Concepts

1. **Multi-key Nodes**: Each node can contain multiple keys, up to a maximum defined by the B-tree order.
2. **Self-Balancing**: The tree maintains balance by splitting nodes when they become too full during insertion.
3. **Ordered Keys**: Keys within a node and across the tree are kept in order.
4. **Depth Property**: All leaf nodes are at the same depth.

## When to Use

Use a B-Tree when:

- Working with large datasets that don't fit entirely in memory.
- Implementing systems that read and write large blocks of data, such as databases and file systems.
- You need to maintain sorted data with efficient insertion, deletion, and search operations.
- Dealing with data that requires frequent range queries.

B-Trees are particularly well-suited for storage systems where the tree resides on disk and minimizing disk I/O is crucial. They're commonly used in:

- Database indexing (e.g., in many relational database management systems)
- File systems (e.g., NTFS, HFS+, ext4)
- Key-value stores

The large branching factor of B-Trees makes them excellent for reducing the number of disk accesses needed for operations, which is crucial for maintaining good performance when working with disk-based storage.
