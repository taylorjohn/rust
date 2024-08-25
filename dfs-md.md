# Depth-First Search (DFS)

Depth-First Search is a graph traversal algorithm that explores as far as possible along each branch before backtracking.

## Implementation

```rust
use std::collections::{HashMap, HashSet};

type Graph = HashMap<i32, Vec<i32>>;

fn dfs(graph: &Graph, start: i32, visited: &mut HashSet<i32>) {
    if !visited.insert(start) {
        return;
    }
    println!("Visited: {}", start);
    if let Some(neighbors) = graph.get(&start) {
        for &neighbor in neighbors {
            dfs(graph, neighbor, visited);
        }
    }
}

// Usage
fn main() {
    let mut graph = Graph::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![0, 3]);
    graph.insert(3, vec![3]);

    let mut visited = HashSet::new();
    dfs(&graph, 2, &mut visited);
}
```

## When to Use

Use Depth-First Search when:

- You need to traverse or search through all the nodes in a graph or tree.
- You want to find a path between two nodes in a graph.
- Solving puzzles with only one solution (e.g., maze problems).
- Detecting cycles in a graph.
- Topological sorting in a directed acyclic graph (DAG).

DFS is particularly useful in:

- Solving maze problems or path-finding in game development.
- Detecting cycles in graphs (e.g., deadlock detection in operating systems).
- Topological sorting for dependency resolution (e.g., build systems, task scheduling).
- Analyzing and processing tree-like structures (e.g., file systems, XML parsing).
- Implementing backtracking algorithms for constraint satisfaction problems.

## Time Complexity

- Time complexity: O(V + E), where V is the number of vertices and E is the number of edges in the graph.
- Space complexity: O(V) in the worst case for the recursion stack.

DFS can be implemented iteratively using a stack, which might be preferable in some cases to avoid stack overflow for very deep graphs.

## Variations

1. Pre-order, In-order, and Post-order traversals for binary trees.
2. Iterative DFS using a stack instead of recursion.
3. DFS with additional processing (e.g., counting components, finding articulation points).

These variations can be implemented with slight modifications to the basic DFS algorithm.
