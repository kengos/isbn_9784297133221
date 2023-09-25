use chrono::prelude::*;
use chrono_tz;
use std::time::SystemTime;

pub fn main() {
    instantiate();
    format();
    from_string();
    get();
    change();
    time_zone();
    unix_epoch();
}

fn instantiate() {
    let now: DateTime<Utc> = Utc::now();
    println!("UTC日時 = {}", &now);
    let now: DateTime<Local> = Local::now();
    println!("ローカル日時 = {}", &now);
}

fn format() {
    let now: DateTime<Utc> = Utc::now();
    let format_date = now.format("%Y年%m月%d日").to_string();
    println!("{:?}", &format_date);

    let now: DateTime<Local> = Local::now();
    let format_date = now.format("%Y年%m月%d日 %H時%M分%S秒").to_string();
    println!("{}", format_date);
}

fn from_string() {
    let rfc2822_type = DateTime::parse_from_rfc2822("Fri, 14 Jan 2022 10:52:37 +0200");
    println!("{}", rfc2822_type.unwrap());

    let rfc3339_type = DateTime::parse_from_rfc3339("2022-01-14T12:00:00-08:00");
    println!("{}", rfc3339_type.unwrap());

    let time_only = NaiveTime::parse_from_str("15:30:00", "%H:%M:%S");
    println!("{}", time_only.unwrap());

    let date_only = NaiveDate::parse_from_str("2022年10月14日", "%Y年%m月%d日");
    println!("{}", date_only.unwrap());

    let custom_format = NaiveDate::parse_from_str("10 2022 14", "%m %Y %d");
    println!("{}", custom_format.unwrap());
}

fn get() {
    let now = Utc::now();
    println!("y = {}, m = {}, d = {}", now.year(), now.month(), now.day());
    println!(
        "h = {}, m = {}, s = {}, n = {}",
        now.hour(),
        now.minute(),
        now.second(),
        now.nanosecond()
    );

    let w = match now.weekday() {
        Weekday::Mon => "月曜日",
        Weekday::Tue => "火曜日",
        Weekday::Wed => "水曜日",
        Weekday::Thu => "木曜日",
        Weekday::Fri => "金曜日",
        Weekday::Sat => "土曜日",
        Weekday::Sun => "日曜日",
    };

    println!("曜日 = {}", w);
}

fn change() {
    let now = Utc::now();
    println!("取得した日時 = {}", now);

    let change = now.with_day(25);
    println!("日を変更 = {}", change.unwrap());

    let change = now.with_month(10);
    println!("月を変更 = {}", change.unwrap());

    let change = now.with_year(2010);
    println!("年を変更 = {}", change.unwrap());
}

fn time_zone() {
    let now = Local::now();
    let tokyo = now.with_timezone(&chrono_tz::Asia::Tokyo);
    println!("東京 = {}", tokyo);

    let chicago = now.with_timezone(&chrono_tz::America::Chicago);
    println!("シカゴ = {}", chicago);

    let tokyo_n = tokyo.naive_local();
    let chicago_n = chicago.naive_local();
    println!("{}", tokyo_n);
    println!("{}", chicago_n);

    let duration = tokyo_n - chicago_n;
    println!("時間数 = {}", duration.num_hours());
    println!("秒数 = {}", duration.num_seconds());
    println!("ナノ秒数 = {}", duration.num_nanoseconds().unwrap());
}

fn unix_epoch() {
    let x = Local::now().timestamp();
    println!("Localで取得 = {}", x);

    let y = Utc::now().timestamp();
    println!("Utcで取得 = {}", y);

    let z = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    println!("SystemTimeで取得 = {}", z.unwrap().as_secs());
}
