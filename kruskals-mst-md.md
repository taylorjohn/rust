# Minimum Spanning Tree (Kruskal's Algorithm)

Kruskal's algorithm finds a minimum spanning tree for a connected weighted graph. This algorithm is a greedy algorithm that finds a subset of the edges that forms a tree that includes every vertex, where the total weight of all the edges in the tree is minimized.

## Implementation

```rust
use std::collections::HashMap;

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        DisjointSet {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, item: usize) -> usize {
        if self.parent[item] != item {
            self.parent[item] = self.find(self.parent[item]);
        }
        self.parent[item]
    }

    fn union(&mut self, x: usize, y: usize) {
        let xroot = self.find(x);
        let yroot = self.find(y);

        if self.rank[xroot] < self.rank[yroot] {
            self.parent[xroot] = yroot;
        } else if self.rank[xroot] > self.rank[yroot] {
            self.parent[yroot] = xroot;
        } else {
            self.parent[yroot] = xroot;
            self.rank[xroot] += 1;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    src: usize,
    dst: usize,
    weight: i32,
}

fn kruskal_mst(graph: &HashMap<usize, Vec<(usize, i32)>>) -> Vec<Edge> {
    let mut edges = Vec::new();
    for (&u, neighbors) in graph {
        for &(v, weight) in neighbors {
            if u < v {  // Avoid duplicate edges
                edges.push(Edge { src: u, dst: v, weight });
            }
        }
    }

    edges.sort_by_key(|e| e.weight);

    let n = graph.len();
    let mut disjoint_set = DisjointSet::new(n);
    let mut mst = Vec::new();

    for edge in edges {
        if disjoint_set.find(edge.src) != disjoint_set.find(edge.dst) {
            disjoint_set.union(edge.src, edge.dst);
            mst.push(edge);
        }
        if mst.len() == n - 1 {
            break;
        }
    }

    mst
}

// Usage
fn main() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![(1, 4), (2, 4)]);
    graph.insert(1, vec![(0, 4), (2, 2)]);
    graph.insert(2, vec![(0, 4), (1, 2), (3, 3), (4, 2)]);
    graph.insert(3, vec![(2, 3), (4, 3)]);
    graph.insert(4, vec![(2, 2), (3, 3)]);

    let mst = kruskal_mst(&graph);

    println!("Minimum Spanning Tree edges:");
    for edge in mst {
        println!("{} -- {} : {}", edge.src, edge.dst, edge.weight);
    }
}
```

## Key Concepts

1. **Greedy Algorithm**: Kruskal's algorithm builds the spanning tree by adding edges one by one in increasing order of their weight.
2. **Disjoint Set**: Used to detect cycles and check if adding an edge will form a cycle.
3. **Edge Sorting**: Edges are sorted by weight to process them in ascending order.
4. **Cycle Prevention**: The algorithm ensures that no cycles are formed while building the MST.

## When to Use

Use Kruskal's Algorithm for Minimum Spanning Tree when:

- You need to find the minimum spanning tree in a weighted, undirected graph.
- The graph is sparse (has relatively few edges compared to the number of vertices).
- You want a simple implementation that's easy to understand and implement.

Kruskal's Algorithm is particularly useful in:

- Network design (e.g., laying out electrical wiring or computer networks).
- Clustering algorithms in data mining.
- Approximation algorithms for NP-hard problems like the traveling salesman problem.
- Image segmentation in computer vision.

The main advantage of Kruskal's algorithm is its simplicity and efficiency for sparse graphs. It has a time complexity of O(E log E) or O(E log V), where E is the number of edges and V is the number of vertices.
