use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height * 0.5
    }
}

fn print_area(shape: &dyn Shape) {
    println!("The area of the shape is: {:.2}", shape.area());
}

fn main() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 5.0,
    };
    let triangle = Triangle {
        base: 6.0,
        height: 7.0,
    };

    print_area(&circle); // Area: 28.27
    print_area(&rectangle); // Area: 20.0
    print_area(&triangle); // Area: 21.0
}
