# Graph Representation and Basic Algorithms

Graphs are used to represent networks of connections between entities. They consist of vertices (or nodes) and edges that connect these vertices.

## Implementation

```rust
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Graph {
    adjacency_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: usize) {
        self.adjacency_list.entry(vertex).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.adjacency_list.entry(from).or_insert(Vec::new()).push(to);
        self.adjacency_list.entry(to).or_insert(Vec::new()); // For undirected graph
    }

    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(vertex) = queue.pop_front() {
            result.push(vertex);

            if let Some(neighbors) = self.adjacency_list.get(&vertex) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        result
    }

    pub fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs_recursive(start, &mut visited, &mut result);
        result
    }

    fn dfs_recursive(&self, vertex: usize, visited: &mut HashSet<usize>, result: &mut Vec<usize>) {
        visited.insert(vertex);
        result.push(vertex);

        if let Some(neighbors) = self.adjacency_list.get(&vertex) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    self.dfs_recursive(neighbor, visited, result);
                }
            }
        }
    }
}
```

## Key Concepts

1. **Adjacency List**: A common way to represent graphs, where each vertex stores a list of its adjacent vertices.
2. **Breadth-First Search (BFS)**: Explores all the neighbor vertices at the present depth before moving to vertices at the next depth level.
3. **Depth-First Search (DFS)**: Explores as far as possible along each branch before backtracking.
4. **Visited Set**: Keeps track of visited vertices to avoid cycles and repeated visits.

## When to Use

Use Graphs and these algorithms when:

- Representing and analyzing networks (social networks, computer networks, etc.).
- Solving problems related to paths, connectivity, or flow in networks.
- Implementing web crawlers or social network analysis tools.
- Solving puzzles or games that involve state transitions.

Graph algorithms are particularly useful in:

- Route planning and navigation systems
- Social network analysis
- Dependency resolution in software packages
- Network flow problems in transportation or computer networks

The choice between BFS and DFS depends on the specific problem:
- Use BFS when you need to find the shortest path in an unweighted graph or explore nodes level by level.
- Use DFS when you need to explore as far as possible along each branch, which is useful in tasks like topological sorting or cycle detection.
