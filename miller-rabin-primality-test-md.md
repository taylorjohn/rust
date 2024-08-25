# Miller-Rabin Primality Test

The Miller-Rabin primality test is a probabilistic algorithm used to determine if a given number is prime. It's widely used in cryptographic applications due to its efficiency for large numbers.

## Implementation

```rust
use rand::Rng;

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus
    }
    result
}

fn miller_rabin_test(n: u64, k: u32) -> bool {
    if n <= 1 || n == 4 { return false; }
    if n <= 3 { return true; }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let mut rng = rand::thread_rng();

    'witness: for _ in 0..k {
        let a = rng.gen_range(2..n-2);
        let mut x = mod_pow(a, d, n);

        if x == 1 || x == n-1 {
            continue 'witness;
        }

        for _ in 0..s-1 {
            x = x * x % n;
            if x == n-1 {
                continue 'witness;
            }
        }

        return false;
    }

    true
}

fn main() {
    let number = 997; // A prime number
    let k = 5; // Number of iterations

    if miller_rabin_test(number, k) {
        println!("{} is probably prime", number);
    } else {
        println!("{} is composite", number);
    }
}
```

## Explanation

1. The algorithm is based on the fact that for a prime `p`, either `a^d ≡ 1 (mod p)` or `a^(2^r * d) ≡ -1 (mod p)` for some `r` where `0 ≤ r < s`.
2. We first compute `s` and `d` such that `n-1 = 2^s * d` where `d` is odd.
3. We then perform `k` rounds of testing, each time with a random base `a`.
4. In each round, we compute `x = a^d mod n` and then square `x` up to `s-1` times.
5. If none of the conditions for primality are met, we declare the number composite.

## Time Complexity

O(k * log³n), where k is the number of rounds and n is the number being tested.

## Use Cases

- Cryptography (e.g., generating large prime numbers for RSA)
- Number theory research
- Primality testing in various algorithms

The Miller-Rabin test is particularly useful when dealing with very large numbers where deterministic primality tests would be too slow. It provides a good balance between speed and accuracy, making it suitable for many practical applications in cryptography and number theory.
