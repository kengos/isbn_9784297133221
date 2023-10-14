pub fn main() {
    instantiate();
    add();
    replace();
    remove();
    find();
    matching();
    get();
}

fn instantiate() {
    let s1 = String::new();
    println!(
        "new() = {:?}, len = {}, capacity = {}",
        &s1,
        &s1.len(),
        &s1.capacity()
    );
    let s2 = String::from("Hello Rust.");
    println!(
        "from() = {:?}, len = {}, capacity = {}",
        &s2,
        &s2.len(),
        &s2.capacity()
    );
    let s3 = String::with_capacity(5);
    println!(
        "with_capacity(5) = {:?}, len = {}, capacity = {}",
        &s3,
        &s3.len(),
        &s3.capacity()
    );
}

fn add() {
    let mut s = String::from("abc");
    s.push('d');
    println!("push() = {:?}", &s);
    s.insert(1, 'd');
    println!("insert() = {:?}", &s);
    let mut s = String::from("山田太郎");
    s.push_str(",東京都新宿区");
    println!("push_str() = {:?}", &s);
    let mut s = String::from("山田太郎");
    s.insert_str(12, ",東京都新宿区");
    println!("insert_str() = {:?}", &s);
}

fn replace() {
    let s = String::from("山田太郎 , 山崎花子");
    let r = &s.replace("山", "吉");
    println!("replace() = {:?}", &r);
    let s = String::from("山田太郎 , 山崎花子");
    let r = &s.replacen("山", "吉", 1);
    println!("replace() = {:?}", &r);
    let mut s = String::from("山田太郎");
    let offset = &s.find("太").unwrap_or(s.len());
    s.replace_range(..offset, "鈴木");
    println!("replace_range() = {:?}", &s);
}

fn remove() {
    let mut s = String::from("abcdefgxyz");
    s.clear();
    println!("値 = {:?}, 容量 = {:?}", &s, &s.capacity());
    let mut s = String::from("abcdefgxyz");
    let r = &s.drain(1..3);
    println!("r = {:?}", &r.as_str());
    let mut s = String::from("abcdefgxyz");
    let r = &s.pop();
    println!("s = {:?}, r = {:?}", &s, &r.unwrap());
    let mut s = String::from("abcdefgxyz");
    let r = s.remove(3);
    println!("s = {:?}, r = {:?}", &s, &r);
}

fn find() {
    let find_result = |ret: Option<usize>, method_name: &str| {
        if ret.is_none() {
            println!("{} = パターンが見つかりません。", &method_name);
        } else {
            println!("{} = {}番目で見つかりました。", &method_name, &ret.unwrap());
        }
        ret
    };
    let s = String::from("abcdefgxyz_xyz");
    find_result(s.find("xyz"), "find()");
    find_result(s.rfind("xyz"), "rfind()");
}

fn matching() {
    let s = String::from("abcdefgxyz_xyz");
    let r: Vec<_> = s.matches("xyz").collect();
    if r.is_empty() {
        println!("matches() = パターンが見つかりません。");
    } else {
        for value in r {
            println!("matches() = {:?}", &value);
        }
    }
    let r: Vec<_> = s.match_indices("xyz").collect();
    if r.is_empty() {
        println!("matches() = パターンが見つかりません。");
    } else {
        for value in r {
            println!("match_indices() = {:?}, {:?}", &value.0, &value.1);
        }
    }
}

fn get() {
    let s = String::from("abcdefgxyz");
    let r = &s.get(0..3);
    println!("get(0..3) = {:?}", &r.unwrap());
    let mut s = String::from("abcdefgxyz");
    let r = s.get_mut(0..5);
    println!("get(0..5) = {:?}", &r.unwrap());
}
