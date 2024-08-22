# Floyd-Warshall Algorithm

The Floyd-Warshall algorithm is used to find shortest paths between all pairs of vertices in a weighted graph with positive or negative edge weights (but with no negative cycles).

## Implementation

```rust
const INF: i32 = i32::MAX;

fn floyd_warshall(graph: &mut Vec<Vec<i32>>) {
    let n = graph.len();
    
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if graph[i][k] != INF && graph[k][j] != INF {
                    graph[i][j] = graph[i][j].min(graph[i][k] + graph[k][j]);
                }
            }
        }
    }
}

fn print_solution(dist: &Vec<Vec<i32>>) {
    let n = dist.len();
    println!("Shortest distances between every pair of vertices:");
    for i in 0..n {
        for j in 0..n {
            if dist[i][j] == INF {
                print!("INF ");
            } else {
                print!("{:3} ", dist[i][j]);
            }
        }
        println!();
    }
}

// Usage
fn main() {
    let mut graph = vec![
        vec![0, 5, INF, 10],
        vec![INF, 0, 3, INF],
        vec![INF, INF, 0, 1],
        vec![INF, INF, INF, 0]
    ];

    floyd_warshall(&mut graph);
    print_solution(&graph);
}
```

## Key Concepts

1. **All-Pairs Shortest Path**: Finds shortest paths between all pairs of vertices.
2. **Dynamic Programming**: Uses a bottom-up approach to build the solution.
3. **Intermediate Vertices**: Considers all vertices as potential intermediate vertices in a path.
4. **Negative Edge Weights**: Can handle negative edge weights, but not negative cycles.

## When to Use

Use the Floyd-Warshall Algorithm when:

- You need to find shortest paths between all pairs of vertices.
- The graph is dense (has many edges).
- You need to run multiple shortest path queries on the same graph.

Floyd-Warshall is particularly useful in:

- Computing the transitive closure of a graph
- Solving the all-pairs shortest path problem in computer networks
- Finding the shortest path in a weighted, potentially negative-edge graph without negative cycles

The main advantage of Floyd-Warshall is its simplicity and ability to handle all pairs of vertices in a single execution, although it has a higher time complexity of O(V³) compared to running Dijkstra's algorithm for each vertex.
