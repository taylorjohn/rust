# Merge Sort

Merge Sort is an efficient, stable, divide-and-conquer sorting algorithm. It works by dividing the unsorted list into n sublists, each containing one element, and then repeatedly merging sublists to produce new sorted sublists until there is only one sublist remaining.

## Implementation

```rust
fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut temp = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut temp);
    arr.copy_from_slice(&temp);
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T], result: &mut [T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i];
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    if i < left.len() {
        result[k..].copy_from_slice(&left[i..]);
    }
    if j < right.len() {
        result[k..].copy_from_slice(&right[j..]);
    }
}

// Usage
fn main() {
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    merge_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
```

## When to Use

Use Merge Sort when:

- You need a stable sorting algorithm (maintains the relative order of equal elements).
- You require guaranteed O(n log n) time complexity for all cases.
- You're dealing with linked lists (Merge Sort is particularly efficient for linked lists).
- You have enough memory to accommodate the additional space complexity.

Merge Sort is particularly useful in:

- External sorting (sorting data that doesn't fit into memory).
- Sorting linked lists.
- Counting inversions in an array.
- Parallel sorting implementations (Merge Sort can be easily parallelized).

## Time Complexity

- Best case: O(n log n)
- Average case: O(n log n)
- Worst case: O(n log n)

## Space Complexity

O(n) - Merge Sort requires additional space proportional to the size of the input array.

## Advantages and Disadvantages

Advantages:
- Stable sort
- Guaranteed O(n log n) time complexity
- Can be easily parallelized

Disadvantages:
- Requires O(n) extra space
- Slower for smaller arrays compared to algorithms like Quicksort

Merge Sort is often the algorithm of choice when stability is needed and when dealing with linked lists or external sorting scenarios.
