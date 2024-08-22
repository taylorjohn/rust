# Suffix Array Construction (DC3 Algorithm)

The DC3 (Difference Cover of 3) algorithm is an efficient method for constructing suffix arrays in linear time. It's particularly useful for large strings where efficiency is crucial.

## Implementation

```rust
fn dc3(s: &[usize]) -> Vec<usize> {
    let n = s.len();
    let mut sa = vec![0; n];
    let mut rank = vec![0; n];
    
    if n < 2 {
        for i in 0..n {
            sa[i] = i;
        }
        return sa;
    }
    
    let mut s12 = (0..n + (n + 2) / 3)
        .filter(|&i| i % 3 != 0)
        .collect::<Vec<_>>();
    s12.push(n);
    s12.push(n);
    
    radix_sort(&mut s12, &|&i| s.get(i + 2).unwrap_or(&0));
    radix_sort(&mut s12, &|&i| s.get(i + 1).unwrap_or(&0));
    radix_sort(&mut s12, &|&i| s.get(i).unwrap_or(&0));
    
    let mut name = 0;
    let mut c0 = usize::MAX;
    let mut c1 = usize::MAX;
    let mut c2 = usize::MAX;
    
    for i in 0..s12.len() - 2 {
        if s[s12[i]] != c0 || s[s12[i] + 1] != c1 || s[s12[i] + 2] != c2 {
            name += 1;
            c0 = s[s12[i]];
            c1 = s[s12[i] + 1];
            c2 = s[s12[i] + 2];
        }
        if s12[i] % 3 == 1 {
            rank[s12[i] / 3] = name;
        } else {
            rank[s12[i] / 3 + n / 3] = name;
        }
    }
    
    if name < s12.len() - 2 {
        let sa12 = dc3(&rank[..rank.len() - 2]);
        for i in 0..s12.len() - 2 {
            s12[sa12[i]] = i;
        }
    } else {
        for i in 0..s12.len() - 2 {
            s12[rank[i] - 1] = i;
        }
    }
    
    let mut s0 = s12.iter()
        .filter(|&&i| i < n && i % 3 == 0)
        .cloned()
        .collect::<Vec<_>>();
    
    radix_sort(&mut s0, &|&i| s.get(i + 2).unwrap_or(&0));
    radix_sort(&mut s0, &|&i| s.get(i + 1).unwrap_or(&0));
    radix_sort(&mut s0, &|&i| s.get(i).unwrap_or(&0));
    
    let mut p = 0;
    let mut t = s12.len() - 3;
    let mut k = 0;
    
    while p < s0.len() && t > 0 {
        let i = s12[t];
        let j = s0[p];
        
        if (i % 3 == 1 && leq(s, i, j, s, s12, rank))
            || (i % 3 == 2 && leq3(s, i, j, s, s12, rank))
        {
            sa[k] = i;
            t -= 1;
        } else {
            sa[k] = j;
            p += 1;
        }
        k += 1;
    }
    
    sa[k..].copy_from_slice(&s0[p..]);
    sa[k + s0.len() - p..].copy_from_slice(&s12[1..t + 1]);
    
    sa
}

fn radix_sort<T, F>(v: &mut [T], key: &F)
where
    F: Fn(&T) -> usize,
{
    let max = v.iter().map(key).max().unwrap_or(0);
    let mut cnt = vec![0; max + 1];
    let mut tmp = vec![Default::default(); v.len()];
    
    for x in v.iter() {
        cnt[key(x)] += 1;
    }
    
    for i in 1..cnt.len() {
        cnt[i] += cnt[i - 1];
    }
    
    for x in v.iter().rev() {
        cnt[key(x)] -= 1;
        tmp[cnt[key(x)]] = x.clone();
    }
    
    v.copy_from_slice(&tmp);
}

fn leq(s1: &[usize], p1: usize, p2: usize, s2: &[usize], sa: &[usize], rank: &[usize]) -> bool {
    if s1[p1] != s2[p2] {
        return s1[p1] < s2[p2];
    }
    if p1 % 3 != 0 || p2 % 3 != 0 {
        return rank[p1 / 3] <= rank[p2 / 3];
    }
    leq(s1, p1 + 1, p2 + 1, s2, sa, rank)
}

fn leq3(s1: &[usize], p1: usize, p2: usize, s2: &[usize], sa: &[usize], rank: &[usize]) -> bool {
    if s1[p1] != s2[p2] {
        return s1[p1] < s2[p2];
    }
    if s1[p1 + 1] != s2[p2 + 1] {
        return s1[p1 + 1] < s2[p2 + 1];
    }
    rank[p1 / 3 + n / 3] <= rank[p2 / 3]
}

// Usage
fn main() {
    let s = "banana".as_bytes().iter().map(|&c| c as usize).collect::<Vec<_>>();
    let sa = dc3(&s);
    println!("Suffix Array: {:?}", sa);
}
```

## Key Concepts

1. **Difference Cover**: The algorithm uses a difference cover of 3 to divide the suffixes into three classes.
2. **Recursion**: The algorithm recursively solves the problem for 2/3 of the suffixes.
3. **Radix Sort**: Used for efficient sorting of suffixes.
4. **Linear Time Complexity**: Achieves O(n) time complexity for suffix array construction.

## When to Use

Use the DC3 Algorithm for Suffix Array Construction when:

- You need to construct a suffix array for a large string efficiently.
- You're working on problems that require fast string processing or pattern matching.
- You need a linear-time algorithm for suffix array construction.

The DC3 Algorithm is particularly useful in:

- Bioinformatics for processing large DNA sequences.
- Text compression algorithms.
- Implementing efficient string matching algorithms.
- Solving complex string-related problems in competitive programming.

The main advantage of the DC3 algorithm is its linear time complexity, making it suitable for constructing suffix arrays for very large strings. It's also theoretically simpler than some other linear-time suffix array construction algorithms.
