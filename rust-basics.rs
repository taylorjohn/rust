// Functions
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    // Variables
    let x = 5; // Immutable by default
    let mut y = 10; // Mutable variable
    y += 1;

    // Constants
    const MAX_POINTS: u32 = 100_000;

    // Loops
    // For loop
    for i in 0..5 {
        println!("For loop iteration: {}", i);
    }

    // While loop
    let mut counter = 0;
    while counter < 5 {
        println!("While loop iteration: {}", counter);
        counter += 1;
    }

    // Loop with break and continue
    let mut sum = 0;
    'outer: loop {
        let mut x = 0;
        loop {
            if x > 10 {
                break 'outer;
            }
            if x % 2 == 0 {
                x += 1;
                continue;
            }
            sum += x;
            x += 1;
        }
    }

    // Conditional statements
    let number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else {
        println!("Number is not divisible by 4 or 3");
    }

    // Match expression
    let dice_roll = 4;
    match dice_roll {
        3 => println!("You rolled a 3"),
        7 => println!("You rolled a 7"),
        _ => println!("You rolled something else"),
    }

    // Structs
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Enums
    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Green;

    // Using the greet function
    println!("{}", greet("Rust"));
}
