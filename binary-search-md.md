# Binary Search

Binary search is a highly efficient algorithm for searching a sorted array by repeatedly dividing the search interval in half.

## Implementation

```rust
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }

    None
}

// Usage
fn main() {
    let arr = vec![1, 3, 4, 6, 8, 9, 11];
    let target = 6;
    if let Some(index) = binary_search(&arr, &target) {
        println!("Found {} at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}
```

## When to Use

Use binary search when:

- You have a sorted array and need to find a specific element.
- You need to find the insertion point for a new element in a sorted array.
- You're working with a large dataset and need an efficient search algorithm.

Binary search is particularly useful in:

- Database indexing and searching
- Implementing efficient lookup tables
- Optimization problems where the search space can be represented as a sorted range

## Time Complexity

Binary search has a time complexity of O(log n), making it much faster than linear search for large datasets.

## Variations

1. Lower Bound (First Occurrence)
2. Upper Bound (Last Occurrence)
3. Binary Search on Answer (for optimization problems)

These variations can be implemented with slight modifications to the basic binary search algorithm.
