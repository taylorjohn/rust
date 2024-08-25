# Longest Common Subsequence (LCS)

The Longest Common Subsequence problem is a classic dynamic programming problem. Given two sequences, find the length of the longest subsequence present in both of them. A subsequence is a sequence that appears in the same relative order, but not necessarily contiguous.

## Basic Implementation

```rust
fn longest_common_subsequence(text1: &str, text2: &str) -> usize {
    let (m, n) = (text1.len(), text2.len());
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if text1.as_bytes()[i - 1] == text2.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[m][n]
}

// Usage
fn main() {
    let text1 = "abcde";
    let text2 = "ace";
    println!("Length of LCS: {}", longest_common_subsequence(text1, text2));
}
```

## Variations

### 1. Print the Longest Common Subsequence

```rust
fn print_lcs(text1: &str, text2: &str) -> String {
    let (m, n) = (text1.len(), text2.len());
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Fill the dp table
    for i in 1..=m {
        for j in 1..=n {
            if text1.as_bytes()[i - 1] == text2.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // Reconstruct the LCS
    let mut lcs = String::new();
    let (mut i, mut j) = (m, n);
    while i > 0 && j > 0 {
        if text1.as_bytes()[i - 1] == text2.as_bytes()[j - 1] {
            lcs.push(text1.as_bytes()[i - 1] as char);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    lcs.chars().rev().collect()
}

// Usage
fn main() {
    let text1 = "ABCDGH";
    let text2 = "AEDFHR";
    println!("Longest Common Subsequence: {}", print_lcs(text1, text2));
}
```

### 2. Space-Optimized LCS

```rust
fn space_optimized_lcs(text1: &str, text2: &str) -> usize {
    let (m, n) = (text1.len(), text2.len());
    let (shorter, longer) = if m < n { (text1, text2) } else { (text2, text1) };
    let (m, n) = (shorter.len(), longer.len());

    let mut prev = vec![0; m + 1];
    let mut current = vec![0; m + 1];

    for i in 1..=n {
        for j in 1..=m {
            if longer.as_bytes()[i - 1] == shorter.as_bytes()[j - 1] {
                current[j] = prev[j - 1] + 1;
            } else {
                current[j] = current[j - 1].max(prev[j]);
            }
        }
        std::mem::swap(&mut prev, &mut current);
    }

    prev[m]
}

// Usage
fn main() {
    let text1 = "AGGTAB";
    let text2 = "GXTXAYB";
    println!("Length of LCS (Space Optimized): {}", space_optimized_lcs(text1, text2));
}
```

## When to Use

Use the Longest Common Subsequence algorithm when:

- You need to find the longest subsequence common to two sequences.
- You're working on problems related to string similarity or difference.
- You need to implement diff tools or file comparison utilities.
- You're solving problems in bioinformatics related to DNA sequence alignment.

LCS is particularly useful in:

- Text comparison and analysis
- Version control systems (for finding differences between file versions)
- Bioinformatics (for comparing genetic sequences)
- Natural Language Processing (for finding similarities between sentences or documents)

## Time and Space Complexity

- Time Complexity: O(mn), where m and n are the lengths of the two input sequences.
- Space Complexity: O(mn) for the basic implementation, O(min(m,n)) for the space-optimized version.

## Advantages and Disadvantages

Advantages:
- Solves a fundamental problem in sequence comparison
- Can be extended to solve related problems (e.g., diff algorithms)

Disadvantages:
- Quadratic time and space complexity in the basic implementation
- May be too slow for very long sequences

The LCS problem is a classic example of dynamic programming and serves as a foundation for understanding and solving many string comparison and analysis problems.
