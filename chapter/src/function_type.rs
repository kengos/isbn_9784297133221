pub fn main() {
    use_function_1();
    use_function_2();
    use_function_3();
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn use_function_1() {
    let func = add;
    let r = func(10, 20);
    println!("x + y = {}", r);
}

type Calc = fn(i32, i32) -> i32;

fn use_calc_type(func: Calc, x: i32, y: i32) -> i32 {
    func(x, y)
}

fn use_function_2() {
    let calc: Calc = add;
    let r = use_calc_type(calc, 10, 20);
    println!("x + y = {}", r);
}

fn return_calc_type() -> Calc {
    add
}

fn use_function_3() {
    let calc = return_calc_type();
    let r = calc(10, 20);
    println!("10 + 20 = {}", r);
}
