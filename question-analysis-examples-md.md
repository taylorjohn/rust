# Question Analysis Examples

Here are some example questions along with an analysis of their keywords and potential solution approaches:

## Example 1: Two Sum

**Question**: Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.

**Analysis**:
- Keywords: "two numbers", "add up to", "target"
- This suggests using a hash map for efficient lookup
- Time complexity requirement (if any) might influence the approach

**Approach**:
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

## Example 2: Longest Palindromic Substring

**Question**: Given a string `s`, return the longest palindromic substring in `s`.

**Analysis**:
- Keywords: "longest", "palindromic", "substring"
- This suggests either dynamic programming or expanding around center
- The word "longest" implies we need to keep track of the maximum length

**Approach**:
```rust
fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let (mut start, mut max_len) = (0, 0);
    
    for i in 0..chars.len() {
        let len1 = expand_around_center(&chars, i, i);
        let len2 = expand_around_center(&chars, i, i + 1);
        let len = len1.max(len2);
        if len > max_len {
            start = i - (len - 1) / 2;
            max_len = len;
        }
    }
    
    chars[start..start + max_len].iter().collect()
}

fn expand_around_center(chars: &[char], mut left: i32, mut right: i32) -> usize {
    while left >= 0 && right < chars.len() as i32 && chars[left as usize] == chars[right as usize] {
        left -= 1;
        right += 1;
    }
    (right - left - 1) as usize
}
```

## Example 3: Maximum Subarray

**Question**: Given an integer array `nums`, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

**Analysis**:
- Keywords: "maximum", "subarray", "largest sum"
- This is a classic problem that can be solved using Kadane's algorithm
- The word "contiguous" is crucial here, allowing for a linear-time solution

**Approach**:
```rust
fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];
    
    for &num in nums.iter().skip(1) {
        current_sum = num.max(current_sum + num);
        max_sum = max_sum.max(current_sum);
    }
    
    max_sum
}
```

## Example 4: Merge K Sorted Lists

**Question**: You are given an array of `k` linked-lists `lists`, each linked-list is sorted in ascending order. Merge all the linked-lists into one sorted linked-list and return it.

**Analysis**:
- Keywords: "merge", "k sorted lists"
- This suggests using a min-heap to efficiently select the smallest element among k lists
- The fact that the lists are already sorted is crucial information

**Approach**:
```rust
use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut curr = &mut dummy;
    let mut heap = BinaryHeap::new();
    
    // Push the head of each list into the heap
    for list in lists {
        if let Some(node) = list {
            heap.push(node);
        }
    }
    
    while let Some(mut node) = heap.pop() {
        if let Some(next) = node.next.take() {
            heap.push(next);
        }
        curr.next = Some(node);
        curr = curr.next.as_mut().unwrap();
    }
    
    dummy.next
}
```

These examples demonstrate how analyzing the keywords and phrases in a question can guide you towards an appropriate solution strategy. Practice identifying these patterns to improve your problem-solving speed and accuracy in coding interviews.
