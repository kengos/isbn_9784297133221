pub fn main() {
    println!("----- ownership_1 -----");
    ownership_1();
    println!("----- ownership_2 -----");
    ownership_2();
    println!("----- ownership_3 -----");
    ownership_3();
    println!("----- ownership_5 -----");
    ownership_5();
    println!("----- ownership_6 -----");
    ownership_6();
    println!("----- ownership_7 -----");
    ownership_7();
}

// 6 |     let x = String::from("ABC");
//   |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
// 7 |     println!("x = {:?}", x);
// 8 |     let y = x;
//   |             - value moved here
// 9 |     println!("x = {:?}", x);
//   |                          ^ value borrowed here after move
fn ownership_1() {
    let x = String::from("ABC");
    println!("x = {:?}", x);
    let y = x;
    // println!("x = {:?}", x);
    println!("y = {:?}", y);
}

fn ownership_2() {
    let x = String::from("ABC");
    println!("x = {:?}", x);
    // xの所有権を移動させないため、参照を渡す
    let y = &x;
    println!("x = {:?}, {:p}", x, &x);
    println!("y = {:?}, {:p}", y, y);
}

fn ownership_3() {
    let x = String::from("ABC");
    println!("x = {:?}", x);
    // xの所有権を移動させないため、クローンで複製する
    let y = x.clone();
    // pointerを表示
    println!("x = {:p}", &x);
    println!("y = {:p}", &y);
}

fn print_message(message: &String) {
    println!("message = {:?}", message);
}

// 49 |     let x = String::from("ABC");
//    |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
// 50 |     print_message(x);
//    |                   - value moved here
// 51 |     println!("x = {:?}", x);
//    |                          ^ value borrowed here after move
// fn ownership_4() {
//     let x = String::from("ABC");
//     print_message(x);
//     println!("x = {:?}", x);
// }
fn ownership_5() {
    let x = String::from("ABC");
    // 参照を渡すことで所有権がmethodに移行することを防げる
    print_message(&x);
    println!("x = {:?}", x);
}

fn message() -> String {
    let r = String::from("good morning");
    r
}

fn ownership_6() {
    let x = message();
    println!("x = {:?}", x);
}

fn ownership_7() {
    {
        let x = String::from("ABC");
        println!("x = {:?}", x);
    }
    //     --> src/ownership.rs:85:14
    //     |
    //  85 |     let y = &x;
    //     |              ^ not found in this scope
    // let y = &x;
    // println!("y = {:?}", y);
}
