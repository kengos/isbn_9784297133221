pub fn main() {
    print_message();
    print_message_2(String::from("引数付き関数"));
    let mut message = String::from("7-1-3");
    print_message_3(&mut message);
    println!("{}", message);
    let s = print_message_4(String::from("戻り値付き関数"));
    println!("{}", s);
    let s = print_message_4(String::from(""));
    println!("{}", s);
}

fn print_message() {
    println!("基本的な関数定義");
}

fn print_message_2(message: String) {
    println!("{}", message);
}

fn print_message_3(message: &mut String) {
    message.push_str("ミュータブルな引数付き関数");
    println!("追加結果 = {}", message);
}

fn print_message_4(message: String) -> String {
    if message.eq("") {
        return String::from("引数が空です。");
    }

    println!("{}", message);
    String::from("引数を出力しました。")
}
