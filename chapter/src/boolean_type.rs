pub fn main() {
    println!("is_adult(18) = {}", is_adult(18));
    println!("is_adult(19) = {}", is_adult(19));
    println!("is_adult(20) = {}", is_adult(20));
    println!("methods(18) = {}", methods(18));
    println!("methods(19) = {}", methods(19));
    println!("methods(20) = {}", methods(20));
}

fn is_adult(age: i32) -> bool {
    if age > 19 {
        true
    } else {
        false
    }
}

fn methods(age: i32) -> String {
    let result: Option<String> = true.then(|| {
        if age > 19 {
            format!("{}歳は成人です", age)
        } else {
            format!("{}歳は未成年です", age)
        }
    });
    result.unwrap()
}
