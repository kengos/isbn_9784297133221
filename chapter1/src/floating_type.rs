pub fn main() {
    floating_literal();
    floating_constant();
    methods();
}

fn floating_literal() {
    println!("----- floating_literal -----");
    println!("10.5f32 = {}", 10.5f32);
    println!("10.5f64 = {}", 10.5f64);
    println!("10.5_f32 = {}", 10.5_f32);
    println!("10.5_f64 = {}", 10.5_f64);
}

fn floating_constant() {
    println!("----- floating_constant -----");
    println!("f32::RADIX = {}", f32::RADIX);
    println!("f32::MANTISSA_DIGITS = {}", f32::MANTISSA_DIGITS);
    println!("f32::DIGITS = {}", f32::DIGITS);
    println!("f32::EPSILON = {}", f32::EPSILON);
    println!("f32::MIN = {}", f32::MIN);
    println!("f32::MIN_POSITIVE = {}", f32::MIN_POSITIVE);
    println!("f32::MIN_EXP = {}", f32::MIN_EXP);
    println!("f32::MIN_10_EXP = {}", f32::MIN_10_EXP);
    println!("f32::MAX = {}", f32::MAX);
    println!("f32::MAX_EXP = {}", f32::MAX_EXP);
    println!("f32::MAX_10_EXP = {}", f32::MAX_10_EXP);
    println!("f32::NAN = {}", f32::NAN);
    println!("f32::INFINITY = {}", f32::INFINITY);
    println!("f32::NEG_INFINITY = {}", f32::NEG_INFINITY);
}

fn methods() {
    println!("----- methods -----");
    let x = 10.5_f64;
    println!("平方根: 10.5_f64.cbrt() = {}", x.cbrt());
    println!("切り上げ: 10.5_f64.ceil() = {}", x.ceil());
    println!("切り捨て: 10.5_f64.floor() = {}", x.floor());
}
