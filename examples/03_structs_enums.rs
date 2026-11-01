// Structs, enums, and pattern matching.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rect(Rectangle),
}

fn describe(shape: &Shape) -> String {
    match shape {
        Shape::Circle(r) => format!("circle with radius {r}"),
        Shape::Rect(rect) => format!("rectangle with area {}", rect.area()),
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(2.0),
        Shape::Rect(Rectangle { width: 3, height: 4 }),
    ];
    for s in &shapes {
        println!("{}", describe(s));
    }
}
