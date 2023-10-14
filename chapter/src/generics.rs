use std::ops::{Add, Sub};

pub fn main() {
    use_add();
    use_sub();
}

fn add<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn sub<T: Sub<Output = T>>(x: T, y: T) -> T {
    x - y
}

fn use_add() {
    let r = add::<i32>(10, 20);
    println!("use_add 10 + 20 = {}", r);
    let r = add::<f32>(10.02, 20.06);
    println!("use_add 10.02 + 20.06 = {}", r);
}

fn use_sub() {
    let r = sub::<i32>(30, 20);
    println!("use_sub 30 - 20 = {}", r);
    let r = sub::<f32>(100.05, 20.006);
    println!("use_sub 100.05 + 20.006 = {}", r);
}
