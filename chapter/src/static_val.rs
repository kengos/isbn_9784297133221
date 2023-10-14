static TAX_RATE: f32 = 0.10;

pub fn main() {
    println!("------ calc_amount -----");
    println!("1000 * 1.10 = {}", calc_amount(1000));
    println!("235 * 1.10 = {}", calc_amount(235));
    println!("234 * 1.10 = {}", calc_amount(234));
    println!("220 * 1.10 = {}", calc_amount(220));
    println!("------ calc_total -----");
    for i in 1..5 {
        calc_total(i);
    }
}

fn calc_amount(price: i32) -> i32 {
    let fprice = price as f32;
    let result = fprice + fprice * TAX_RATE;
    result as i32
}

static mut TOTAL_VALUE: i32 = 0;

fn calc_total(value: i32) {
    // error[E0133]: use of mutable static is unsafe and requires unsafe function or block
    // TOTAL_VALUE += value;
    // println!("TOTAL_VALUE = {}", TOTAL_VALUE);
    unsafe {
        TOTAL_VALUE += value;
        println!("TOTAL_VALUE = {}", TOTAL_VALUE);
    }
}
