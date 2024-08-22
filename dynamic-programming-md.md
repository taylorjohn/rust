# Dynamic Programming

Dynamic Programming (DP) is an algorithmic paradigm that solves complex problems by breaking them down into simpler subproblems. It is mainly an optimization over plain recursion.

## Example Implementation: Fibonacci Sequence

```rust
fn fibonacci_dp(n: usize) -> u64 {
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

// Usage
fn main() {
    println!("10th Fibonacci number: {}", fibonacci_dp(10));
}
```

## Key Concepts

1. **Optimal Substructure**: The optimal solution to the problem can be constructed from optimal solutions of its subproblems.
2. **Overlapping Subproblems**: The problem can be broken down into subproblems which are reused several times.
3. **Memoization**: Storing the results of expensive function calls and returning the cached result when the same inputs occur again (top-down approach).
4. **Tabulation**: Building a table of results from the bottom up, typically using iteration.

## When to Use

Use Dynamic Programming when:

- The problem has overlapping subproblems and optimal substructure.
- You need to find an optimal solution (minimization or maximization).
- The problem involves making choices at each step with the consequences of these choices not obvious.

Dynamic Programming is particularly useful in:

- Optimization problems (e.g., finding the shortest path, minimum cost)
- Counting problems (e.g., number of ways to do something)
- Probability problems (e.g., stochastic processes)

Common problems solved with DP include:

- Longest Common Subsequence
- Knapsack Problem
- Matrix Chain Multiplication
- Shortest Path in a Graph
- Optimal Binary Search Tree

The key to solving DP problems is identifying the recurrence relation and deciding between a top-down (memoization) or bottom-up (tabulation) approach.
