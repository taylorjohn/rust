# Convex Hull (Graham Scan Algorithm)

The Graham Scan algorithm computes the convex hull of a set of points in a plane. The convex hull is the smallest convex polygon that contains all the points.

## Implementation

```rust
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn orientation(p: Point, q: Point, r: Point) -> i32 {
    (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y)
}

fn graham_scan(mut points: Vec<Point>) -> Vec<Point> {
    if points.len() < 3 {
        return points;
    }

    // Find the point with the lowest y-coordinate (and leftmost if tie)
    let mut lowest = 0;
    for i in 1..points.len() {
        if points[i].y < points[lowest].y || 
           (points[i].y == points[lowest].y && points[i].x < points[lowest].x) {
            lowest = i;
        }
    }
    points.swap(0, lowest);

    // Sort points by polar angle with respect to the lowest point
    let p0 = points[0];
    points[1..].sort_by(|a, b| {
        let order = orientation(p0, *a, *b);
        if order == 0 {
            // If collinear, sort by distance from p0
            let dist_a = (a.x - p0.x).pow(2) + (a.y - p0.y).pow(2);
            let dist_b = (b.x - p0.x).pow(2) + (b.y - p0.y).pow(2);
            dist_a.cmp(&dist_b)
        } else {
            order.cmp(&0).reverse()
        }
    });

    // Build the convex hull
    let mut stack = Vec::new();
    stack.push(points[0]);
    stack.push(points[1]);

    for i in 2..points.len() {
        while stack.len() > 1 && orientation(stack[stack.len() - 2], *stack.last().unwrap(), points[i]) <= 0 {
            stack.pop();
        }
        stack.push(points[i]);
    }

    stack
}

// Usage
fn main() {
    let points = vec![
        Point::new(0, 3),
        Point::new(2, 2),
        Point::new(1, 1),
        Point::new(2, 1),
        Point::new(3, 0),
        Point::new(0, 0),
        Point::new(3, 3),
    ];

    let hull = graham_scan(points);

    println!("Convex Hull Points:");
    for point in hull {
        println!("({}, {})", point.x, point.y);
    }
}
```

## Key Concepts

1. **Polar Angle Sorting**: Points are sorted based on their polar angle with respect to the lowest point.
2. **Orientation Test**: Used to determine whether to make a left turn or right turn.
3. **Stack-based Approach**: The algorithm maintains a stack of points that form the convex hull.
4. **Backtracking**: Points are removed from the stack if they create a non-left turn.

## When to Use

Use the Graham Scan Algorithm for Convex Hull when:

- You need to find the smallest convex polygon that encloses a set of points in a 2D plane.
- Working on computational geometry problems.
- Implementing shape analysis or object detection in computer vision.
- Solving problems related to collision detection in graphics or game development.

Graham Scan is particularly useful in:

- Geographic Information Systems (GIS) for boundary detection.
- Computer graphics for creating bounding volumes.
- Pattern recognition and image processing.
- Robotics for path planning and obstacle avoidance.

The main advantage of Graham Scan is its efficiency, with a time complexity of O(n log n), where n is the number of points. It's also relatively simple to implement compared to some other convex hull algorithms.
