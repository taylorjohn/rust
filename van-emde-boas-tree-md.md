# Van Emde Boas Tree

A Van Emde Boas Tree (vEB tree) is a tree data structure which implements an associative array with m-bit integer keys. It supports operations such as insert, delete, predecessor, and successor in O(log m) time, or equivalently in O(log log M) time, where M = 2^m is the maximum number of elements that can be stored in the tree.

## Implementation

```rust
use std::collections::HashMap;

struct VEBTree {
    universe_size: usize,
    min: Option<usize>,
    max: Option<usize>,
    summary: Option<Box<VEBTree>>,
    cluster: Option<HashMap<usize, VEBTree>>,
}

impl VEBTree {
    fn new(universe_size: usize) -> Self {
        VEBTree {
            universe_size,
            min: None,
            max: None,
            summary: None,
            cluster: None,
        }
    }

    fn high(&self, x: usize) -> usize {
        x / self.lower_sqrt()
    }

    fn low(&self, x: usize) -> usize {
        x % self.lower_sqrt()
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x * self.lower_sqrt() + y
    }

    fn upper_sqrt(&self) -> usize {
        1 << (self.universe_size.ilog2() / 2 + self.universe_size.ilog2() % 2)
    }

    fn lower_sqrt(&self) -> usize {
        1 << (self.universe_size.ilog2() / 2)
    }

    fn insert(&mut self, x: usize) {
        if self.min.is_none() {
            self.min = Some(x);
            self.max = Some(x);
            return;
        }

        if x < self.min.unwrap() {
            std::mem::swap(&mut self.min, &mut Some(x));
        }

        if self.universe_size > 2 {
            if self.summary.is_none() {
                self.summary = Some(Box::new(VEBTree::new(self.upper_sqrt())));
                self.cluster = Some(HashMap::new());
            }

            let high = self.high(x);
            if !self.cluster.as_ref().unwrap().contains_key(&high) {
                self.summary.as_mut().unwrap().insert(high);
                self.cluster.as_mut().unwrap().insert(high, VEBTree::new(self.lower_sqrt()));
            }
            self.cluster.as_mut().unwrap().get_mut(&high).unwrap().insert(self.low(x));
        }

        if x > self.max.unwrap() {
            self.max = Some(x);
        }
    }

    fn member(&self, x: usize) -> bool {
        if x == self.min.unwrap_or(usize::MAX) || x == self.max.unwrap_or(usize::MAX) {
            return true;
        }
        if self.universe_size <= 2 {
            return false;
        }
        let high = self.high(x);
        self.cluster.as_ref().unwrap().get(&high).map_or(false, |cluster| cluster.member(self.low(x)))
    }

    fn successor(&self, x: usize) -> Option<usize> {
        if self.universe_size <= 2 {
            if x == 0 && self.max == Some(1) {
                return Some(1);
            } else {
                return None;
            }
        }
        if self.min.is_some() && x < self.min.unwrap() {
            return self.min;
        }
        let high = self.high(x);
        let low = self.low(x);
        let max_low = self.cluster.as_ref().and_then(|c| c.get(&high)).and_then(|c| c.max);
        if max_low.is_some() && low < max_low.unwrap() {
            let offset = self.cluster.as_ref().unwrap()[&high].successor(low).unwrap();
            return Some(self.index(high, offset));
        }
        let succ_cluster = self.summary.as_ref().and_then(|s| s.successor(high));
        if let Some(succ_cluster) = succ_cluster {
            let offset = self.cluster.as_ref().unwrap()[&succ_cluster].min.unwrap();
            return Some(self.index(succ_cluster, offset));
        }
        None
    }
}

fn main() {
    let mut veb = VEBTree::new(16);
    veb.insert(2);
    veb.insert(3);
    veb.insert(4);
    veb.insert(5);
    veb.insert(7);
    veb.insert(14);
    veb.insert(15);

    println!("Is 3 a member? {}", veb.member(3));
    println!("Is 6 a member? {}", veb.member(6));
    println!("Successor of 5: {:?}", veb.successor(5));
    println!("Successor of 6: {:?}", veb.successor(6));
}
```

## Key Concepts

1. **Recursive Structure**: The tree recursively divides the universe into clusters.
2. **Fast Operations**: Supports insert, delete, predecessor, successor, minimum, and maximum operations in O(log log M) time.
3. **Space Efficiency**: Uses O(M) space in this implementation, which can be improved to O(n) where n is the number of elements stored.
4. **Integer Keys**: Designed for integer keys in a known universe.

## When to Use

Use a Van Emde Boas Tree when:

- You need to perform fast predecessor/successor operations on integers.
- Working with a set of integers from a known universe.
- You need a data structure that supports both dynamic updates and fast queries.

Van Emde Boas Trees are particularly useful in:

- IP routing tables
- Scheduling algorithms
- Implementing fast priority queues for integers

The main advantage of Van Emde Boas Trees is their ability to perform operations in O(log log M) time, which can be very fast for practical universe sizes.
