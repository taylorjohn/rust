# Skip List

A Skip List is a probabilistic data structure that allows O(log n) search complexity as well as O(log n) insertion complexity within an ordered sequence of elements.

## Implementation

```rust
use rand::Rng;

struct Node {
    value: i32,
    forward: Vec<Option<Box<Node>>>,
}

impl Node {
    fn new(value: i32, level: usize) -> Self {
        Node {
            value,
            forward: vec![None; level + 1],
        }
    }
}

struct SkipList {
    head: Box<Node>,
    level: usize,
    max_level: usize,
}

impl SkipList {
    fn new(max_level: usize) -> Self {
        SkipList {
            head: Box::new(Node::new(i32::MIN, max_level)),
            level: 0,
            max_level,
        }
    }

    fn random_level(&self) -> usize {
        let mut rng = rand::thread_rng();
        let mut lvl = 0;
        while rng.gen::<bool>() && lvl < self.max_level {
            lvl += 1;
        }
        lvl
    }

    fn insert(&mut self, value: i32) {
        let mut update = vec![None; self.max_level + 1];
        let mut current = &mut self.head;

        for i in (0..=self.level).rev() {
            while let Some(ref next) = current.forward[i] {
                if next.value < value {
                    current = current.forward[i].as_mut().unwrap();
                } else {
                    break;
                }
            }
            update[i] = Some(current);
        }

        let new_level = self.random_level();
        if new_level > self.level {
            for i in self.level + 1..=new_level {
                update[i] = Some(&mut self.head);
            }
            self.level = new_level;
        }

        let new_node = Box::new(Node::new(value, new_level));
        for i in 0..=new_level {
            let next = update[i].as_mut().unwrap().forward[i].take();
            new_node.forward[i] = next;
            update[i].as_mut().unwrap().forward[i] = Some(new_node.clone());
        }
    }

    fn search(&self, value: i32) -> bool {
        let mut current = &self.head;
        for i in (0..=self.level).rev() {
            while let Some(ref next) = current.forward[i] {
                if next.value < value {
                    current = next;
                } else {
                    break;
                }
            }
        }
        current.forward[0].as_ref().map_or(false, |node| node.value == value)
    }
}

fn main() {
    let mut skip_list = SkipList::new(4);
    skip_list.insert(3);
    skip_list.insert(6);
    skip_list.insert(7);
    skip_list.insert(9);
    skip_list.insert(12);

    println!("Search for 7: {}", skip_list.search(7));
    println!("Search for 10: {}", skip_list.search(10));
}
```

## Key Concepts

1. **Probabilistic Structure**: Skip Lists use randomization to maintain balance.
2. **Multiple Levels**: Each node can have multiple forward pointers, allowing for faster traversal.
3. **Logarithmic Operations**: Search, insert, and delete operations have an average time complexity of O(log n).
4. **Simplicity**: Skip Lists are simpler to implement than many balanced tree structures.

## When to Use

Use Skip Lists when:

- You need a data structure with fast search, insert, and delete operations.
- You want a simpler alternative to balanced trees.
- You're comfortable with probabilistic balancing.
- You need to implement a sorted list or dictionary.

## Time Complexity

- Average case for search, insert, and delete: O(log n)
- Worst case: O(n) (very unlikely due to probabilistic balancing)

## Space Complexity

O(n) where n is the number of elements

Skip Lists provide a good balance between simplicity and performance, making them a useful alternative to more complex balanced tree structures in many scenarios.
