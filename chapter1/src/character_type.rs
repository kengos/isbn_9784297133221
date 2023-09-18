pub fn main() {
    char_literal();
    char_constant();
    methods();
}

fn char_literal() {
    println!("'a' = {}", 'a');
    println!("'b' = {}", 'b');
    println!("'あ' = {}", 'あ');
    println!("'い' = {}", 'い');
}

fn char_constant() {
    println!("char::MAX = {:?}", char::MAX);
    println!("char::UNICODE_VERSION = {:?}", char::UNICODE_VERSION);
}

fn methods() {
    println!(
        "#is_ascii_alphabetic, 'a' = {}, '0' = {}, 'あ' = {}",
        'a'.is_ascii_alphabetic(),
        '0'.is_ascii_alphabetic(),
        'あ'.is_ascii_alphabetic()
    );
    println!(
        "#is_numeric, 'a' = {}, '0' = {}, 'あ' = {}",
        'a'.is_numeric(),
        '0'.is_numeric(),
        'あ'.is_numeric()
    );
    println!(
        "#is_lowercase, 'a' = {}, '0' = {}, 'A' = {}",
        'a'.is_lowercase(),
        '0'.is_lowercase(),
        'A'.is_lowercase()
    );
    println!(
        "#is_uppercase, 'a' = {}, '0' = {}, 'A' = {}",
        'a'.is_uppercase(),
        '0'.is_uppercase(),
        'A'.is_uppercase()
    );
    println!(
        "#to_ascii_lowercase, 'a' = {}, '0' = {}, 'A' = {}",
        'a'.to_ascii_lowercase(),
        '0'.to_ascii_lowercase(),
        'A'.to_ascii_lowercase()
    );
    println!(
        "#to_ascii_uppercase, 'a' = {}, '0' = {}, 'A' = {}",
        'a'.to_ascii_uppercase(),
        '0'.to_ascii_uppercase(),
        'A'.to_ascii_uppercase()
    );
}
