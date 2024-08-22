use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};

fn main() {
    // Vector (dynamic array)
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.push(6);
    println!("Vector: {:?}", vec);

    // HashMap
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    println!("HashMap: {:?}", map);

    // HashSet
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    println!("HashSet: {:?}", set);

    // VecDeque (double-ended queue)
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_front(2);
    println!("VecDeque: {:?}", deque);

    // BinaryHeap (max-heap)
    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(4);
    println!("BinaryHeap: {:?}", heap);

    // Custom struct for binary tree
    struct TreeNode<T> {
        value: T,
        left: Option<Box<TreeNode<T>>>,
        right: Option<Box<TreeNode<T>>>,
    }

    // Binary Search
    let sorted_vec = vec![1, 3, 4, 6, 8, 9, 11];
    let target = 6;
    match sorted_vec.binary_search(&target) {
        Ok(index) => println!("Found {} at index {}", target, index),
        Err(_) => println!("{} not found", target),
    }

    // Quick Sort
    fn quick_sort<T: Ord>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        let pivot = partition(arr);
        quick_sort(&mut arr[0..pivot]);
        quick_sort(&mut arr[pivot + 1..]);
    }

    fn partition<T: Ord>(arr: &mut [T]) -> usize {
        let pivot = arr.len() - 1;
        let mut i = 0;
        for j in 0..pivot {
            if arr[j] <= arr[pivot] {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, pivot);
        i
    }

    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    quick_sort(&mut numbers);
    println!("Sorted array: {:?}", numbers);

    // Depth-First Search (DFS) on a graph
    fn dfs(graph: &HashMap<i32, Vec<i32>>, start: i32, visited: &mut HashSet<i32>) {
        if !visited.insert(start) {
            return;
        }
        println!("Visited: {}", start);
        if let Some(neighbors) = graph.get(&start) {
            for &neighbor in neighbors {
                dfs(graph, neighbor, visited);
            }
        }
    }

    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![0, 3]);
    graph.insert(3, vec![3]);

    let mut visited = HashSet::new();
    dfs(&graph, 2, &mut visited);
}
