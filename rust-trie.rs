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
        
        // Navigate to the node representing the prefix
        for ch in prefix.chars() {
            match current_node.children.get(&ch) {
                Some(node) => current_node = node,
                None => return result, // Prefix not found
            }
        }

        // Perform DFS from the prefix node
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

fn main() {
    let mut trie = Trie::new();

    // Insert words
    trie.insert("apple");
    trie.insert("app");
    trie.insert("apply");
    trie.insert("banana");
    trie.insert("bath");
    trie.insert("bat");

    // Search for words
    println!("Search 'apple': {}", trie.search("apple"));   // true
    println!("Search 'app': {}", trie.search("app"));       // true
    println!("Search 'appl': {}", trie.search("appl"));     // false
    println!("Search 'bat': {}", trie.search("bat"));       // true

    // Check prefixes
    println!("Starts with 'app': {}", trie.starts_with("app")); // true
    println!("Starts with 'banan': {}", trie.starts_with("banan")); // true

    // Find words with prefix
    println!("Words with prefix 'app': {:?}", trie.find_words_with_prefix("app"));
    println!("Words with prefix 'ba': {:?}", trie.find_words_with_prefix("ba"));

    // Delete a word
    println!("Delete 'apple': {}", trie.delete("apple"));
    println!("Search 'apple' after deletion: {}", trie.search("apple")); // false
    println!("Search 'app' after deletion: {}", trie.search("app")); // true
}
