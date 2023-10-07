pub fn main() {
    println!("----- closure_sum() -----");
    closure_sum();
    println!("----- move() -----");
    move_1();
    println!("----- move_2() -----");
    move_2();
    println!("----- use_impl_where() -----");
    use_impl_where();
    println!("----- use_impl_where_2() -----");
    use_impl_where_2();
}

fn closure_sum() {
    let sum = |values: &Vec<i32>| {
        let mut sum = 0;
        for value in values.iter() {
            sum += value;
        }
        sum
    };
    let values = vec![1, 2, 3, 4, 5];
    println!("{}", sum(&values));
}

fn move_1() {
    let values = vec![1, 2, 3, 4, 5];
    let sum = || {
        let mut sum = 0;
        for value in values.iter() {
            sum += value;
        }
        sum
    };
    println!("sum = {}", sum());
    println!("values = {:?}", values);
}

fn move_2() {
    let values = vec![1, 2, 3, 4, 5];
    let sum = move || {
        let mut sum = 0;
        for value in values.iter() {
            sum += value;
        }
        sum
    };
    println!("sum = {}", sum());
    // borrow of moved value: `values`
    // println!("values = {:?}", values);
}

use std::ops::Fn;

fn impl_1(values: Vec<i32>) -> impl Fn() -> i32 {
    move || {
        let mut sum = 0;
        for value in values.iter() {
            sum += value;
        }
        sum
    }
}

fn where_1<F>(f: F)
where
    F: Fn() -> i32,
{
    let sum = f();
    println!("sum = {}", sum);
}

fn use_impl_where() {
    let values = vec![1, 2, 3, 4, 5];
    let f = impl_1(values);
    where_1(f);
}

fn impl_2() -> impl Fn(Vec<i32>) -> i32 {
    move |values: Vec<i32>| {
        let mut sum = 0;
        for value in values.iter() {
            sum += value;
        }
        sum
    }
}

fn where_2<F>(f: F, values: Vec<i32>)
where
    F: Fn(Vec<i32>) -> i32,
{
    let sum = f(values);
    println!("sum = {}", sum);
}

fn use_impl_where_2() {
    let f = impl_2();
    let values = vec![1, 2, 3, 4, 5];
    where_2(f, values);
}
