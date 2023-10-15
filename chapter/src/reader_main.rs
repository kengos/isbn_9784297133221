use crate::csv_reader::CsvReaderImpl;
use crate::entities::Product;

pub fn main() {
    let csv_path = concat!(env!("CARGO_MANIFEST_DIR"), "resources/products.csv");
    let csv_result = CsvReaderImpl::<Product>::read(csv_path).unwrap();
    println!("<<CSV>>");
    for result in csv_result {
        println!("{:?}", result);
    }
}
