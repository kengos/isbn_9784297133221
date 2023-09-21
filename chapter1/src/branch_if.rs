pub fn main() {
    println!("----- branch_1 -----");
    branch_1();
    println!("----- branch_2 -----");
    branch_2(1);
    branch_2(2);
    branch_2(3);
    println!("----- branch_3 -----");
    branch_3(1);
    branch_3(2);
    branch_3(3);
}

fn branch_1() {
    let name = "山田太郎";
    if name.eq("山田太郎") {
        println!("変数nameの値は「山田太郎」です。");
    }

    let num = 120;
    if num % 2 == 0 {
        println!("変数numの値は偶数です。");
    } else {
        println!("変数numの値は奇数です。");
    }
}

fn branch_2(num: i32) {
    if num == 1 {
        println!("変数numの値は1です。");
    } else if num == 2 {
        println!("変数numの値は2です。");
    } else {
        println!("変数numの値は1でも2でもありません。");
    }
}

fn branch_3(num: i32) {
    let result = if num == 1 {
        "変数numの値は1です。"
    } else if num == 2 {
        "変数numの値は2です。"
    } else {
        "変数numの値は1でも2でもありません。"
    };
    println!("{}", result);
}
