// 1. Bubble Sort
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// 2. Selection Sort
fn selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in i + 1..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}

// 3. Insertion Sort
fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

// 4. Merge Sort
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge(arr, mid);
    }
}

fn merge<T: Ord + Clone>(arr: &mut [T], mid: usize) {
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

// 5. Quick Sort
fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    quick_sort(&mut arr[..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

// 6. Heap Sort
fn heap_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    // Build max heap
    for i in (0..arr.len() / 2).rev() {
        heapify(arr, arr.len(), i);
    }

    // Extract elements from the heap one by one
    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

// 7. Counting Sort (for u32 only)
fn counting_sort(arr: &mut [u32]) {
    if arr.is_empty() {
        return;
    }

    let max = *arr.iter().max().unwrap();
    let mut count = vec![0; (max + 1) as usize];

    for &value in arr.iter() {
        count[value as usize] += 1;
    }

    let mut output_index = 0;
    for (i, &count) in count.iter().enumerate() {
        for _ in 0..count {
            arr[output_index] = i as u32;
            output_index += 1;
        }
    }
}

// 8. Radix Sort (for u32 only)
fn radix_sort(arr: &mut [u32]) {
    if arr.is_empty() {
        return;
    }

    let max = *arr.iter().max().unwrap();
    let mut exp = 1;

    while max / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_by_digit(arr: &mut [u32], exp: u32) {
    let mut output = vec![0; arr.len()];
    let mut count = vec![0; 10];

    for &value in arr.iter() {
        let digit = (value / exp) % 10;
        count[digit as usize] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for &value in arr.iter().rev() {
        let digit = (value / exp) % 10;
        let index = count[digit as usize] - 1;
        output[index] = value;
        count[digit as usize] -= 1;
    }

    arr.copy_from_slice(&output);
}

fn main() {
    // Example usage of sorting algorithms
    let mut arr1 = vec![64, 34, 25, 12, 22, 11, 90];
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();

    bubble_sort(&mut arr1);
    println!("Bubble Sort: {:?}", arr1);

    selection_sort(&mut arr2);
    println!("Selection Sort: {:?}", arr2);

    insertion_sort(&mut arr3);
    println!("Insertion Sort: {:?}", arr3);

    merge_sort(&mut arr4);
    println!("Merge Sort: {:?}", arr4);

    quick_sort(&mut arr5);
    println!("Quick Sort: {:?}", arr5);

    heap_sort(&mut arr6);
    println!("Heap Sort: {:?}", arr6);

    counting_sort(&mut arr7);
    println!("Counting Sort: {:?}", arr7);

    radix_sort(&mut arr8);
    println!("Radix Sort: {:?}", arr8);
}
