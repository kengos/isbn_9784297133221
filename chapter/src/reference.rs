pub fn main() {
    reference_2();
}

// イミュータブルな値からミュータブルな参照の取得はできない
// fn reference_1() {
//     let x = Vec::<i32>::new();
//     // error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
//     // 7 |     let y = &mut x;
//     //   |             ^^^^^^ cannot borrow as mutable
//     let y = &mut x;
//     y.push(1);
//     y.push(2);
//     y.push(3);
//     println!("{:?}", y);
// }

fn reference_2() {
    let mut x = Vec::<i32>::new();
    let y = &mut x;
    y.push(1);
    y.push(2);
    y.push(3);
    println!("{:?}", x);
}

// ミュータブルな参照は一つだけ
// fn reference_3() {
//     let mut x = Vec::<i32>::new();
//     let y = &mut x;
//  error[E0499]: cannot borrow `x` as mutable more than once at a time
//     --> src/reference.rs:30:13
//      |
//   29 |     let y = &mut x;
//      |             ------ first mutable borrow occurs here
//   30 |     let z = &mut x;
//      |             ^^^^^^ second mutable borrow occurs here
//   31 |     y.push(100);
//      |     ----------- first borrow later used here
//     let z = &mut x;
//     y.push(100);
//     z.push(200);
//     println!("{:?}", x);
// }

// 参照の混在はできない
// error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
//   --> src/reference.rs:49:13
//    |
// 48 |     let y = &mut x;
//    |             ------ mutable borrow occurs here
// 49 |     let z = &x;
//    |             ^^ immutable borrow occurs here
// 50 |     y.push(100);
//    |     ----------- mutable borrow later used here
// fn reference_4() {
//     let mut x = Vec::<i32>::new();
//     let y = &mut x;
//     let z = &x;
//     y.push(100);
//     println!("{:?}", x);
// }
