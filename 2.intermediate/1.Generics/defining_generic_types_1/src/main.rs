// Define the generic struct Point.
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Point { x: 20, y: 10 };
    let p2 = Point { x: 22.3, y: 3.14 };
    println!("Point1: ({}, {})", p1.x, p1.y);
    println!("Point2: ({}, {})", p2.x, p2.y);
}
