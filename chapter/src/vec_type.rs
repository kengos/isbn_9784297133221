pub fn main() {
    instantiate();
    add_1();
    get_and_change();
    remove();
    sort();
}

fn instantiate() {
    let v = Vec::<i32>::with_capacity(5);
    println!(
        "ポインタ = {:p}, 長さ = {:?}, 容量 = {}",
        &v,
        v,
        v.capacity()
    );

    let v = vec![(10, 20), (100, 200)];
    println!(
        "ポインタ = {:p}, 長さ = {:?}, 容量 = {}",
        &v,
        v,
        v.capacity()
    );
}

fn add_1() {
    let mut v = Vec::<i32>::with_capacity(5);
    println!(
        "before\n内容 = {:?}, 長さ = {}, キャパシティ = {}",
        v,
        v.len(),
        v.capacity()
    );
    for i in 0..10 {
        v.push(i);
    }
    println!(
        "after\n内容 = {:?}, 長さ = {}, キャパシティ = {}",
        v,
        v.len(),
        v.capacity()
    );

    let mut v = vec![(10, 20), (100, 200)];
    println!(
        "before\n内容 = {:?}, 長さ = {}, キャパシティ = {}",
        v,
        v.len(),
        v.capacity()
    );
    v.insert(1, (1000, 2000));
    println!(
        "after\n内容 = {:?}, 長さ = {}, キャパシティ = {}",
        v,
        v.len(),
        v.capacity()
    );
}

fn get_and_change() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("v[1] = {}", v[1]);
    v[2] *= 100;
    println!("v[2] = {:?}", v);
}

fn remove() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let x: Vec<i32> = nums.drain(2..5).collect();
    println!("num.drain(2..5) = {:?}", x);
    println!("nums = {:?}", nums);

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut index = 0;
    while index < nums.len() {
        if nums[index] % 2 == 1 {
            let num = nums.remove(index);
            println!("削除した要素 = {}, index = {}", num, index);
        }
        index += 1;
    }
    println!("nums = {:?}", nums);
}

fn sort() {
    let mut nums = vec![8, 5, 3, 1, 4, 6, 2, 5, 9, 2];
    nums.sort();
    println!("nums.sort() = {:?}", nums);
    nums.dedup();
    println!("nums.dedup() = {:?}", nums);
    use std::cmp::Reverse;
    let mut nums = vec![8, 5, 3, 1, 4, 6, 2, 5, 9, 2];
    nums.sort_by_key(|e: &i32| Reverse(*e));
    println!("nums.sort_by_key() = {:?}", nums);
}
