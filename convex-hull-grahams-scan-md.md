# Convex Hull using Graham's Scan

The Convex Hull of a set of points is the smallest convex polygon that contains all the points. Graham's Scan is an efficient algorithm for computing the convex hull of a set of points in a plane.

## Implementation

```rust
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

fn orientation(p: Point, q: Point, r: Point) -> f64 {
    (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y)
}

fn distance_squared(p: Point, q: Point) -> f64 {
    let dx = p.x - q.x;
    let dy = p.y - q.y;
    dx * dx + dy * dy
}

fn graham_scan(mut points: Vec<Point>) -> Vec<Point> {
    if points.len() <= 3 {
        return points;
    }

    // Find the point with the lowest y-coordinate (and leftmost if tie)
    let mut lowest = 0;
    for i in 1..points.len() {
        if points[i].y < points[lowest].y || (points[i].y == points[lowest].y && points[i].x < points[lowest].x) {
            lowest = i;
        }
    }
    points.swap(0, lowest);

    // Sort points by polar angle with respect to the lowest point
    let p0 = points[0];
    points[1..].sort_by(|a, b| {
        let o = orientation(p0, *a, *b);
        if o == 0.0 {
            distance_squared(p0, *b).partial_cmp(&distance_squared(p0, *a)).unwrap()
        } else {
            o.partial_cmp(&0.0).unwrap().reverse()
        }
    });

    // Build the convex hull
    let mut stack = Vec::new();
    for point in points {
        while stack.len() > 1 && orientation(*stack.get(stack.len() - 2).unwrap(), *stack.last().unwrap(), point) <= 0.0 {
            stack.pop();
        }
        stack.push(point);
    }

    stack
}

fn main() {
    let points = vec![
        Point::new(0.0, 3.0),
        Point::new(2.0, 2.0),
        Point::new(1.0, 1.0),
        Point::new(2.0, 1.0),
        Point::new(3.0, 0.0),
        Point::new(0.0, 0.0),
        Point::new(3.0, 3.0),
    ];

    let hull = graham_scan(points);

    println!("Convex Hull Points:");
    for point in hull {
        println!("({:.1}, {:.1})", point.x, point.y);
    }
}
```

## Key Concepts

1. **Convex Hull**: The smallest convex polygon that encloses all points in a set.
2. **Graham's Scan**: An efficient algorithm for finding the convex hull.
3. **Polar Angle Sorting**: Points are sorted based on their polar angle with respect to the lowest point.
4. **Orientation Test**: Used to determine whether to make a left turn or right turn.
5. **Stack-based Approach**: The algorithm maintains a stack of points that form the convex hull.

## When to Use

The Convex Hull algorithm is useful in various scenarios:

1. **Collision Detection**: In computer graphics and game development.
2. **Shape Analysis**: In image processing and computer vision.
3. **Nearest Neighbor Search**: As a preprocessing step to speed up nearest neighbor queries.
4. **Pattern Recognition**: For identifying shapes or patterns in point sets.
5. **Geographical Information Systems (GIS)**: For analyzing geographical data.
6. **Robot Motion Planning**: For determining safe paths for robot movement.

## Time Complexity

Graham's Scan algorithm has a time complexity of O(n log n), where n is the number of points. This is due to the sorting step, as the actual scan takes linear time.

## Space Complexity

The space complexity is O(n), as we need to store all points and the stack for the convex hull.

## Advantages and Limitations

Advantages:
- Efficient for large sets of points
- Guarantees to find the correct convex hull
- Relatively simple to implement compared to some other geometric algorithms

Limitations:
- Requires careful handling of floating-point arithmetic to avoid precision issues
- Not suitable for higher-dimensional spaces (3D or more)
- Doesn't handle collinear points on the hull boundary well without modifications

Graham's Scan is one of the most popular and efficient algorithms for computing the convex hull of a set of points in 2D space. It's widely used in computational geometry and has applications in various fields where geometric computations are required.
