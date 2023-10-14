pub fn main() {
    loop_1();
}

fn enter_station() -> String {
    println!("駅名を入力してください");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().to_owned()
}

fn loop_1() {
    let stations = ["品川", "大崎", "五反田", "目黒", "恵比寿", "渋谷"];
    loop {
        let station = enter_station();
        if station.eq("end") {
            println!("終了しました。");
            break;
        }

        if !stations.contains(&station.as_str()) {
            println!("駅名: {} は存在しません。", &station);
            continue;
        }

        let mut count = 1;
        for s in stations {
            if s.ne(&station) {
                count += 1;
                continue;
            } else {
                break;
            }
        }

        println!("駅名: {} は {} 番目の駅です。", &station, count);
    }
}
