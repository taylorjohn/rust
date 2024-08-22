# Lowest Common Ancestor (LCA) using Binary Lifting

The Lowest Common Ancestor (LCA) of two nodes in a tree is the deepest node that is an ancestor of both nodes. Binary Lifting is an efficient technique to compute LCA in O(log N) time after O(N log N) preprocessing.

## Implementation

```rust
use std::collections::HashMap;

struct LCA {
    up: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
    log: usize,
}

impl LCA {
    fn new(graph: &HashMap<usize, Vec<usize>>, root: usize) -> Self {
        let n = graph.len();
        let log = (n as f64).log2().ceil() as usize;

        let mut up = vec![vec![None; log]; n];
        let mut depth = vec![0; n];

        LCA::dfs(graph, root, root, &mut up, &mut depth);

        for j in 1..log {
            for i in 0..n {
                up[i][j] = up[i][j-1].and_then(|p| up[p][j-1]);
            }
        }

        LCA { up, depth, log }
    }

    fn dfs(graph: &HashMap<usize, Vec<usize>>, v: usize, p: usize, 
           up: &mut Vec<Vec<Option<usize>>>, depth: &mut Vec<usize>) {
        up[v][0] = if v != p { Some(p) } else { None };

        if let Some(children) = graph.get(&v) {
            for &u in children {
                if u != p {
                    depth[u] = depth[v] + 1;
                    LCA::dfs(graph, u, v, up, depth);
                }
            }
        }
    }

    fn get_lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] < self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }

        let mut k = self.depth[a] - self.depth[b];
        for j in 0..self.log {
            if k & (1 << j) != 0 {
                a = self.up[a][j].unwrap();
            }
        }

        if a == b {
            return a;
        }

        for j in (0..self.log).rev() {
            if self.up[a][j] != self.up[b][j] {
                a = self.up[a][j].unwrap();
                b = self.up[b][j].unwrap();
            }
        }

        self.up[a][0].unwrap()
    }
}

// Usage
fn main() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![0, 3, 4]);
    graph.insert(2, vec![0, 5]);
    graph.insert(3, vec![1]);
    graph.insert(4, vec![1]);
    graph.insert(5, vec![2]);

    let lca = LCA::new(&graph, 0);

    println!("LCA of 3 and 4: {}", lca.get_lca(3, 4));
    println!("LCA of 2 and 4: {}", lca.get_lca(2, 4));
}
```

## Key Concepts

1. **Binary Lifting**: Precomputes ancestors at power-of-two distances for each node.
2. **Depth-First Search (DFS)**: Used for initial tree traversal and depth calculation.
3. **Sparse Table**: The `up` array acts as a sparse table for efficient ancestor lookup.
4. **Logarithmic Query Time**: LCA queries are answered in O(log N) time.

## When to Use

Use LCA with Binary Lifting when:

- You need to find the lowest common ancestor of two nodes in a tree efficiently.
- You have multiple LCA queries on a fixed tree structure.
- Working with problems involving tree paths or distances between nodes.

LCA is particularly useful in:

- Solving graph and tree problems in competitive programming.
- Analyzing hierarchical structures in data.
- Optimizing queries on tree-like structures in databases.
- Computational biology for analyzing phylogenetic trees.

The main advantage of this LCA implementation is its ability to answer queries in O(log N) time after O(N log N) preprocessing, making it efficient for multiple queries on the same tree.
