pub fn main() {
    println!("----- get ------");
    get();
    println!("----- range -----");
    range();
    println!("----- multibyte_slice -----");
    multibyte_slice();
    println!("----- fat_pointer -----");
    fat_pointer();
    println!("----- methods_1 -----");
    methods_1();
    println!("----- methods_2 -----");
    methods_2();
    println!("----- methods_3 -----");
    methods_3();
}

fn get() {
    let str_array = ["ABC", "DEF", "GHI", "JKL", "MNO", "PQR", "STU"];
    let slice1: &[&str] = &str_array[3..=5];
    println!("slice1 = {:?}", slice1);
    let slice2: &[&str] = &str_array[0..2];
    println!("slice2 = {:?}", slice2);
    let slice3: &[&str] = &str_array[..];
    println!("slice3 = {:?}", slice3);
}

fn range() {
    let int_array = [1, 2, 3, 4, 5, 6, 7];
    let range = std::ops::Range { start: 1, end: 3 };
    let slice = &int_array[range];
    println!("slice = {:?}", slice);
}

fn multibyte_slice() {
    let company_name = "株式会社フルネス";
    let slice = &company_name[..12];
    println!("参照範囲 = {:?}, 大きさ = {}", slice, slice.len());
    let slice: &str = &company_name[12..];
    println!("参照範囲 = {:?}, 大きさ = {}", slice, slice.len());
}

fn fat_pointer() {
    let int_array = [0, 1, 2, 3, 4, 5, 6];
    let slice: &[i32] = &int_array[0..];
    println!(
        "参照範囲 = {:?}, ポインタ = {:p}, 要素数 = {}",
        slice,
        slice,
        slice.len()
    );
    let slice: &[i32] = &int_array[3..5];
    println!(
        "参照範囲 = {:?}, ポインタ = {:p}, 要素数 = {}",
        slice,
        slice,
        slice.len()
    );
}

fn methods_1() {
    let arr = [100, 101, 102, 103, 104];
    let slice: &[i32] = &arr[2..];
    println!("first() = {:?}", slice.first().unwrap());
    println!("last() = {:?}", slice.last().unwrap());
    println!("get(2) = {:?}", slice.get(2).unwrap());
    println!("is_empty() = {:?}", slice.is_empty());
    println!("len() = {:?}", slice.len());
}

fn methods_2() {
    let mut arr = [103, 101, 100, 104, 102];
    let slice: &mut [i32] = &mut arr[..];
    slice.reverse();
    println!("reverse() = {:?}", slice);
    let slice: &mut [i32] = &mut arr[..];
    slice.sort();
    println!("sort() = {:?}", slice);
}

fn methods_3() {
    let v = vec!["abc", "def", "hij", "rst", "uvw", "xyz"];
    let slice = &v[..];
    let chks = slice.chunks(3);
    for chk in chks {
        println!("chunks() = {:?}", chk);
    }
    let r = slice.join("/");
    println!("join() = {:?}", r);
    let it = slice.iter();
    println!("iter() = {:?}", it);
    let v2 = slice.to_vec();
    println!("to_vec = {:?}", v2);
    let arr = [100, 101, 102, 103, 104];
    let slice = &arr[..];
    let spts = slice.split(|value| value % 4 == 0);
    for spt in spts {
        println!("split() = {:?}", spt);
    }
}
