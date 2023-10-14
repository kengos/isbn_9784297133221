pub fn main() {
    use_season();
}

#[derive(Debug)]
enum Season<'a> {
    Spring(u8, Vec<&'a str>),
    Summer(u8, Vec<&'a str>),
    Autumn(u8, Vec<&'a str>),
    Winter(u8, Vec<&'a str>),
}

impl<'a> Season<'a> {
    pub fn format_variant(&self) -> String {
        match self {
            Self::Spring(x, y) => format!("春: {}ヶ月 {:?}", x, y),
            Self::Summer(x, y) => format!("夏: {}ヶ月 {:?}", x, y),
            Self::Autumn(x, y) => format!("秋: {}ヶ月 {:?}", x, y),
            Self::Winter(x, y) => format!("冬: {}ヶ月 {:?}", x, y),
        }
    }
}

fn use_season() {
    let spring = Season::Spring(3, vec!["3月", "4月", "5月"]);
    let summer = Season::Summer(3, vec!["6月", "7月", "8月"]);
    let autumn = Season::Autumn(3, vec!["9月", "10月", "11月"]);
    let winter = Season::Winter(3, vec!["12月", "1月", "2月"]);
    println!("{}", spring.format_variant());
    println!("{}", summer.format_variant());
    println!("{}", autumn.format_variant());
    println!("{}", winter.format_variant());
}
