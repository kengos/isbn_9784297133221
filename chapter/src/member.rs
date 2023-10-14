struct Member<'a> {
    id: u32,
    name: &'a str,
    address: &'a str,
    email: &'a str,
}

impl<'a> Member<'a> {
    fn new(id: u32, name: &'a str, address: &'a str, email: &'a str) -> Self {
        Self {
            id,
            name,
            address,
            email,
        }
    }

    fn get_name(&self) -> &str {
        self.name.clone()
    }

    fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }
}

pub fn main() {
    println!("----- use_method -----");
    use_method();
}

fn use_method() {
    let mut member = Member::new(100, "山田太郎", "東京都新宿区", "yamada@example.com");

    member.set_name("鈴木花子");
    println!("{}", member.id);
    println!("{}", member.get_name());
    println!("{}", member.address);
    println!("{}", member.email);
}
