struct Customer {
    name: String,
}

impl Customer {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

pub fn main() {
    println!("----- use_method -----");
    use_method();
}

fn use_method() {
    let mut customer = Customer::new(String::from("山田太郎"));

    customer.set_name(String::from("鈴木花子"));
    println!("{}", customer.get_name());
}
