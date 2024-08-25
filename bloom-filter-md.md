# Bloom Filter

A Bloom filter is a space-efficient probabilistic data structure used to test whether an element is a member of a set. It can have false positives but no false negatives.

## Implementation

```rust
use bit_vec::BitVec;
use siphasher::sip::SipHasher;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

struct BloomFilter<T> {
    bit_vec: BitVec,
    num_hashes: usize,
    phantom: PhantomData<T>,
}

impl<T: Hash> BloomFilter<T> {
    fn new(size: usize, num_hashes: usize) -> Self {
        BloomFilter {
            bit_vec: BitVec::from_elem(size, false),
            num_hashes,
            phantom: PhantomData,
        }
    }

    fn insert(&mut self, item: &T) {
        for i in 0..self.num_hashes {
            let index = self.hash(item, i);
            self.bit_vec.set(index, true);
        }
    }

    fn contains(&self, item: &T) -> bool {
        (0..self.num_hashes).all(|i| {
            let index = self.hash(item, i);
            self.bit_vec[index]
        })
    }

    fn hash(&self, item: &T, seed: usize) -> usize {
        let mut hasher = SipHasher::new_with_keys(0, seed as u64);
        item.hash(&mut hasher);
        (hasher.finish() % self.bit_vec.len() as u64) as usize
    }
}

fn main() {
    let mut bloom = BloomFilter::<String>::new(1000, 3);
    
    bloom.insert(&"apple".to_string());
    bloom.insert(&"banana".to_string());
    bloom.insert(&"cherry".to_string());

    println!("Contains 'apple': {}", bloom.contains(&"apple".to_string()));
    println!("Contains 'date': {}", bloom.contains(&"date".to_string()));
}
```

## Key Concepts

1. **Bit Array**: The Bloom filter uses a bit array to represent the set.
2. **Multiple Hash Functions**: Several hash functions are used to set bits for each element.
3. **Probabilistic**: False positives are possible, but false negatives are not.
4. **Space Efficiency**: Bloom filters use much less space than conventional hash tables.

## When to Use

Use a Bloom Filter when:

- You need to quickly check if an element might be in a set.
- You can tolerate false positives but not false negatives.
- Memory usage is a concern and you're dealing with large datasets.
- You're implementing a cache or a spell checker.

## Time Complexity

- Insert: O(k), where k is the number of hash functions
- Query: O(k)

## Space Complexity

O(m), where m is the size of the bit vector

Bloom Filters are particularly useful in scenarios where you need to quickly filter out elements that are definitely not in a set, such as in database query optimization, network routers, or cache systems. They provide a good trade-off between space efficiency and accuracy for membership queries.
