fn main() {
    println!("Hello, world!");
    let rec1: Rectangle = Rectangle {
        width: 3f64,
        length: 4f64,
    };

    println!("Area: {} - Perimeter: {}", rec1.area(), rec1.perimeter());
}
struct Rectangle {
    width: f64,
    length: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.length
    }
    fn perimeter(&self) -> f64 {
        (self.length + self.width) * 2f64
    }
}
