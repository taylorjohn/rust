use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::cmp::Ordering;

// 1. Two Pointers Technique
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let sum = nums[left] + nums[right];
        match sum.cmp(&target) {
            Ordering::Equal => return Some((left, right)),
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
        }
    }
    None
}

// 2. Sliding Window
fn max_sum_subarray(nums: &[i32], k: usize) -> i32 {
    let mut max_sum = nums.iter().take(k).sum();
    let mut window_sum = max_sum;
    for i in k..nums.len() {
        window_sum = window_sum - nums[i - k] + nums[i];
        max_sum = max_sum.max(window_sum);
    }
    max_sum
}

// 3. HashMap for Frequency Count
fn most_frequent_element(nums: &[i32]) -> Option<i32> {
    let mut freq_map = HashMap::new();
    for &num in nums {
        *freq_map.entry(num).or_insert(0) += 1;
    }
    freq_map.into_iter().max_by_key(|&(_, count)| count).map(|(num, _)| num)
}

// 4. Binary Search
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }
    None
}

// 5. Depth-First Search (DFS) on Graph
fn dfs(graph: &HashMap<i32, Vec<i32>>, start: i32, visited: &mut HashSet<i32>) {
    if !visited.insert(start) {
        return;
    }
    println!("Visited: {}", start);
    if let Some(neighbors) = graph.get(&start) {
        for &neighbor in neighbors {
            dfs(graph, neighbor, visited);
        }
    }
}

// 6. Breadth-First Search (BFS) on Graph
fn bfs(graph: &HashMap<i32, Vec<i32>>, start: i32) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(start);
    visited.insert(start);
    while let Some(node) = queue.pop_front() {
        println!("Visited: {}", node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
    }
}

// 7. Dynamic Programming: Fibonacci
fn fibonacci(n: usize) -> u64 {
    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}

// 8. Backtracking: Generate Parentheses
fn generate_parenthesis(n: i32) -> Vec<String> {
    fn backtrack(s: &mut String, open: i32, close: i32, result: &mut Vec<String>) {
        if open == 0 && close == 0 {
            result.push(s.clone());
            return;
        }
        if open > 0 {
            s.push('(');
            backtrack(s, open - 1, close, result);
            s.pop();
        }
        if close > open {
            s.push(')');
            backtrack(s, open, close - 1, result);
            s.pop();
        }
    }
    let mut result = Vec::new();
    backtrack(&mut String::new(), n, n, &mut result);
    result
}

// 9. Union-Find (Disjoint Set)
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            match self.rank[root_x].cmp(&self.rank[root_y]) {
                Ordering::Less => self.parent[root_x] = root_y,
                Ordering::Greater => self.parent[root_y] = root_x,
                Ordering::Equal => {
                    self.parent[root_y] = root_x;
                    self.rank[root_x] += 1;
                }
            }
        }
    }
}

// 10. Priority Queue (Binary Heap)
#[derive(Eq, PartialEq)]
struct Item {
    value: i32,
    priority: i32,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn priority_queue_example() {
    let mut pq = BinaryHeap::new();
    pq.push(Item { value: 5, priority: 1 });
    pq.push(Item { value: 3, priority: 3 });
    pq.push(Item { value: 7, priority: 2 });
    while let Some(Item { value, priority }) = pq.pop() {
        println!("Value: {}, Priority: {}", value, priority);
    }
}

// 11. Trie (Prefix Tree)
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::new() }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TrieNode::new());
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
            } else {
                return false;
            }
        }
        node.is_end
    }
}

fn main() {
    // Example usage of the snippets
    let nums = vec![2, 7, 11, 15];
    println!("Two Sum: {:?}", two_sum(&nums, 9));

    let nums = vec![1, 4, 2, 10, 23, 3, 1, 0, 20];
    println!("Max Sum Subarray: {}", max_sum_subarray(&nums, 4));

    let nums = vec![1, 2, 3, 4, 1, 2, 3, 1];
    println!("Most Frequent Element: {:?}", most_frequent_element(&nums));

    let sorted_arr = vec![1, 3, 5, 7, 9];
    println!("Binary Search: {:?}", binary_search(&sorted_arr, &5));

    println!("Fibonacci(10): {}", fibonacci(10));

    println!("Generate Parentheses: {:?}", generate_parenthesis(3));

    priority_queue_example();

    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    println!("Trie Search 'apple': {}", trie.search("apple".to_string()));
    println!("Trie Search 'app': {}", trie.search("app".to_string()));
}
