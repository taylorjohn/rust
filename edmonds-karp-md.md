# Edmonds-Karp Algorithm for Maximum Flow

The Edmonds-Karp algorithm is an implementation of the Ford-Fulkerson method for computing the maximum flow in a flow network. It uses breadth-first search to find augmenting paths.

## Implementation

```rust
use std::collections::{HashMap, VecDeque};

type Graph = HashMap<usize, HashMap<usize, i32>>;

fn edmonds_karp(graph: &mut Graph, source: usize, sink: usize) -> i32 {
    let mut flow = 0;
    loop {
        let (path, min_flow) = bfs(graph, source, sink);
        if path.is_empty() {
            break;
        }
        flow += min_flow;
        let mut v = sink;
        for &u in path.iter().rev() {
            *graph.get_mut(&u).unwrap().entry(v).or_insert(0) -= min_flow;
            *graph.get_mut(&v).unwrap().entry(u).or_insert(0) += min_flow;
            v = u;
        }
    }
    flow
}

fn bfs(graph: &Graph, source: usize, sink: usize) -> (Vec<usize>, i32) {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back(source);
    visited.insert(source, (None, i32::MAX));

    while let Some(u) = queue.pop_front() {
        if u == sink {
            break;
        }
        if let Some(edges) = graph.get(&u) {
            for (&v, &capacity) in edges {
                if capacity > 0 && !visited.contains_key(&v) {
                    visited.insert(v, (Some(u), capacity.min(visited[&u].1)));
                    queue.push_back(v);
                }
            }
        }
    }

    if !visited.contains_key(&sink) {
        return (Vec::new(), 0);
    }

    let mut path = Vec::new();
    let mut v = sink;
    let min_flow = visited[&sink].1;
    while let Some(u) = visited[&v].0 {
        path.push(u);
        v = u;
    }
    path.reverse();
    (path, min_flow)
}

// Usage
fn main() {
    let mut graph = Graph::new();
    graph.insert(0, [(1, 10), (2, 10)].iter().cloned().collect());
    graph.insert(1, [(2, 2), (3, 4), (4, 8)].iter().cloned().collect());
    graph.insert(2, [(4, 9)].iter().cloned().collect());
    graph.insert(3, [(5, 10)].iter().cloned().collect());
    graph.insert(4, [(3, 6), (5, 10)].iter().cloned().collect());
    graph.insert(5, [].iter().cloned().collect());

    let max_flow = edmonds_karp(&mut graph, 0, 5);
    println!("Maximum flow: {}", max_flow);
}
```

## Key Concepts

1. **Augmenting Path**: A path from source to sink in the residual graph.
2. **Breadth-First Search**: Used to find the shortest augmenting path.
3. **Residual Graph**: Represents the remaining capacity of the network.
4. **Flow Conservation**: The total flow into a node equals the total flow out of it, except for source and sink.

## When to Use

Use the Edmonds-Karp Algorithm when:

- You need to find the maximum flow in a flow network.
- You're working on problems that can be modeled as network flow problems.
- You need a polynomial-time algorithm for the maximum flow problem.

Edmonds-Karp is particularly useful in:

- Transportation and logistics for optimizing routes and capacities.
- Network design and analysis.
- Bipartite matching problems.
- Image segmentation in computer vision.

The main advantage of Edmonds-Karp over the basic Ford-Fulkerson method is its polynomial time complexity of O(VE^2), where V is the number of vertices and E is the number of edges in the graph.
