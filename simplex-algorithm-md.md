# Simplex Algorithm for Linear Programming

The Simplex Algorithm is used to solve linear programming problems. It finds the optimal solution to a linear objective function subject to linear equality and inequality constraints.

## Implementation

```rust
use ndarray::{Array1, Array2, s};

struct SimplexTableau {
    tableau: Array2<f64>,
    basic_variables: Vec<usize>,
}

impl SimplexTableau {
    fn new(c: &[f64], a: &[Vec<f64>], b: &[f64]) -> Self {
        let m = a.len();
        let n = c.len();
        let mut tableau = Array2::zeros((m + 1, n + m + 1));
        
        // Set objective function coefficients
        tableau.slice_mut(s![0, ..n]).assign(&Array1::from_vec(c.to_vec()));
        
        // Set constraint coefficients and right-hand side
        for i in 0..m {
            tableau.slice_mut(s![i + 1, ..n]).assign(&Array1::from_vec(a[i].clone()));
            tableau[[i + 1, n + m]] = b[i];
        }
        
        // Set slack variables
        for i in 0..m {
            tableau[[i + 1, n + i]] = 1.0;
        }
        
        SimplexTableau {
            tableau,
            basic_variables: (n..n+m).collect(),
        }
    }
    
    fn solve(&mut self) -> Option<Vec<f64>> {
        loop {
            if let Some(pivot_column) = self.find_pivot_column() {
                if let Some(pivot_row) = self.find_pivot_row(pivot_column) {
                    self.pivot(pivot_row, pivot_column);
                } else {
                    return None; // Unbounded
                }
            } else {
                break;
            }
        }
        
        let mut solution = vec![0.0; self.tableau.ncols() - 1];
        for (i, &var) in self.basic_variables.iter().enumerate() {
            if var < solution.len() {
                solution[var] = self.tableau[[i + 1, self.tableau.ncols() - 1]];
            }
        }
        Some(solution)
    }
    
    fn find_pivot_column(&self) -> Option<usize> {
        self.tableau.row(0)
            .slice(s![..self.tableau.ncols()-1])
            .iter()
            .enumerate()
            .filter(|(_, &value)| value < 0.0)
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(index, _)| index)
    }
    
    fn find_pivot_row(&self, pivot_column: usize) -> Option<usize> {
        let ratios: Vec<_> = self.tableau.slice(s![1.., [pivot_column, -1]])
            .axis_iter(ndarray::Axis(0))
            .enumerate()
            .filter(|(_, row)| row[0] > 0.0)
            .map(|(i, row)| (i, row[1] / row[0]))
            .collect();
        
        ratios.into_iter()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(index, _)| index + 1)
    }
    
    fn pivot(&mut self, pivot_row: usize, pivot_column: usize) {
        let pivot_value = self.tableau[[pivot_row, pivot_column]];
        self.tableau.row_mut(pivot_row).mapv_inplace(|x| x / pivot_value);
        
        for i in 0..self.tableau.nrows() {
            if i != pivot_row {
                let factor = self.tableau[[i, pivot_column]];
                self.tableau.row_mut(i).zip_mut_with(
                    &self.tableau.row(pivot_row),
                    |x, &y| *x -= factor * y
                );
            }
        }
        
        self.basic_variables[pivot_row - 1] = pivot_column;
    }
}

fn main() {
    // Example: Maximize 3x + 4y subject to:
    // x + 2y <= 14
    // 3x - y <= 0
    // x - y <= 2
    // x, y >= 0
    let c = vec![3.0, 4.0];
    let a = vec![
        vec![1.0, 2.0],
        vec![3.0, -1.0],
        vec![1.0, -1.0],
    ];
    let b = vec![14.0, 0.0, 2.0];
    
    let mut simplex = SimplexTableau::new(&c, &a, &b);
    if let Some(solution) = simplex.solve() {
        println!("Optimal solution: x = {}, y = {}", solution[0], solution[1]);
        println!("Optimal value: {}", c[0] * solution[0] + c[1] * solution[1]);
    } else {
        println!("Problem is unbounded");
    }
}
```

## Explanation

1. The Simplex Algorithm works by iteratively improving a feasible solution until an optimal solution is found.
2. It uses a tableau representation of the linear program, including slack variables.
3. In each iteration, it selects a pivot element and performs row operations to improve the solution.
4. The algorithm terminates when no further improvements can be made or when it determines the problem is unbounded.

## Time Complexity

The Simplex Algorithm has exponential worst-case time complexity, but it performs well in practice and often has polynomial average-case complexity.

## Use Cases

- Resource allocation problems
- Production planning and scheduling
- Transportation and logistics optimization
- Financial portfolio optimization

The Simplex Algorithm is a fundamental tool in operations research and mathematical optimization, widely used in various industries for solving complex linear programming problems.
