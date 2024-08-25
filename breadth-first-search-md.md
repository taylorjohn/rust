# Breadth-First Search (BFS)

Breadth-First Search is a graph traversal algorithm that explores all the vertices of a graph or all the nodes of a tree at the present depth before moving to the vertices at the next depth level.

## Basic Implementation

```rust
use std::collections::{HashMap, HashSet, VecDeque};

type Graph = HashMap<i32, Vec<i32>>;

fn bfs(graph: &Graph, start: i32) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(vertex) = queue.pop_front() {
        result.push(vertex);

        if let Some(neighbors) = graph.get(&vertex) {
            for &neighbor in neighbors {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    result
}

// Usage
fn main() {
    let mut graph = Graph::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![0, 3]);
    graph.insert(3, vec![3]);

    let traversal = bfs(&graph, 2);
    println!("BFS traversal: {:?}", traversal);
}
```

## Variations

### 1. BFS for Shortest Path in Unweighted Graph

```rust
fn bfs_shortest_path(graph: &Graph, start: i32, end: i32) -> Option<Vec<i32>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parent = HashMap::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(vertex) = queue.pop_front() {
        if vertex == end {
            return Some(reconstruct_path(start, end, &parent));
        }

        if let Some(neighbors) = graph.get(&vertex) {
            for &neighbor in neighbors {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                    parent.insert(neighbor, vertex);
                }
            }
        }
    }

    None
}

fn reconstruct_path(start: i32, end: i32, parent: &HashMap<i32, i32>) -> Vec<i32> {
    let mut path = vec![end];
    let mut current = end;
    while current != start {
        current = *parent.get(&current).unwrap();
        path.push(current);
    }
    path.reverse();
    path
}

// Usage
fn main() {
    let mut graph = Graph::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![]);

    if let Some(path) = bfs_shortest_path(&graph, 0, 3) {
        println!("Shortest path: {:?}", path);
    } else {
        println!("No path found");
    }
}
```

### 2. BFS for Bipartite Graph Check

```rust
fn is_bipartite(graph: &Graph) -> bool {
    let mut color = HashMap::new();
    let mut queue = VecDeque::new();

    for &start in graph.keys() {
        if !color.contains_key(&start) {
            color.insert(start, false);
            queue.push_back(start);

            while let Some(v) = queue.pop_front() {
                if let Some(neighbors) = graph.get(&v) {
                    for &u in neighbors {
                        if !color.contains_key(&u) {
                            color.insert(u, !color[&v]);
                            queue.push_back(u);
                        } else if color[&u] == color[&v] {
                            return false;
                        }
                    }
                }
            }
        }
    }

    true
}

// Usage
fn main() {
    let mut graph = Graph::new();
    graph.insert(0, vec![1, 3]);
    graph.insert(1, vec![0, 2]);
    graph.insert(2, vec![1, 3]);
    graph.insert(3, vec![0, 2]);

    println!("Is the graph bipartite? {}", is_bipartite(&graph));
}
```

## When to Use

Use Breadth-First Search when:

- You need to find the shortest path between two nodes in an unweighted graph.
- You want to explore all neighbors at the present depth before moving to nodes at the next depth level.
- You're working with a graph that may have cycles.
- You need to find all nodes within a connected component.

BFS is particularly useful in:

- Social networking applications (finding degrees of separation)
- Web crawlers
- GPS navigation systems
- Puzzle solving (e.g., solving a Rubik's cube in the fewest moves)
- Testing graph bipartiteness

## Time Complexity

- Time Complexity: O(V + E), where V is the number of vertices and E is the number of edges in the graph.
- Space Complexity: O(V) for the queue and visited set.

## Advantages and Disadvantages

Advantages:
- Guarantees the shortest path in unweighted graphs
- Explores nodes level by level
- Can find all nodes at a given depth from the start node

Disadvantages:
- Uses more memory than depth-first search
- May be slower than depth-first search for finding a specific target node that's far from the start

BFS is often the algorithm of choice when you need to explore a graph in a breadth-first manner or when finding the shortest path in an unweighted graph.
