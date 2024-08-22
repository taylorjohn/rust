use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::cmp::Ordering;

// 1. Two Pointers Technique
fn two_pointers_examples() {
    // Example 1: Two Sum II - Input array is sorted
    fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = numbers[left] + numbers[right];
            match sum.cmp(&target) {
                Ordering::Equal => return vec![(left + 1) as i32, (right + 1) as i32],
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
            }
        }
        vec![]
    }
    println!("Two Sum II: {:?}", two_sum(vec![2, 7, 11, 15], 9));

    // Example 2: Remove Duplicates from Sorted Array
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut i = 0;
        for j in 1..nums.len() {
            if nums[j] != nums[i] {
                i += 1;
                nums[i] = nums[j];
            }
        }
        (i + 1) as i32
    }
    let mut nums = vec![1, 1, 2, 2, 3, 4, 4, 5];
    println!("Remove Duplicates: {}", remove_duplicates(&mut nums));
    println!("Updated Array: {:?}", nums);
}

// 2. Sliding Window
fn sliding_window_examples() {
    // Example 1: Maximum Sum Subarray of Size K
    fn max_sum_subarray(nums: &[i32], k: usize) -> i32 {
        let mut max_sum = nums.iter().take(k).sum();
        let mut window_sum = max_sum;
        for i in k..nums.len() {
            window_sum = window_sum - nums[i - k] + nums[i];
            max_sum = max_sum.max(window_sum);
        }
        max_sum
    }
    println!("Max Sum Subarray: {}", max_sum_subarray(&[1, 4, 2, 10, 23, 3, 1, 0, 20], 4));

    // Example 2: Longest Substring with At Most K Distinct Characters
    fn longest_substring_with_k_distinct(s: String, k: i32) -> i32 {
        let mut char_frequency = HashMap::new();
        let mut max_length = 0;
        let mut window_start = 0;
        
        for (window_end, ch) in s.chars().enumerate() {
            *char_frequency.entry(ch).or_insert(0) += 1;
            
            while char_frequency.len() as i32 > k {
                if let Some(count) = char_frequency.get_mut(&s.chars().nth(window_start).unwrap()) {
                    *count -= 1;
                    if *count == 0 {
                        char_frequency.remove(&s.chars().nth(window_start).unwrap());
                    }
                }
                window_start += 1;
            }
            
            max_length = max_length.max(window_end - window_start + 1);
        }
        
        max_length as i32
    }
    println!("Longest Substring with K Distinct: {}", longest_substring_with_k_distinct("araaci".to_string(), 2));
}

// 3. HashMap Techniques
fn hashmap_examples() {
    // Example 1: Two Sum
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![j as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
    println!("Two Sum: {:?}", two_sum(vec![2, 7, 11, 15], 9));

    // Example 2: Group Anagrams
    fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_groups: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut key = vec![0; 26];
            for c in s.chars() {
                key[(c as u8 - b'a') as usize] += 1;
            }
            anagram_groups.entry(key).or_default().push(s);
        }
        anagram_groups.into_values().collect()
    }
    println!("Group Anagrams: {:?}", group_anagrams(vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()]));
}

// 4. Binary Search
fn binary_search_examples() {
    // Example 1: Basic Binary Search
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
    let arr = vec![1, 3, 5, 7, 9];
    println!("Binary Search: {:?}", binary_search(&arr, &5));

    // Example 2: Search in Rotated Sorted Array
    fn search_in_rotated_array(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            
            if nums[left as usize] <= nums[mid as usize] {
                if nums[left as usize] <= target && target < nums[mid as usize] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid as usize] < target && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        -1
    }
    println!("Search in Rotated Array: {}", search_in_rotated_array(vec![4,5,6,7,0,1,2], 0));
}

// 5. Depth-First Search (DFS)
fn dfs_examples() {
    // Example 1: DFS on Graph
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
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![0, 3]);
    graph.insert(3, vec![3]);
    let mut visited = HashSet::new();
    println!("DFS on Graph:");
    dfs(&graph, 2, &mut visited);

    // Example 2: Number of Islands
    fn num_islands(grid: &mut Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    dfs_island(grid, i, j);
                    count += 1;
                }
            }
        }
        count
    }

    fn dfs_island(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] == '0' {
            return;
        }
        grid[i][j] = '0';
        dfs_island(grid, i + 1, j);
        if i > 0 { dfs_island(grid, i - 1, j); }
        dfs_island(grid, i, j + 1);
        if j > 0 { dfs_island(grid, i, j - 1); }
    }
    let mut grid = vec![
        vec!['1','1','1','1','0'],
        vec!['1','1','0','1','0'],
        vec!['1','1','0','0','0'],
        vec!['0','0','0','0','0']
    ];
    println!("Number of Islands: {}", num_islands(&mut grid));
}

// 6. Breadth-First Search (BFS)
fn bfs_examples() {
    // Example 1: BFS on Graph
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
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![0, 3]);
    graph.insert(3, vec![3]);
    println!("BFS on Graph:");
    bfs(&graph, 2);

    // Example 2: Word Ladder
    fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();
        if !word_set.contains(&end_word) {
            return 0;
        }
        let mut queue = VecDeque::new();
        queue.push_back((begin_word, 1));
        
        while let Some((word, level)) = queue.pop_front() {
            if word == end_word {
                return level;
            }
            
            for i in 0..word.len() {
                let mut new_word = word.clone();
                for c in 'a'..='z' {
                    new_word.replace_range(i..i+1, &c.to_string());
                    if word_set.remove(&new_word) {
                        queue.push_back((new_word.clone(), level + 1));
                    }
                }
            }
        }
        0
    }
    println!("Word Ladder Length: {}", ladder_length(
        "hit".to_string(),
        "cog".to_string(),
        vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string(), "cog".to_string()]
    ));
}

// 7. Dynamic Programming
fn dynamic_programming_examples() {
    // Example 1: Fibonacci
    fn fibonacci(n: usize) -> u64 {
        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        for i in 2..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n]
    }
    println!("Fibonacci(10): {}", fibonacci(10));

    // Example 2: Longest Increasing Subsequence
    fn longest_increasing_subsequence(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dp = vec![1; nums.len()];
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        *dp.iter().max().unwrap()
    }
    println!("Longest Increasing Subsequence: {}", longest_increasing_subsequence(vec![10,9,2,5,3,7,101,18]));
}

// 8. Backtracking
fn backtracking_examples() {
    // Example 1: Generate Parentheses
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
    println!("Generate Parentheses: {:?}", generate_parenthesis(3));

    // Example 2: Subsets
    fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &[i32], start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            result.push(current.clone());
            for i in start..nums.len() {
                current.push(nums[i]);
                backtrack(nums, i + 1, current, result);
                current.pop();
            }
        }
        let mut result = Vec::new();
        backtrack(&nums, 0, &mut Vec::new(), &mut result);
        result
    }
    println!("Subsets: {:?}", subsets(vec![1, 2, 3]));
}

// 9. Union-Find (Disjoint Set)
fn union_find_examples() {
    struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
    }

    impl UnionFind {
        fn new(size: usize) -> Self {
            UnionFind {