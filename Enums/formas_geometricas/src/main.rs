use std::f32::consts::PI;

struct Square {
    side: f32,
}

struct Circle {
    radius: f32,
}

struct Ellipse {
    major_axis: f32,
    minor_axis: f32,
}

struct Triangle {
    base: f32,
    height: f32, // isosceles
}

struct Cube {
    side: f32,
}

struct Cylinder {
    radius: f32,
    height: f32,
}

struct Sphere {
    radius: f32,
}


enum Shape {
    Square(Square),
    Circle(Circle),
    Ellipse(Ellipse),
    Triangle(Triangle),
    Cube(Cube),
    Cylinder(Cylinder),
    Sphere(Sphere),
}


impl Shape {
    fn area(&self) -> Option<f32> {
        match self {
            Shape::Square(s) => Some(s.side * s.side),
            Shape::Circle(c) => Some(PI * c.radius * c.radius),
            Shape::Ellipse(e) => Some(PI * e.major_axis * e.minor_axis),
            Shape::Triangle(t) => Some(0.5 * t.base * t.height),
            Shape::Cube(c) => Some(6.0 * c.side * c.side),
            Shape::Cylinder(c) => Some(2.0 * PI * c.radius * (c.radius + c.height)),
            Shape::Sphere(s) => Some(4.0 * PI * s.radius * s.radius),
        }
    }

    fn perimeter(&self) -> Option<f32> {
        match self {
            Shape::Square(s) => Some(4.0 * s.side),
            Shape::Circle(c) => Some(2.0 * PI * c.radius),
            Shape::Ellipse(e) => Some(PI * (e.major_axis + e.minor_axis)), // crude approximation
            Shape::Triangle(t) => {
                let half_base = 0.5 * t.base;
                let leg = (half_base * half_base + t.height * t.height).sqrt();
                Some(t.base + 2.0 * leg)
            }
            Shape::Cube(c) => Some(12.0 * c.side),
            _ => None,
        }
    }

    fn volume(&self) -> Option<f32> {
        match self {
            Shape::Cube(c) => Some(c.side.powi(3)),
            Shape::Cylinder(c) => Some(PI * c.radius.powi(2) * c.height),
            Shape::Sphere(s) => Some((4.0 / 3.0) * PI * s.radius.powi(3)),
            _ => None,
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Square(Square { side: 2.0 }),
        Shape::Circle(Circle { radius: 2.0 }),
        Shape::Ellipse(Ellipse { major_axis: 1.0, minor_axis: 2.0 }),
        Shape::Triangle(Triangle { height: 3.0, base: 2.0 }),
        Shape::Cube(Cube { side: 2.0 }),
        Shape::Cylinder(Cylinder { radius: 2.0, height: 2.0 }),
        Shape::Sphere(Sphere { radius: 2.0 }),
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape {}:", i + 1);
        println!("  Area: {:?}", shape.area());
        println!("  Perimeter: {:?}", shape.perimeter());
        println!("  Volume: {:?}", shape.volume());
        println!();
    }
}
