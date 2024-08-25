# Aho-Corasick Algorithm for Multiple Pattern Matching

The Aho-Corasick algorithm is an efficient string matching algorithm that can find multiple patterns simultaneously in a given text.

## Implementation

```rust
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct AhoCorasickNode {
    children: HashMap<char, usize>,
    failure_link: usize,
    output_link: usize,
    pattern_end: Option<usize>,
}

struct AhoCorasick {
    trie: Vec<AhoCorasickNode>,
    patterns: Vec<String>,
}

impl AhoCorasick {
    fn new(patterns: Vec<String>) -> Self {
        let mut ac = AhoCorasick {
            trie: vec![AhoCorasickNode {
                children: HashMap::new(),
                failure_link: 0,
                output_link: 0,
                pattern_end: None,
            }],
            patterns,
        };
        ac.build_trie();
        ac.build_failure_and_output_links();
        ac
    }
    
    fn build_trie(&mut self) {
        for (i, pattern) in self.patterns.iter().enumerate() {
            let mut node = 0;
            for c in pattern.chars() {
                node = *self.trie[node].children.entry(c).or_insert_with(|| {
                    self.trie.push(AhoCorasickNode {
                        children: HashMap::new(),
                        failure_link: 0,
                        output_link: 0,
                        pattern_end: None,
                    });
                    self.trie.len() - 1
                });
            }
            self.trie[node].pattern_end = Some(i);
        }
    }
    
    fn build_failure_and_output_links(&mut self) {
        let mut queue = VecDeque::new();
        for &child in self.trie[0].children.values() {
            queue.push_back(child);
            self.trie[child].failure_link = 0;
        }
        
        while let Some(node) = queue.pop_front() {
            for (&c, &child) in &self.trie[node].children {
                queue.push_back(child);
                
                let mut failure = self.trie[node].failure_link;
                while failure != 0 && !self.trie[failure].children.contains_key(&c) {
                    failure = self.trie[failure].failure_link;
                }
                
                if let Some(&next) = self.trie[failure].children.get(&c) {
                    self.trie[child].failure_link = next;
                    self.trie[child].output_link = if self.trie[next].pattern_end.is_some() {
                        next
                    } else {
                        self.trie[next].output_link
                    };
                } else {
                    self.trie[child].failure_link = 0;
                }
            }
        }
    }
    
    fn search(&self, text: &str) -> Vec<(usize, usize)> {
        let mut results = Vec::new();
        let mut node = 0;
        
        for (i, c) in text.char_indices() {
            while node != 0 && !self.trie[node].children.contains_key(&c) {
                node = self.trie[node].failure_link;
            }
            
            if let Some(&next) = self.trie[node].children.get(&c) {
                node = next;
            }
            
            if let Some(pattern_index) = self.trie[node].pattern_end {
                results.push((i + 1 - self.patterns[pattern_index].len(), pattern_index));
            }
            
            let mut output = self.trie[node].output_link;
            while output != 0 {
                if let Some(pattern_index) = self.trie[output].pattern_end {
                    results.push((i + 1 - self.patterns[pattern_index].len(), pattern_index));
                }
            }
        }
        
        results
    }
}
```

## Key Concepts

1. **Trie Construction**: The algorithm starts by building a trie from the set of patterns.
2. **Failure Links**: These links are used to efficiently move to the next possible match when a mismatch occurs.
3. **Output Links**: These help in identifying all matched patterns, not just the longest one.
4. **Pattern Matching**: The algorithm can find all occurrences of all patterns in a single pass through the text.

## When to Use

Use the Aho-Corasick algorithm when:

- You need to find multiple patterns in a text simultaneously.
- You're working with a fixed set of patterns and a streaming text.
- You need efficient pattern matching for applications like intrusion detection systems, spam filters, or DNA sequence analysis.

Aho-Corasick is particularly useful in:

- Network security (e.g., deep packet inspection)
- Bioinformatics (e.g., searching for DNA sequences)
- Information retrieval systems
- Implementing `grep`-like functionality for multiple patterns

## Time Complexity

- Preprocessing (building the automaton): O(m), where m is the sum of the lengths of all patterns
- Searching: O(n + k), where n is the length of the text and k is the number of pattern occurrences

## Space Complexity

O(m), where m is the sum of the lengths of all patterns

The Aho-Corasick algorithm provides a powerful and efficient solution for multiple pattern matching problems, especially when dealing with a large number of patterns or long texts.
