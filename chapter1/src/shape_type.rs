pub fn main() {
    area();
    to_string();
}

enum Shape {
    Rectangle {
        height: f64,
        width: f64,
    },
    Triangle {
        height: f64,
        bottom: f64,
    },
    Circle {
        radius: f64,
    },
    Trapezium {
        upper: f64,
        bottom: f64,
        height: f64,
    },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Self::Rectangle { height, width } => height * width,
            Self::Triangle { height, bottom } => height * bottom / 2.0,
            Self::Circle { radius } => radius * radius * std::f64::consts::PI,
            Self::Trapezium {
                upper,
                bottom,
                height,
            } => (upper + bottom) * height / 2.0,
        }
    }
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        match self {
            Self::Rectangle { .. } => "四角形です。",
            Self::Triangle { .. } => "三角形です。",
            Self::Circle { .. } => "円です。",
            Self::Trapezium { .. } => "台形です。",
        }
        .to_string()
    }
}

fn area() {
    let rectangle = Shape::Rectangle {
        height: 10.0,
        width: 5.5,
    };
    let triangle = Shape::Triangle {
        height: 10.0,
        bottom: 5.0,
    };
    let circle = Shape::Circle { radius: 3.5 };
    let trapezium = Shape::Trapezium {
        upper: 5.0,
        bottom: 3.0,
        height: 6.0,
    };
    println!("{}", rectangle.area());
    println!("{}", triangle.area());
    println!("{}", circle.area());
    println!("{}", trapezium.area());
}

fn to_string() {
    let rectangle = Shape::Rectangle {
        height: 10.0,
        width: 5.5,
    };
    let triangle = Shape::Triangle {
        height: 10.0,
        bottom: 5.0,
    };
    let circle = Shape::Circle { radius: 3.5 };
    let trapezium = Shape::Trapezium {
        upper: 5.0,
        bottom: 3.0,
        height: 6.0,
    };
    println!("{}", rectangle.to_string());
    println!("{}", triangle.to_string());
    println!("{}", circle.to_string());
    println!("{}", trapezium.to_string());
}
