pub fn main() {
    instantiate();
}

fn instantiate() {
    use std::ops::Add;
    let x = Box::new(100);
    let y = Box::new(200);
    println!("x.add(*y) = {}", x.add(*y));
    println!("*x + *y = {}", *x + *y);

    let a = Box::new("ABCXYZ");
    let b = Box::new(String::from("XYZ"));
    println!("{:?} {:?} {}", a, b.as_str(), a.contains(b.as_str()));
}
