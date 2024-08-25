# Johnson's Algorithm for All Pairs Shortest Paths

Johnson's algorithm finds the shortest paths between all pairs of vertices in a sparse, weighted graph. It works with graphs that may have negative edge weights, but not negative cycles.

## Implementation

```rust
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn bellman_ford(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> Option<Vec<i32>> {
    let n = graph.len();
    let mut dist = vec![i32::MAX; n];
    dist[start] = 0;

    for _ in 0..n-1 {
        for u in 0..n {
            for &(v, w) in &graph[u] {
                if dist[u] != i32::MAX && dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                }
            }
        }
    }

    // Check for negative cycles
    for u in 0..n {
        for &(v, w) in &graph[u] {
            if dist[u] != i32::MAX && dist[u] + w < dist[v] {
                return None; // Negative cycle detected
            }
        }
    }

    Some(dist)
}

fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> Vec<i32> {
    let n = graph.len();
    let mut dist = vec![i32::MAX; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, node: start });

    while let Some(State { cost, node }) = heap.pop() {
        if cost > dist[node] { continue; }

        for &(next, weight) in &graph[node] {
            let next_cost = cost + weight;
            if next_cost < dist[next] {
                dist[next] = next_cost;
                heap.push(State { cost: next_cost, node: next });
            }
        }
    }

    dist
}

fn johnson(graph: &Vec<Vec<(usize, i32)>>) -> Option<Vec<Vec<i32>>> {
    let n = graph.len();
    let mut augmented_graph = graph.clone();
    augmented_graph.push(vec![(0, 0); n]);

    // Run Bellman-Ford from the new vertex
    let h = bellman_ford(&augmented_graph, n)?;

    // Reweight edges
    let mut reweighted_graph = vec![Vec::new(); n];
    for u in 0..n {
        for &(v, w) in &graph[u] {
            reweighted_graph[u].push((v, w + h[u] - h[v]));
        }
    }

    // Run Dijkstra from each vertex
    let mut dist = vec![vec![0; n]; n];
    for u in 0..n {
        let d = dijkstra(&reweighted_graph, u);
        for v in 0..n {
            dist[u][v] = d[v] + h[v] - h[u];
        }
    }

    Some(dist)
}

fn main() {
    let graph = vec![
        vec![(1, -1), (2, 4)],
        vec![(2, 3), (3, 2), (4, 2)],
        vec![],
        vec![(2, 5), (4, -3)],
        vec![(3, 1)]
    ];

    match johnson(&graph) {
        Some(dist) => {
            println!("All pairs shortest paths:");
            for (i, row) in dist.iter().enumerate() {
                println!("From vertex {}: {:?}", i, row);
            }
        },
        None => println!("Graph contains a negative cycle")
    }
}
```

## Explanation

1. Johnson's algorithm first adds a new vertex connected to all other vertices with zero-weight edges.
2. It then runs the Bellman-Ford algorithm from this new vertex to compute h-values for each vertex.
3. These h-values are used to reweight the edges, making all edge weights non-negative.
4. Dijkstra's algorithm is then run from each vertex on the reweighted graph.
5. Finally, the distances are adjusted back using the h-values.

## Time Complexity

O(V^2 * log V + VE), where V is the number of vertices and E is the number of edges.

## Use Cases

- Finding shortest paths in sparse graphs with negative edge weights
- Network routing protocols
- Analyzing social networks or transportation systems

Johnson's Algorithm is particularly useful when dealing with sparse graphs that may contain negative edge weights, but no negative cycles. It's more efficient than the Floyd-Warshall algorithm for sparse graphs.
