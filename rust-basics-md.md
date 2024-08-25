# Rust Basics and Application Structure

This guide covers the fundamental concepts of Rust programming and the typical structure of a Rust application.

## Basic Syntax

### Variables and Mutability

```rust
// Immutable variable (default)
let x = 5;

// Mutable variable
let mut y = 10;
y += 1;

// Constants
const MAX_POINTS: u32 = 100_000;
```

### Data Types

```rust
// Integer types
let i: i32 = 42;  // 32-bit signed integer
let u: u64 = 42;  // 64-bit unsigned integer

// Floating-point types
let f: f64 = 3.14;

// Boolean type
let b: bool = true;

// Character type
let c: char = 'z';

// String types
let s: &str = "Hello";  // String slice
let string: String = String::from("Hello");  // Owned String

// Array and Vector
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
```

### Functions

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("{}", greet("Alice"));
}
```

### Control Flow

```rust
// If-else
let number = 6;
if number % 4 == 0 {
    println!("Number is divisible by 4");
} else if number % 3 == 0 {
    println!("Number is divisible by 3");
} else {
    println!("Number is not divisible by 4 or 3");
}

// Loop
let mut counter = 0;
loop {
    counter += 1;
    if counter == 10 {
        break;
    }
}

// While loop
while counter > 0 {
    println!("{}!", counter);
    counter -= 1;
}

// For loop
for i in 0..5 {
    println!("{}!", i);
}
```

## Ownership and Borrowing

Rust's ownership system is one of its most unique and important features.

```rust
fn main() {
    // Ownership
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2, s1 is no longer valid

    // Borrowing
    let s3 = String::from("world");
    let len = calculate_length(&s3);  // s3 is borrowed

    // Mutable borrowing
    let mut s4 = String::from("hello");
    change(&mut s4);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

## Structs and Enums

```rust
// Struct
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

## Error Handling

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Using match
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // Using unwrap_or_else
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

## Application Structure

A typical Rust application might have the following structure:

```
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   └── bin/
│       └── another_executable.rs
├── tests/
│   └── integration_test.rs
└── examples/
    └── example1.rs
```

- `Cargo.toml`: Project configuration file
- `src/main.rs`: Entry point for a binary application
- `src/lib.rs`: Root module of a library crate
- `src/bin/`: Additional binary targets
- `tests/`: Integration tests
- `examples/`: Example code

### Modules

```rust
// In src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
}
```

This basic guide covers the fundamental concepts of Rust programming and the typical structure of a Rust application. It serves as a starting point for understanding Rust's syntax, key features like ownership and borrowing, and how to structure a Rust project.
