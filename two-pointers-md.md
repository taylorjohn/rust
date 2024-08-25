# Two Pointers Technique

The Two Pointers technique is an algorithmic pattern where two pointers iterate through the data structure in tandem until one or both of the pointers hit a certain condition.

## Implementation

Here's an example of using the Two Pointers technique to solve the "Container With Most Water" problem:

```rust
fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_area = 0;

    while left < right {
        let width = (right - left) as i32;
        let area = width * height[left].min(height[right]);
        max_area = max_area.max(area);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

// Usage
fn main() {
    let heights = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("Maximum area: {}", max_area(heights));
}
```

## When to Use

Use the Two Pointers technique when:

- Dealing with sorted arrays (or linked lists) and need to find a set of elements that fulfill certain constraints.
- The problem requires you to compare elements from both ends of an array.
- You need to solve problems involving subarrays or sublists.
- Searching pairs in an array, especially when the array is sorted.

The Two Pointers technique is particularly useful in:

- Solving the "reverse array" problem in-place.
- Finding if a string is a palindrome.
- Merging two sorted arrays.
- Finding pairs with a specific sum in a sorted array.
- Removing duplicates from a sorted array.

## Time Complexity

The time complexity of algorithms using the Two Pointers technique is often O(n), where n is the size of the input data structure. This makes it very efficient compared to nested loop approaches, which would typically be O(n^2).

## Variations

1. Fast and Slow Pointers: Used in cycle detection problems.
2. Sliding Window: A variation where the two pointers move in the same direction.
3. Multiple Pointers: Some problems might require more than two pointers.

These variations can be applied to solve a wide range of problems efficiently.
