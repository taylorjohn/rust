# Suffix Automaton

A Suffix Automaton is a compressed representation of all substrings of a given string. It's the smallest DFA (Deterministic Finite Automaton) that recognizes all suffixes of a string.

## Implementation

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct State {
    length: usize,
    link: Option<usize>,
    next: HashMap<char, usize>,
}

struct SuffixAutomaton {
    states: Vec<State>,
    last: usize,
}

impl SuffixAutomaton {
    fn new(s: &str) -> Self {
        let mut sa = SuffixAutomaton {
            states: vec![State {
                length: 0,
                link: None,
                next: HashMap::new(),
            }],
            last: 0,
        };

        for c in s.chars() {
            sa.extend(c);
        }

        sa
    }

    fn extend(&mut self, c: char) {
        let curr = self.states.len();
        self.states.push(State {
            length: self.states[self.last].length + 1,
            link: None,
            next: HashMap::new(),
        });

        let mut p = self.last;
        while p != usize::MAX && !self.states[p].next.contains_key(&c) {
            self.states[p].next.insert(c, curr);
            p = self.states[p].link.unwrap_or(usize::MAX);
        }

        if p == usize::MAX {
            self.states[curr].link = Some(0);
        } else {
            let q = self.states[p].next[&c];
            if self.states[p].length + 1 == self.states[q].length {
                self.states[curr].link = Some(q);
            } else {
                let clone = self.states.len();
                self.states.push(State {
                    length: self.states[p].length + 1,
                    link: self.states[q].link,
                    next: self.states[q].next.clone(),
                });
                while p != usize::MAX && self.states[p].next[&c] == q {
                    self.states[p].next.insert(c, clone);
                    p = self.states[p].link.unwrap_or(usize::MAX);
                }
                self.states[q].link = Some(clone);
                self.states[curr].link = Some(clone);
            }
        }

        self.last = curr;
    }

    fn contains(&self, s: &str) -> bool {
        let mut state = 0;
        for c in s.chars() {
            if let Some(&next) = self.states[state].next.get(&c) {
                state = next;
            } else {
                return false;
            }
        }
        true
    }
}

fn main() {
    let sa = SuffixAutomaton::new("abcbc");
    println!("Contains 'bc': {}", sa.contains("bc"));
    println!("Contains 'cb': {}", sa.contains("cb"));
}
```

## Explanation

1. The Suffix Automaton is built incrementally, adding one character at a time.
2. Each state represents an equivalence class of substrings.
3. The `link` of a state points to the longest suffix that is in a different equivalence class.
4. The `extend` method adds a new character to the automaton, creating new states as necessary.
5. The `contains` method checks if a given string is a substring of the original string.

## Time and Space Complexity

- Construction: O(n) time and space
- Query (contains): O(m) time, where m is the length of the query string

## Use Cases

- Efficient substring searching
- Finding the lexicographically minimal rotation of a string
- Computing the number of distinct substrings

The Suffix Automaton is a powerful data structure for string processing tasks. It provides a compact representation of all substrings of a given string and allows for efficient operations on these substrings.
