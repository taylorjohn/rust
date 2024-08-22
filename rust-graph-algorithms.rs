use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::cmp::Ordering;

// Graph representation using adjacency list
type Graph = HashMap<usize, Vec<(usize, i32)>>;

// 1. Depth-First Search (DFS)
fn dfs(graph: &Graph, start: usize, visited: &mut HashSet<usize>) {
    if !visited.insert(start) {
        return;
    }
    println!("Visited: {}", start);
    if let Some(neighbors) = graph.get(&start) {
        for &(next, _) in neighbors {
            dfs(graph, next, visited);
        }
    }
}

// 2. Breadth-First Search (BFS)
fn bfs(graph: &Graph, start: usize) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        println!("Visited: {}", node);
        if let Some(neighbors) = graph.get(&node) {
            for &(next, _) in neighbors {
                if visited.insert(next) {
                    queue.push_back(next);
                }
            }
        }
    }
}

// 3. Dijkstra's Shortest Path Algorithm
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

fn dijkstra(graph: &Graph, start: usize, end: usize) -> Option<(Vec<usize>, i32)> {
    let mut dist: HashMap<usize, i32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut prev: HashMap<usize, usize> = HashMap::new();

    dist.insert(start, 0);
    heap.push(State { cost: 0, node: start });

    while let Some(State { cost, node }) = heap.pop() {
        if node == end {
            let mut path = vec![end];
            let mut current = end;
            while let Some(&previous) = prev.get(&current) {
                path.push(previous);
                current = previous;
            }
            path.reverse();
            return Some((path, cost));
        }

        if cost > *dist.get(&node).unwrap_or(&i32::MAX) {
            continue;
        }

        if let Some(neighbors) = graph.get(&node) {
            for &(next, edge_cost) in neighbors {
                let next_cost = cost + edge_cost;
                if next_cost < *dist.get(&next).unwrap_or(&i32::MAX) {
                    heap.push(State { cost: next_cost, node: next });
                    dist.insert(next, next_cost);
                    prev.insert(next, node);
                }
            }
        }
    }

    None
}

// 4. Bellman-Ford Algorithm
fn bellman_ford(graph: &Graph, start: usize, n: usize) -> Option<HashMap<usize, i32>> {
    let mut dist: HashMap<usize, i32> = (0..n).map(|i| (i, if i == start { 0 } else { i32::MAX })).collect();

    for _ in 0..n-1 {
        for (&u, edges) in graph {
            for &(v, weight) in edges {
                if dist[&u] != i32::MAX && dist[&u] + weight < dist[&v] {
                    dist.insert(v, dist[&u] + weight);
                }
            }
        }
    }

    // Check for negative-weight cycles
    for (&u, edges) in graph {
        for &(v, weight) in edges {
            if dist[&u] != i32::MAX && dist[&u] + weight < dist[&v] {
                return None; // Negative-weight cycle detected
            }
        }
    }

    Some(dist)
}

// 5. Prim's Minimum Spanning Tree Algorithm
fn prim_mst(graph: &Graph) -> Vec<(usize, usize, i32)> {
    let mut mst = Vec::new();
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    // Start with node 0
    visited.insert(0);
    if let Some(edges) = graph.get(&0) {
        for &(v, weight) in edges {
            heap.push(std::cmp::Reverse((weight, 0, v)));
        }
    }

    while let Some(std::cmp::Reverse((weight, u, v))) = heap.pop() {
        if visited.insert(v) {
            mst.push((u, v, weight));
            if let Some(edges) = graph.get(&v) {
                for &(next_v, next_weight) in edges {
                    if !visited.contains(&next_v) {
                        heap.push(std::cmp::Reverse((next_weight, v, next_v)));
                    }
                }
            }
        }
    }

    mst
}

// 6. Topological Sort
fn topological_sort(graph: &Graph) -> Option<Vec<usize>> {
    let mut in_degree: HashMap<usize, usize> = HashMap::new();
    for edges in graph.values() {
        for &(v, _) in edges {
            *in_degree.entry(v).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<usize> = graph.keys().filter(|&k| !in_degree.contains_key(k)).cloned().collect();
    let mut result = Vec::new();

    while let Some(node) = queue.pop_front() {
        result.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &(next, _) in neighbors {
                if let Some(degree) = in_degree.get_mut(&next) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(next);
                    }
                }
            }
        }
    }

    if result.len() == graph.len() { Some(result) } else { None }
}

fn main() {
    // Example graph
    let mut graph: Graph = HashMap::new();
    graph.insert(0, vec![(1, 4), (2, 1)]);
    graph.insert(1, vec![(3, 1)]);
    graph.insert(2, vec![(1, 2), (3, 5)]);
    graph.insert(3, vec![]);

    // 1. DFS
    println!("DFS:");
    let mut visited = HashSet::new();
    dfs(&graph, 0, &mut visited);

    // 2. BFS
    println!("\nBFS:");
    bfs(&graph, 0);

    // 3. Dijkstra's Algorithm
    println!("\nDijkstra's Shortest Path:");
    if let Some((path, cost)) = dijkstra(&graph, 0, 3) {
        println!("Path: {:?}, Cost: {}", path, cost);
    } else {
        println!("No path found");
    }

    // 4. Bellman-Ford Algorithm
    println!("\nBellman-Ford Shortest Paths:");
    if let Some(distances) = bellman_ford(&graph, 0, 4) {
        for (node, dist) in distances {
            println!("Node {}: Distance {}", node, dist);
        }
    } else {
        println!("Negative cycle detected");
    }

    // 5. Prim's MST Algorithm
    println!("\nPrim's Minimum Spanning Tree:");
    let mst = prim_mst(&graph);
    for (u, v, weight) in mst {
        println!("Edge ({}, {}): Weight {}", u, v, weight);
    }

    // 6. Topological Sort
    println!("\nTopological Sort:");
    if let Some(order) = topological_sort(&graph) {
        println!("Topological Order: {:?}", order);
    } else {
        println!("Graph contains a cycle");
    }
}
