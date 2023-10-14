pub fn main() {
    declare();
    use_div(10, 5);
    use_div(10, 0);
    method_1(20, 0);
    method_1(20, 4);
    method_2(30, 5);
    method_2(30, 0);
    method_3(40, 5);
    method_3(40, 0);
    method_4(50, 5);
    method_4(50, 0);
    println!("{:?}", method_5(60, 5));
    println!("{:?}", method_5(60, 0));
}

fn declare() {
    let mut a = None;
    println!("a = {:?}", a);
    a = Some(100);
    println!("a = {:?}", a);
    let mut b = Some(String::from("ABCD"));
    println!("b = {:?}", b);
    b = None;
    println!("b = {:?}", b);
}

fn div(value1: i32, value2: i32) -> Option<i32> {
    if value2 == 0 {
        return None;
    }

    let r = (value1 / value2) as i32;
    Some(r)
}

fn use_div(x: i32, y: i32) {
    let r = match div(x, y) {
        None => "割り算できません".to_owned(),
        Some(result) => format!("{} / {} = {}", x, y, result),
    };
    println!("{}", r);
}

fn method_1(x: i32, y: i32) {
    let result = div(x, y);
    let r = if result.is_some() {
        format!("{} / {} = {}", x, y, result.unwrap())
    } else {
        "割り算できません。".to_owned()
    };
    println!("{}", r);
}

fn method_2(x: i32, y: i32) {
    let r = div(x, y).unwrap_or(-1);
    println!("unwrap_or: div({}, {}) = {}", x, y, r);
    let r = div(x, y).unwrap_or_else(|| -100);
    println!("unwrap_or_else: div({}, {}) = {}", x, y, r);
    let r = div(x, y).unwrap_or_default();
    println!("unwrap_or_default: div({}, {}) = {}", x, y, r);
}

fn method_3(x: i32, y: i32) {
    let r = div(x, y).and_then(|result| Some(format!("{} / {} = {}", x, y, result)));
    println!("div({}, {}).and_then() {:?}", x, y, r);
    let r = div(x, y).map(|result| format!("{} / {} = {}", x, y, result));
    println!("div({}, {}).map() {:?}", x, y, r);
    let r = div(x, y).map_or("計算できません".to_owned(), |result| {
        format!("{} / {} = {}", x, y, result)
    });
    println!("div({}, {}).map_or() {:?}", x, y, r);
    let r = div(x, y).map_or_else(
        || "計算できません".to_owned(),
        |result| format!("{} / {} = {}", x, y, result),
    );
    println!("div({}, {}).map_or_else() {:?}", x, y, r);
}

fn method_4(x: i32, y: i32) {
    let r = match div(x, y).ok_or("計算できません。".to_owned()) {
        Ok(result) => format!("{} / {} = {}", x, y, result),
        Err(e) => e,
    };
    println!("div({}, {}).ok_or() = {:?}", x, y, r);

    let r = match div(x, y).ok_or_else(|| "計算できません。".to_owned()) {
        Ok(result) => format!("{} / {} = {}", x, y, result),
        Err(e) => e,
    };
    println!("div({}, {}).ok_or_else() = {:?}", x, y, r);
}

fn method_5(x: i32, y: i32) -> Option<String> {
    let result = div(x, y)?;
    Some(format!("div({}, {})? = {:?}", x, y, result))
}
