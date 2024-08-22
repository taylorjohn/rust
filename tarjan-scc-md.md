# Tarjan's Algorithm for Strongly Connected Components

Tarjan's algorithm is a graph theory algorithm for finding the strongly connected components of a directed graph in linear time.

## Implementation

```rust
use std::collections::HashMap;

struct TarjanSCC {
    graph: HashMap<usize, Vec<usize>>,
    index: usize,
    stack: Vec<usize>,
    on_stack: Vec<bool>,
    indexes: Vec<Option<usize>>,
    lowlinks: Vec<usize>,
    sccs: Vec<Vec<usize>>,
}

impl TarjanSCC {
    fn new(graph: HashMap<usize, Vec<usize>>) -> Self {
        let n = graph.keys().max().map_or(0, |&k| k + 1);
        TarjanSCC {
            graph,
            index: 0,
            stack: Vec::new(),
            on_stack: vec![false; n],
            indexes: vec![None; n],
            lowlinks: vec![0; n],
            sccs: Vec::new(),
        }
    }

    fn run(&mut self) -> Vec<Vec<usize>> {
        for v in 0..self.indexes.len() {
            if self.indexes[v].is_none() {
                self.strongconnect(v);
            }
        }
        self.sccs.clone()
    }

    fn strongconnect(&mut self, v: usize) {
        self.indexes[v] = Some(self.index);
        self.lowlinks[v] = self.index;
        self.index += 1;
        self.stack.push(v);
        self.on_stack[v] = true;

        if let Some(neighbors) = self.graph.get(&v) {
            for &w in neighbors {
                if self.indexes[w].is_none() {
                    self.strongconnect(w);
                    self.lowlinks[v] = self.lowlinks[v].min(self.lowlinks[w]);
                } else if self.on_stack[w] {
                    self.lowlinks[v] = self.lowlinks[v].min(self.indexes[w].unwrap());
                }
            }
        }

        if self.lowlinks[v] == self.indexes[v].unwrap() {
            let mut scc = Vec::new();
            loop {
                let w = self.stack.pop().unwrap();
                self.on_stack[w] = false;
                scc.push(w);
                if w == v {
                    break;
                }
            }
            self.sccs.push(scc);
        }
    }
}

// Usage
fn main() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![3]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![4, 5]);
    graph.insert(4, vec![2, 5]);
    graph.insert(5, vec![4]);

    let mut tarjan = TarjanSCC::new(graph);
    let sccs = tarjan.run();

    println!("Strongly Connected Components:");
    for (i, scc) in sccs.iter().enumerate() {
        println!("Component {}: {:?}", i + 1, scc);
    }
}
```

## Key Concepts

1. **Depth-First Search (DFS)**: The algorithm uses DFS as its basis.
2. **Lowlink Values**: Each vertex is assigned a lowlink value, which is the smallest index of any vertex known to be reachable from v through v's DFS subtree, including v itself.
3. **Stack**: Maintains a stack of vertices that have been visited but not yet assigned to a strongly connected component.
4. **Single Pass**: Finds all strongly connected components in a single pass over the graph.

## When to Use

Use Tarjan's Algorithm for Strongly Connected Components when:

- You need to find all strongly connected components in a directed graph.
- You're working with problems that involve cycle detection in directed graphs.
- You need to analyze the structure of directed graphs efficiently.

Tarjan's Algorithm is particularly useful in:

- Solving problems in graph theory and network analysis.
- Analyzing dependencies in software systems.
- Detecting cycles in workflows or processes.
- Simplifying complex directed graphs by collapsing strongly connected components.

The main advantage of Tarjan's algorithm is its linear time complexity O(V + E), where V is the number of vertices and E is the number of edges. It's also memory-efficient as it performs the computation in a single pass over the graph.
