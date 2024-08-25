# Burrows-Wheeler Transform

The Burrows-Wheeler Transform (BWT) is a string transformation algorithm used in data compression. It's particularly useful in the context of the Burrows-Wheeler compression algorithm.

## Implementation

```rust
fn burrows_wheeler_transform(s: &str) -> (String, usize) {
    let mut rotations: Vec<_> = (0..s.len()).map(|i| &s[i..]).collect();
    rotations.sort_unstable();
    
    let last_column: String = rotations.iter().map(|&r| r.chars().last().unwrap()).collect();
    let original_index = rotations.iter().position(|&r| r.len() == s.len()).unwrap();
    
    (last_column, original_index)
}

fn inverse_burrows_wheeler_transform(s: &str, index: usize) -> String {
    let n = s.len();
    let mut t: Vec<_> = s.chars().enumerate().collect();
    t.sort_unstable_by_key(|&(_, c)| c);
    
    let mut next = vec![0; n];
    for i in 0..n {
        next[t[i].0] = i;
    }
    
    let mut result = String::with_capacity(n);
    let mut i = index;
    for _ in 0..n {
        result.push(t[i].1);
        i = next[i];
    }
    
    result
}

fn main() {
    let s = "banana$";
    let (bwt, index) = burrows_wheeler_transform(s);
    println!("BWT: {}", bwt);
    println!("Index: {}", index);
    
    let original = inverse_burrows_wheeler_transform(&bwt, index);
    println!("Original: {}", original);
}
```

## Explanation

1. The BWT is constructed by creating all rotations of the input string, sorting them lexicographically, and then taking the last column.
2. The index of the original string in the sorted rotations is also returned, as it's needed for the inverse transform.
3. The inverse transform reconstructs the original string using the last column and the index.
4. It works by iteratively building the first column and using it to navigate through the last column.

## Time Complexity

- Transform: O(n^2 log n) with this naive implementation. Can be improved to O(n) using suffix arrays.
- Inverse Transform: O(n)

## Use Cases

- Data compression (e.g., bzip2 compression algorithm)
- Bioinformatics (e.g., compressing genomic sequences)
- Text indexing and searching

The Burrows-Wheeler Transform is a powerful tool in data compression and string processing. It tends to group similar characters together, which makes it amenable to further compression techniques. It's also reversible, allowing for lossless compression.
