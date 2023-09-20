pub fn main() {
    println!("----- declare -----");
    declare();
    println!("----- mut_declare -----");
    mut_declare();
    println!("----- mut_declare_2 -----");
    mut_declare_2();
}

fn declare() {
    let x: i32 = 100;
    let y: &str = "ABC";
    let x_ptr: *const i32 = &x;
    let y_ptr: *const &str = &y;
    unsafe {
        println!("x_ptr = {}", *x_ptr);
        println!("y_ptr = {}", *y_ptr);
    }
    println!("x_ptr = {:?}", x_ptr);
    println!("y_ptr = {:?}", y_ptr);
}

fn mut_declare() {
    let mut x: i32 = 100;
    let mut y: &str = "ABC";
    let x_ptr: *mut i32 = &mut x;
    let y_ptr: *mut &str = &mut y;
    unsafe {
        println!("変更前 x_ptr = {}", *x_ptr);
        println!("変更前 y_ptr = {}", *y_ptr);
        *x_ptr += 100;
        let str_val = "ポインタの利用".to_string();
        *y_ptr = &str_val;
        println!("変更後 x_ptr = {}", *x_ptr);
        println!("変更後 y_ptr = {}", *y_ptr);
    }
}

fn mut_declare_2() {
    let x = 100;
    let y = 200;
    let mut ptr: *const i32 = &x;
    unsafe {
        println!("ptrの値 = {}", *ptr);
        println!("ptrのアドレス = {:?}", ptr);
    }
    // ptrの指す先を yに変更
    ptr = &y;
    unsafe {
        println!("ptrの値 = {}", *ptr);
        println!("ptrのアドレス = {:?}", ptr);
    }
}
