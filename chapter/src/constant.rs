const SAMPLE_NAME: &str = "Rust サンプルプログラム";

pub fn main() {
    use_constant();
}

fn use_constant() {
    println!("----- use_constant -----");
    const CALC_VALUE: i32 = 100;
    let result = 10 * CALC_VALUE;
    println!("10 * CALC_VALUE = {}", result);
    println!("SAMPLE_NAME = {}", SAMPLE_NAME);
}
