# Kosaraju's Algorithm for Strongly Connected Components

Kosaraju's algorithm finds all strongly connected components in a directed graph. A strongly connected component is a portion of a directed graph in which there is a path from each vertex to every other vertex.

## Implementation

```rust
use std::collections::{HashMap, HashSet};

struct Graph {
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Graph { edges: HashMap::new() }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.entry(from).or_insert(Vec::new()).push(to);
        self.edges.entry(to).or_insert(Vec::new());
    }

    fn transpose(&self) -> Self {
        let mut transposed = Graph::new();
        for (&from, tos) in &self.edges {
            for &to in tos {
                transposed.add_edge(to, from);
            }
        }
        transposed
    }
}

fn dfs(graph: &Graph, v: usize, visited: &mut HashSet<usize>, stack: &mut Vec<usize>) {
    if visited.insert(v) {
        if let Some(neighbors) = graph.edges.get(&v) {
            for &neighbor in neighbors {
                dfs(graph, neighbor, visited, stack);
            }
        }
        stack.push(v);
    }
}

fn kosaraju(graph: &Graph) -> Vec<Vec<usize>> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    // First DFS to fill the stack
    for &v in graph.edges.keys() {
        dfs(graph, v, &mut visited, &mut stack);
    }

    let transposed = graph.transpose();
    visited.clear();
    let mut components = Vec::new();

    // Second DFS to find SCCs
    while let Some(v) = stack.pop() {
        if !visited.contains(&v) {
            let mut component = Vec::new();
            dfs(&transposed, v, &mut visited, &mut component);
            components.push(component);
        }
    }

    components
}

fn main() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);
    graph.add_edge(1, 3);
    graph.add_edge(3, 4);

    let sccs = kosaraju(&graph);
    println!("Strongly Connected Components:");
    for (i, scc) in sccs.iter().enumerate() {
        println!("Component {}: {:?}", i + 1, scc);
    }
}
```

## Explanation

1. Kosaraju's algorithm performs two depth-first searches (DFS) of the graph.
2. The first DFS is on the original graph and fills a stack with vertices in order of finishing times.
3. The graph is then transposed (all edges reversed).
4. The second DFS is performed on the transposed graph, using the order of vertices in the stack.
5. Each tree in the second DFS forest is a strongly connected component.

## Time Complexity

O(V + E), where V is the number of vertices and E is the number of edges.

## Space Complexity

O(V)

## Use Cases

- Analyzing dependencies in software systems
- Finding cycles in directed graphs
- Social network analysis
- Solving 2-SAT problem

Kosaraju's Algorithm is particularly useful in understanding the structure of directed graphs and finding sets of mutually reachable vertices, which has applications in various fields of computer science and network analysis.
