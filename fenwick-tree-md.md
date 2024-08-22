# Fenwick Tree (Binary Indexed Tree)

A Fenwick Tree, also known as a Binary Indexed Tree, is a data structure that can efficiently update elements and calculate prefix sums in a table of numbers.

## Implementation

```rust
struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        FenwickTree {
            tree: vec![0; size + 1],
        }
    }

    fn update(&mut self, mut index: usize, value: i32) {
        index += 1; // 1-based indexing
        while index < self.tree.len() {
            self.tree[index] += value;
            index += index & (!index + 1);
        }
    }

    fn sum(&self, mut index: usize) -> i32 {
        index += 1; // 1-based indexing
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= index & (!index + 1);
        }
        sum
    }

    fn range_sum(&self, left: usize, right: usize) -> i32 {
        self.sum(right) - self.sum(left - 1)
    }
}

// Usage
fn main() {
    let mut ft = FenwickTree::new(10);
    
    // Update values
    ft.update(1, 3);
    ft.update(3, 2);
    ft.update(5, 1);

    println!("Sum up to index 5: {}", ft.sum(5));
    println!("Sum from index 2 to 6: {}", ft.range_sum(2, 6));

    // Update existing value
    ft.update(3, 4); // Adds 4 to the existing value at index 3

    println!("New sum up to index 5: {}", ft.sum(5));
}
```

## Key Concepts

1. **Binary Representation**: Uses the binary representation of indices to efficiently update and query.
2. **Prefix Sums**: Efficiently computes prefix sums in logarithmic time.
3. **Range Queries**: Supports range sum queries through the difference of two prefix sums.
4. **Space Efficiency**: Uses only O(n) space for n elements.

## When to Use

Use a Fenwick Tree when:

- You need to perform both element updates and prefix sum queries efficiently.
- Working with a changeable array and require fast range sum calculations.
- Implementing algorithms that require frequent updates and sum queries, like counting inversions.

Fenwick Trees are particularly useful in:

- Computational geometry for range searching
- Implementing dynamic cumulative frequency tables
- Solving problems in competitive programming that involve range queries and updates
- Data analysis tasks requiring dynamic histogram computations

The main advantage of Fenwick Trees is their ability to perform both updates and prefix sum queries in O(log n) time, making them more efficient than a simple prefix sum array for scenarios with frequent updates.
