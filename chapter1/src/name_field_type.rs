struct Consumer {
    id: u32,
    name: String,
    address: String,
    email: String,
}

struct Member<'a> {
    id: u32,
    name: &'a str,
    address: &'a str,
    email: &'a str,
}

pub fn main() {
    println!("----- generate_1 -----");
    generate_1();
    println!("----- generate_2 -----");
    generate_2();
}

fn generate_1() {
    println!("====== Consumer");
    let consumer = Consumer {
        id: 100,
        name: String::from("山田太郎"),
        address: String::from("東京都新宿区"),
        email: String::from("yamada@example.com"),
    };
    println!("id = {}", consumer.id);
    println!("name = {}", consumer.name);
    println!("address = {}", consumer.address);
    println!("email = {}", consumer.email);
}

fn generate_2() {
    println!("====== Member");
    let member = Member {
        id: 100,
        name: "山田太郎",
        address: "東京都新宿区",
        email: "yamada@example.com",
    };
    println!("id = {}", member.id);
    println!("name = {}", member.name);
    println!("address = {}", member.address);
    println!("email = {}", member.email);
}
