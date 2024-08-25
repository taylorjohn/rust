# Hierholzer's Algorithm for Euler Paths and Circuits

Hierholzer's Algorithm is used to find Euler paths or circuits in a graph. An Euler path visits every edge exactly once, while an Euler circuit is an Euler path that starts and ends at the same vertex.

## Implementation

```rust
use std::collections::{HashMap, HashSet};

fn hierholzer(graph: &HashMap<usize, Vec<usize>>) -> Option<Vec<usize>> {
    let mut degree = HashMap::new();
    for (&u, neighbors) in graph {
        *degree.entry(u).or_insert(0) += neighbors.len();
        for &v in neighbors {
            *degree.entry(v).or_insert(0) += 1;
        }
    }

    let start = *graph.keys().next()?;
    let mut odd_degree = degree.values().filter(|&&d| d % 2 != 0).count();

    // Check if Euler path/circuit exists
    if odd_degree != 0 && odd_degree != 2 {
        return None;
    }

    if odd_degree == 2 {
        // Find a vertex with odd degree to start
        for (&v, &d) in &degree {
            if d % 2 != 0 {
                start = v;
                break;
            }
        }
    }

    let mut stack = vec![start];
    let mut path = Vec::new();
    let mut current_path = Vec::new();
    let mut graph = graph.clone();

    while let Some(v) = stack.pop() {
        while let Some(u) = graph.get_mut(&v).and_then(|neighbors| neighbors.pop()) {
            stack.push(v);
            stack.push(u);
            graph.get_mut(&u).unwrap().retain(|&x| x != v);
        }
        current_path.push(v);
    }

    // Reconstruct the path
    while let Some(v) = current_path.pop() {
        path.push(v);
    }

    // Check if all edges were used
    if path.len() == degree.values().sum::<usize>() / 2 + 1 {
        Some(path)
    } else {
        None
    }
}

fn main() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![0, 2]);
    graph.insert(2, vec![0, 1, 3]);
    graph.insert(3, vec![2]);

    match hierholzer(&graph) {
        Some(path) => println!("Euler path/circuit: {:?}", path),
        None => println!("No Euler path/circuit exists"),
    }
}
```

## Explanation

1. We first check if an Euler path or circuit exists by counting vertices with odd degree.
2. We start from a vertex with odd degree (for Euler path) or any vertex (for Euler circuit).
3. We use a stack to keep track of the current path and backtrack when necessary.
4. We remove edges as we traverse them to avoid using them again.
5. Finally, we reconstruct the path and check if all edges were used.

## Time Complexity

Hierholzer's Algorithm has a time complexity of O(|E|), where |E| is the number of edges in the graph.

## Use Cases

- Solving maze problems
- Optimizing routes in logistics
- DNA fragment assembly in bioinformatics

Hierholzer's Algorithm is particularly useful when you need to find a path or circuit that uses every edge in a graph exactly once, which has applications in various fields including network design and genetic sequencing.
