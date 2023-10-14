use std::collections::HashSet;

pub fn main() {
    instantiate();
    add_and_remove();
    get();
    set_operation();
}

fn instantiate() {
    let a = HashSet::<i32>::new();
    let b = HashSet::<i32>::with_capacity(5);
    let c: HashSet<i32> = vec![10, 20, 30].into_iter().collect();
    println!("a = {:?}, len() = {}", a, a.len());
    println!("b = {:?}, len() = {}", b, b.len());
    println!("c = {:?}, len() = {}", c, c.len());
}

fn add_and_remove() {
    let mut a: HashSet<i32> = vec![10, 20, 30].into_iter().collect();
    a.extend([50, 60, 70]);
    println!("extend() = {:?}", a);
    let x = a.insert(100);
    if x {
        println!("insert() = {:?}", a);
    } else {
        println!("要素を追加できませんでした。");
    }

    let x = a.remove(&100);
    if x {
        println!("remove() = {:?}", a);
    } else {
        println!("要素を削除できませんでした。");
    }

    a.retain(|&v| v % 4 == 0);
    println!("retain() = {:?}", a);
}

fn get() {
    let a: HashSet<i32> = vec![10, 20, 30].into_iter().collect::<HashSet<i32>>();
    let r = match a.get(&10) {
        None => String::from("要素は存在しません。"),
        Some(x) => format!("要素{}を取得しました。", x),
    };
    println!("{}", r);
    for v in a.iter() {
        println!("{}", v);
    }
}

fn set_operation() {
    let a = vec![10, 20, 30, 50, 60]
        .into_iter()
        .collect::<HashSet<i32>>();
    let b = vec![30, 40, 50, 70, 80]
        .into_iter()
        .collect::<HashSet<i32>>();
    println!("a = {:?}", a);
    println!("a = {:?}", b);
    let x = a.union(&b);
    println!("a.union(&b) = {:?}", x);
    let x = a.intersection(&b);
    println!("a.intersection(&b) = {:?}", x);
    let x = a.difference(&b);
    println!("a.difference(&b) = {:?}", x);
    let x = a.symmetric_difference(&b);
    println!("a.symmetric_difference(&b) = {:?}", x);
}
