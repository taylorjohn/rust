# Trie (Prefix Tree)

A Trie, also called a prefix tree, is an efficient information retrieval data structure. Each node of the Trie represents a string and each edge represents a character.

## Implementation

```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for ch in word.chars() {
            current_node = current_node.children.entry(ch).or_insert(TrieNode::new());
        }
        current_node.is_end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current_node = &self.root;
        for ch in word.chars() {
            match current_node.children.get(&ch) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        current_node.is_end_of_word
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current_node = &self.root;
        for ch in prefix.chars() {
            match current_node.children.get(&ch) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        true
    }

    pub fn find_words_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_node = &self.root;
        
        for ch in prefix.chars() {
            match current_node.children.get(&ch) {
                Some(node) => current_node = node,
                None => return result,
            }
        }

        self.dfs(current_node, &mut String::from(prefix), &mut result);
        result
    }

    fn dfs(&self, node: &TrieNode, current_word: &mut String, result: &mut Vec<String>) {
        if node.is_end_of_word {
            result.push(current_word.clone());
        }

        for (&ch, child) in &node.children {
            current_word.push(ch);
            self.dfs(child, current_word, result);
            current_word.pop();
        }
    }

    pub fn delete(&mut self, word: &str) -> bool {
        self.delete_recursive(&mut self.root, word, 0)
    }

    fn delete_recursive(&mut self, node: &mut TrieNode, word: &str, depth: usize) -> bool {
        if depth == word.len() {
            if !node.is_end_of_word {
                return false;
            }
            node.is_end_of_word = false;
            return node.children.is_empty();
        }

        let current_char = word.chars().nth(depth).unwrap();
        if let Some(child) = node.children.get_mut(&current_char) {
            if self.delete_recursive(child, word, depth + 1) {
                node.children.remove(&current_char);
                return node.children.is_empty() && !node.is_end_of_word;
            }
        }
        false
    }
}
```

## Key Concepts

1. **Prefix-based Structure**: The trie efficiently stores strings by sharing common prefixes.
2. **Node Structure**: Each node contains a HashMap of its children (keyed by characters) and a boolean indicating if it's the end of a word.
3. **Efficient Prefix Operations**: Tries excel at operations involving string prefixes.
4. **Space Efficiency**: Tries can be space-efficient for storing many words with common prefixes.

## When to Use

Use a Trie when:

- You need to perform operations based on string prefixes efficiently.
- Working with a large set of strings, especially those with common prefixes.
- Implementing autocomplete or spell-check functionalities.
- Storing and searching for IP addresses in networking applications.
- Implementing dictionaries with prefix-based lookups.

Tries are particularly useful in:

- Autocomplete systems in search engines or text editors
- Spell checkers
- IP routing tables (for longest prefix matching)
- Solving word games (like finding all valid words in a Boggle board)
- Storing a dictionary of words for quick prefix-based retrieval

The main advantage of tries is their ability to perform prefix-based operations very efficiently, which makes them superior to hash tables or binary search trees for certain types of string-based tasks.
