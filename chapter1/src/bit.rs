pub fn use_symbol(x: i8, y: i8) {
    println!("x & y = {:b}", x & y);
    println!("x | y = {:b}", x | y);
    println!("!x = {:b}", !x);
    println!("x >> 3 = {:b}", x >> 3);
    println!("y << 3 = {:b}", y << 3);
}

pub fn use_format_specifier() {
    let x = 127;
    println!("2進数 = {:b}", x);
    println!("8進数 = {:o}", x);
    println!("16進数 = {:x}", x);
    println!("16進数 = {:X}", x);
    println!("指数 = {:e}", x);
    println!("指数 = {:E}", x);
}

pub fn use_method(x: i8, y: i8) {
    use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
    println!("x = {:08b}", x);
    println!("y = {:08b}", y);
    println!("x & y = {:08b}", x.bitand(y));
    println!("x | y = {:08b}", x.bitor(y));
    println!("x ^ y = {:08b}", x.bitxor(y));
    println!("!x = {:08b}", x.not());
    println!("x.shr(3) = {:08b}", x.shr(3));
    println!("y.shl(3) = {:08b}", x.shl(3));
}

pub fn compound_assign(mut x: i32, mut y: i32) {
    x &= y;
    println!("x &= y = {:b}", x);
    x |= y;
    println!("x |= y = {:b}", x);
    x ^= y;
    println!("x ^= y = {:b}", x);
    x >>= 3;
    println!("x >>= 3 = {:b}", x);
    y <<= 3;
    println!("y <<= 3 = {:b}", y);
}

pub fn compound_assign_method(mut x: i32, mut y: i32) {
    use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    x.bitand_assign(y);
    println!("x.bitand_assign(y) = {:b}", x);
    x.bitor_assign(y);
    println!("x.bitor_assign(y) = {:b}", x);
    x.bitxor_assign(y);
    println!("x.bitxor_assign(y) = {:b}", x);
    x.shr_assign(3);
    println!("x.shr_assign(3) = {:b}", x);
    y.shl_assign(3);
    println!("y.shl_assign(3) = {:b}", y);
}
