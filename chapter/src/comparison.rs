pub fn symbol(x: i32, y: i32) {
    println!("{} == {} = {}", x, y, x == y);
    println!("{} != {} = {}", x, y, x != y);
    println!("{} <= {} = {}", x, y, x <= y);
    println!("{} >= {} = {}", x, y, x >= y);
    println!("{} < {} = {}", x, y, x < y);
    println!("{} > {} = {}", x, y, x > y);
}

pub fn methods(x: i32, y: i32) {
    println!("{} eq {} = {}", x, y, x.eq(&y));
    println!("{} ne {} = {}", x, y, x.ne(&y));
    println!("{} le {} = {}", x, y, x.le(&y));
    println!("{} ge {} = {}", x, y, x.ge(&y));
    println!("{} lt {} = {}", x, y, x.lt(&y));
    println!("{} gt {} = {}", x, y, x.gt(&y));
}

pub fn use_symbol(x: i32, y: i32) {
    let mut result = (x == 10) && (y == 6);
    println!("(x == {}) && (y == {}) = {}", 10, 6, result);
    result = (x == 10) || (y == 6);
    println!("(x == {}) || (y == {}) = {}", 10, 20, result);
    result = !(x == 10);
    println!("!(x == {})= {}", 10, result);
}
