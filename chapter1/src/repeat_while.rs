pub fn main() {
    println!("----- while_1 -----");
    while_1();
    println!("----- while_2 -----");
    while_2();
}

fn while_1() {
    let mut counter = 1;
    while counter < 5 {
        if counter % 2 == 0 {
            println!("counterの値 {} は偶数です。", counter);
        } else {
            println!("counterの値 {} は奇数です。", counter);
        }
        counter += 1;
    }
}

fn while_2() {
    let x = ["ABC", "ABC", "ABC", "XYZ"];
    let mut counter = 0;
    while let "ABC" = x[counter] {
        println!("x[counter] = {}", x[counter]);
        counter += 1;
    }
}
