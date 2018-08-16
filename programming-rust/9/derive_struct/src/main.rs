#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

fn main() {
    let p = Point::new();
    println!("{} {}", p.x,p.y);
    println!("Hello, world!");
}
