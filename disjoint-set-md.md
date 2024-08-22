# Disjoint Set (Union-Find)

A Disjoint Set data structure, also known as Union-Find, is used to efficiently keep track of a partition of a set into disjoint subsets. It provides near-constant-time operations to add new sets, merge existing sets, and determine whether elements are in the same set.

## Implementation

```rust
pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        DisjointSet {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    pub fn find(&mut self, item: usize) -> usize {
        if self.parent[item] != item {
            self.parent[item] = self.find(self.parent[item]);
        }
        self.parent[item]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        match self.rank[x_root].cmp(&self.rank[y_root]) {
            std::cmp::Ordering::Less => self.parent[x_root] = y_root,
            std::cmp::Ordering::Greater => self.parent[y_root] = x_root,
            std::cmp::Ordering::Equal => {
                self.parent[y_root] = x_root;
                self.rank[x_root] += 1;
            }
        }
    }

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

// Usage
fn main() {
    let mut ds = DisjointSet::new(5);
    ds.union(0, 2);
    ds.union(4, 2);
    ds.union(3, 1);

    println!("0 and 4 connected: {}", ds.connected(0, 4)); // true
    println!("1 and 4 connected: {}", ds.connected(1, 4)); // false
}
```

## Key Concepts

1. **Disjoint Sets**: Maintains a collection of disjoint sets of elements.
2. **Union by Rank**: A optimization technique that always attaches the shorter tree to the root of the taller tree.
3. **Path Compression**: An optimization technique that flattens the structure of the tree by making every node point to the root when find is used on it.
4. **Amortized Time Complexity**: With both optimizations, operations take nearly constant amortized time.

## When to Use

Use Disjoint Set (Union-Find) when:

- You need to maintain a collection of disjoint sets and perform unions and finds efficiently.
- Implementing Kruskal's algorithm for finding the Minimum Spanning Tree of a graph.
- Detecting cycles in an undirected graph.
- Finding connected components in a graph.
- Implementing an online algorithm for checking connectivity.

Disjoint Sets are particularly useful in:

- Network connectivity problems
- Image processing for finding connected components
- Percolation theory in physics
- Least Common Ancestor in trees

The main advantage of Disjoint Set is its ability to perform union and find operations in nearly constant amortized time, which makes it very efficient for problems involving set partitioning and merging.
