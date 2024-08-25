# Z-Algorithm for Pattern Matching

The Z-Algorithm is a linear time string matching algorithm that can be used to find all occurrences of a pattern in a text.

## Implementation

```rust
fn z_function(s: &str) -> Vec<usize> {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i <= r {
            z[i] = z[i - l].min(r - i + 1);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}

fn z_algorithm(text: &str, pattern: &str) -> Vec<usize> {
    let combined = format!("{}${}", pattern, text);
    let z = z_function(&combined);
    let pattern_len = pattern.len();

    z.iter()
        .enumerate()
        .skip(pattern_len + 1)
        .filter_map(|(i, &v)| if v == pattern_len { Some(i - pattern_len - 1) } else { None })
        .collect()
}

fn main() {
    let text = "ABABDABACDABABCABAB";
    let pattern = "ABABC";
    let matches = z_algorithm(text, pattern);
    println!("Pattern found at indices: {:?}", matches);
}
```

## Explanation

1. The `z_function` computes the Z-array for a given string. Each `z[i]` represents the length of the longest substring starting from `i` which is also a prefix of the string.
2. In the `z_algorithm` function, we combine the pattern and text with a separator '$'.
3. We then compute the Z-array for this combined string.
4. Finally, we find all positions where the Z-value equals the pattern length, which indicates a match.

## Time Complexity

The Z-Algorithm has a linear time complexity of O(n + m), where n is the length of the text and m is the length of the pattern.

## Use Cases

- Efficient string matching and searching
- Detecting repetitions in strings
- Compression algorithms

The Z-Algorithm is particularly useful when you need to find all occurrences of a pattern in a text efficiently, or when you need to analyze the structure of a string in terms of its repeated substrings.
