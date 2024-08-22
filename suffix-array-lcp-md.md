# Suffix Array and Longest Common Prefix (LCP) Array

A Suffix Array is a sorted array of all suffixes of a string. The Longest Common Prefix (LCP) Array stores the lengths of the longest common prefixes between consecutive suffixes in the Suffix Array.

## Implementation

```rust
fn build_suffix_array(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by_key(|&i| &s[i..]);
    sa
}

fn build_lcp_array(s: &str, sa: &[usize]) -> Vec<usize> {
    let n = s.len();
    let mut rank = vec![0; n];
    for i in 0..n {
        rank[sa[i]] = i;
    }
    
    let mut lcp = vec![0; n - 1];
    let mut k = 0;
    for i in 0..n-1 {
        if rank[i] == n - 1 {
            k = 0;
            continue;
        }
        let j = sa[rank[i] + 1];
        while i + k < n && j + k < n && s.as_bytes()[i + k] == s.as_bytes()[j + k] {
            k += 1;
        }
        lcp[rank[i]] = k;
        if k > 0 {
            k -= 1;
        }
    }
    lcp
}

fn longest_repeated_substring(s: &str) -> String {
    let sa = build_suffix_array(s);
    let lcp = build_lcp_array(s, &sa);
    
    let max_lcp = lcp.iter().max().unwrap_or(&0);
    let pos = lcp.iter().position(|&x| x == *max_lcp).unwrap_or(0);
    
    s[sa[pos]..sa[pos] + max_lcp].to_string()
}

// Usage
fn main() {
    let s = "banana";
    let sa = build_suffix_array(s);
    let lcp = build_lcp_array(s, &sa);
    
    println!("String: {}", s);
    println!("Suffix Array: {:?}", sa);
    println!("LCP Array: {:?}", lcp);
    println!("Longest Repeated Substring: {}", longest_repeated_substring(s));
}
```

## Key Concepts

1. **Suffix Array**: A sorted array of all suffixes of a string.
2. **LCP Array**: Stores the lengths of longest common prefixes between consecutive suffixes in the Suffix Array.
3. **Lexicographic Ordering**: Suffixes are sorted lexicographically.
4. **Efficient String Processing**: Enables efficient solutions to various string problems.

## When to Use

Use Suffix Arrays and LCP Arrays when:

- You need to perform multiple string matching queries on a fixed text.
- Solving problems related to substrings, like finding the longest repeated substring.
- Working on problems that involve lexicographic ordering of suffixes.
- You need to efficiently compute the longest common prefix of any two suffixes.

Suffix Arrays and LCP Arrays are particularly useful in:

- Bioinformatics for DNA sequence analysis
- Data compression algorithms
- Full-text indices in search engines
- String processing in competitive programming

The main advantages of Suffix Arrays over other string data structures (like tries) are their space efficiency and cache-friendliness, making them suitable for processing large strings or texts.
