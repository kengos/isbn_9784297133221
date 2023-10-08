#[derive(Debug, Clone)]
struct Customer {
    id: u32,
    name: String,
    address: String,
    email: String,
}

impl Customer {
    fn new(id: u32, name: String, address: String, email: String) -> Self {
        Self {
            id,
            name,
            address,
            email,
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Drop for Customer {
    fn drop(&mut self) {
        println!("{} のインスタンスを破棄します。", self.name);
    }
}

impl Default for Customer {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from(""),
            address: String::from(""),
            email: String::from(""),
        }
    }
}

// impl From<&Vec<&str>> for Customer {
//     fn from(value: &Vec<&str>) -> Self {
//         Self {
//             id: value[0].parse::<u32>().unwrap(),
//             name: value[1].to_owned(),
//             address: value[2].to_owned(),
//             email: value[3].to_owned(),
//         }
//     }
// }

// From or TryFrom どちらかしかimplできない
impl TryFrom<&Vec<&str>> for Customer {
    type Error = String;

    fn try_from(value: &Vec<&str>) -> Result<Self, Self::Error> {
        let new_id = match value[0].parse::<u32>() {
            Ok(value) => value,
            Err(err) => return Err(err.to_string()),
        };
        Ok(Self {
            id: new_id,
            name: value[1].to_owned(),
            address: value[2].to_owned(),
            email: value[3].to_owned(),
        })
    }
}

pub fn main() {
    println!("----- use_method -----");
    use_method();
    println!("----- use_debug -----");
    use_debug();
    println!("----- use_clone -----");
    use_clone();
    println!("----- use_drop -----");
    use_drop();
    println!("----- use_default -----");
    use_default();
    // println!("----- use_from -----");
    // use_from();
    println!("----- use_try_from -----");
    use_try_from();
}

fn use_method() {
    let mut customer = Customer::new(
        100,
        String::from("山田太郎"),
        String::from("東京都新宿区"),
        String::from("yamada@example.com"),
    );

    customer.set_name(String::from("鈴木花子"));
    println!("{}", customer.get_name());
}

fn use_debug() {
    let customer = Customer::new(
        100,
        String::from("山田太郎"),
        String::from("東京都新宿区"),
        String::from("yamada@example.com"),
    );
    println!("{:?}", customer);
}

fn use_clone() {
    let customer = Customer::new(
        100,
        String::from("山田太郎"),
        String::from("東京都新宿区"),
        String::from("yamada@example.com"),
    );
    println!("customer の複製 = {:?}", customer.clone());
}

fn use_drop() {
    let customer1 = Customer::new(
        100,
        String::from("山田太郎"),
        String::from("東京都新宿区"),
        String::from("yamada@example.com"),
    );
    let mut customer2 = customer1.clone();
    customer2.set_name(String::from("田中次郎"));
}

fn use_default() {
    let customer = Customer::default();
    println!("default() = {:?}", customer);
}

// fn use_from() {
//     let values = vec!["100", "山田太郎", "東京都新宿区", "yamada@example.com"];
//     let customer = Customer::from(&values);
//     println!("{:?}", customer);
// }

fn use_try_from() {
    let values = vec!["ABC", "山田太郎", "東京都新宿区", "yamada@example.com"];
    let customer = Customer::try_from(&values);
    if customer.is_ok() {
        println!("try_from() = {:?}", customer.unwrap());
    } else {
        println!("try_from() = {:?}", customer.unwrap_err());
    }
}
