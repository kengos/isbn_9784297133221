use anyhow::Result;

pub fn main() {
    use_rectangle();
}

trait Calculator {
    fn calc(&self) -> Result<u64> {
        todo!("まだ実装されていません。");
    }
}

struct Rectangle {
    width: u64,
    height: u64,
}

impl Calculator for Rectangle {
    fn calc(&self) -> Result<u64> {
        Ok(self.height * self.width)
    }
}

fn use_rectangle() {
    let r = Rectangle {
        width: 100,
        height: 50,
    };
    let result = r.calc();
    println!("面積 = {}", result.unwrap());
}
