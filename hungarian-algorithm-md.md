# Hungarian Algorithm (Maximum Bipartite Matching)

The Hungarian Algorithm, also known as the Munkres assignment algorithm, solves the assignment problem in polynomial time. It finds a maximum weight matching in a weighted bipartite graph.

## Implementation

```rust
fn hungarian_algorithm(cost_matrix: &Vec<Vec<i32>>) -> (i32, Vec<usize>) {
    let n = cost_matrix.len();
    let mut assignment = vec![None; n];
    let mut dual_u = vec![0; n];
    let mut dual_v = vec![0; n];
    
    for u in 0..n {
        let mut used_v = vec![false; n];
        let mut min_slack_v = vec![i32::MAX; n];
        let mut prev_v = vec![0; n];
        let mut v = 0;
        let mut matched_v = vec![None; n];
        
        'outer: loop {
            used_v[v] = true;
            let mut u1 = u;
            let mut min_slack = i32::MAX;
            let mut next_v = 0;
            
            for v1 in 0..n {
                if !used_v[v1] {
                    let slack = cost_matrix[u1][v1] - dual_u[u1] - dual_v[v1];
                    if slack < min_slack_v[v1] {
                        min_slack_v[v1] = slack;
                        prev_v[v1] = v;
                    }
                    if min_slack_v[v1] < min_slack {
                        min_slack = min_slack_v[v1];
                        next_v = v1;
                    }
                }
            }

            if min_slack != 0 {
                for v1 in 0..n {
                    if used_v[v1] {
                        dual_u[matched_v[v1].unwrap()] += min_slack;
                        dual_v[v1] -= min_slack;
                    } else {
                        min_slack_v[v1] -= min_slack;
                    }
                }
            }

            v = next_v;
            if matched_v[v].is_none() {
                break 'outer;
            }
            u1 = matched_v[v].unwrap();
        }

        while v != n {
            let u1 = matched_v[v].unwrap_or(u);
            matched_v[v] = Some(u1);
            assignment[u1] = Some(v);
            v = prev_v[v];
        }
    }

    let total_cost = assignment.iter().enumerate()
        .map(|(u, &v)| cost_matrix[u][v.unwrap()])
        .sum();

    (total_cost, assignment.iter().map(|&v| v.unwrap()).collect())
}

fn main() {
    let cost_matrix = vec![
        vec![250, 400, 350],
        vec![400, 600, 350],
        vec![200, 400, 250]
    ];

    let (total_cost, assignment) = hungarian_algorithm(&cost_matrix);
    println!("Total cost: {}", total_cost);
    println!("Assignment: {:?}", assignment);
}
```

## Explanation

1. The algorithm starts by initializing dual variables and finding an initial matching.
2. It then iteratively improves the matching by finding augmenting paths.
3. The dual variables are adjusted to maintain complementary slackness conditions.
4. The process continues until a perfect matching is found.

## Time Complexity

O(n³), where n is the number of workers/jobs

## Use Cases

- Assignment problems (e.g., assigning tasks to workers)
- Image recognition and computer vision
- Network flow problems

The Hungarian Algorithm is particularly useful in scenarios where you need to find the optimal assignment between two sets of equal size, minimizing the total cost or maximizing the total profit.
