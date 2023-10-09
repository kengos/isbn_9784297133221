#[derive(Debug, Clone)]
struct Customer<T> {
    id: T,
}

impl<T> Customer<T> {
    fn new(id: T) -> Self {
        Self { id }
    }

    fn change_id(&mut self, value: T) {
        self.id = value;
    }
}

pub fn main() {
    println!("----- use_new() -----");
    use_new();
    println!("----- change_id() -----");
    change_id();
}

fn use_new() {
    let customer = Customer::<u64>::new(100);
    println!("{:?}", customer);
}

fn change_id() {
    let mut customer = Customer::<u64>::new(100);
    customer.change_id(200);
    println!("{:?}", customer);
}
