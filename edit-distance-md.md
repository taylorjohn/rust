# Edit Distance (Levenshtein Distance)

The Edit Distance between two strings is the minimum number of operations required to transform one string into the other. The allowed operations are:
1. Insert a character
2. Delete a character
3. Replace a character

## Implementation

```rust
fn edit_distance(s1: &str, s2: &str) -> usize {
    let (m, n) = (s1.len(), s2.len());
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize first row and column
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    // Fill the dp table
    for i in 1..=m {
        for j in 1..=n {
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
            }
        }
    }

    dp[m][n]
}

fn backtrack(s1: &str, s2: &str, dp: &Vec<Vec<usize>>) -> Vec<String> {
    let (mut i, mut j) = (s1.len(), s2.len());
    let mut operations = Vec::new();

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
            i -= 1;
            j -= 1;
        } else if i > 0 && j > 0 && dp[i][j] == dp[i - 1][j - 1] + 1 {
            operations.push(format!("Replace '{}' with '{}'", s1.chars().nth(i - 1).unwrap(), s2.chars().nth(j - 1).unwrap()));
            i -= 1;
            j -= 1;
        } else if i > 0 && dp[i][j] == dp[i - 1][j] + 1 {
            operations.push(format!("Delete '{}'", s1.chars().nth(i - 1).unwrap()));
            i -= 1;
        } else {
            operations.push(format!("Insert '{}'", s2.chars().nth(j - 1).unwrap()));
            j -= 1;
        }
    }

    operations.reverse();
    operations
}

fn main() {
    let s1 = "kitten";
    let s2 = "sitting";

    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    let distance = edit_distance(s1, s2);
    println!("Edit Distance between '{}' and '{}': {}", s1, s2, distance);

    // Recompute dp table for backtracking
    for i in 0..=s1.len() { dp[i][0] = i; }
    for j in 0..=s2.len() { dp[0][j] = j; }
    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
            }
        }
    }

    let operations = backtrack(s1, s2, &dp);
    println!("Operations to transform '{}' into '{}':", s1, s2);
    for (i, op) in operations.iter().enumerate() {
        println!("{}. {}", i + 1, op);
    }
}
```

## Key Concepts

1. **Dynamic Programming**: The problem is solved by breaking it down into smaller subproblems and storing their solutions.
2. **Optimal Substructure**: The solution to a larger problem can be constructed from solutions of its subproblems.
3. **Memoization**: Results of subproblems are stored in a table to avoid redundant calculations.
4. **Backtracking**: After finding the minimum edit distance, we can backtrack through the DP table to find the actual sequence of operations.

## When to Use

The Edit Distance algorithm is useful in various scenarios:

1. **Spell Checking**: To suggest corrections for misspelled words.
2. **DNA Sequence Alignment**: To measure the similarity between genetic sequences.
3. **Natural Language Processing**: For tasks like autocorrect and fuzzy string matching.
4. **Plagiarism Detection**: To measure the similarity between texts.
5. **File Difference**: To compute the difference between two files.
6. **Data Cleansing**: To identify and correct errors in datasets.

## Time Complexity

The time complexity of this algorithm is O(mn), where m and n are the lengths of the two strings being compared.

## Space Complexity

The space complexity is also O(mn) due to the 2D DP table used.

## Advantages and Limitations

Advantages:
- Finds the optimal solution (minimum number of operations)
- Can be extended to handle different costs for different operations
- Provides a basis for more complex string similarity algorithms

Limitations:
- Quadratic time and space complexity can be prohibitive for very long strings
- Basic version doesn't account for real-world typing patterns (e.g., common typos)
- Doesn't handle transpositions (swapping adjacent characters) as a single operation

The Edit Distance algorithm is a fundamental tool in string processing and serves as a building block for more advanced algorithms in various fields, particularly in bioinformatics and natural language processing.
