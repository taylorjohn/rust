// 1. Fibonacci Sequence
// Time Complexity: O(n), Space Complexity: O(n)
fn fibonacci(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}

// 2. Longest Increasing Subsequence (LIS)
// Time Complexity: O(n^2), Space Complexity: O(n)
fn longest_increasing_subsequence(nums: &[i32]) -> usize {
    if nums.is_empty() {
        return 0;
    }
    let mut dp = vec![1; nums.len()];
    let mut max_length = 1;
    
    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] > nums[j] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
        max_length = max_length.max(dp[i]);
    }
    
    max_length
}

// 3. Knapsack Problem
// Time Complexity: O(nW), Space Complexity: O(nW)
fn knapsack(weights: &[i32], values: &[i32], capacity: i32) -> i32 {
    let n = weights.len();
    let mut dp = vec![vec![0; (capacity + 1) as usize]; n + 1];
    
    for i in 1..=n {
        for w in 0..=capacity {
            if weights[i - 1] <= w {
                dp[i][w as usize] = dp[i - 1][w as usize].max(
                    values[i - 1] + dp[i - 1][(w - weights[i - 1]) as usize]
                );
            } else {
                dp[i][w as usize] = dp[i - 1][w as usize];
            }
        }
    }
    
    dp[n][capacity as usize]
}

// 4. Coin Change Problem
// Time Complexity: O(amount * coins.len()), Space Complexity: O(amount)
fn coin_change(coins: &[i32], amount: i32) -> i32 {
    let mut dp = vec![amount + 1; (amount + 1) as usize];
    dp[0] = 0;
    
    for i in 1..=amount {
        for &coin in coins {
            if coin <= i {
                dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize] + 1);
            }
        }
    }
    
    if dp[amount as usize] > amount {
        -1
    } else {
        dp[amount as usize]
    }
}

// 5. Edit Distance
// Time Complexity: O(mn), Space Complexity: O(mn)
fn edit_distance(word1: &str, word2: &str) -> usize {
    let m = word1.len();
    let n = word2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    
    for (i, c1) in word1.chars().enumerate() {
        for (j, c2) in word2.chars().enumerate() {
            if c1 == c2 {
                dp[i + 1][j + 1] = dp[i][j];
            } else {
                dp[i + 1][j + 1] = 1 + dp[i][j].min(dp[i + 1][j]).min(dp[i][j + 1]);
            }
        }
    }
    
    dp[m][n]
}

// 6. Longest Common Subsequence (LCS)
// Time Complexity: O(mn), Space Complexity: O(mn)
fn longest_common_subsequence(text1: &str, text2: &str) -> usize {
    let m = text1.len();
    let n = text2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    for (i, c1) in text1.chars().enumerate() {
        for (j, c2) in text2.chars().enumerate() {
            if c1 == c2 {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }
    
    dp[m][n]
}

// 7. Matrix Chain Multiplication
// Time Complexity: O(n^3), Space Complexity: O(n^2)
fn matrix_chain_multiplication(dimensions: &[i32]) -> i32 {
    let n = dimensions.len() - 1;
    let mut dp = vec![vec![0; n]; n];
    
    for len in 2..=n {
        for i in 0..n - len + 1 {
            let j = i + len - 1;
            dp[i][j] = i32::MAX;
            for k in i..j {
                let cost = dp[i][k] + dp[k + 1][j] + dimensions[i] * dimensions[k + 1] * dimensions[j + 1];
                dp[i][j] = dp[i][j].min(cost);
            }
        }
    }
    
    dp[0][n - 1]
}

fn main() {
    println!("Fibonacci(10): {}", fibonacci(10));
    
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    println!("Longest Increasing Subsequence: {}", longest_increasing_subsequence(&nums));
    
    let weights = vec![10, 20, 30];
    let values = vec![60, 100, 120];
    println!("Knapsack (capacity 50): {}", knapsack(&weights, &values, 50));
    
    let coins = vec![1, 2, 5];
    println!("Coin Change (amount 11): {}", coin_change(&coins, 11));
    
    println!("Edit Distance between 'horse' and 'ros': {}", edit_distance("horse", "ros"));
    
    println!("Longest Common Subsequence of 'abcde' and 'ace': {}", longest_common_subsequence("abcde", "ace"));
    
    let matrix_dimensions = vec![40, 20, 30, 10, 30];
    println!("Matrix Chain Multiplication: {}", matrix_chain_multiplication(&matrix_dimensions));
}
