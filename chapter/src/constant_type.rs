struct Customer {
    id: u32,
    name: String,
    address: String,
    email: String,
}

impl Customer {
    const ID_MIN: u32 = 1;
    const ID_MAX: u32 = 10000;

    fn new(id: u32, name: String, address: String, email: String) -> Self {
        Self {
            id,
            name,
            address,
            email,
        }
    }
}

pub fn main() {
    println!("----- use_constant -----");
    use_constant();
    println!("----- use_new -----");
    use_new();
}

fn use_constant() {
    println!("{}", Customer::ID_MIN);
    println!("{}", Customer::ID_MAX);
}

fn use_new() {
    let customer = Customer::new(
        100,
        String::from("山田太郎"),
        String::from("東京都新宿区"),
        String::from("yamada@example.com"),
    );
    println!("id = {}", customer.id);
    println!("name = {}", customer.name);
    println!("address = {}", customer.address);
    println!("email = {}", customer.email);
}
