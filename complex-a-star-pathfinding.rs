use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

// Terrain types with associated movement costs
#[derive(Clone, Copy, PartialEq, Eq)]
enum Terrain {
    Plain = 1,
    Forest = 3,
    Mountain = 5,
    Water = 10,
    Obstacle = u8::MAX as isize,
}

// Direction enum for neighbor calculation
enum Direction {
    North, South, East, West,
    NorthEast, NorthWest, SouthEast, SouthWest,
}

impl Direction {
    fn all() -> [Direction; 8] {
        use Direction::*;
        [North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest]
    }
}

// Position on the grid
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn manhattan_distance(&self, other: &Position) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

// Node for A* algorithm
#[derive(Clone, Eq, PartialEq)]
struct Node {
    position: Position,
    g_cost: u32,
    f_cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost.cmp(&self.f_cost)
            .then_with(|| self.position.x.cmp(&other.position.x))
            .then_with(|| self.position.y.cmp(&other.position.y))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Grid representation
struct Grid {
    width: i32,
    height: i32,
    terrain: HashMap<Position, Terrain>,
    elevation: HashMap<Position, i32>,
}

impl Grid {
    fn new(width: i32, height: i32) -> Self {
        Grid {
            width,
            height,
            terrain: HashMap::new(),
            elevation: HashMap::new(),
        }
    }

    fn set_terrain(&mut self, pos: Position, terrain: Terrain) {
        self.terrain.insert(pos, terrain);
    }

    fn set_elevation(&mut self, pos: Position, elevation: i32) {
        self.elevation.insert(pos, elevation);
    }

    fn get_terrain(&self, pos: &Position) -> Terrain {
        *self.terrain.get(pos).unwrap_or(&Terrain::Plain)
    }

    fn get_elevation(&self, pos: &Position) -> i32 {
        *self.elevation.get(pos).unwrap_or(&0)
    }

    fn is_valid_position(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }

    fn get_neighbors(&self, pos: &Position) -> Vec<Position> {
        Direction::all().iter().filter_map(|dir| {
            let new_pos = match dir {
                Direction::North => Position::new(pos.x, pos.y + 1),
                Direction::South => Position::new(pos.x, pos.y - 1),
                Direction::East => Position::new(pos.x + 1, pos.y),
                Direction::West => Position::new(pos.x - 1, pos.y),
                Direction::NorthEast => Position::new(pos.x + 1, pos.y + 1),
                Direction::NorthWest => Position::new(pos.x - 1, pos.y + 1),
                Direction::SouthEast => Position::new(pos.x + 1, pos.y - 1),
                Direction::SouthWest => Position::new(pos.x - 1, pos.y - 1),
            };
            if self.is_valid_position(&new_pos) {
                Some(new_pos)
            } else {
                None
            }
        }).collect()
    }
}

// Complex heuristic function
fn complex_heuristic(grid: &Grid, current: &Position, goal: &Position) -> u32 {
    let base_distance = current.manhattan_distance(goal);
    let terrain_cost = match grid.get_terrain(current) {
        Terrain::Plain => 1,
        Terrain::Forest => 2,
        Terrain::Mountain => 3,
        Terrain::Water => 5,
        Terrain::Obstacle => u32::MAX,
    };
    let elevation_diff = (grid.get_elevation(current) - grid.get_elevation(goal)).abs() as u32;
    let elevation_penalty = elevation_diff * 2;

    // Encourage moving towards areas with better terrain
    let terrain_improvement = Direction::all().iter()
        .filter_map(|dir| {
            let new_pos = match dir {
                Direction::North => Position::new(current.x, current.y + 1),
                Direction::South => Position::new(current.x, current.y - 1),
                Direction::East => Position::new(current.x + 1, current.y),
                Direction::West => Position::new(current.x - 1, current.y),
                Direction::NorthEast => Position::new(current.x + 1, current.y + 1),
                Direction::NorthWest => Position::new(current.x - 1, current.y + 1),
                Direction::SouthEast => Position::new(current.x + 1, current.y - 1),
                Direction::SouthWest => Position::new(current.x - 1, current.y - 1),
            };
            if grid.is_valid_position(&new_pos) {
                Some(grid.get_terrain(&new_pos) as u32)
            } else {
                None
            }
        })
        .min()
        .unwrap_or(u32::MAX);

    // Weighted sum of all factors
    base_distance * 2 + terrain_cost * 3 + elevation_penalty + terrain_improvement
}

// A* pathfinding algorithm
fn a_star(grid: &Grid, start: Position, goal: Position) -> Option<Vec<Position>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();

    g_score.insert(start, 0);
    f_score.insert(start, complex_heuristic(grid, &start, &goal));
    open_set.push(Node { position: start, g_cost: 0, f_cost: f_score[&start] });

    while let Some(current) = open_set.pop() {
        if current.position == goal {
            return Some(reconstruct_path(came_from, current.position));
        }

        for neighbor in grid.get_neighbors(&current.position) {
            let neighbor_terrain = grid.get_terrain(&neighbor);
            if neighbor_terrain == Terrain::Obstacle {
                continue;
            }

            let tentative_g_score = g_score[&current.position] + neighbor_terrain as u32;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, current.position);
                g_score.insert(neighbor, tentative_g_score);
                let f_score_neighbor = tentative_g_score + complex_heuristic(grid, &neighbor, &goal);
                f_score.insert(neighbor, f_score_neighbor);
                open_set.push(Node { position: neighbor, g_cost: tentative_g_score, f_cost: f_score_neighbor });
            }
        }
    }

    None
}

fn reconstruct_path(came_from: HashMap<Position, Position>, current: Position) -> Vec<Position> {
    let mut path = vec![current];
    let mut current = current;
    while let Some(&prev) = came_from.get(&current) {
        path.push(prev);
        current = prev;
    }
    path.reverse();
    path
}

fn main() {
    let mut grid = Grid::new(10, 10);

    // Set up some terrain and elevation
    for x in 0..10 {
        for y in 0..10 {
            let pos = Position::new(x, y);
            if x == 5 && y < 8 {
                grid.set_terrain(pos, Terrain::Obstacle);
            } else if x < 3 && y < 3 {
                grid.set_terrain(pos, Terrain::Water);
            } else if x > 7 && y > 7 {
                grid.set_terrain(pos, Terrain::Mountain);
            } else if (x + y) % 3 == 0 {
                grid.set_terrain(pos, Terrain::Forest);
            }
            grid.set_elevation(pos, ((x + y) % 5) as i32);
        }
    }

    let start = Position::new(0, 0);
    let goal = Position::new(9, 9);

    if let Some(path) = a_star(&grid, start, goal) {
        println!("Path found: {:?}", path);
    } else {
        println!("No path found");
    }
}
