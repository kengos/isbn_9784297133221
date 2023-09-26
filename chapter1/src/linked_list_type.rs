use std::collections::LinkedList;

pub fn main() {
    instantiate();
    add();
    change();
    remove();
}

fn instantiate() {
    let string_list = LinkedList::<String>::new();
    println!("len() = {}", string_list.len());
    println!("is_empty() = {}", string_list.is_empty());
}

fn add() {
    let mut list_a = LinkedList::<String>::new();
    list_a.push_back("ABC".to_owned());
    list_a.push_back(String::from("DEF"));
    println!("list_a = {:?}", list_a);

    let mut list_b = LinkedList::<String>::new();
    list_b.push_back(String::from("OPQ"));
    list_b.push_back(String::from("RST"));
    list_a.append(&mut list_b);
    println!("list_a = {:?}", list_a);

    list_a.push_front(String::from("ABC"));
    println!("list_a = {:?}", list_a);
}

fn change() {
    let mut list_a = LinkedList::<u32>::new();
    list_a.extend([10, 20, 30, 40, 50, 60, 70, 80, 90]);
    for v in list_a.iter_mut() {
        if *v % 4 == 0 {
            *v *= 10;
        }
    }
    println!("list_a = {:?}", list_a);

    match list_a.back_mut() {
        None => {}
        Some(x) => *x *= 5,
    }
    println!("list_a = {:?}", list_a);
}

fn remove() {
    let mut list_a = LinkedList::<u32>::new();
    list_a.extend([10, 20, 30, 40, 50, 60, 70, 80, 90]);
    let r = list_a.pop_front();
    println!(
        "list_a.pop_front() = {}, list_a = {:?}",
        r.unwrap_or_default(),
        list_a
    );
    let r = list_a.pop_back();
    println!(
        "list_a.pop_back() = {}, list_a = {:?}",
        r.unwrap_or_default(),
        list_a
    );

    list_a.clear();
    println!("list_a.clear() = {:?}", list_a);
}
