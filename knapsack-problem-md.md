# Knapsack Problem

The Knapsack Problem is a problem in combinatorial optimization: Given a set of items, each with a weight and a value, determine the number of each item to include in a collection so that the total weight is less than or equal to a given limit and the total value is as large as possible.

## Implementation

```rust
#[derive(Debug)]
struct Item {
    weight: usize,
    value: usize,
}

fn knapsack(items: &[Item], capacity: usize) -> usize {
    let n = items.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    for i in 1..=n {
        for w in 1..=capacity {
            if items[i-1].weight <= w {
                dp[i][w] = dp[i-1][w].max(
                    dp[i-1][w - items[i-1].weight] + items[i-1].value
                );
            } else {
                dp[i][w] = dp[i-1][w];
            }
        }
    }

    dp[n][capacity]
}

// Usage
fn main() {
    let items = vec![
        Item { weight: 10, value: 60 },
        Item { weight: 20, value: 100 },
        Item { weight: 30, value: 120 },
    ];
    let capacity = 50;

    let max_value = knapsack(&items, capacity);
    println!("Maximum value: {}", max_value);
}
```

## Key Concepts

1. **Dynamic Programming**: The problem is solved by breaking it down into smaller subproblems.
2. **Optimal Substructure**: The optimal solution can be constructed from optimal solutions of its subproblems.
3. **2D Table**: Uses a 2D table to store intermediate results.
4. **Bottom-up Approach**: Builds the solution for larger problems using solutions of smaller problems.

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

The main advantage of the Dynamic Programming approach to the Knapsack Problem is its ability to find the optimal solution in pseudo-polynomial time, which is much faster than the exponential time of a naive recursive approach for practical problem sizes.
