# Aho-Corasick Algorithm for Multiple Pattern Matching

The Aho-Corasick algorithm is an efficient string matching algorithm that can find multiple patterns simultaneously in a given text. It constructs a finite state machine from the patterns and then uses it to process the text in a single pass.

## Implementation

```rust
use std::collections::{HashMap, VecDeque};

struct AhoCorasick {
    goto: Vec<HashMap<char, usize>>,
    failure: Vec<usize>,
    output: Vec<Vec<usize>>,
    patterns: Vec<String>,
}

impl AhoCorasick {
    fn new(patterns: Vec<String>) -> Self {
        let mut ac = AhoCorasick {
            goto: vec![HashMap::new()],
            failure: vec![0],
            output: vec![Vec::new()],
            patterns,
        };
        ac.build_automaton();
        ac
    }

    fn build_automaton(&mut self) {
        let mut queue = VecDeque::new();

        // Construct goto function and output function for depth 1 nodes
        for (i, pattern) in self.patterns.iter().enumerate() {
            let mut current = 0;
            for c in pattern.chars() {
                current = *self.goto[current].entry(c).or_insert_with(|| {
                    self.goto.push(HashMap::new());
                    self.failure.push(0);
                    self.output.push(Vec::new());
                    self.goto.len() - 1
                });
            }
            self.output[current].push(i);
        }

        // Construct failure function and complete output function
        for c in self.goto[0].values() {
            queue.push_back(*c);
        }

        while let Some(state) = queue.pop_front() {
            for (&c, &next_state) in &self.goto[state] {
                queue.push_back(next_state);
                
                let mut failure = self.failure[state];
                while failure != 0 && !self.goto[failure].contains_key(&c) {
                    failure = self.failure[failure];
                }
                if let Some(&next_failure) = self.goto[failure].get(&c) {
                    failure = next_failure;
                }
                self.failure[next_state] = failure;
                self.output[next_state].extend_from_slice(&self.output[failure]);
            }
        }
    }

    fn search(&self, text: &str) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let mut current = 0;

        for (i, c) in text.char_indices() {
            while current != 0 && !self.goto[current].contains_key(&c) {
                current = self.failure[current];
            }
            if let Some(&next) = self.goto[current].get(&c) {
                current = next;
            }
            for &pattern_index in &self.output[current] {
                let start = i + 1 - self.patterns[pattern_index].len();
                result.push((start, pattern_index));
            }
        }

        result
    }
}

// Usage
fn main() {
    let patterns = vec![
        "he".to_string(),
        "she".to_string(),
        "his".to_string(),
        "hers".to_string(),
    ];
    let ac = AhoCorasick::new(patterns);
    let text = "ahishers";

    let matches = ac.search(text);
    for (pos, pattern_index) in matches {
        println!("Pattern '{}' found at position {}", ac.patterns[pattern_index], pos);
    }
}
```

## Key Concepts

1. **Trie Construction**: Builds a trie-like structure from the pattern strings.
2. **Failure Function**: Determines where to continue matching if the current state fails.
3. **Output Function**: Keeps track of patterns matched at each state.
4. **Single-Pass Text Processing**: Scans the text only once to find all occurrences of all patterns.

## When to Use

Use the Aho-Corasick Algorithm when:

- You need to find multiple patterns in a text simultaneously.
- Working with large texts and/or large sets of patterns.
- Implementing systems that require high-performance string matching, like intrusion detection systems or spam filters.

Aho-Corasick is particularly useful in:

- Bioinformatics for DNA sequence matching.
- Network security for packet inspection and malware detection.
- Information retrieval and text processing applications.
- Implementing efficient search functionality in text editors or databases.

The main advantage of Aho-Corasick is its ability to match multiple patterns simultaneously in linear time with respect to the length of the text plus the total length of the patterns.
