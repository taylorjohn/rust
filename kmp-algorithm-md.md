# KMP (Knuth-Morris-Pratt) Algorithm for String Matching

The KMP algorithm is an efficient string-matching algorithm that uses the observation that when a mismatch occurs, the pattern itself embodies sufficient information to determine where the next match could begin, thus bypassing re-examination of previously matched characters.

## Implementation

```rust
fn compute_lps(pattern: &str) -> Vec<usize> {
    let n = pattern.len();
    let mut lps = vec![0; n];
    let mut len = 0;
    let mut i = 1;
    let pattern_bytes = pattern.as_bytes();

    while i < n {
        if pattern_bytes[i] == pattern_bytes[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }

    lps
}

fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut matches = Vec::new();
    let m = pattern.len();
    let n = text.len();

    if m == 0 || n == 0 {
        return matches;
    }

    let lps = compute_lps(pattern);
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    let mut i = 0; // index for text
    let mut j = 0; // index for pattern

    while i < n {
        if pattern_bytes[j] == text_bytes[i] {
            i += 1;
            j += 1;
        }

        if j == m {
            matches.push(i - j);
            j = lps[j - 1];
        } else if i < n && pattern_bytes[j] != text_bytes[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    matches
}

// Usage
fn main() {
    let text = "ABABDABACDABABCABAB";
    let pattern = "ABABCABAB";

    let matches = kmp_search(text, pattern);

    println!("Pattern found at positions:");
    for pos in matches {
        println!("{}", pos);
    }
}
```

## Key Concepts

1. **Longest Proper Prefix which is also Suffix (LPS)**: The KMP algorithm preprocesses the pattern to compute an auxiliary array lps[] to store the longest proper prefix which is also a suffix.
2. **Efficient Matching**: Uses the LPS array to skip characters when a mismatch occurs, avoiding unnecessary comparisons.
3. **Linear Time Complexity**: Achieves O(n + m) time complexity, where n is the length of the text and m is the length of the pattern.
4. **No Backtracking**: The algorithm never backtracks on the text; it only backtracks on the pattern.

## When to Use

Use the KMP Algorithm when:

- You need to find all occurrences of a pattern in a text efficiently.
- The pattern may appear multiple times in the text.
- You're working with large texts where efficiency is crucial.
- You need a string matching algorithm that doesn't use extra space proportional to the text size.

KMP is particularly useful in:

- Text editors for implementing "find" functionality.
- Bioinformatics for DNA sequence matching.
- Network security for intrusion detection systems.
- Data compression algorithms.

The main advantage of KMP over naive string matching is its linear time complexity and its ability to avoid backtracking in the text. This makes it especially efficient for large texts or when the pattern appears frequently.
