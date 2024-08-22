# Heavy-Light Decomposition

Heavy-Light Decomposition (HLD) is a technique used to decompose a tree into a set of paths, which allows for efficient operations on paths in the tree. It's particularly useful for solving problems on trees that involve path queries or updates.

## Implementation

```rust
use std::collections::HashMap;

struct HeavyLightDecomposition {
    parent: Vec<usize>,
    depth: Vec<usize>,
    size: Vec<usize>,
    heavy: Vec<Option<usize>>,
    head: Vec<usize>,
    pos: Vec<usize>,
    values: Vec<i32>,
    tree: Vec<i32>,
}

impl HeavyLightDecomposition {
    fn new(n: usize, values: Vec<i32>, edges: &[(usize, usize)]) -> Self {
        let mut adj = vec![Vec::new(); n];
        for &(u, v) in edges {
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut hld = HeavyLightDecomposition {
            parent: vec![0; n],
            depth: vec![0; n],
            size: vec![0; n],
            heavy: vec![None; n],
            head: (0..n).collect(),
            pos: vec![0; n],
            values,
            tree: vec![0; 4 * n],
        };

        hld.dfs(0, 0, 0, &adj);
        hld.decompose(0, 0, &adj);
        hld.build(1, 0, n - 1);
        hld
    }

    fn dfs(&mut self, v: usize, p: usize, d: usize, adj: &[Vec<usize>]) {
        self.parent[v] = p;
        self.depth[v] = d;
        self.size[v] = 1;
        for &u in &adj[v] {
            if u != p {
                self.dfs(u, v, d + 1, adj);
                self.size[v] += self.size[u];
                if self.heavy[v].is_none() || self.size[u] > self.size[self.heavy[v].unwrap()] {
                    self.heavy[v] = Some(u);
                }
            }
        }
    }

    fn decompose(&mut self, v: usize, h: usize, adj: &[Vec<usize>]) {
        self.head[v] = h;
        self.pos[v] = self.pos.iter().filter(|&&x| x != 0).count();
        if let Some(u) = self.heavy[v] {
            self.decompose(u, h, adj);
        }
        for &u in &adj[v] {
            if u != self.parent[v] && Some(u) != self.heavy[v] {
                self.decompose(u, u, adj);
            }
        }
    }

    fn build(&mut self, v: usize, tl: usize, tr: usize) {
        if tl == tr {
            self.tree[v] = self.values[self.pos.iter().position(|&x| x == tl).unwrap()];
            return;
        }
        let tm = (tl + tr) / 2;
        self.build(v * 2, tl, tm);
        self.build(v * 2 + 1, tm + 1, tr);
        self.tree[v] = self.tree[v * 2] + self.tree[v * 2 + 1];
    }

    fn update(&mut self, v: usize, tl: usize, tr: usize, pos: usize, new_val: i32) {
        if tl == tr {
            self.tree[v] = new_val;
            return;
        }
        let tm = (tl + tr) / 2;
        if pos <= tm {
            self.update(v * 2, tl, tm, pos, new_val);
        } else {
            self.update(v * 2 + 1, tm + 1, tr, pos, new_val);
        }
        self.tree[v] = self.tree[v * 2] + self.tree[v * 2 + 1];
    }

    fn query(&self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> i32 {
        if l > r {
            return 0;
        }
        if l == tl && r == tr {
            return self.tree[v];
        }
        let tm = (tl + tr) / 2;
        self.query(v * 2, tl, tm, l, r.min(tm)) +
        self.query(v * 2 + 1, tm + 1, tr, l.max(tm + 1), r)
    }

    fn path_query(&self, mut a: usize, mut b: usize) -> i32 {
        let mut res = 0;
        while self.head[a] != self.head[b] {
            if self.depth[self.head[a]] < self.depth[self.head[b]] {
                std::mem::swap(&mut a, &mut b);
            }
            let head = self.head[a];
            res += self.query(1, 0, self.pos.len() - 1, self.pos[head], self.pos[a]);
            a = self.parent[head];
        }
        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        res + self.query(1, 0, self.pos.len() - 1, self.pos[a], self.pos[b])
    }

    fn update_node(&mut self, node: usize, new_val: i32) {
        self.update(1, 0, self.pos.len() - 1, self.pos[node], new_val);
    }
}

// Usage
fn main() {
    let n = 5;
    let values = vec![1, 2, 3, 4, 5];
    let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4)];

    let mut hld = HeavyLightDecomposition::new(n, values, &edges);

    println!("Sum of path from 3 to 4: {}", hld.path_query(3, 4));

    hld.update_node(2, 10);
    println!("Sum of path from 0 to 2 after update: {}", hld.path_query(0, 2));
}
```

## Key Concepts

1. **Heavy and Light Edges**: Each node's heaviest child (the one with the largest subtree) is connected by a heavy edge, all others by light edges.
2. **Chain Decomposition**: The tree is decomposed into chains of heavy edges.
3. **Path Representation**: Any root-to-leaf path intersects at most log(n) light edges, allowing efficient path operations.
4. **Segment Tree Integration**: Often combined with a segment tree for range queries and updates on paths.

## When to Use

Use Heavy-Light Decomposition when:

- You need to perform path queries or updates on a tree efficiently.
- Working with problems that involve finding LCA (Lowest Common Ancestor) or distance between nodes in a tree.
- Solving problems that require maintaining information along paths in a tree structure.

Heavy-Light Decomposition is particularly useful in:

- Competitive programming for solving tree-based problems efficiently.
- Graph algorithms that work on tree structures.
- Network analysis where the network has a tree-like structure.
- Game development for hierarchical game world representations.

The main advantage of Heavy-Light Decomposition is its ability to reduce path operations on trees to a logarithmic number of contiguous range operations, which can then be handled efficiently using data structures like segment trees or fenwick trees.

This implementation combines HLD with a segment tree to allow for efficient path sum queries and point updates. The time complexity for both path queries and updates is O(log^2 n) in the worst case.
