use std::marker::Copy;
use std::ops::Add;

#[derive(Debug, Clone)]
struct Customer<T> {
    id: T,
}

impl<T> Customer<T>
where
    T: Copy + Add,
{
    fn new(id: T) -> Self {
        Self { id }
    }

    fn change_id(&mut self, value: T) {
        self.id = value;
    }
}

pub fn main() {
    // the function or associated item `new` exists for struct `Customer<String>`, but its trait bounds were not satisfied
    // function or associated item cannot be called on `Customer<String>` due to unsatisfied trait boundsrustcClick for full compiler diagnostic
    // border.rs(5, 1): function or associated item `new` not found for this struct
    // string.rs(365, 1): doesn't satisfy `String: Add`
    // string.rs(365, 1): doesn't satisfy `String: Copy`
    // let customer = Customer::<String>::new("a".to_owned());

    let mut customer = Customer::<f32>::new(3.5);
    customer.change_id(5.5);
    println!("{:?}", customer);
}
