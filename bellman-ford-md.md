# Bellman-Ford Algorithm

The Bellman-Ford algorithm is used to find the shortest paths from a single source vertex to all other vertices in a weighted graph. Unlike Dijkstra's algorithm, it can handle graphs with negative weight edges.

## Implementation

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Edge {
    from: usize,
    to: usize,
    weight: i32,
}

fn bellman_ford(graph: &[Edge], vertex_count: usize, start: usize) -> Option<Vec<i32>> {
    let mut distances = vec![i32::MAX; vertex_count];
    distances[start] = 0;

    // Relax edges |V| - 1 times
    for _ in 0..vertex_count - 1 {
        for edge in graph {
            if distances[edge.from] != i32::MAX && 
               distances[edge.from] + edge.weight < distances[edge.to] {
                distances[edge.to] = distances[edge.from] + edge.weight;
            }
        }
    }

    // Check for negative-weight cycles
    for edge in graph {
        if distances[edge.from] != i32::MAX && 
           distances[edge.from] + edge.weight < distances[edge.to] {
            return None; // Negative-weight cycle detected
        }
    }

    Some(distances)
}

// Usage
fn main() {
    let edges = vec![
        Edge { from: 0, to: 1, weight: -1 },
        Edge { from: 0, to: 2, weight: 4 },
        Edge { from: 1, to: 2, weight: 3 },
        Edge { from: 1, to: 3, weight: 2 },
        Edge { from: 1, to: 4, weight: 2 },
        Edge { from: 3, to: 2, weight: 5 },
        Edge { from: 3, to: 1, weight: 1 },
        Edge { from: 4, to: 3, weight: -3 },
    ];

    let vertex_count = 5;
    let start = 0;

    match bellman_ford(&edges, vertex_count, start) {
        Some(distances) => {
            println!("Shortest distances from vertex {}:", start);
            for (vertex, distance) in distances.iter().enumerate() {
                println!("To vertex {}: {}", vertex, distance);
            }
        },
        None => println!("Graph contains a negative-weight cycle"),
    }
}
```

## Key Concepts

1. **Negative Edge Weights**: Can handle graphs with negative edge weights.
2. **Negative Cycle Detection**: Can detect negative cycles in the graph.
3. **Relaxation**: Repeatedly relaxes all edges to find shortest paths.
4. **Time Complexity**: O(VE), where V is the number of vertices and E is the number of edges.

## When to Use

Use the Bellman-Ford Algorithm when:

- The graph might contain negative edge weights.
- You need to detect negative cycles in the graph.
- You need to find shortest paths from a single source to all other vertices.

Bellman-Ford is particularly useful in:

- Routing protocols like RIP (Routing Information Protocol)
- Arbitrage detection in currency exchange
- Finding maximum flow in flow networks with negative edge capacities

The main advantage of Bellman-Ford over Dijkstra's algorithm is its ability to handle negative edge weights and detect negative cycles, although it is generally slower for graphs without negative edges.
