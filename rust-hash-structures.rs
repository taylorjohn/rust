use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// 1. Using Rust's standard HashMap
fn standard_hashmap_example() {
    let mut book_reviews = HashMap::new();

    // Insert some values
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );

    // Get a value
    if let Some(review) = book_reviews.get("Pride and Prejudice") {
        println!("Review for 'Pride and Prejudice': {}", review);
    }

    // Update a value
    book_reviews.entry("Pride and Prejudice".to_string())
        .and_modify(|review| *review += " Fantastic.")
        .or_insert("Decent.".to_string());

    // Remove a value
    book_reviews.remove("Adventures of Huckleberry Finn");

    // Iterate over key-value pairs
    for (book, review) in &book_reviews {
        println!("{}: {}", book, review);
    }
}

// 2. Custom Hash Function Example
#[derive(Debug, Eq)]
struct Person {
    id: u32,
    name: String,
    age: u8,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// 3. Simple Custom HashMap Implementation
const INITIAL_BUCKETS: usize = 16;

struct MyHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> MyHashMap<K, V>
where
    K: Hash + Eq,
{
    fn new() -> Self {
        MyHashMap {
            buckets: vec![Vec::new(); INITIAL_BUCKETS],
            items: 0,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let bucket = self.get_bucket(&key);
        for &mut (ref ekey, ref mut evalue) in &mut self.buckets[bucket] {
            if ekey == &key {
                return Some(std::mem::replace(evalue, value));
            }
        }
        self.buckets[bucket].push((key, value));
        self.items += 1;
        None
    }

    fn get(&self, key: &K) -> Option<&V> {
        let bucket = self.get_bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|&&(ref ekey, _)| ekey == key)
            .map(|&(_, ref v)| v)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let bucket = self.get_bucket(key);
        let bucket_vec = &mut self.buckets[bucket];
        let index = bucket_vec.iter().position(|&(ref ekey, _)| ekey == key)?;
        self.items -= 1;
        Some(bucket_vec.swap_remove(index).1)
    }

    fn get_bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }
}

fn main() {
    println!("1. Standard HashMap Example:");
    standard_hashmap_example();

    println!("\n2. Custom Hash Function Example:");
    let mut person_map = HashMap::new();
    let person1 = Person { id: 1, name: "Alice".to_string(), age: 30 };
    let person2 = Person { id: 2, name: "Bob".to_string(), age: 25 };
    person_map.insert(&person1, "Employee");
    person_map.insert(&person2, "Manager");
    println!("{:?}", person_map.get(&Person { id: 1, name: "Alice".to_string(), age: 30 }));

    println!("\n3. Custom HashMap Implementation:");
    let mut custom_map = MyHashMap::new();
    custom_map.insert("key1", "value1");
    custom_map.insert("key2", "value2");
    println!("Get key1: {:?}", custom_map.get(&"key1"));
    println!("Remove key2: {:?}", custom_map.remove(&"key2"));
    println!("Get key2 after removal: {:?}", custom_map.get(&"key2"));
}
