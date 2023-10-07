pub fn main() {
    value_setting();
    use_div(10, 5);
    use_div(10, 0);
    method_1(20, 5);
    method_1(20, 0);
    method_2(30, 5);
    method_2(30, 0);
    method_3(40, 5);
    method_3(40, 0);
    method_4(50, 5);
    method_4(50, 0);
    match method_5(50, 5) {
        Ok(r) => println!("Ok {}", r),
        Err(err) => println!("Err {}", err),
    }
    match method_5(50, 0) {
        Ok(r) => println!("Ok {}", r),
        Err(err) => println!("Err {}", err),
    }
}

fn value_setting() {
    let mut x: Result<i32, String>;
    x = Ok(100);
    println!("x = {:?}", x);
    x = Err(String::from("エラーです。"));
    println!("x = {:?}", x);
}

fn div(value1: i32, value2: i32) -> Result<i32, String> {
    if value2 == 0 {
        return Err("ゼロ除算です。".to_owned());
    }
    Ok(value1 / value2 as i32)
}

fn use_div(x: i32, y: i32) {
    let r = match div(x, y) {
        Ok(result) => format!("{} / {} = {}", x, y, result),
        Err(err) => err,
    };
    println!("{}", r);
}

fn method_1(x: i32, y: i32) {
    let result = div(x, y);
    let r = if result.is_ok() {
        format!("{} / {} = {}", x, y, result.unwrap())
    } else {
        result.unwrap_err()
    };
    println!("{}", r);
}

fn method_2(x: i32, y: i32) {
    let r = div(x, y).unwrap_or(-100);
    println!("unwrap_or() = {}", r);
    let r = div(x, y).unwrap_or_else(|e| {
        println!("{:?}", e);
        -100
    });
    println!("unwrap_or_else() = {}", r);
    let r = div(x, y).unwrap_or_default();
    println!("unwrap_or_default() = {}", r);
}

fn method_3(x: i32, y: i32) {
    let r = div(x, y).and_then(|result| Ok(result * 2));
    println!("and_then() = {:?}", r);
    let r = div(x, y).map(|result| result * 2);
    println!("map() = {:?}", r);
    let r = div(x, y).map_err(|e| e);
    println!("map_err() = {:?}", r);
    let r = div(x, y).map_or(-100, |r| r);
    println!("map_or() = {:?}", r);
    let r = div(x, y).map_or_else(|e| e, |r| r.to_string());
    println!("map_or_else() = {:?}", r);
    let r = div(x, y).or_else(|e| Err(e));
    println!("or_else() = {:?}", r);
}

fn method_4(x: i32, y: i32) {
    let r = div(x, y).ok();
    println!("ok() = {:?}", r);
    let r = div(x, y).err();
    println!("err() = {:?}", r);
}

fn method_5(x: i32, y: i32) -> Result<String, String> {
    let r = div(x, y)?;
    Ok(format!("{} / {} = {}", x, y, r))
}
