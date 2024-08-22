# Advanced Trie (Prefix Tree)

A Trie, or prefix tree, is a tree data structure used to store and retrieve strings. This advanced implementation includes additional operations like prefix matching and autocomplete suggestions.

## Implementation

```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
    count: usize,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
            count: 0,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: TrieNode::new() }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TrieNode::new());
            node.count += 1;
        }
        node.is_end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        self.find_node(word).map_or(false, |node| node.is_end_of_word)
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.find_node(prefix).is_some()
    }

    fn find_node(&self, prefix: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(next_node) => node = next_node,
                None => return None,
            }
        }
        Some(node)
    }

    pub fn count_prefix(&self, prefix: &str) -> usize {
        self.find_node(prefix).map_or(0, |node| node.count)
    }

    pub fn autocomplete(&self, prefix: &str) -> Vec<String> {
        let mut results = Vec::new();
        if let Some(node) = self.find_node(prefix) {
            self.dfs(node, &mut String::from(prefix), &mut results);
        }
        results
    }

    fn dfs(&self, node: &TrieNode, current: &mut String, results: &mut Vec<String>) {
        if node.is_end_of_word {
            results.push(current.clone());
        }
        for (&ch, child) in &node.children {
            current.push(ch);
            self.dfs(child, current, results);
            current.pop();
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

        let ch = word.chars().nth(depth).unwrap();
        if let Some(child) = node.children.get_mut(&ch) {
            if self.delete_recursive(child, word, depth + 1) {
                node.children.remove(&ch);
                return !node.is_end_of_word && node.children.is_empty();
            }
        }
        false
    }
}

// Usage
fn main() {
    let mut trie = Trie::new();
    trie.insert("apple");
    trie.insert("app");
    trie.insert("apricot");
    trie.insert("banana");

    println!("Search 'apple': {}", trie.search("apple"));
    println!("Starts with 'app': {}", trie.starts_with("app"));
    println!("Count prefix 'app': {}", trie.count_prefix("app"));
    println!("Autocomplete 'ap': {:?}", trie.autocomplete("ap"));

    trie.delete("apple");
    println!("Search 'apple' after deletion: {}", trie.search("apple"));
}
```

## Key Concepts

1. **Prefix-based Structure**: Efficiently stores and retrieves strings based on their prefixes.
2. **Count Tracking**: Keeps track of how many words share each prefix.
3. **Autocomplete**: Implements prefix-based autocompletion.
4. **Deletion**: Supports removal of words while maintaining the trie's integrity.

## When to Use

Use an Advanced Trie when:

- You need efficient prefix-based operations on a large set of strings.
- Implementing autocomplete or spell-check functionalities.
- You require fast insertion, deletion, and lookup of strings.
- You need to count strings with a common prefix.

Advanced Tries are particularly useful in:

- Search engines for query autocomplete
- Spell checkers and word processors
- IP routing tables for longest prefix matching
- Bioinformatics for DNA sequence matching

The main advantages of this advanced Trie implementation are its efficient prefix operations, ability to count strings with common prefixes, and support for autocomplete functionality, making it suitable for more complex string-based applications.
