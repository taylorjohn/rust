# Rabin-Karp Algorithm for String Matching

The Rabin-Karp algorithm is a string-searching algorithm that uses hashing to find patterns in strings. It's particularly useful when searching for multiple patterns simultaneously.

## Implementation

```rust
const BASE: u64 = 256;
const MOD: u64 = 1_000_000_007;

fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    let n = text.len();
    let m = pattern.len();
    let mut result = Vec::new();

    if m > n {
        return result;
    }

    let mut pattern_hash: u64 = 0;
    let mut text_hash: u64 = 0;
    let mut h: u64 = 1;

    // Calculate h = BASE^(m-1) % MOD
    for _ in 0..m - 1 {
        h = (h * BASE) % MOD;
    }

    // Calculate hash value for pattern and first window of text
    for i in 0..m {
        pattern_hash = (pattern_hash * BASE + pattern.as_bytes()[i] as u64) % MOD;
        text_hash = (text_hash * BASE + text.as_bytes()[i] as u64) % MOD;
    }

    // Slide the pattern over text one by one
    for i in 0..=n - m {
        if pattern_hash == text_hash {
            // Check characters one by one
            if text[i..i + m] == pattern {
                result.push(i);
            }
        }

        if i < n - m {
            // Calculate hash value for next window of text
            text_hash = (BASE * (text_hash + MOD - (text.as_bytes()[i] as u64 * h) % MOD) % MOD
                         + text.as_bytes()[i + m] as u64) % MOD;
        }
    }

    result
}

// Usage
fn main() {
    let text = "ABABDABACDABABCABAB";
    let pattern = "ABABCABAB";

    let matches = rabin_karp(text, pattern);

    println!("Pattern found at positions:");
    for pos in matches {
        println!("{}", pos);
    }
}
```

## Key Concepts

1. **Rolling Hash**: Efficiently computes hash values for substrings of the text.
2. **Modular Arithmetic**: Used to prevent integer overflow in hash calculations.
3. **Hash Comparison**: Compares hash values before comparing actual substrings to improve efficiency.
4. **Multiple Pattern Matching**: Can be easily extended to search for multiple patterns simultaneously.

## When to Use

Use the Rabin-Karp Algorithm for String Matching when:

- You need to find a pattern (or multiple patterns) in a text.
- The expected number of matches is small.
- You're working on problems that benefit from its rolling hash technique.

Rabin-Karp is particularly useful in:

- Plagiarism detection systems.
- Finding duplicate files or data by comparing hash values.
- Implementing search functionality in text editors.
- Bioinformatics for DNA sequence matching.

The main advantage of Rabin-Karp is its ability to search for multiple patterns simultaneously and its average-case time complexity of O(n+m), where n is the length of the text and m is the length of the pattern. However, its worst-case time complexity is O(nm), which occurs when there are many hash collisions.
