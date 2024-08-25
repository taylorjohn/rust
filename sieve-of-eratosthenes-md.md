# Sieve of Eratosthenes

The Sieve of Eratosthenes is an ancient algorithm for finding all prime numbers up to a given limit.

## Implementation

```rust
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    is_prime.iter()
            .enumerate()
            .filter_map(|(num, &is_prime)| if is_prime { Some(num) } else { None })
            .collect()
}

fn main() {
    let limit = 50;
    let primes = sieve_of_eratosthenes(limit);
    println!("Primes up to {}: {:?}", limit, primes);
}
```

## Explanation

1. We create a boolean vector `is_prime` initialized to `true` for all numbers up to `n`.
2. We mark 0 and 1 as not prime.
3. We iterate from 2 to the square root of `n`:
   - If a number `i` is marked prime, we mark all its multiples starting from `i*i` as not prime.
4. Finally, we collect all the numbers that are still marked as prime into a vector.

## Time Complexity

The time complexity of the Sieve of Eratosthenes is O(n log log n), which makes it very efficient for generating primes up to a given limit.

## Use Cases

- Generating prime numbers for cryptographic purposes
- Solving number theory problems in competitive programming
- Educational tool for teaching about prime numbers

The Sieve of Eratosthenes is one of the most efficient ways to find all primes smaller than a given number, especially when the range is not too large. It's simple to implement and understand, making it a popular choice for many applications involving prime numbers.
