// 1. Merge Sort
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut temp = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut temp);
    arr.clone_from_slice(&temp);
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], result: &mut [T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i].clone();
            i += 1;
        } else {
            result[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    if i < left.len() {
        result[k..].clone_from_slice(&left[i..]);
    }
    if j < right.len() {
        result[k..].clone_from_slice(&right[j..]);
    }
}

// 2. Quick Sort
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

// 3. Binary Search
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

// 4. Maximum Subarray Sum
fn max_subarray_sum(arr: &[i32]) -> i32 {
    if arr.len() == 1 {
        return arr[0];
    }
    let mid = arr.len() / 2;
    let left_sum = max_subarray_sum(&arr[..mid]);
    let right_sum = max_subarray_sum(&arr[mid..]);
    let cross_sum = max_crossing_sum(arr, mid);
    left_sum.max(right_sum).max(cross_sum)
}

fn max_crossing_sum(arr: &[i32], mid: usize) -> i32 {
    let mut left_sum = i32::MIN;
    let mut sum = 0;
    for i in (0..mid).rev() {
        sum += arr[i];
        left_sum = left_sum.max(sum);
    }
    let mut right_sum = i32::MIN;
    sum = 0;
    for i in mid..arr.len() {
        sum += arr[i];
        right_sum = right_sum.max(sum);
    }
    left_sum + right_sum
}

// 5. Closest Pair of Points
#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn closest_pair(points: &mut [Point]) -> f64 {
    if points.len() <= 3 {
        return brute_force_closest(points);
    }
    
    points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    let mid = points.len() / 2;
    let mid_point = points[mid];
    
    let left_min = closest_pair(&mut points[..mid]);
    let right_min = closest_pair(&mut points[mid..]);
    let mut min_dist = left_min.min(right_min);
    
    let mut strip = Vec::new();
    for point in points.iter() {
        if (point.x - mid_point.x).abs() < min_dist {
            strip.push(*point);
        }
    }
    
    strip.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    
    for i in 0..strip.len() {
        for j in i+1..strip.len() {
            if strip[j].y - strip[i].y >= min_dist {
                break;
            }
            min_dist = min_dist.min(strip[i].distance(&strip[j]));
        }
    }
    
    min_dist
}

fn brute_force_closest(points: &[Point]) -> f64 {
    let mut min_dist = f64::INFINITY;
    for i in 0..points.len() {
        for j in i+1..points.len() {
            min_dist = min_dist.min(points[i].distance(&points[j]));
        }
    }
    min_dist
}

// 6. Karatsuba Multiplication
fn karatsuba(x: u64, y: u64) -> u64 {
    if x < 10 || y < 10 {
        return x * y;
    }

    let n = x.max(y).to_string().len();
    let m = n / 2;

    let power = 10u64.pow(m as u32);
    let (a, b) = (x / power, x % power);
    let (c, d) = (y / power, y % power);

    let ac = karatsuba(a, c);
    let bd = karatsuba(b, d);
    let ad_plus_bc = karatsuba(a + b, c + d) - ac - bd;

    ac * power.pow(2) + ad_plus_bc * power + bd
}

fn main() {
    // 1. Merge Sort
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    merge_sort(&mut arr);
    println!("Merge Sort: {:?}", arr);

    // 2. Quick Sort
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    quick_sort(&mut arr);
    println!("Quick Sort: {:?}", arr);

    // 3. Binary Search
    let arr = vec![1, 3, 4, 6, 8, 9, 11];
    println!("Binary Search for 6: {:?}", binary_search(&arr, &6));

    // 4. Maximum Subarray Sum
    let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum Subarray Sum: {}", max_subarray_sum(&arr));

    // 5. Closest Pair of Points
    let mut points = vec![
        Point { x: 2.0, y: 3.0 },
        Point { x: 12.0, y: 30.0 },
        Point { x: 40.0, y: 50.0 },
        Point { x: 5.0, y: 1.0 },
        Point { x: 12.0, y: 10.0 },
        Point { x: 3.0, y: 4.0 },
    ];
    println!("Closest Pair Distance: {}", closest_pair(&mut points));

    // 6. Karatsuba Multiplication
    let x = 1234;
    let y = 5678;
    println!("Karatsuba Multiplication of {} and {}: {}", x, y, karatsuba(x, y));
}
