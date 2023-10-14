use std::fmt::Display;
use std::fmt::Formatter;

#[repr(u32)]
#[derive(Debug)]
enum Season {
    Spring = 100,
    Summer = 200,
    Autumn,
    Winter,
}

impl Display for Season {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Spring => write!(f, "Sprint(春) : {}", Self::Spring as u32),
            Self::Summer => write!(f, "Sprint(夏) : {}", Self::Summer as u32),
            Self::Autumn => write!(f, "Sprint(秋) : {}", Self::Autumn as u32),
            Self::Winter => write!(f, "Sprint(冬) : {}", Self::Winter as u32),
        }
    }
}
pub fn main() {
    println!("----- use_season() -----");
    use_season();
    println!("----- use_fmt() -----");
    use_fmt();
    println!("----- use_repl() -----");
    use_repl();
}

fn use_season() {
    let summer = Season::Summer;
    let winter = Season::Winter;
    println!("{:?}", summer);
    println!("{:?}", winter);
    let spring_num = Season::Spring as u32;
    let autumn_num = Season::Autumn as u32;
    println!("{:?}", spring_num);
    println!("{:?}", autumn_num);
}

fn use_fmt() {
    println!("{}", Season::Spring);
    println!("{}", Season::Summer);
    println!("{}", Season::Autumn);
    println!("{}", Season::Winter);
}

fn use_repl() {
    println!("{}", Season::Spring);
    println!("{}", Season::Summer);
    println!("{}", Season::Autumn);
    println!("{}", Season::Winter);
}
