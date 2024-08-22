use std::str::FromStr;

fn main() {
    // 1. Creating Strings
    let s1 = String::from("Hello, world!");
    let s2 = "Hello, world!".to_string();
    let s3 = String::new();
    println!("s1: {}, s2: {}, s3 length: {}", s1, s2, s3.len());

    // 2. String Slices
    let slice = &s1[0..5];
    println!("Slice: {}", slice);

    // 3. Concatenation
    let s4 = s1 + &s2; // s1 is moved here and can no longer be used
    println!("Concatenated: {}", s4);

    let s5 = format!("{} {}", "Hello", "world");
    println!("Formatted: {}", s5);

    // 4. Iteration
    for c in s5.chars() {
        print!("{} ", c);
    }
    println!();

    // 5. Splitting
    let words: Vec<&str> = s5.split_whitespace().collect();
    println!("Words: {:?}", words);

    // 6. Trimming
    let s6 = "  Hello, world!  ".to_string();
    println!("Trimmed: '{}'", s6.trim());

    // 7. Changing case
    println!("Uppercase: {}", s6.to_uppercase());
    println!("Lowercase: {}", s6.to_lowercase());

    // 8. Replacing
    let s7 = s6.replace("world", "Rust");
    println!("Replaced: {}", s7);

    // 9. Checking content
    println!("Starts with 'Hello': {}", s7.starts_with("Hello"));
    println!("Ends with 'Rust!': {}", s7.ends_with("Rust!"));
    println!("Contains 'Rust': {}", s7.contains("Rust"));

    // 10. Parsing strings
    let num: i32 = "42".parse().unwrap();
    println!("Parsed number: {}", num);

    // 11. String to vector of bytes and back
    let bytes = s7.into_bytes();
    let s8 = String::from_utf8(bytes).unwrap();
    println!("Bytes to string: {}", s8);

    // 12. Working with individual Unicode scalar values
    let unicode = "é".chars().next().unwrap();
    println!("Unicode scalar value: U+{:04X}", unicode as u32);

    // 13. Finding substrings
    let haystack = "Find a needle in this haystack";
    match haystack.find("needle") {
        Some(index) => println!("Found 'needle' at index: {}", index),
        None => println!("'needle' not found"),
    }

    // 14. Reversing a string
    let reversed: String = s8.chars().rev().collect();
    println!("Reversed: {}", reversed);

    // 15. Removing prefix and suffix
    let s9 = "Hello, world!".to_string();
    println!("Without prefix: {}", s9.strip_prefix("Hello, ").unwrap_or(&s9));
    println!("Without suffix: {}", s9.strip_suffix("!").unwrap_or(&s9));

    // 16. Joining strings
    let words = vec!["Hello", "world", "of", "Rust"];
    let joined = words.join(" ");
    println!("Joined: {}", joined);

    // 17. Extracting substrings
    let substring = &s9[7..12];
    println!("Substring: {}", substring);

    // 18. Converting between String and str
    let string_slice: &str = &s9[..];
    let owned_string: String = string_slice.to_owned();
    println!("Owned string from slice: {}", owned_string);

    // 19. String interpolation (using format! macro)
    let name = "Alice";
    let age = 30;
    let interpolated = format!("My name is {} and I am {} years old", name, age);
    println!("Interpolated: {}", interpolated);

    // 20. Handling UTF-8
    let utf8_string = "こんにちは世界"; // "Hello world" in Japanese
    println!("UTF-8 string length: {}", utf8_string.len());
    println!("UTF-8 char count: {}", utf8_string.chars().count());

    // 21. Custom string parsing
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl FromStr for Point {
        type Err = std::num::ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')').split(',').collect();
            let x = coords[0].parse::<i32>()?;
            let y = coords[1].parse::<i32>()?;
            Ok(Point { x, y })
        }
    }

    let point: Point = "(10,20)".parse().unwrap();
    println!("Parsed point: {:?}", point);
}
