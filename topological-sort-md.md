# Topological Sort

Topological sorting for Directed Acyclic Graph (DAG) is a linear ordering of vertices such that for every directed edge uv, vertex u comes before v in the ordering. Topological Sorting is possible if and only if the graph has no directed cycles.

## Implementation

```rust
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Graph {
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { edges: HashMap::new() }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.entry(from).or_insert(Vec::new()).push(to);
        self.edges.entry(to).or_insert(Vec::new()); // Ensure all nodes are in the map
    }

    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let mut in_degree: HashMap<usize, usize> = self.edges.keys().map(|&k| (k, 0)).collect();
        for (_node, neighbors) in &self.edges {
            for &neighbor in neighbors {
                *in_degree.entry(neighbor).or_insert(0) += 1;
            }
        }

        let mut queue: VecDeque<usize> = in_degree.iter()
            .filter(|&(_, &degree)| degree == 0)
            .map(|(&node, _)| node)
            .collect();

        let mut result = Vec::new();

        while let Some(node) = queue.pop_front() {
            result.push(node);

            if let Some(neighbors) = self.edges.get(&node) {
                for &neighbor in neighbors {
                    let degree = in_degree.get_mut(&neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        if result.len() == self.edges.len() {
            Some(result)
        } else {
            None // Graph has a cycle
        }
    }
}

// Usage
fn main() {
    let mut graph = Graph::new();
    graph.add_edge(5, 2);
    graph.add_edge(5, 0);
    graph.add_edge(4, 0);
    graph.add_edge(4, 1);
    graph.add_edge(2, 3);
    graph.add_edge(3, 1);

    match graph.topological_sort() {
        Some(order) => println!("Topological Order: {:?}", order),
        None => println!("Graph contains a cycle"),
    }
}
```

## Key Concepts

1. **Directed Acyclic Graph (DAG)**: The graph must be directed and have no cycles.
2. **In-degree**: The number of incoming edges to a vertex.
3. **Kahn's Algorithm**: Used here, it works by choosing vertices in the same order as the eventual topological sort.
4. **Cycle Detection**: If the algorithm can't produce a complete ordering, it means the graph has a cycle.

## When to Use

Use Topological Sort when:

- You need to find a linear ordering of elements that have dependencies on each other.
- Scheduling tasks with prerequisites.
- Determining the order of compilation tasks.
- Analyzing dependencies in build systems or package managers.

Topological Sort is particularly useful in:

- Build systems (like Make) to determine the order of compilation
- Task scheduling in project management
- Course scheduling in academic planning
- Dependency resolution in software installation

The main advantage of Topological Sort is its ability to provide a valid sequence for processing interdependent items, ensuring that all prerequisites are handled before dependent items.
