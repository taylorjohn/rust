// Basic enum
enum Color {
    Red,
    Green,
    Blue,
}

// Enum with associated values
enum Shape {
    Circle(f64),             // radius
    Rectangle(f64, f64),     // width, height
    Triangle(f64, f64, f64), // side1, side2, side3
}

// Enum with struct-like variants
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // Using a basic enum
    let color = Color::Green;

    // Pattern matching on enum
    match color {
        Color::Red => println!("The color is red!"),
        Color::Green => println!("The color is green!"),
        Color::Blue => println!("The color is blue!"),
    }

    // Using an enum with associated values
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 6.0);

    // Function to calculate area using pattern matching
    fn area(shape: Shape) -> f64 {
        match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    println!("Circle area: {}", area(circle));
    println!("Rectangle area: {}", area(rectangle));

    // Using an enum with struct-like variants
    let message = Message::Move { x: 3, y: 4 };

    match message {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
    }

    // Option enum
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    match some_number {
        Some(i) => println!("Got a number: {}", i),
        None => println!("No number"),
    }

    // Result enum
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err("Cannot divide by zero".to_string())
        } else {
            Ok(numerator / denominator)
        }
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("Division result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
