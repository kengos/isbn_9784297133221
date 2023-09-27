use std::collections::HashMap;

pub fn main() {
    instantiate();
    add();
    get_and_change();
    remove();
}

fn instantiate() {
    let x = HashMap::<i32, String>::new();
    let y = HashMap::<i32, i32>::with_capacity(5);
    println!("x = {:?}, x.len() = {}", x, x.len());
    println!("y = {:?}, y.len() = {}", y, y.len());
}

fn add() {
    let mut x = HashMap::<i32, &str>::new();
    x.extend([(1, "ABC"), (2, "DEF"), (10, "XYZ")]);
    println!("x = {:?}", x);
    let mut x = HashMap::<i32, &str>::new();
    x.insert(1, "あいうえお");
    x.insert(2, "かきくけこ");
    x.insert(3, "さしすせそ");
    println!("insert = {:?}", x);
    let x: HashMap<i32, &str> = vec![(1, "ABC"), (2, "DEF")].into_iter().collect();
    println!("vec! = {:?}", x);
}

fn get_and_change() {
    let mut x = HashMap::<i32, &str>::new();
    x.extend([(1, "ABC"), (2, "DEF"), (10, "XYZ")]);
    let v = x.get(&1);
    println!("get(&1) = {:?}", v);

    for (k, v) in x.iter() {
        println!("key := {}, value := {}", k, v);
    }

    if let Some(v) = x.get_mut(&2) {
        *v = "あいうえお";
    }
    println!("{:?}", x);

    let mut y: HashMap<i32, i32> = vec![(1, 10), (2, 11), (3, 12), (4, 13)]
        .into_iter()
        .collect();
    for (_, v) in y.iter_mut() {
        if *v & 2 == 0 {
            *v *= 10;
        }
    }
    println!("{:?}", y);
}

fn remove() {
    let mut x = HashMap::<i32, &str>::new();
    x.extend([(1, "ABC"), (2, "DEF"), (10, "XYZ")]);

    let v = match x.remove(&2) {
        None => String::from("指定されたキーペアが見つかりません。"),
        Some(x) => format!("値: {}が削除されました。", x),
    };
    println!("{}", v);
    println!("{:?}", x);
    x.clear();
    println!("clear = {:?}", x);
}
