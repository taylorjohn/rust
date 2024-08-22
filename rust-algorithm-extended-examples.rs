// Continuing from the previous Union-Find structure
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
                std::cmp::Ordering::Less => self.parent[root_x] = root_y,
                std::cmp::Ordering::Greater => self.parent[root_y] = root_x,
                std::cmp::Ordering::Equal => {
                    self.parent[root_y] = root_x;
                    self.rank[root_x] += 1;
                }
            }
        }
    }
}

fn union_find_examples() {
    // Example: Number of Connected Components
    fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n as usize);
        let mut count = n;

        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            if uf.find(x) != uf.find(y) {
                uf.union(x, y);
                count -= 1;
            }
        }

        count
    }

    let n = 5;
    let edges = vec![vec![0,1], vec![1,2], vec![3,4]];
    println!("Number of Connected Components: {}", count_components(n, edges));
}

// 10. Greedy Algorithms
fn greedy_examples() {
    // Example 1: Jump Game
    fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach = 0;
        for (i, &jump) in nums.iter().enumerate() {
            if i > max_reach {
                return false;
            }
            max_reach = max_reach.max(i + jump as usize);
        }
        true
    }
    println!("Can Jump: {}", can_jump(vec![2,3,1,1,4]));

    // Example 2: Coin Change (Greedy approach - not always optimal)
    fn coin_change_greedy(coins: &[i32], amount: i32) -> i32 {
        let mut remaining = amount;
        let mut count = 0;
        let mut coins = coins.to_vec();
        coins.sort_unstable_by(|a, b| b.cmp(a));  // Sort coins in descending order

        for &coin in &coins {
            while remaining >= coin {
                remaining -= coin;
                count += 1;
            }
        }

        if remaining == 0 { count } else { -1 }
    }
    println!("Coin Change (Greedy): {}", coin_change_greedy(&[1, 2, 5], 11));
}

// 11. Divide and Conquer
fn divide_and_conquer_examples() {
    // Example 1: Merge Sort
    fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        let mut temp = arr.to_vec();
        merge(&arr[..mid], &arr[mid..], &mut temp);
        arr.copy_from_slice(&temp);
    }

    fn merge<T: Ord + Copy>(left: &[T], right: &[T], result: &mut [T]) {
        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                result[k] = left[i];
                i += 1;
            } else {
                result[k] = right[j];
                j += 1;
            }
            k += 1;
        }
        if i < left.len() {
            result[k..].copy_from_slice(&left[i..]);
        }
        if j < right.len() {
            result[k..].copy_from_slice(&right[j..]);
        }
    }

    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    merge_sort(&mut arr);
    println!("Merge Sort: {:?}", arr);

    // Example 2: Quick Sort
    fn quick_sort<T: Ord>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        let pivot = partition(arr);
        quick_sort(&mut arr[..pivot]);
        quick_sort(&mut arr[pivot + 1..]);
    }

    fn partition<T: Ord>(arr: &mut [T]) -> usize {
        let len = arr.len();
        let pivot_index = len / 2;
        arr.swap(pivot_index, len - 1);
        let mut i = 0;
        for j in 0..len - 1 {
            if arr[j] <= arr[len - 1] {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, len - 1);
        i
    }

    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    quick_sort(&mut arr);
    println!("Quick Sort: {:?}", arr);
}

// 12. Graph Algorithms
fn graph_algorithms_examples() {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    // Example: Dijkstra's Algorithm
    fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> Vec<i32> {
        let n = graph.len();
        let mut dist = vec![i32::MAX; n];
        let mut pq = BinaryHeap::new();

        dist[start] = 0;
        pq.push(Reverse((0, start)));

        while let Some(Reverse((d, u))) = pq.pop() {
            if d > dist[u] {
                continue;
            }

            for &(v, w) in &graph[u] {
                if dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                    pq.push(Reverse((dist[v], v)));
                }
            }
        }

        dist
    }

    let graph = vec![
        vec![(1, 4), (2, 1)],
        vec![(3, 1)],
        vec![(1, 2), (3, 5)],
        vec![(4, 3)],
        vec![],
    ];
    let start = 0;
    let distances = dijkstra(&graph, start);
    println!("Dijkstra's Shortest Paths: {:?}", distances);
}

fn main() {
    union_find_examples();
    greedy_examples();
    divide_and_conquer_examples();
    graph_algorithms_examples();
}
