trait Area {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        let (a, b, c) = (self.a, self.b, self.c);
        let p = (a + b + c) / 2.;
        (p * (p - a) * (p - b) * (p - c)).sqrt()
    }
}

enum Shape {
    Rec(Rectangle),
    Cir(Circle),
    Tri(Triangle),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rec(r) => r.area(),
            Shape::Cir(c) => c.area(),
            Shape::Tri(t) => t.area(),
        }
    }
}

fn main() {
    let triangle = Triangle { a: 4., b: 2., c: 3. };
    let shape = Shape::Tri(triangle);
    println!("{}", shape.area());

    let rectangle = Rectangle { width: 3., height: 5. };
    let shape = Shape::Rec(rectangle);
    println!("{}", shape.area());

    let circle = Circle { radius: 9. };
    let shape = Shape::Cir(circle);

    println!("{}", shape.area());
}
