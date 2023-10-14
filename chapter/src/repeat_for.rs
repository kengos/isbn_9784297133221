pub fn main() {
    println!("----- for_1 -----");
    for_1();
    println!("----- for_2 -----");
    for_2();
    println!("----- for_3 -----");
    for_3();
}

fn for_1() {
    println!("0~4までループする");
    for i in 0..5 {
        print!("i = {} ", i);
    }
    println!("\n");

    println!("0~5までループする");
    for i in 0..=5 {
        print!("i = {} ", i);
    }
    println!("\n");
}

fn for_2() {
    println!("4~0までループする");
    for i in (0..5).rev() {
        print!("i = {} ", i);
    }
    println!("\n");

    println!("5~0までループする");
    for i in (0..=5).rev() {
        print!("i = {} ", i);
    }
    println!("\n");
}

fn for_3() {
    let values = vec![100, 200, 300, 400, 500];
    let mut result: i32 = 0;
    for value in values {
        result += value;
    }
    println!("result = {}", result);
}
