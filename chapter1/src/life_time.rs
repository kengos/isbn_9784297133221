pub fn main() {
    life_time_1();
    life_time_2();
    life_time_4();
    life_time_5();
}

fn life_time_1() {
    let x = vec![1, 2, 3];
    let a = String::from("ABC");
    let y = &x;
    let b = &a;
    println!("y = {:?}", y);
    println!("b = {:?}", b);
    println!("b = {:?}", b);
    println!("プログラム終了");
}

fn life_time_2() {
    let a = String::from("ABC");
    let b = &a;
    let c = b;
    println!("c = {:?}", c);
    println!("プログラム終了");
}

// error[E0106]: missing lifetime specifier
//   --> src/life_time.rs:26:21
//    |
// 26 | fn life_time_3() -> &String {
//    |                     ^ expected named lifetime parameter
//    |
//    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// help: consider using the `'static` lifetime
//    |
// 26 | fn life_time_3() -> &'static String {
//    |                      +++++++
// fn life_time_3() -> &String {
//     let x = String::from("ABC");
//     &x;
// }

// 7.6.4 これはできない
// error[E0106]: missing lifetime specifier
//   --> src/life_time.rs:42:49
//    |
// 42 | fn compare(value1: &String, value2: &String) -> &String {
//    |                    -------          -------     ^ expected named lifetime parameter
//    |
//    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `value1` or `value2`
// help: consider introducing a named lifetime parameter
//    |
// 42 | fn compare<'a>(value1: &'a String, value2: &'a String) -> &'a String {
//    |           ++++          ++                  ++             ++
// fn compare(value1: &String, value2: &String) -> &String {
//     if value1.len() > value2.len() {
//         value1
//     } else {
//         value2
//     }
// }

fn compare<'a>(value1: &'a String, value2: &'a String) -> &'a String {
    if value1.len() > value2.len() {
        value1
    } else {
        value2
    }
}

fn life_time_4() {
    let a = String::from("ABC");
    let b = String::from("DEF");
    let r = compare(&a, &b);
    println!("r = {:?}", r);
}

fn life_time_5() {
    let r;
    let a = String::from("ABC");
    {
        let b = String::from("DEF");
        r = compare(&a, &b);
        println!("r = {:?}", r);
    }
    // この位置だとコンパイルエラー
    // println!("r = {:?}", r);
}
