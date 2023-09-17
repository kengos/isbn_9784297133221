pub fn main() {
    block_and_scope();
}

fn block_and_scope() {
    {
        let mut total = 0;
        for i in 1..10 {
            total += i;
        }
        println!("total = {}", total);
    }
    // the binding `total` is available in a different scope in the same functio
    // println!("total = {}", total);
}
