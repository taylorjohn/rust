use std::collections::{BinaryHeap, HashMap};

// 1. Coin Change (Greedy approach - not always optimal)
fn coin_change_greedy(coins: &[u32], amount: u32) -> Option<Vec<u32>> {
    let mut remaining = amount;
    let mut result = Vec::new();
    let mut coins = coins.to_vec();
    coins.sort_unstable_by(|a, b| b.cmp(a)); // Sort coins in descending order

    for &coin in &coins {
        while remaining >= coin {
            result.push(coin);
            remaining -= coin;
        }
    }

    if remaining == 0 {
        Some(result)
    } else {
        None
    }
}

// 2. Activity Selection
#[derive(Debug, Clone, Copy)]
struct Activity {
    start: u32,
    end: u32,
}

fn activity_selection(activities: &mut [Activity]) -> Vec<Activity> {
    activities.sort_unstable_by_key(|a| a.end);
    let mut result = Vec::new();
    let mut last_end = 0;

    for &activity in activities {
        if activity.start >= last_end {
            result.push(activity);
            last_end = activity.end;
        }
    }

    result
}

// 3. Fractional Knapsack
#[derive(Debug, Clone, Copy)]
struct Item {
    weight: f64,
    value: f64,
}

fn fractional_knapsack(items: &mut [Item], capacity: f64) -> f64 {
    items.sort_unstable_by(|a, b| {
        (b.value / b.weight).partial_cmp(&(a.value / a.weight)).unwrap()
    });

    let mut total_value = 0.0;
    let mut current_weight = 0.0;

    for item in items {
        if current_weight + item.weight <= capacity {
            current_weight += item.weight;
            total_value += item.value;
        } else {
            let remaining = capacity - current_weight;
            total_value += item.value * (remaining / item.weight);
            break;
        }
    }

    total_value
}

// 4. Huffman Coding
#[derive(Debug, Clone, Eq, PartialEq)]
struct HuffmanNode {
    char: Option<char>,
    freq: u32,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn build_huffman_tree(freq: &HashMap<char, u32>) -> Option<HuffmanNode> {
    let mut pq = BinaryHeap::new();
    for (&char, &freq) in freq {
        pq.push(HuffmanNode {
            char: Some(char),
            freq,
            left: None,
            right: None,
        });
    }

    while pq.len() > 1 {
        let left = Box::new(pq.pop()?);
        let right = Box::new(pq.pop()?);
        let parent = HuffmanNode {
            char: None,
            freq: left.freq + right.freq,
            left: Some(left),
            right: Some(right),
        };
        pq.push(parent);
    }

    pq.pop()
}

fn generate_huffman_codes(root: &HuffmanNode) -> HashMap<char, String> {
    let mut codes = HashMap::new();
    generate_codes_recursive(root, String::new(), &mut codes);
    codes
}

fn generate_codes_recursive(node: &HuffmanNode, code: String, codes: &mut HashMap<char, String>) {
    if let Some(c) = node.char {
        codes.insert(c, code);
    } else {
        if let Some(left) = &node.left {
            generate_codes_recursive(left, code.clone() + "0", codes);
        }
        if let Some(right) = &node.right {
            generate_codes_recursive(right, code + "1", codes);
        }
    }
}

// 5. Job Sequencing with Deadlines
#[derive(Debug, Clone, Copy)]
struct Job {
    id: usize,
    deadline: usize,
    profit: u32,
}

fn job_sequencing(jobs: &mut [Job]) -> Vec<Job> {
    jobs.sort_unstable_by_key(|j| std::cmp::Reverse(j.profit));
    let max_deadline = jobs.iter().map(|j| j.deadline).max().unwrap_or(0);
    let mut slot = vec![None; max_deadline];
    let mut result = Vec::new();

    for &job in jobs {
        for i in (0..job.deadline).rev() {
            if slot[i].is_none() {
                slot[i] = Some(job);
                result.push(job);
                break;
            }
        }
    }

    result
}

// 6. Minimum Spanning Tree (Prim's Algorithm)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Edge {
    to: usize,
    weight: u32,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn prim_mst(graph: &Vec<Vec<Edge>>) -> u32 {
    let n = graph.len();
    let mut total_weight = 0;
    let mut visited = vec![false; n];
    let mut pq = BinaryHeap::new();

    // Start from vertex 0
    visited[0] = true;
    for edge in &graph[0] {
        pq.push(*edge);
    }

    while let Some(Edge { to, weight }) = pq.pop() {
        if visited[to] {
            continue;
        }

        visited[to] = true;
        total_weight += weight;

        for edge in &graph[to] {
            if !visited[edge.to] {
                pq.push(*edge);
            }
        }
    }

    total_weight
}

fn main() {
    // 1. Coin Change
    let coins = vec![25, 10, 5, 1];
    let amount = 67;
    println!("Coin Change: {:?}", coin_change_greedy(&coins, amount));

    // 2. Activity Selection
    let mut activities = vec![
        Activity { start: 1, end: 4 },
        Activity { start: 3, end: 5 },
        Activity { start: 0, end: 6 },
        Activity { start: 5, end: 7 },
        Activity { start: 3, end: 8 },
        Activity { start: 5, end: 9 },
        Activity { start: 6, end: 10 },
        Activity { start: 8, end: 11 },
        Activity { start: 8, end: 12 },
        Activity { start: 2, end: 13 },
        Activity { start: 12, end: 14 },
    ];
    println!("Activity Selection: {:?}", activity_selection(&mut activities));

    // 3. Fractional Knapsack
    let mut items = vec![
        Item { weight: 10.0, value: 60.0 },
        Item { weight: 20.0, value: 100.0 },
        Item { weight: 30.0, value: 120.0 },
    ];
    let capacity = 50.0;
    println!("Fractional Knapsack: {}", fractional_knapsack(&mut items, capacity));

    // 4. Huffman Coding
    let mut freq = HashMap::new();
    freq.insert('a', 5);
    freq.insert('b', 9);
    freq.insert('c', 12);
    freq.insert('d', 13);
    freq.insert('e', 16);
    freq.insert('f', 45);
    let tree = build_huffman_tree(&freq).unwrap();
    let codes = generate_huffman_codes(&tree);
    println!("Huffman Codes: {:?}", codes);

    // 5. Job Sequencing with Deadlines
    let mut jobs = vec![
        Job { id: 1, deadline: 4, profit: 20 },
        Job { id: 2, deadline: 1, profit: 10 },
        Job { id: 3, deadline: 1, profit: 40 },
        Job { id: 4, deadline: 1, profit: 30 },
    ];
    println!("Job Sequencing: {:?}", job_sequencing(&mut jobs));

    // 6. Minimum Spanning Tree (Prim's Algorithm)
    let graph = vec![
        vec![Edge { to: 1, weight: 2 }, Edge { to: 3, weight: 6 }],
        vec![Edge { to: 0, weight: 2 }, Edge { to: 2, weight: 3 }, Edge { to: 3, weight: 8 }, Edge { to: 4, weight: 5 }],
        vec![Edge { to: 1, weight: 3 }, Edge { to: 4, weight: 7 }],
        vec![Edge { to: 0, weight: 6 }, Edge { to: 1, weight: 8 }, Edge { to: 4, weight: 9 }],
        vec![Edge { to: 1, weight: 5 }, Edge { to: 2, weight: 7 }, Edge { to: 3, weight: 9 }],
    ];
    println!("Minimum Spanning Tree Weight: {}", prim_mst(&graph));
}
