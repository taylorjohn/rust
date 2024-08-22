// 1. Factorial calculation
fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// 2. Fibonacci sequence
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// 3. Binary search (recursive implementation)
fn binary_search<T: Ord>(arr: &[T], target: &T, low: usize, high: usize) -> Option<usize> {
    if low > high {
        return None;
    }
    let mid = low + (high - low) / 2;
    if arr[mid] == *target {
        Some(mid)
    } else if arr[mid] > *target {
        binary_search(arr, target, low, mid.saturating_sub(1))
    } else {
        binary_search(arr, target, mid + 1, high)
    }
}

// 4. Tower of Hanoi
fn tower_of_hanoi(n: u32, from: char, to: char, aux: char) {
    if n == 1 {
        println!("Move disk 1 from {} to {}", from, to);
    } else {
        tower_of_hanoi(n - 1, from, aux, to);
        println!("Move disk {} from {} to {}", n, from, to);
        tower_of_hanoi(n - 1, aux, to, from);
    }
}

// 5. Merge Sort
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

// 6. Tree traversal (using a binary tree)
#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

fn inorder_traversal<T: std::fmt::Debug>(node: &Option<Box<TreeNode<T>>>) {
    if let Some(n) = node {
        inorder_traversal(&n.left);
        println!("{:?}", n.value);
        inorder_traversal(&n.right);
    }
}

// 7. Recursive string reversal
fn reverse_string(s: &str) -> String {
    if s.is_empty() {
        String::new()
    } else {
        format!("{}{}", reverse_string(&s[1..]), &s[0..1])
    }
}

// 8. Recursive sum of digits
fn sum_of_digits(n: u32) -> u32 {
    if n < 10 {
        n
    } else {
        n % 10 + sum_of_digits(n / 10)
    }
}

// 9. Recursive exponentiation
fn power(base: i32, exponent: u32) -> i32 {
    if exponent == 0 {
        1
    } else if exponent % 2 == 0 {
        let half = power(base, exponent / 2);
        half * half
    } else {
        base * power(base, exponent - 1)
    }
}

// 10. Recursive Greatest Common Divisor (GCD)
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    // 1. Factorial
    println!("Factorial of 5: {}", factorial(5));

    // 2. Fibonacci
    println!("10th Fibonacci number: {}", fibonacci(10));

    // 3. Binary Search
    let arr = vec![1, 3, 5, 7, 9, 11, 13, 15];
    println!("Binary Search for 7: {:?}", binary_search(&arr, &7, 0, arr.len() - 1));

    // 4. Tower of Hanoi
    println!("Tower of Hanoi for 3 disks:");
    tower_of_hanoi(3, 'A', 'C', 'B');

    // 5. Merge Sort
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    merge_sort(&mut arr);
    println!("Merge Sort result: {:?}", arr);

    // 6. Tree Traversal
    let mut root = TreeNode::new(1);
    root.left = Some(Box::new(TreeNode::new(2)));
    root.right = Some(Box::new(TreeNode::new(3)));
    root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
    root.left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));
    println!("Inorder Traversal:");
    inorder_traversal(&Some(Box::new(root)));

    // 7. String Reversal
    println!("Reversed 'hello': {}", reverse_string("hello"));

    // 8. Sum of Digits
    println!("Sum of digits of 12345: {}", sum_of_digits(12345));

    // 9. Recursive Exponentiation
    println!("2^10: {}", power(2, 10));

    // 10. Greatest Common Divisor
    println!("GCD of 48 and 18: {}", gcd(48, 18));
}
