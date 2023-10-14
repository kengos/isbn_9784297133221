pub fn main() {
    declare();
    calc((10, 6));
    calc((20, 3));
    methods();
}

fn declare() {
    println!("------ declare -----");
    let x: (i32, &str, bool) = (100, "Hello", true);
    println!("x = {:?}", x);
    let (a, b, c) = (100, "Hello", true);
    println!("a = {:?}, b = {:?}, c = {:?}", a, b, c);
    let (l, _, m) = (100, "Hello", true);
    println!("l = {:?}, m = {:?}", l, m);
}

fn calc(value: (i32, i32)) {
    println!("----- calc -----");
    println!("{} + {} = {}", value.0, value.1, value.0 + value.1);
    println!("{} - {} = {}", value.0, value.1, value.0 - value.1);
    println!("{} * {} = {}", value.0, value.1, value.0 * value.1);
    println!("{} % {} = {}", value.0, value.1, value.0 / value.1);
    println!("{} % {} = {}", value.0, value.1, value.0 % value.1);
}

fn methods() {
    println!("----- methods -----");
    let a: (i32, i32, i32) = (100, 200, 300);
    println!("clone() = {:?}", a.clone());
    println!("eq() = {}", a.eq(&(100, 200, 300)));
    println!("eq() = {}", a.eq(&(100, 201, 300)));
    println!("cmp() = {:?}", a.cmp(&(200, 200, 300)));
    println!("max() = {:?}", a.max((99, 199, 301)));
    println!("min() = {:?}", a.min((99, 199, 300)));
}
