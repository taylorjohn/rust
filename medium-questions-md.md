# Medium Interview Questions

Medium difficulty questions often require a combination of data structures or more complex algorithms. They may involve multiple steps or require optimization beyond the naive approach.

## Example 1: Longest Substring Without Repeating Characters

**Question**: Given a string s, find the length of the longest substring without repeating characters.

**Analysis**:
- Keywords: "longest substring", "without repeating characters"
- This suggests using a sliding window technique
- Need to keep track of character occurrences

**Approach**:
1. Use two pointers to define the window
2. Use a hash set to keep track of characters in the current window
3. Expand the window as long as there are no repeating characters
4. When a repeat is found, contract the window from the left
5. Keep track of the maximum window size

**Code Snippet**:
```rust
use std::collections::HashSet;

fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut set = HashSet::new();
    let mut max_length = 0;
    let mut left = 0;
    let mut right = 0;

    while right < chars.len() {
        if !set.contains(&chars[right]) {
            set.insert(chars[right]);
            max_length = max_length.max(right - left + 1);
            right += 1;
        } else {
            set.remove(&chars[left]);
            left += 1;
        }
    }

    max_length as i32
}
```

## Example 2: Binary Tree Level Order Traversal

**Question**: Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).

**Analysis**:
- Keywords: "binary tree", "level order traversal"
- This suggests using a breadth-first search (BFS) approach
- Need to keep track of levels

**Approach**:
1. Use a queue to perform BFS
2. Keep track of the current level
3. Process all nodes at the current level before moving to the next
4. Add the values to the result list level by level

**Code Snippet**:
```rust
use std::collections::VecDeque;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::rc::Rc;
use std::cell::RefCell;

fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if root.is_none() {
        return result;
    }

    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut current_level = Vec::new();

        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                let node = node.borrow();
                current_level.push(node.val);

                if let Some(left) = &node.left {
                    queue.push_back(Rc::clone(left));
                }
                if let Some(right) = &node.right {
                    queue.push_back(Rc::clone(right));
                }
            }
        }

        result.push(current_level);
    }

    result
}
```

For medium questions, focus on identifying the primary algorithm or data structure needed, and then consider how to optimize or combine techniques to solve the problem efficiently.
