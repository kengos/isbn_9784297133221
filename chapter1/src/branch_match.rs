pub fn main() {
    println!("------ branch_1 -----");
    branch_1(1);
    branch_1(2);
    branch_1(3);
    println!("------ branch_2 -----");
    branch_2("山田太郎");
    branch_2("鈴木花子");
    branch_2("山田花子");
    println!("------ branch_3 -----");
    branch_3(1);
    branch_3(2);
    branch_3(3);
    branch_3(4);
    println!("----- branch_4 -----");
    branch_4(1);
    branch_4(3);
    branch_4(4);
    branch_4(6);
    branch_4(7);
    branch_4(9);
    branch_4(10);
    branch_4(11);
    branch_4(20);
    branch_4(30);
    branch_4(21);
    branch_4(25);
    branch_4(30);
    branch_4(31);
    branch_4(35);
    branch_4(36);
    println!("----- branch_5 ------");
    branch_5((0, 0));
    branch_5((0, 1));
    branch_5((1, 0));
    branch_5((1, 1));
    branch_5((2, 1));
    branch_5((1, 2));
    branch_5((2, 2));
}

fn branch_1(x: i32) {
    match x {
        1 => println!("値は1"),
        2 => println!("値は1"),
        _ => println!("値は不正です"),
    }

    match x {
        1 => {
            let y = 100;
            println!("y = {}", y);
        }
        2 => {
            let y = 200;
            println!("y = {}", y);
        }
        _ => {
            let y = 300;
            println!("y = {}", y);
        }
    }
}

fn branch_2(x: &str) {
    match x {
        "山田太郎" => println!("山田太郎"),
        "鈴木花子" => println!("鈴木花子"),
        _ => println!("誰?"),
    }
}

fn branch_3(x: u32) {
    let calc = |x: u32| x * 30;
    let result = match x {
        1 => calc(10),
        2 => calc(20),
        3 => calc(30),
        _ => calc(0),
    };
    println!("{}", result);
}

fn branch_4(x: u32) {
    let calc = |x: u32| x * 10;
    let result = match x {
        1..=3 => calc(10),
        4..=6 => calc(20),
        7..=9 => calc(30),
        10 | 20 | 30 => calc(40),
        21..=25 | 31..=35 => calc(50),
        _ => calc(0),
    };
    println!("{}", result);
}

fn branch_5(value: (i32, i32)) {
    let result = match value {
        (x, y) if x == 0 && y == 0 => "xとyは0です。",
        (x, y) if x % 2 == 0 && y % 2 == 0 => "xとyは偶数です。",
        (x, y) if x % 2 == 1 && y % 2 == 1 => "xとyは奇数です。",
        _ => "どのパターンにも一致しません。",
    };
    println!("result = {}", result);
}
