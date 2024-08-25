# Dijkstra's Algorithm

Dijkstra's algorithm is used to find the shortest path between nodes in a graph. It works on both directed and undirected graphs and can handle weighted edges.

## Implementation

```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
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

fn dijkstra(graph: &HashMap<usize, Vec<(usize, usize)>>, start: usize, goal: usize) -> Option<(Vec<usize>, usize)> {
    let mut dist: HashMap<usize, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut prev: HashMap<usize, usize> = HashMap::new();

    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            let mut path = vec![goal];
            let mut current = goal;
            while let Some(&previous) = prev.get(&current) {
                path.push(previous);
                current = previous;
            }
            path.reverse();
            return Some((path, cost));
        }

        if cost > *dist.get(&position).unwrap_or(&std::usize::MAX) {
            continue;
        }

        if let Some(neighbors) = graph.get(&position) {
            for &(neighbor, edge_cost) in neighbors {
                let next = State { cost: cost + edge_cost, position: neighbor };
                if next.cost < *dist.get(&neighbor).unwrap_or(&std::usize::MAX) {
                    heap.push(next);
                    dist.insert(neighbor, next.cost);
                    prev.insert(neighbor, position);
                }
            }
        }
    }

    None
}

// Usage
fn main() {
    let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    graph.insert(0, vec![(1, 4), (2, 1)]);
    graph.insert(1, vec![(3, 1)]);
    graph.insert(2, vec![(1, 2), (3, 5)]);
    graph.insert(3, vec![]);

    if let Some((path, cost)) = dijkstra(&graph, 0, 3) {
        println!("Shortest path: {:?}", path);
        println!("Total cost: {}", cost);
    } else {
        println!("No path found");
    }
}
```

## When to Use

Use Dijkstra's algorithm when:

- You need to find the shortest path between a start node and all other nodes in a weighted graph.
- The graph has non-negative edge weights.
- You're working with road networks, computer networks, or any problem that can be modeled as a graph with weighted edges.

Dijkstra's algorithm is particularly useful in:

- GPS and mapping systems for finding the shortest route
- Network routing protocols
- Solving puzzles and games with weighted state transitions
- Robotics for path planning

## Time Complexity

The time complexity of Dijkstra's algorithm with a binary heap is O((V + E) log V), where V is the number of vertices and E is the number of edges in the graph.

## Space Complexity

The space complexity is O(V), where V is the number of vertices.

## Advantages and Disadvantages

Advantages:
- Finds the shortest path in a weighted graph
- Efficient for sparse graphs

Disadvantages:
- Doesn't work with negative edge weights
- Can be slower for dense graphs compared to the Floyd-Warshall algorithm

## Variations

1. Bidirectional Dijkstra: Runs two simultaneous searches, one from the start and one from the goal.
2. A* Search: An extension of Dijkstra's algorithm that uses heuristics to guide the search.
3. Dijkstra with Priority Queue: Uses a priority queue instead of a binary heap for better performance in some cases.

Dijkstra's algorithm is a fundamental graph algorithm and serves as a basis for many other path-finding algorithms.
