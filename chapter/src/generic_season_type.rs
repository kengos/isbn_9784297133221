pub fn main() {
    get_months();
}

#[derive(Debug, Clone)]
enum Season<T> {
    Spring(u8, T),
    Summer(u8, T),
    Autumn(u8, T),
    Winter(u8, T),
}

impl<T> Season<T> {
    pub fn get_months(&self) -> &T
    where
        T: std::iter::IntoIterator,
    {
        match self {
            Self::Spring(_, months) => months,
            Self::Summer(_, months) => months,
            Self::Autumn(_, months) => months,
            Self::Winter(_, months) => months,
        }
    }
}

fn get_months() {
    use std::collections::LinkedList;

    let spring = Season::Spring(3, vec!["3月", "4月", "5月"]);
    println!("春: {:?}", spring.get_months());
    let summer = Season::Summer(3, ["6月", "7月", "8月"]);
    println!("夏: {:?}", summer.get_months());
    let autumn = Season::Autumn(3, LinkedList::from(["9月", "10月", "11月"]));
    println!("秋: {:?}", autumn.get_months());
    let winter: Season<LinkedList<&str>> =
        Season::Winter(3, LinkedList::from(["12月", "1月", "2月"]));
    println!("冬: {:?}", winter.get_months());
}
