use std::cmp;

// 1. Basic Array Operations
fn basic_array_ops() {
    // Creating an array
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr1: {:?}", arr1);

    // Creating an array with default values
    let arr2 = [0; 5];
    println!("arr2: {:?}", arr2);

    // Accessing elements
    println!("First element of arr1: {}", arr1[0]);

    // Slicing
    let slice = &arr1[1..4];
    println!("Slice of arr1: {:?}", slice);

    // Iterating
    for &item in arr1.iter() {
        print!("{} ", item);
    }
    println!();

    // Length of array
    println!("Length of arr1: {}", arr1.len());
}

// 2. Array Rotation
fn rotate_array<T: Clone>(arr: &mut [T], k: usize) {
    if arr.is_empty() {
        return;
    }
    let k = k % arr.len();
    arr.reverse();
    arr[..k].reverse();
    arr[k..].reverse();
}

// 3. Two Sum Problem
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return Some((j, i));
        }
        map.insert(num, i);
    }
    None
}

// 4. Kadane's Algorithm (Maximum Subarray Sum)
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_so_far = arr[0];
    let mut max_ending_here = arr[0];
    for &item in arr.iter().skip(1) {
        max_ending_here = cmp::max(item, max_ending_here + item);
        max_so_far = cmp::max(max_so_far, max_ending_here);
    }
    max_so_far
}

// 5. Dutch National Flag Problem
fn dutch_flag_sort(arr: &mut [i32]) {
    let mut low = 0;
    let mut mid = 0;
    let mut high = arr.len() - 1;
    while mid <= high {
        match arr[mid] {
            0 => {
                arr.swap(low, mid);
                low += 1;
                mid += 1;
            }
            1 => mid += 1,
            2 => {
                arr.swap(mid, high);
                if high == 0 {
                    break;
                }
                high -= 1;
            }
            _ => panic!("Array should only contain 0, 1, or 2"),
        }
    }
}

// 6. Merge Sorted Arrays
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&arr1[i..]);
    result.extend_from_slice(&arr2[j..]);
    result
}

// 7. Binary Search
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();
    while low < high {
        let mid = low + (high - low) / 2;
        match arr[mid].cmp(target) {
            cmp::Ordering::Equal => return Some(mid),
            cmp::Ordering::Less => low = mid + 1,
            cmp::Ordering::Greater => high = mid,
        }
    }
    None
}

// 8. Finding Majority Element (Boyer-Moore Voting Algorithm)
fn majority_element(arr: &[i32]) -> Option<i32> {
    let mut candidate = None;
    let mut count = 0;
    for &num in arr {
        if count == 0 {
            candidate = Some(num);
        }
        count += if Some(num) == candidate { 1 } else { -1 };
    }
    let candidate = candidate?;
    if arr.iter().filter(|&&x| x == candidate).count() > arr.len() / 2 {
        Some(candidate)
    } else {
        None
    }
}

// 9. Quick Select (Finding kth Smallest Element)
fn quick_select(arr: &mut [i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let pos = partition(arr);
        match pos.cmp(&(k - 1)) {
            cmp::Ordering::Equal => Some(arr[pos]),
            cmp::Ordering::Greater => quick_select(&mut arr[..pos], k),
            cmp::Ordering::Less => quick_select(&mut arr[pos + 1..], k - pos - 1),
        }
    } else {
        None
    }
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

fn main() {
    // 1. Basic Array Operations
    basic_array_ops();

    // 2. Array Rotation
    let mut arr = [1, 2, 3, 4, 5];
    rotate_array(&mut arr, 2);
    println!("\nRotated Array: {:?}", arr);

    // 3. Two Sum Problem
    let nums = [2, 7, 11, 15];
    let target = 9;
    if let Some((i, j)) = two_sum(&nums, target) {
        println!("Two Sum: indices {:?} sum to {}", (i, j), target);
    }

    // 4. Maximum Subarray Sum
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum Subarray Sum: {}", max_subarray_sum(&arr));

    // 5. Dutch National Flag Problem
    let mut arr = [2, 0, 1, 1, 0, 2, 1, 2, 0];
    dutch_flag_sort(&mut arr);
    println!("Sorted Array (Dutch Flag): {:?}", arr);

    // 6. Merge Sorted Arrays
    let arr1 = [1, 3, 5, 7];
    let arr2 = [2, 4, 6, 8];
    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged Sorted Arrays: {:?}", merged);

    // 7. Binary Search
    let arr = [1, 3, 5, 7, 9, 11, 13, 15];
    if let Some(index) = binary_search(&arr, &7) {
        println!("Binary Search: Found 7 at index {}", index);
    }

    // 8. Majority Element
    let arr = [2, 2, 1, 1, 1, 2, 2];
    if let Some(maj) = majority_element(&arr) {
        println!("Majority Element: {}", maj);
    }

    // 9. Quick Select
    let mut arr = [3, 2, 1, 5, 6, 4];
    if let Some(kth) = quick_select(&mut arr, 2) {
        println!("2nd smallest element: {}", kth);
    }
}
