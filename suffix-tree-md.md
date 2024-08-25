# Suffix Tree

A Suffix Tree is a compressed trie containing all the suffixes of the given text as their keys and positions in the text as their values. It's a powerful data structure for string processing, allowing for operations like searching for a substring, finding the longest repeated substring, and more.

## Implementation

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct SuffixTreeNode {
    children: HashMap<char, SuffixTreeNode>,
    start: usize,
    end: Option<usize>,
    suffix_link: Option<*mut SuffixTreeNode>,
}

impl SuffixTreeNode {
    fn new(start: usize, end: Option<usize>) -> Self {
        SuffixTreeNode {
            children: HashMap::new(),
            start,
            end,
            suffix_link: None,
        }
    }
}

struct SuffixTree {
    root: SuffixTreeNode,
    text: String,
    remainder: usize,
    active_node: *mut SuffixTreeNode,
    active_edge: Option<char>,
    active_length: usize,
}

impl SuffixTree {
    fn new(text: String) -> Self {
        let mut tree = SuffixTree {
            root: SuffixTreeNode::new(0, None),
            text,
            remainder: 0,
            active_node: std::ptr::null_mut(),
            active_edge: None,
            active_length: 0,
        };
        tree.active_node = &mut tree.root as *mut SuffixTreeNode;
        tree.build();
        tree
    }

    fn build(&mut self) {
        for i in 0..self.text.len() {
            self.extend(i);
        }
    }

    fn extend(&mut self, pos: usize) {
        self.remainder += 1;
        let mut last_new_node: Option<*mut SuffixTreeNode> = None;

        while self.remainder > 0 {
            if self.active_length == 0 {
                self.active_edge = Some(self.text.chars().nth(pos).unwrap());
            }

            let next = self.active_edge.unwrap();
            if !unsafe { &*self.active_node }.children.contains_key(&next) {
                unsafe { &mut *self.active_node }.children.insert(next, SuffixTreeNode::new(pos, None));
                if let Some(last) = last_new_node {
                    unsafe { (*last).suffix_link = Some(self.active_node) };
                }
                last_new_node = Some(&mut unsafe { &mut *self.active_node }.children.get_mut(&next).unwrap() as *mut SuffixTreeNode);
            } else {
                let mut next_node = unsafe { &mut *self.active_node }.children.get_mut(&next).unwrap();
                let edge_length = next_node.end.unwrap_or(pos + 1) - next_node.start;
                if self.active_length >= edge_length {
                    self.active_node = next_node as *mut SuffixTreeNode;
                    self.active_length -= edge_length;
                    self.active_edge = self.text.chars().nth(pos - self.remainder + 1 + edge_length);
                    continue;
                }

                if self.text.chars().nth(next_node.start + self.active_length).unwrap()
                    == self.text.chars().nth(pos).unwrap()
                {
                    self.active_length += 1;
                    if let Some(last) = last_new_node {
                        unsafe { (*last).suffix_link = Some(self.active_node) };
                    }
                    break;
                }

                let split = SuffixTreeNode::new(next_node.start, Some(next_node.start + self.active_length));
                *next_node = SuffixTreeNode::new(next_node.start + self.active_length, next_node.end);
                split.children.insert(self.text.chars().nth(next_node.start + self.active_length).unwrap(), *next_node);
                split.children.insert(self.text.chars().nth(pos).unwrap(), SuffixTreeNode::new(pos, None));
                unsafe { &mut *self.active_node }.children.insert(next, split);

                if let Some(last) = last_new_node {
                    unsafe { (*last).suffix_link = Some(&mut unsafe { &mut *self.active_node }.children.get_mut(&next).unwrap() as *mut SuffixTreeNode) };
                }
                last_new_node = Some(&mut unsafe { &mut *self.active_node }.children.get_mut(&next).unwrap() as *mut SuffixTreeNode);
            }

            self.remainder -= 1;
            if self.active_node as *const SuffixTreeNode == &self.root as *const SuffixTreeNode && self.active_length > 0 {
                self.active_length -= 1;
                self.active_edge = Some(self.text.chars().nth(pos - self.remainder + 1).unwrap());
            } else {
                self.active_node = self.active_node.suffix_link.unwrap_or(&mut self.root as *mut SuffixTreeNode);
            }
        }
    }

    fn find(&self, pattern: &str) -> bool {
        let mut node = &self.root;
        let mut i = 0;
        for c in pattern.chars() {
            if let Some(child) = node.children.get(&c) {
                node = child;
                let end = child.end.unwrap_or(self.text.len());
                while i < pattern.len() && child.start + i - child.start < end - child.start {
                    if pattern.chars().nth(i).unwrap() != self.text.chars().nth(child.start + i - child.start).unwrap() {
                        return false;
                    }
                    i += 1;
                }
                if i == pattern.len() {
                    return true;
                }
            } else {
                return false;
            }
        }
        false
    }
}

fn main() {
    let text = String::from("banana$");
    let tree = SuffixTree::new(text);
    
    println!("Search for 'ana': {}", tree.find("ana"));
    println!("Search for 'ann': {}", tree.find("ann"));
}
```

## Key Concepts

1. **Suffix**: A substring that extends from any position to the end of the string.
2. **Compressed Trie**: A trie where chains of single-child nodes are compressed into individual edges.
3. **Suffix Link**: A pointer from an internal node representing a string aS to another node representing S.
4. **Active Point**: Consists of active node, active edge, and active length, used in Ukkonen's algorithm for efficient construction.
5. **Ukkonen's Algorithm**: An online algorithm for constructing suffix trees in linear time.

## When to Use

Suffix Trees are particularly useful in:

1. **String Matching**: Quickly find if a string is a substring of another string.
2. **Longest Common Substring**: Find the longest substring common to two or more strings.
3. **Longest Repeated Substring**: Find the longest substring that appears more than once in a string.
4. **Palindrome Detection**: Find all palindromes in a string.
5. **Genome Analysis**: Used in various bioinformatics applications for DNA sequence analysis.
6. **Data Compression**: Some compression algorithms use suffix trees.

## Time Complexity

- Construction: O(n) using Ukkonen's algorithm, where n is the length of the string.
- Search: O(m) for a pattern of length m.

## Space Complexity

O(n) space, where n is the length of the string.

## Advantages and Limitations

Advantages:
- Linear-time construction
- Efficient for multiple string queries on the same text
- Solves many string problems efficiently

Limitations:
- High memory usage compared to simpler data structures
- Complex implementation
- Not easily updatable for dynamic texts

Suffix Trees are powerful data structures that solve many string-related problems efficiently. While they have a high memory footprint and complex implementation, their ability to solve diverse string problems makes them valuable in various applications, especially in fields like bioinformatics and text processing.
