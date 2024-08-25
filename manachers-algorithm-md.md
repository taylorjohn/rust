# Manacher's Algorithm

Manacher's Algorithm is a linear-time algorithm for finding the longest palindromic substring in a string.

## Implementation

```rust
fn manachers_algorithm(s: &str) -> String {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    if n == 0 {
        return String::new();
    }

    // Transform S to T
    let mut t = vec!['#'; 2 * n + 1];
    for i in 0..n {
        t[2 * i + 1] = s[i];
    }

    let n = t.len();
    let mut p = vec![0; n];
    let (mut c, mut r) = (0, 0);

    for i in 1..n-1 {
        let mirror = 2 * c - i;

        if i < r {
            p[i] = p[mirror].min(r - i);
        }

        // Attempt to expand palindrome centered at i
        while i + p[i] + 1 < n && i - p[i] - 1 >= 0 && t[i + p[i] + 1] == t[i - p[i] - 1] {
            p[i] += 1;
        }

        // If palindrome centered at i expands past r,
        // adjust center based on expanded palindrome.
        if i + p[i] > r {
            c = i;
            r = i + p[i];
        }
    }

    // Find the maximum element in p
    let (mut max_len, mut center_index) = (0, 0);
    for i in 0..n {
        if p[i] > max_len {
            max_len = p[i];
            center_index = i;
        }
    }

    let start = (center_index - max_len) / 2;
    s[start..start + max_len].iter().collect()
}

fn main() {
    let s = "babad";
    println!("Longest palindromic substring: {}", manachers_algorithm(s));
}
```

## Explanation

1. We first transform the input string by inserting '#' between each character and at the start and end. This handles both odd and even length palindromes uniformly.
2. We use an array `p` where `p[i]` represents the radius of the palindrome centered at index `i` in the transformed string.
3. We iterate through the string, using previously computed values to avoid unnecessary comparisons:
   - If the current index is within the rightmost known palindrome, we can use the value of its mirrored index.
   - We then attempt to expand the palindrome centered at the current index.
   - If this expansion goes beyond the rightmost known palindrome, we update our center and right boundary.
4. Finally, we find the maximum value in `p`, which gives us the center and length of the longest palindrome.

## Time Complexity

Manacher's Algorithm has a linear time complexity of O(n), where n is the length of the input string.

## Use Cases

- Finding the longest palindromic substring in a string
- Text analysis and pattern recognition
- Bioinformatics for analyzing DNA sequences

Manacher's Algorithm is particularly useful when you need to find the longest palindromic substring efficiently, which has applications in various fields including text processing and computational biology.
