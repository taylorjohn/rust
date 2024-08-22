use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

// Define a simple graph structure
type Graph = HashMap<char, Vec<(char, u32)>>;

// Node for A* and Dijkstra's algorithms
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: char,
    cost: u32,
    heuristic: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Breadth-First Search (BFS)
fn bfs(graph: &Graph, start: char, goal: char) -> Option<Vec<char>> {
    let mut queue = std::collections::VecDeque::new();
    let mut visited = HashSet::new();
    let mut came_from = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == goal {
            return Some(reconstruct_path(&came_from, start, goal));
        }

        if let Some(neighbors) = graph.get(&current) {
            for &(next, _) in neighbors {
                if visited.insert(next) {
                    queue.push_back(next);
                    came_from.insert(next, current);
                }
            }
        }
    }

    None
}

// Depth-First Search (DFS)
fn dfs(graph: &Graph, start: char, goal: char) -> Option<Vec<char>> {
    let mut stack = vec![start];
    let mut visited = HashSet::new();
    let mut came_from = HashMap::new();

    while let Some(current) = stack.pop() {
        if current == goal {
            return Some(reconstruct_path(&came_from, start, goal));
        }

        if visited.insert(current) {
            if let Some(neighbors) = graph.get(&current) {
                for &(next, _) in neighbors {
                    if !visited.contains(&next) {
                        stack.push(next);
                        came_from.insert(next, current);
                    }
                }
            }
        }
    }

    None
}

// Dijkstra's Algorithm
fn dijkstra(graph: &Graph, start: char, goal: char) -> Option<(Vec<char>, u32)> {
    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::new();
    let mut came_from = HashMap::new();

    heap.push(Node { position: start, cost: 0, heuristic: 0 });
    dist.insert(start, 0);

    while let Some(Node { position, cost, .. }) = heap.pop() {
        if position == goal {
            return Some((reconstruct_path(&came_from, start, goal), cost));
        }

        if cost > *dist.get(&position).unwrap_or(&u32::MAX) {
            continue;
        }

        if let Some(neighbors) = graph.get(&position) {
            for &(next, edge_cost) in neighbors {
                let new_cost = cost + edge_cost;
                if new_cost < *dist.get(&next).unwrap_or(&u32::MAX) {
                    heap.push(Node { position: next, cost: new_cost, heuristic: 0 });
                    dist.insert(next, new_cost);
                    came_from.insert(next, position);
                }
            }
        }
    }

    None
}

// A* Algorithm
fn a_star(graph: &Graph, start: char, goal: char, h: impl Fn(char) -> u32) -> Option<(Vec<char>, u32)> {
    let mut heap = BinaryHeap::new();
    let mut g_score = HashMap::new();
    let mut came_from = HashMap::new();

    heap.push(Node { position: start, cost: 0, heuristic: h(start) });
    g_score.insert(start, 0);

    while let Some(Node { position, cost, .. }) = heap.pop() {
        if position == goal {
            return Some((reconstruct_path(&came_from, start, goal), cost));
        }

        if cost > *g_score.get(&position).unwrap_or(&u32::MAX) {
            continue;
        }

        if let Some(neighbors) = graph.get(&position) {
            for &(next, edge_cost) in neighbors {
                let tentative_g_score = cost + edge_cost;
                if tentative_g_score < *g_score.get(&next).unwrap_or(&u32::MAX) {
                    came_from.insert(next, position);
                    g_score.insert(next, tentative_g_score);
                    heap.push(Node {
                        position: next,
                        cost: tentative_g_score,
                        heuristic: h(next),
                    });
                }
            }
        }
    }

    None
}

// Helper function to reconstruct the path
fn reconstruct_path(came_from: &HashMap<char, char>, start: char, goal: char) -> Vec<char> {
    let mut path = vec![goal];
    let mut current = goal;
    while current != start {
        current = *came_from.get(&current).unwrap();
        path.push(current);
    }
    path.reverse();
    path
}

fn main() {
    // Create a sample graph
    let mut graph = Graph::new();
    graph.insert('A', vec![('B', 4), ('C', 2)]);
    graph.insert('B', vec![('D', 3), ('E', 1)]);
    graph.insert('C', vec![('F', 5)]);
    graph.insert('D', vec![('G', 2)]);
    graph.insert('E', vec![('G', 3)]);
    graph.insert('F', vec![('G', 1)]);
    graph.insert('G', vec![]);

    let start = 'A';
    let goal = 'G';

    // BFS
    if let Some(path) = bfs(&graph, start, goal) {
        println!("BFS Path: {:?}", path);
    }

    // DFS
    if let Some(path) = dfs(&graph, start, goal) {
        println!("DFS Path: {:?}", path);
    }

    // Dijkstra's Algorithm
    if let Some((path, cost)) = dijkstra(&graph, start, goal) {
        println!("Dijkstra Path: {:?}, Cost: {}", path, cost);
    }

    // A* Algorithm
    let h = |_| 0; // No heuristic for this example
    if let Some((path, cost)) = a_star(&graph, start, goal, h) {
        println!("A* Path: {:?}, Cost: {}", path, cost);
    }
}
