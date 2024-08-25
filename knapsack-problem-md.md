# Knapsack Problem

The Knapsack Problem is a problem in combinatorial optimization: Given a set of items, each with a weight and a value, determine the number of each item to include in a collection so that the total weight is less than or equal to a given limit and the total value is as large as possible.

## Implementation

```rust
fn knapsack(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let n = weights.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    for i in 1..=n {
        for w in 0..=capacity {
            if weights[i-1] <= w {
                dp[i][w] = dp[i-1][w].max(dp[i-1][w - weights[i-1]] + values[i-1]);
            } else {
                dp[i][w] = dp[i-1][w];
            }
        }
    }

    dp[n][capacity]
}

// Usage
fn main() {
    let weights = vec![10, 20, 30];
    let values = vec![60, 100, 120];
    let capacity = 50;
    
    let max_value = knapsack(&weights, &values, capacity);
    println!("Maximum value: {}", max_value);
}
```

## When to Use

Use the Knapsack algorithm when:

- You need to select a subset of items with maximum value, given a weight constraint.
- Optimizing resource allocation problems.
- Solving packing or loading problems in logistics.
- Portfolio optimization in finance.

The Knapsack Problem is particularly useful in:

- Resource allocation in various industries
- Cargo loading
- Budget-constrained procurement decisions
- Cutting stock problems in manufacturing

## Time Complexity

The time complexity of this dynamic programming solution is O(nW), where n is the number of items and W is the capacity of the knapsack.

## Space Complexity

The space complexity is also O(nW) due to the 2D array used for memoization.

## Variations

1. 0/1 Knapsack: Each item can be picked only once (implemented above).
2. Fractional Knapsack: Items can be broken into smaller units (solved using a greedy approach).
3. Bounded Knapsack: Each item has a limited number of copies.
4. Unbounded Knapsack: Each item has unlimited copies.

These variations can be solved by modifying the basic dynamic programming approach or using different algorithms altogether.

The Knapsack Problem is a classic example of dynamic programming and serves as a foundation for understanding and solving many optimization problems.
