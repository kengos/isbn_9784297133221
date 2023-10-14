use std::ops::{Add, Div, Mul, Rem, Sub};
pub fn symbol() {
    println!("10 + 20 = {}", 10 + 20);
    println!("20 - 10 = {}", 20 - 10);
    println!("20 * 10 = {}", 20 * 10);
    println!("10 / 5 = {}", 10 / 5);
    println!("10 % 3 = {}", 10 % 3);
}

pub fn methods(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x.add(y));
    println!("{} - {} = {}", x, y, x.sub(y));
    println!("{} * {} = {}", x, y, x.mul(y));
    println!("{} / {} = {}", x, y, x.div(y));
    println!("{} % {} = {}", x, y, x.rem(y));
}

pub fn overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let result = x + y;
    println!("{} + {} = {}", x, y, result);
}

pub fn ignore_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let result: u8 = x.wrapping_add(y);
    println!("{} + {} = {}", x, y, result);
}

pub fn check_option_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    match x.checked_add(y) {
        Some(result) => println!("{} + {} = {}", x, y, result),
        None => println!("オーバーフローしました"),
    }
}

pub fn check_boolean_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let (result, overflow) = x.overflowing_add(y);
    if overflow {
        println!("オーバーフローしました");
    } else {
        println!("{} + {} = {}", x, y, result);
    }
}

pub fn return_max_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let result = x.saturating_add(y);
    if result == u8::MAX {
        println!("オーバーフローしました");
    } else {
        println!("{} + {} = {}", x, y, result);
    }
}
