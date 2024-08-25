# Hard Interview Questions

Hard questions often require advanced algorithms, complex data structures, or a combination of multiple techniques. They may also involve optimizing for extreme cases or handling intricate constraints.

## Example 1: Median of Two Sorted Arrays

**Question**: Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).

**Analysis**:
- Keywords: "sorted arrays", "median", "O(log (m+n))"
- The logarithmic time complexity suggests a binary search approach
- Need to handle even and odd total lengths

**Approach**:
1. Ensure nums1 is the smaller array
2. Perform binary search on the smaller array
3. Calculate the partition point in the larger array
4. Compare elements around the partition points
5. Adjust the binary search accordingly
6. Handle edge cases and odd/even total lengths

**Code Snippet**:
```rust
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.len() > nums2.len() {
        return find_median_sorted_arrays(nums2, nums1);
    }
    
    let (m, n) = (nums1.len(), nums2.len());
    let mut low = 0;
    let mut high = m;
    
    while low <= high {
        let partition_x = (low + high) / 2;
        let partition_y = (m + n + 1) / 2 - partition_x;
        
        let max_left_x = if partition_x == 0 { std::i32::MIN } else { nums1[partition_x - 1] };
        let min_right_x = if partition_x == m { std::i32::MAX } else { nums1[partition_x] };
        
        let max_left_y = if partition_y == 0 { std::i32::MIN } else { nums2[partition_y - 1] };
        let min_right_y = if partition_y == n { std::i32::MAX } else { nums2[partition_y] };
        
        if max_left_x <= min_right_y && max_left_y <= min_right_x {
            if (m + n) % 2 == 0 {
                return (max(max_left_x, max_left_y) as f64 + min(min_right_x, min_right_y) as f64) / 2.0;
            } else {
                return max(max_left_x, max_left_y) as f64;
            }
        } else if max_left_x > min_right_y {
            high = partition_x - 1;
        } else {
            low = partition_x + 1;
        }
    }
    
    panic!("Input arrays are not sorted");
}

fn max(a: i32, b: i32) -> i32 { if a > b { a } else { b } }
fn min(a: i32, b: i32) -> i32 { if a < b { a } else { b } }
```

## Example 2: Regular Expression Matching

**Question**: Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
- '.' Matches any single character.
- '*' Matches zero or more of the preceding element.
The matching should cover the entire input string (not partial).

**Analysis**:
- Keywords: "regular expression", "matching", "entire input"
- This suggests a dynamic programming approach
- Need to handle special characters '.' and '*'

**Approach**:
1. Create a 2D DP table
2. Initialize base cases
3. Fill the table considering different cases for '.' and '*'
4. The final cell will contain the answer

**Code Snippet**:
```rust
fn is_match(s: String, p: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let (m, n) = (s.len(), p.len());
    let mut dp = vec![vec![false; n + 1]; m + 1];
    
    dp[0][0] = true;
    for j in 1..=n {
        if p[j-1] == '*' {
            dp[0][j] = dp[0][j-2];
        }
    }
    
    for i in 1..=m {
        for j in 1..=n {
            if p[j-1] == '.' || p[j-1] == s[i-1] {
                dp[i][j] = dp[i-1][j-1];
            } else if p[j-1] == '*' {
                dp[i][j] = dp[i][j-2];
                if p[j-2] == '.' || p[j-2] == s[i-1] {
                    dp[i][j] |= dp[i-1][j];
                }
            }
        }
    }
    
    dp[m][n]
}
```

For hard questions, it's crucial to break down the problem into smaller, manageable parts. Identify the core challenge and consider various advanced techniques that might be applicable. Don't be afraid to combine multiple approaches to arrive at an optimal solution.
