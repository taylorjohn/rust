# Fast Exponentiation (Binary Exponentiation)

Fast Exponentiation is an algorithm to compute a^n efficiently, where a is a base number and n is an exponent. It reduces the number of multiplications needed from O(n) to O(log n).

## Implementation

```rust
// Fast exponentiation for integers
fn fast_exp(mut base: i64, mut exp: u64, modulus: i64) -> i64 {
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    result
}

// Fast exponentiation for floating-point numbers
fn fast_exp_float(mut base: f64, mut exp: i32) -> f64 {
    let mut result = 1.0;
    if exp < 0 {
        base = 1.0 / base;
        exp = -exp;
    }
    while exp > 0 {
        if exp & 1 == 1 {
            result *= base;
        }
        base *= base;
        exp >>= 1;
    }
    result
}

// Fast exponentiation for matrices
use std::ops::Mul;

#[derive(Clone)]
struct Matrix {
    data: Vec<Vec<i64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(data: Vec<Vec<i64>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        Matrix { data, rows, cols }
    }

    fn identity(n: usize) -> Self {
        let mut data = vec![vec![0; n]; n];
        for i in 0..n {
            data[i][i] = 1;
        }
        Matrix { data, rows: n, cols: n }
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut result = vec![vec![0; other.cols]; self.rows];
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Matrix::new(result)
    }
}

fn matrix_fast_exp(mut base: Matrix, mut exp: u64) -> Matrix {
    let mut result = Matrix::identity(base.rows);
    while exp > 0 {
        if exp & 1 == 1 {
            result = &result * &base;
        }
        base = &base * &base;
        exp >>= 1;
    }
    result
}

fn main() {
    // Integer exponentiation
    println!("2^10 mod 1000 = {}", fast_exp(2, 10, 1000));

    // Floating-point exponentiation
    println!("2.5^3 = {}", fast_exp_float(2.5, 3));

    // Matrix exponentiation
    let base = Matrix::new(vec![vec![1, 1], vec![1, 0]]);
    let result = matrix_fast_exp(base, 10);
    println!("Fibonacci matrix^10 = {:?}", result.data);
}
```

## Key Concepts

1. **Divide and Conquer**: The algorithm uses the principle of dividing the exponent by 2 at each step.
2. **Bit Manipulation**: It uses bitwise operations to check if the current bit of the exponent is set.
3. **Modular Arithmetic**: For integer exponentiation, it often includes modular arithmetic to prevent overflow.
4. **Matrix Exponentiation**: An extension of the concept to matrices, useful for solving recurrence relations.

## When to Use

Fast Exponentiation is particularly useful in:

1. **Cryptography**: Many cryptographic algorithms rely on efficient modular exponentiation.
2. **Competitive Programming**: It's a fundamental technique for solving many problems efficiently.
3. **Number Theory**: Used in various number-theoretic computations.
4. **Dynamic Programming**: Matrix exponentiation can solve certain types of recurrence relations efficiently.
5. **Computer Graphics**: For repeated transformations or scaling operations.

## Time Complexity

The time complexity of Fast Exponentiation is O(log n), where n is the exponent. This is a significant improvement over the naive O(n) approach, especially for large exponents.

## Space Complexity

The space complexity is O(1) for the iterative version, making it very memory-efficient.

Fast Exponentiation is a powerful technique that demonstrates how algorithmic thinking can dramatically improve the efficiency of computations. It's a prime example of how understanding binary representation and clever mathematics can lead to significant optimizations.
