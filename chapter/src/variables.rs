pub fn main() {
    declare_variables();
    declare_mut_variables();
    shadowing()
}

fn declare_variables() {
    println!("----- declare_variables -----");
    let x: i32 = 0;
    let y = 100;

    println!("{}, {}", x, y);
}

fn declare_mut_variables() {
    println!("----- declare_mut_variables -----");
    let mut x = 100;
    x = x + 100;
    println!("x = {}", x);
}

fn shadowing() {
    println!("----- shadowing -----");
    let value1: u32 = 100;
    println!("value1 = {}", value1);
    let value1: i32 = 100;
    println!("value1 = {}", value1);
    let value2: f32 = 100.1;
    println!("value2 = {}", value2);
    let value2: i32 = 100;
    println!("value2 = {}", value2);
}
