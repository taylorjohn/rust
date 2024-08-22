# Binary Search and Variations

Binary search is a highly efficient algorithm for searching a sorted array by repeatedly dividing the search interval in half.

## Basic Binary Search Implementation

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

## Lower Bound (First Occurrence)

```rust
fn lower_bound<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if &arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    if low < arr.len() && &arr[low] == target {
        Some(low)
    } else {
        None
    }
}
```

## Upper Bound (Last Occurrence)

```rust
fn upper_bound<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if &arr[mid] <= target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    if low > 0 && &arr[low - 1] == target {
        Some(low - 1)
    } else {
        None
    }
}
```

## Key Concepts

1. **Divide and Conquer**: The algorithm divides the search space in half at each step.
2. **Logarithmic Time Complexity**: Binary search runs in O(log n) time.
3. **Sorted Input**: The input array must be sorted for binary search to work correctly.
4. **Variations**: Lower and upper bound variations are useful for finding the first or last occurrence of an element in a sorted array with duplicates.

## When to Use

Use Binary Search when:

- You have a sorted array and need to search for a specific element or the insertion point for a new element.
- You need to find the first or last occurrence of an element in a sorted array with potential duplicates.
- Implementing algorithms that require efficient searching in sorted data.

Binary Search is particularly useful in:

- Database indexing and searching
- Implementing efficient lookup tables
- Optimization problems where the search space can be represented as a sorted range

The main advantage of Binary Search is its logarithmic time complexity, which makes it extremely efficient for large datasets compared to linear search algorithms.
