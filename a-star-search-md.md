# A* Search Algorithm

A* is a best-first search algorithm that finds the least-cost path from a given initial node to one goal node (out of one or more possible goals). It uses a heuristic to guide its search, which makes it more efficient than algorithms like Dijkstra's for many problems.

## Implementation

```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (i32, i32),
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

fn heuristic(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn a_star(start: (i32, i32), goal: (i32, i32), obstacles: &[(i32, i32)]) -> Option<Vec<(i32, i32)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();

    g_score.insert(start, 0);
    f_score.insert(start, heuristic(start, goal));
    open_set.push(State { cost: heuristic(start, goal), position: start });

    while let Some(State { cost: _, position: current }) = open_set.pop() {
        if current == goal {
            return Some(reconstruct_path(came_from, current));
        }

        for neighbor in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let neighbor = (current.0 + neighbor.0, current.1 + neighbor.1);
            
            if obstacles.contains(&neighbor) {
                continue;
            }

            let tentative_g_score = g_score[&current] + 1;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                let f = tentative_g_score + heuristic(neighbor, goal);
                f_score.insert(neighbor, f);
                open_set.push(State { cost: f, position: neighbor });
            }
        }
    }

    None
}

fn reconstruct_path(came_from: HashMap<(i32, i32), (i32, i32)>, mut current: (i32, i32)) -> Vec<(i32, i32)> {
    let mut path = vec![current];
    while let Some(&prev) = came_from.get(&current) {
        path.push(prev);
        current = prev;
    }
    path.reverse();
    path
}

// Usage
fn main() {
    let start = (0, 0);
    let goal = (5, 5);
    let obstacles = vec![(2, 2), (2, 3), (2, 4), (3, 2), (4, 2)];

    match a_star(start, goal, &obstacles) {
        Some(path) => println!("Path found: {:?}", path),
        None => println!("No path found"),
    }
}
```

## Key Concepts

1. **Heuristic Function**: Estimates the cost from any node to the goal, guiding the search.
2. **Open Set**: A priority queue of nodes to be evaluated.
3. **Closed Set**: Nodes that have been evaluated (implicitly managed in this implementation).
4. **G-Score**: The cost of the path from the start node to the current node.
5. **F-Score**: The estimated total cost from start to goal through the current node.

## When to Use

Use A* Search when:

- You need to find the shortest path in a graph or grid with obstacles.
- You can define a heuristic function that estimates the distance to the goal.
- The search space is large and you need an efficient pathfinding algorithm.
- You're working on problems like map navigation, puzzle solving, or game AI.

A* is particularly useful in:

- Video game pathfinding
- Robotics and motion planning
- Route planning in maps and navigation systems
- Solving puzzles like the 15-puzzle or Rubik's cube

The main advantage of A* over algorithms like Dijkstra's is its use of a heuristic to guide the search towards the goal, often resulting in faster pathfinding, especially in large search spaces.
