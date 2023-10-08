pub fn main() {
    println!("----- generate_tuple -----");
    generate_tuple();
    println!("----- generate_unit -----");
    generate_unit();
}

struct Coordinates(usize, usize);

fn generate_tuple() {
    let c = Coordinates(100, 200);
    println!("{}, {}", c.0, c.1);
}

use std::borrow::Borrow;

struct OneState;

fn generate_unit() {
    let o = OneState;
    println!("{:p}", o.borrow());
}
