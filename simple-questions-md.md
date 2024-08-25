# Simple Interview Questions

These questions typically involve basic data structures and straightforward algorithms. They often require a single pass through the data or a simple transformation.

## Example 1: Two Sum

**Question**: Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.

**Analysis**:
- Keywords: "array", "two numbers", "add up to target"
- This suggests using a hash map to store complements
- Single pass through the array

**Approach**:
1. Initialize a hash map
2. Iterate through the array
3. For each number, check if its complement exists in the hash map
4. If found, return the indices; if not, add the current number to the hash map

**Code Snippet**:
```rust
use std::collections::HashMap;

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
```

## Example 2: Reverse a String

**Question**: Write a function that reverses a string. The input string is given as an array of characters s.

**Analysis**:
- Keywords: "reverse", "string", "array of characters"
- This suggests using the two-pointer technique
- In-place reversal to optimize space complexity

**Approach**:
1. Initialize two pointers, one at the start and one at the end
2. Swap characters at these pointers
3. Move the pointers towards the center
4. Repeat until the pointers meet

**Code Snippet**:
```rust
fn reverse_string(s: &mut Vec<char>) {
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}
```

Remember, for simple questions, the key is to identify the most straightforward approach that solves the problem efficiently. Don't overcomplicate your solution.
