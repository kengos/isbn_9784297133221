pub fn main() {
    integer_literal();
    byte_literal();
    i32_constant();
    methods();
}

fn integer_literal() {
    println!("----- integer_literal -----");
    println!("10進数形式の整数リテラル,      100i64, {}", 100i64);
    println!(" 8進数形式の整数リテラル,    0o123u64, {}", 0o123u64);
    println!("16進数形式の整数リテラル,   0xf124i32, {}", 0xf124i32);
    println!(" 8進数形式の整数リテラル,   0o123_u64, {}", 0o123_u64);
    println!("16進数形式の整数リテラル,  0x_f123i32, {}", 0x_f123i32);
    println!("3桁単位でのアンダースコア, 123_456_789, {}", 123_456_789);
}

fn byte_literal() {
    println!("----- byte_literal -----");
    println!("x01 = {}", b'\x01');
    println!("x0A = {}", b'\x0A');
    println!("x1D = {}", b'\x1D');
}

fn i32_constant() {
    println!("----- i32_constant -----");
    println!("i32::BITS = {}", i32::BITS);
    println!("i32::MIN = {}", i32::MIN);
    println!("i32::MAX = {}", i32::MAX);
}

fn methods() {
    println!("----- methods -----");
    println!("絶対値: abs() = {}", -10_i32.abs());
    println!(
        "符号: -10.signum() = {}, 10.signum() = {}",
        -10_i32.signum(),
        10_i32.signum()
    );
    println!(
        "累乗: 10.pow(3) = {}, 10.pow(4) = {}",
        10_i32.pow(3),
        10_i32.pow(4)
    );
    println!(
        "-1.is_negative() = {}, 0.is_negative() = {}, 1.is_negative() = {}",
        (-1_i32).is_negative(),
        0_i32.is_negative(),
        1_i32.is_negative()
    );
    println!(
        "-1.is_positive() = {}, 0.is_positive() = {}, 1.is_positive() = {}",
        (-1_i32).is_positive(),
        0_i32.is_positive(),
        1_i32.is_positive()
    );
    let x = 1_000;
    let y = x.clone();
    println!("x = {}, x.clone() = {}", x, y);
    println!("y.to_string() = {}", y.to_string());
}
