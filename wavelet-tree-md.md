# Wavelet Tree

A Wavelet Tree is a data structure that supports rank, select, and access operations on a sequence of symbols from an alphabet in O(log σ) time, where σ is the alphabet size.

## Implementation

```rust
use std::collections::HashMap;

struct WaveletTree {
    root: Option<Box<WaveletNode>>,
    alphabet: Vec<char>,
}

struct WaveletNode {
    left: Option<Box<WaveletNode>>,
    right: Option<Box<WaveletNode>>,
    bitmap: Vec<bool>,
}

impl WaveletTree {
    fn new(s: &str) -> Self {
        let mut alphabet: Vec<char> = s.chars().collect();
        alphabet.sort_unstable();
        alphabet.dedup();
        
        let root = Self::build_tree(&alphabet, s);
        
        WaveletTree { root, alphabet }
    }
    
    fn build_tree(alphabet: &[char], s: &str) -> Option<Box<WaveletNode>> {
        if alphabet.len() <= 1 {
            return None;
        }
        
        let mid = alphabet.len() / 2;
        let left_alphabet = &alphabet[..mid];
        let right_alphabet = &alphabet[mid..];
        
        let mut bitmap = Vec::with_capacity(s.len());
        let mut left_s = String::new();
        let mut right_s = String::new();
        
        for c in s.chars() {
            if left_alphabet.contains(&c) {
                bitmap.push(false);
                left_s.push(c);
            } else {
                bitmap.push(true);
                right_s.push(c);
            }
        }
        
        Some(Box::new(WaveletNode {
            left: Self::build_tree(left_alphabet, &left_s),
            right: Self::build_tree(right_alphabet, &right_s),
            bitmap,
        }))
    }

    fn rank(&self, c: char, pos: usize) -> usize {
        let mut node = self.root.as_ref()?;
        let mut alphabet = &self.alphabet[..];
        let mut curr_pos = pos;
        
        loop {
            let mid = alphabet.len() / 2;
            let left_alphabet = &alphabet[..mid];
            let right_alphabet = &alphabet[mid..];
            
            let count = node.bitmap[..=curr_pos].iter().filter(|&&b| !b).count();
            
            if left_alphabet.contains(&c) {
                if node.left.is_none() {
                    return count;
                }
                node = node.left.as_ref()?;
                alphabet = left_alphabet;
                curr_pos = count - 1;
            } else {
                if node.right.is_none() {
                    return curr_pos - count + 1;
                }
                node = node.right.as_ref()?;
                alphabet = right_alphabet;
                curr_pos -= count;
            }
        }
    }

    fn access(&self, pos: usize) -> Option<char> {
        let mut node = self.root.as_ref()?;
        let mut alphabet = &self.alphabet[..];
        let mut curr_pos = pos;
        
        loop {
            let mid = alphabet.len() / 2;
            let left_alphabet = &alphabet[..mid];
            let right_alphabet = &alphabet[mid..];
            
            if !node.bitmap[curr_pos] {
                if node.left.is_none() {
                    return Some(left_alphabet[0]);
                }
                node = node.left.as_ref()?;
                alphabet = left_alphabet;
                curr_pos = node.bitmap[..=curr_pos].iter().filter(|&&b| !b).count() - 1;
            } else {
                if node.right.is_none() {
                    return Some(right_alphabet[0]);
                }
                node = node.right.as_ref()?;
                alphabet = right_alphabet;
                curr_pos = curr_pos - node.bitmap[..curr_pos].iter().filter(|&&b| !b).count();
            }
        }
    }
}
```

## Key Concepts

1. **Binary Representation**: Uses the binary representation of indices to efficiently update and query.
2. **Hierarchical Structure**: Divides the alphabet recursively, creating a tree-like structure.
3. **Bitmap**: Each node contains a bitmap representing the division of characters into left and right subtrees.
4. **Rank and Access Operations**: Supports efficient rank and access operations.

## When to Use

Use a Wavelet Tree when:

- You need to perform rank and select operations on a sequence over a large alphabet.
- Working with strings or sequences where you need to query subranges efficiently.
- Implementing algorithms that require fast access to character frequencies in subranges.

Wavelet Trees are particularly useful in:

- Text indexing and compressed text representations
- Range query problems in computational geometry
- Bioinformatics for efficient DNA sequence analysis
- Data compression algorithms

## Time Complexity

- Construction: O(n log σ)
- Rank and Access: O(log σ)

Where n is the length of the sequence and σ is the size of the alphabet.

## Space Complexity

O(n log σ) bits

Wavelet Trees provide a powerful trade-off between time and space efficiency for various string and sequence operations, especially when dealing with large alphabets.
